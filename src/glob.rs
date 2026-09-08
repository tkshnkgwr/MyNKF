//! ワイルドカード展開およびファイルサイズ整形モジュール。

/// ワイルドカード展開時に一度に処理できる最大ファイル数（100ファイル）。
///
/// メモリ消費およびCPU負荷の急増を防ぐための安全上限値です。
pub const MAX_GLOB_FILES: usize = 100;

/// ワイルドカードパターン（`*` や `?`）が指定されたテキストにマッチするか判定します。
///
/// - `*`: 0文字以上の任意の文字列にマッチ
/// - `?`: 任意の1文字にマッチ
///
/// # 引数
///
/// * `pattern` - ワイルドカードパターン（例: `*.txt`, `a?c.txt`）
/// * `text` - マッチング対象のテキスト
///
/// # 戻り値
///
/// マッチした場合は `true`、そうでない場合は `false` を返します。
///
/// # 例
/// ```rust
/// use mynkf::wildcard_match;
///
/// assert!(wildcard_match("*.txt", "readme.txt"));
/// assert!(wildcard_match("test?.rs", "test1.rs"));
/// assert!(!wildcard_match("*.txt", "image.png"));
/// ```
pub fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    let mut p_idx = 0;
    let mut t_idx = 0;
    let mut p_star = None;
    let mut t_star = None;

    while t_idx < text_chars.len() {
        if p_idx < pattern_chars.len()
            && (pattern_chars[p_idx] == '?' || pattern_chars[p_idx] == text_chars[t_idx])
        {
            p_idx += 1;
            t_idx += 1;
        } else if p_idx < pattern_chars.len() && pattern_chars[p_idx] == '*' {
            p_star = Some(p_idx);
            t_star = Some(t_idx);
            p_idx += 1;
        } else if let Some(star) = p_star {
            p_idx = star + 1;
            t_star = Some(t_star.unwrap() + 1);
            t_idx = t_star.unwrap();
        } else {
            return false;
        }
    }

    while p_idx < pattern_chars.len() && pattern_chars[p_idx] == '*' {
        p_idx += 1;
    }

    p_idx == pattern_chars.len()
}

/// 指定された引数（ワイルドカードを含む場合があるパス）を展開し、ファイルリストに追加します。
///
/// パターンマッチによりファイル名部分が一致するローカルファイルを探して `files` に格納します。
/// 最大ファイル数 `MAX_GLOB_FILES` を超えた場合はエラーを返します。
///
/// # 引数
///
/// * `arg` - 展開対象のパスパターン
/// * `files` - 展開されたファイル名が追加されるベクター
///
/// # 戻り値
///
/// 成功した場合は `Ok(())`、ファイル数上限超過やディレクトリの読み込みに失敗した場合は `Err` を返します。
pub fn expand_wildcard(arg: &str, files: &mut Vec<String>) -> Result<(), String> {
    if !arg.contains('*') && !arg.contains('?') {
        files.push(arg.to_string());
        if files.len() > MAX_GLOB_FILES {
            return Err(format!(
                "Maximum limit of {} files exceeded.",
                MAX_GLOB_FILES
            ));
        }
        return Ok(());
    }

    let path = std::path::Path::new(arg);
    let parent_dir = path.parent().unwrap_or_else(|| std::path::Path::new(""));
    let file_pattern = match path.file_name().and_then(|f| f.to_str()) {
        Some(p) => p,
        None => {
            files.push(arg.to_string());
            return Ok(());
        }
    };

    let dir_to_read = if parent_dir.as_os_str().is_empty() {
        std::path::Path::new(".")
    } else {
        parent_dir
    };

    let entries = match std::fs::read_dir(dir_to_read) {
        Ok(e) => e,
        Err(err) => {
            return Err(format!(
                "Failed to read directory '{:?}': {}",
                dir_to_read, err
            ));
        }
    };

    let mut matched_any = false;
    for entry in entries.flatten() {
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if file_type.is_file()
            && let Some(name_str) = entry.file_name().to_str()
            && wildcard_match(&file_pattern.to_lowercase(), &name_str.to_lowercase())
        {
            let matched_path = if parent_dir.as_os_str().is_empty() {
                std::path::PathBuf::from(entry.file_name())
            } else {
                parent_dir.join(entry.file_name())
            };
            if let Some(path_str) = matched_path.to_str() {
                files.push(path_str.to_string());
                matched_any = true;
                if files.len() > MAX_GLOB_FILES {
                    return Err(format!(
                        "Maximum limit of {} files exceeded.",
                        MAX_GLOB_FILES
                    ));
                }
            }
        }
    }

    if !matched_any {
        files.push(arg.to_string());
    }

    Ok(())
}

/// 指定されたバイト数を読みやすい形式（B, KB, MB, GB）に整形します。
///
/// 1024バイト単位で計算し、小数点以下1桁で丸めて出力します。
///
/// # 引数
///
/// * `bytes` - 整形対象のバイト数（`usize`）
///
/// # 戻り値
///
/// 整形されたサイズの文字列（例: `"100 B"`, `"1.2 KB"`, `"15.4 MB"`, `"2.1 GB"`）を返します。
///
/// # 例
/// ```rust
/// use mynkf::format_size;
///
/// assert_eq!(format_size(500), "500 B");
/// assert_eq!(format_size(1536), "1.5 KB");
/// assert_eq!(format_size(10 * 1024 * 1024), "10.0 MB");
/// ```
pub fn format_size(bytes: usize) -> String {
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;
    let bytes_f = bytes as f64;

    if bytes_f >= gb {
        format!("{:.1} GB", bytes_f / gb)
    } else if bytes_f >= mb {
        format!("{:.1} MB", bytes_f / mb)
    } else if bytes_f >= kb {
        format!("{:.1} KB", bytes_f / kb)
    } else {
        format!("{} B", bytes)
    }
}
