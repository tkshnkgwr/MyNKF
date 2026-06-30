// =========================================================================
// MyNKF GUI Edition
// Powered by eframe / egui
// UPDATE 2026-06-29: Windows10/11低リソース環境用・超軽量・高審美性文字コードコンバータ
// =========================================================================

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};
use mynkf::*;
use eframe::egui;

// Windows用のネイティブ関数と二重起動防止処理の自前バインディング (低フットプリント)
#[cfg(target_os = "windows")]
#[allow(clippy::upper_case_acronyms)]
mod win32 {
    use std::ffi::c_void;
    pub type HWND = *mut c_void;
    pub type HANDLE = *mut c_void;
    pub type HRESULT = i32;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn CreateMutexW(
            lpMutexAttributes: *const c_void,
            bInitialOwner: i32,
            lpName: *const u16,
        ) -> HANDLE;
        pub fn GetLastError() -> u32;
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        pub fn GetActiveWindow() -> HWND;
    }

    #[link(name = "dwmapi")]
    unsafe extern "system" {
        pub fn DwmSetWindowAttribute(
            hwnd: HWND,
            dwAttribute: u32,
            pvAttribute: *const c_void,
            cbAttribute: u32,
        ) -> HRESULT;
    }

    pub const ERROR_ALREADY_EXISTS: u32 = 183;
    pub const DWMWA_NCRENDERING_POLICY: u32 = 2;
    pub const DWMNCRP_DISABLED: i32 = 1;
}

#[cfg(target_os = "windows")]
fn prevent_double_start() {
    unsafe {
        let name: Vec<u16> = "Global\\MyNKF_GUI_SingleInstance_Mutex\0".encode_utf16().collect();
        let handle = win32::CreateMutexW(std::ptr::null(), 1, name.as_ptr());
        if handle.is_null() {
            std::process::exit(1);
        }
        if win32::GetLastError() == win32::ERROR_ALREADY_EXISTS {
            std::process::exit(0);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn prevent_double_start() {}

#[cfg(target_os = "windows")]
fn remove_shadow_and_borders() {
    unsafe {
        let hwnd = win32::GetActiveWindow();
        if !hwnd.is_null() {
            let policy = win32::DWMNCRP_DISABLED;
            win32::DwmSetWindowAttribute(
                hwnd,
                win32::DWMWA_NCRENDERING_POLICY,
                &policy as *const _ as *const std::ffi::c_void,
                std::mem::size_of::<i32>() as u32,
            );
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn remove_shadow_and_borders() {}

#[derive(Debug, Clone)]
struct FileItem {
    path: PathBuf,
    name: String,
    size: usize,
    current_encoding: Encoding,
    current_line_ending: LineEnding,
    status: String,
    success: Option<bool>,
}

#[derive(PartialEq, Eq)]
enum Tab {
    FileConversion,
    TextConversion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineEndingOption {
    Lf,
    Crlf,
    AsIs,
}

struct MyNkfGuiApp {
    table: Vec<u16>,
    unicode_to_jis: HashMap<u16, u16>,
    first_frame: bool,
    current_tab: Tab,
    
    // ファイル一括変換タブ
    files: Vec<FileItem>,
    target_encoding: Encoding,
    target_line_ending: LineEndingOption,
    
    // テキスト変換タブ
    input_text: String,
    text_target_encoding: Encoding,
    text_target_line_ending: LineEndingOption,
}

impl MyNkfGuiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // フォントの設定 (日本語表示対応)
        let mut fonts = egui::FontDefinitions::default();
        let font_paths = [
            "C:\\Windows\\Fonts\\meiryo.ttc",
            "C:\\Windows\\Fonts\\msgothic.ttc",
            "C:\\Windows\\Fonts\\msmincho.ttc",
        ];
        
        let mut loaded = false;
        for path in &font_paths {
            if let Ok(data) = std::fs::read(path) {
                fonts.font_data.insert(
                    "japanese_font".to_owned(),
                    egui::FontData::from_owned(data),
                );
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
                    .insert(0, "japanese_font".to_owned());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
                    .insert(0, "japanese_font".to_owned());
                loaded = true;
                break;
            }
        }
        
        if loaded {
            cc.egui_ctx.set_fonts(fonts);
        }

        // ビジュアル（ダークテーマ）の設定
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 36);
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 205));
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 53);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 60, 72);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 80, 96);
        visuals.window_rounding = 8.0.into();
        cc.egui_ctx.set_visuals(visuals);

        // テーブルの構築
        let table = load_jis_table();
        let mut unicode_to_jis = HashMap::new();
        for (idx, &uni) in table.iter().enumerate() {
            if uni != 0 {
                unicode_to_jis.insert(uni, idx as u16);
            }
        }

        Self {
            table,
            unicode_to_jis,
            first_frame: true,
            current_tab: Tab::FileConversion,
            files: Vec::new(),
            target_encoding: Encoding::Utf8,
            target_line_ending: LineEndingOption::Lf,
            input_text: String::new(),
            text_target_encoding: Encoding::Utf8,
            text_target_line_ending: LineEndingOption::Lf,
        }
    }

    fn add_file_paths(&mut self, paths: &[PathBuf]) {
        for path in paths {
            if path.is_file() && let Ok(mut file) = File::open(path) {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    let guessed = guess_encoding(&buffer);
                    let size = buffer.len();
                    let ending = detect_line_ending(&buffer);
                    
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")
                        .to_string();

                    // 重複チェック
                    if !self.files.iter().any(|f| f.path == *path) {
                        self.files.push(FileItem {
                            path: path.clone(),
                            name,
                            size,
                            current_encoding: guessed,
                            current_line_ending: ending,
                            status: "待機中".to_string(),
                            success: None,
                        });
                    }
                }
            }
        }
    }
}

impl eframe::App for MyNkfGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.first_frame {
            remove_shadow_and_borders();
            self.first_frame = false;
        }

        // ドラッグ＆ドロップファイルの監視
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let paths: Vec<PathBuf> = i.raw.dropped_files.iter()
                    .filter_map(|f| f.path.clone())
                    .collect();
                self.add_file_paths(&paths);
            }
        });

        // 影と枠の無いフラットでモダンなカスタムウィンドウの描画
        let window_frame = egui::containers::Frame {
            inner_margin: egui::Margin::same(12.0),
            rounding: egui::Rounding::same(10.0),
            shadow: egui::epaint::Shadow::NONE,
            fill: egui::Color32::from_rgb(26, 26, 32),
            stroke: egui::Stroke::new(1.5, egui::Color32::from_rgb(70, 70, 85)),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(window_frame).show(ctx, |ui| {
            // ==========================================
            // 1. 自作タイトルバー (ドラッグ可能領域)
            // ==========================================
            let title_bar_height = 24.0;
            let (title_rect, _) = ui.allocate_at_least(
                egui::vec2(ui.available_width(), title_bar_height),
                egui::Sense::hover()
            );

            // 背後にドラッグ判定を設定。これでタイトルバー全体の余白を掴んでもドラッグ移動可能になる。
            // ※前面のボタンが最優先でイベントを処理するため、ボタンのクリックとは競合しません。
            let drag_interact = ui.interact(title_rect, ui.id().with("window_drag"), egui::Sense::drag());
            if drag_interact.dragged() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            // タイトルバー領域内にコンテンツを重ねて描画
            ui.allocate_ui_at_rect(title_rect, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing.x = 8.0;
                    
                    // アイコン風飾り
                    ui.colored_label(egui::Color32::from_rgb(0, 191, 255), "⚡");
                    
                    // タイトルテキスト
                    ui.label(egui::RichText::new("MyNKF GUI 文字コードコンバータ")
                        .strong()
                        .color(egui::Color32::from_rgb(230, 230, 240))
                        .size(15.0));

                    // バージョン情報 (Cargo.tomlから動的ロード)
                    ui.label(egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .color(egui::Color32::from_rgb(120, 120, 140))
                        .size(12.0));

                    // 右寄せで閉じる・最小化ボタンを描画
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 閉じるボタン
                        let close_btn = ui.add(egui::Button::new(
                            egui::RichText::new(" ❌ ").color(egui::Color32::from_rgb(250, 100, 100))
                        ).fill(egui::Color32::TRANSPARENT));
                        if close_btn.clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        // 最小化ボタン
                        let min_btn = ui.add(egui::Button::new(
                            egui::RichText::new(" ➖ ").color(egui::Color32::from_rgb(200, 200, 210))
                        ).fill(egui::Color32::TRANSPARENT));
                        if min_btn.clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    });
                });
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // ==========================================
            // 2. タブナビゲーション
            // ==========================================
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::FileConversion, "📁 ファイル一括変換");
                ui.selectable_value(&mut self.current_tab, Tab::TextConversion, "📝 テキスト直接変換");
            });

            ui.add_space(8.0);

            // ==========================================
            // 3. コンテンツエリア
            // ==========================================
            match self.current_tab {
                Tab::FileConversion => {
                    // -- ファイル一括変換タブ --
                    
                    // 操作パネル (上部)
                    ui.horizontal(|ui| {
                        if ui.button("➕ ファイルを追加").clicked()
                            && let Some(paths) = rfd::FileDialog::new()
                                .set_title("変換するファイルを選択")
                                .pick_files()
                        {
                            self.add_file_paths(&paths);
                        }

                        if ui.button("🗑️ リストをクリア").clicked() {
                            self.files.clear();
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(format!("ファイル数: {}", self.files.len())).weak());
                        });
                    });

                    ui.add_space(6.0);

                    // ドラッグ＆ドロップエリア・リスト表示
                    let list_height = ui.available_height() - 110.0;
                    
                    if self.files.is_empty() {
                        // ファイルが空の時のドラッグ＆ドロップガイド
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(32, 32, 40))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 75)))
                            .rounding(6.0)
                            .inner_margin(egui::Margin::same(20.0))
                            .show(ui, |ui| {
                                ui.set_min_height(list_height);
                                ui.vertical_centered(|ui| {
                                    ui.add_space(list_height / 2.0 - 30.0);
                                    ui.centered_and_justified(|ui| {
                                        ui.vertical(|ui| {
                                            ui.centered_and_justified(|ui| {
                                                ui.label(egui::RichText::new("📁 ここにファイルをドラッグ＆ドロップ")
                                                    .size(16.0)
                                                    .color(egui::Color32::from_rgb(140, 140, 160)));
                                            });
                                            ui.centered_and_justified(|ui| {
                                                ui.label(egui::RichText::new("または「ファイルを追加」ボタンをクリック")
                                                    .weak());
                                            });
                                        });
                                    });
                                });
                            });
                    } else {
                        // テーブルによる一覧表示
                        egui::ScrollArea::vertical().max_height(list_height).show(ui, |ui| {
                            egui::Frame::none()
                                .fill(egui::Color32::from_rgb(22, 22, 27))
                                .rounding(6.0)
                                .inner_margin(egui::Margin::same(4.0))
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    
                                    // ヘッダー行
                                    ui.horizontal(|ui| {
                                        ui.set_min_height(24.0);
                                        ui.add_space(6.0);
                                        ui.horizontal(|ui| {
                                            ui.set_min_width(180.0);
                                            ui.label(egui::RichText::new("ファイル名").strong());
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_min_width(80.0);
                                            ui.label(egui::RichText::new("サイズ").strong());
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_min_width(90.0);
                                            ui.label(egui::RichText::new("元文字コード").strong());
                                        });
                                        ui.horizontal(|ui| {
                                            ui.set_min_width(70.0);
                                            ui.label(egui::RichText::new("元改行").strong());
                                        });
                                        ui.label(egui::RichText::new("状態").strong());
                                    });
                                    
                                    ui.separator();

                                    // データ行
                                    let mut to_remove = None;
                                    for (idx, item) in self.files.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.set_min_height(26.0);
                                            
                                            // 削除ボタン
                                            if ui.add(egui::Button::new("❌").small().fill(egui::Color32::TRANSPARENT)).clicked() {
                                                to_remove = Some(idx);
                                            }

                                            // ファイル名
                                            ui.horizontal(|ui| {
                                                ui.set_min_width(180.0);
                                                ui.label(&item.name).on_hover_text(item.path.to_string_lossy());
                                            });

                                            // サイズ
                                            ui.horizontal(|ui| {
                                                ui.set_min_width(80.0);
                                                ui.label(format_size(item.size));
                                            });

                                            // 文字コード
                                            ui.horizontal(|ui| {
                                                ui.set_min_width(90.0);
                                                let color = match item.current_encoding {
                                                    Encoding::Sjis => egui::Color32::from_rgb(255, 165, 0),
                                                    Encoding::EucJp => egui::Color32::from_rgb(186, 85, 211),
                                                    Encoding::Utf8 => egui::Color32::from_rgb(50, 205, 50),
                                                    _ => egui::Color32::from_rgb(160, 160, 160),
                                                };
                                                ui.colored_label(color, item.current_encoding.as_str());
                                            });

                                            // 改行コード
                                            ui.horizontal(|ui| {
                                                ui.set_min_width(70.0);
                                                ui.label(item.current_line_ending.as_str());
                                            });

                                            // ステータス
                                            let status_color = match item.success {
                                                Some(true) => egui::Color32::from_rgb(50, 205, 50),
                                                Some(false) => egui::Color32::from_rgb(220, 20, 60),
                                                None => egui::Color32::from_rgb(220, 220, 220),
                                            };
                                            ui.colored_label(status_color, &item.status);
                                        });
                                    }

                                    if let Some(idx) = to_remove {
                                        self.files.remove(idx);
                                    }
                                });
                        });
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // 変換設定と実行エリア (下部)
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("出力文字コード:");
                                ui.radio_value(&mut self.target_encoding, Encoding::Utf8, "UTF-8");
                                ui.radio_value(&mut self.target_encoding, Encoding::Sjis, "Shift-JIS");
                                ui.radio_value(&mut self.target_encoding, Encoding::EucJp, "EUC-JP");
                            });
                            ui.horizontal(|ui| {
                                ui.label("出力改行コード:");
                                ui.radio_value(&mut self.target_line_ending, LineEndingOption::Lf, "LF");
                                ui.radio_value(&mut self.target_line_ending, LineEndingOption::Crlf, "CRLF");
                                ui.radio_value(&mut self.target_line_ending, LineEndingOption::AsIs, "元のまま");
                            });
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let execute_btn = ui.add_enabled(
                                !self.files.is_empty(),
                                egui::Button::new(
                                    egui::RichText::new("⚡ 一括変換を実行")
                                        .strong()
                                        .color(egui::Color32::from_rgb(255, 255, 255))
                                ).fill(egui::Color32::from_rgb(0, 128, 192))
                            );

                            if execute_btn.clicked() {
                                // 変換実行
                                for item in &mut self.files {
                                    item.status = "処理中...".to_string();
                                    item.success = None;
                                    
                                    // 読み込み
                                    match File::open(&item.path) {
                                        Ok(mut file) => {
                                            let mut buffer = Vec::new();
                                            if file.read_to_end(&mut buffer).is_ok() {
                                                let guessed = guess_encoding(&buffer);
                                                let unicode = decode_to_unicode(&buffer, guessed, &self.table);
                                                
                                                let actual_crlf = match self.target_line_ending {
                                                    LineEndingOption::Crlf => true,
                                                    LineEndingOption::Lf => false,
                                                    LineEndingOption::AsIs => matches!(item.current_line_ending, LineEnding::Crlf | LineEnding::Cr),
                                                };

                                                let output = encode_from_unicode(
                                                    &unicode,
                                                    self.target_encoding,
                                                    &self.unicode_to_jis,
                                                    actual_crlf
                                                );

                                                // 書き出し (上書き)
                                                match File::create(&item.path) {
                                                    Ok(mut out_file) => {
                                                        if out_file.write_all(&output).is_ok() {
                                                            item.status = "完了".to_string();
                                                            item.success = Some(true);
                                                            // 表示情報の更新
                                                            item.size = output.len();
                                                            item.current_encoding = self.target_encoding;
                                                            item.current_line_ending = if actual_crlf { LineEnding::Crlf } else { LineEnding::Lf };
                                                        } else {
                                                            item.status = "書込エラー".to_string();
                                                            item.success = Some(false);
                                                        }
                                                    }
                                                    Err(_) => {
                                                        item.status = "作成エラー".to_string();
                                                        item.success = Some(false);
                                                    }
                                                }
                                            } else {
                                                item.status = "読込エラー".to_string();
                                                item.success = Some(false);
                                            }
                                        }
                                        Err(_) => {
                                            item.status = "開けません".to_string();
                                            item.success = Some(false);
                                        }
                                    }
                                }
                            }
                        });
                    });
                }
                Tab::TextConversion => {
                    // -- テキスト直接変換タブ --
                    ui.label("テキストを入力または貼り付けしてください:");
                    
                    let text_box_height = ui.available_height() - 110.0;
                    
                    // 入力テキストエリア
                    egui::ScrollArea::vertical().max_height(text_box_height).show(ui, |ui| {
                        ui.add(egui::TextEdit::multiline(&mut self.input_text)
                            .hint_text("ここに変換したい日本語テキストを入力...")
                            .font(egui::TextStyle::Monospace)
                            .desired_width(ui.available_width())
                            .desired_rows(15));
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // 変換設定とコピー
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("出力文字コード:");
                                ui.radio_value(&mut self.text_target_encoding, Encoding::Utf8, "UTF-8");
                                ui.radio_value(&mut self.text_target_encoding, Encoding::Sjis, "Shift-JIS");
                                ui.radio_value(&mut self.text_target_encoding, Encoding::EucJp, "EUC-JP");
                            });
                            ui.horizontal(|ui| {
                                ui.label("出力改行コード:");
                                ui.radio_value(&mut self.text_target_line_ending, LineEndingOption::Lf, "LF");
                                ui.radio_value(&mut self.text_target_line_ending, LineEndingOption::Crlf, "CRLF");
                            });
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let copy_btn = ui.add_enabled(
                                !self.input_text.is_empty(),
                                egui::Button::new(
                                    egui::RichText::new("📋 クリップボードにコピー")
                                        .strong()
                                        .color(egui::Color32::from_rgb(255, 255, 255))
                                ).fill(egui::Color32::from_rgb(0, 128, 192))
                            );

                            if copy_btn.clicked() {
                                let chars: Vec<char> = self.input_text.chars().collect();
                                let actual_crlf = self.text_target_line_ending == LineEndingOption::Crlf;
                                let encoded = encode_from_unicode(
                                    &chars,
                                    self.text_target_encoding,
                                    &self.unicode_to_jis,
                                    actual_crlf
                                );

                                let raw_str = match self.text_target_encoding {
                                    Encoding::Utf8 => String::from_utf8_lossy(&encoded).into_owned(),
                                    Encoding::Sjis => {
                                        let unicode = decode_to_unicode(&encoded, Encoding::Sjis, &self.table);
                                        unicode.into_iter().collect::<String>()
                                    }
                                    Encoding::EucJp => {
                                        let unicode = decode_to_unicode(&encoded, Encoding::EucJp, &self.table);
                                        unicode.into_iter().collect::<String>()
                                    }
                                    _ => self.input_text.clone()
                                };

                                ctx.output_mut(|o| o.copied_text = raw_str);
                            }
                        });
                    });
                }
            }
        });
    }
}

fn main() {
    // 二重起動防止
    prevent_double_start();

    // eframe 0.27 の仕様に合わせて ViewportBuilder を用いた設定を行う
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_inner_size(egui::vec2(680.0, 480.0))
            .with_min_inner_size(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };

    eframe::run_native(
        "MyNKF GUI",
        options,
        Box::new(|cc| Box::new(MyNkfGuiApp::new(cc))),
    ).unwrap();
}
