//! # MyNKF CLI (標準ライブラリ限定版)
//!
//! コマンドラインからファイルの日本語文字コード（UTF-8, Shift_JIS, EUC-JP）検出および相互変換を行うツールです。
//!
//! ## 主な特徴
//! - **超軽量バイナリ**: 外部依存関係を持たず、リリースビルド時のバイナリサイズは約200〜250KB。
//! - **標準入出力（パイプ）対応**: 引数省略時は `stdin` から読み込み、結果を `stdout` にストリーム出力。
//! - **ワイルドカード展開**: Windows環境でも `*` や `?` パターンを自前で安全に展開（最大100ファイル）。
//! - **二段階ヘルプ**: `-h`（構文と要約）と `--help`（詳細マニュアル・使用例）に対応。

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

use mynkf::*;

/// 簡潔なヘルプ情報（コマンド構文、主要オプション一覧、`--help` への案内）を標準出力に表示します。
fn print_usage() {
    println!(
        "MyNKF [Rust Standard Library Edition] v{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Usage: MyNKF [options] [file...]");
    println!();
    println!("Options:");
    println!("  -w, --utf8       Convert output to UTF-8 (LF)");
    println!("  -s, --sjis       Convert output to Shift-JIS (CRLF)");
    println!("  -e, --euc        Convert output to EUC-JP (LF)");
    println!("  -g, --guess      Guess the character encoding of the input");
    println!("      --line       Show line count in guess mode (ignored for BINARY)");
    println!("      --size       Show formatted file size in guess mode");
    println!("  -d, --lf         Force Line Endings as LF");
    println!("  -c, --crlf       Force Line Endings as CRLF");
    println!("  -h               Show concise help");
    println!("      --help       Show detailed help and examples");
    println!("  -v, --version    Show version information");
    println!("      --versio     Show version information (alias)");
    println!();
    println!("For detailed help and usage examples, run 'MyNKF --help'.");
}

/// 詳細なヘルプ情報（概要、分類別オプション詳細、注意事項、実践的な使用例）を標準出力に表示します。
fn print_detailed_help() {
    println!(
        "MyNKF [Rust Standard Library Edition] v{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("A lightweight Japanese character code and line ending converter.");
    println!();
    println!("Usage:");
    println!("  MyNKF [options] [file...]");
    println!();
    println!("Options:");
    println!("  Encoding Conversion:");
    println!("    -w, --utf8           Convert text encoding to UTF-8 (normalized to LF)");
    println!("    -s, --sjis           Convert text encoding to Shift-JIS (normalized to CRLF)");
    println!("    -e, --euc            Convert text encoding to EUC-JP (normalized to LF)");
    println!();
    println!("  Character Code Guessing:");
    println!("    -g, --guess          Detect character encoding and line ending");
    println!("        --line           Show logical line count (text files only)");
    println!("        --size           Show formatted file size (e.g. [1.2 KB])");
    println!();
    println!("  Line Ending Control:");
    println!("    -d, --lf             Force line endings to LF");
    println!("    -c, --crlf           Force line endings to CRLF");
    println!();
    println!("  Help & Version:");
    println!("    -h                   Show concise help");
    println!("        --help           Show this detailed help and examples");
    println!("    -v, --version        Show version information");
    println!("        --versio         Show version information (nkf compatibility alias)");
    println!();
    println!("Notes:");
    println!("  - If no encoding conversion option (-w, -s, -e) is specified, the input");
    println!("    encoding is preserved. You can use -d or -c alone to convert line endings only.");
    println!("  - When no file is specified, input is read from standard input (stdin) and");
    println!("    written to standard output (stdout).");
    println!("  - Multiple files and wildcards (*, ?) are supported (up to 100 files).");
    println!();
    println!("Examples:");
    println!("  # Guess encoding and line ending of a file:");
    println!("    MyNKF -g input.txt");
    println!();
    println!("  # Guess encoding with line count and file size:");
    println!("    MyNKF -g --line --size input.txt");
    println!();
    println!("  # Convert a file to UTF-8 (LF) and save to a new file:");
    println!("    MyNKF -w input.txt > output_utf8.txt");
    println!();
    println!("  # Convert a file to Shift-JIS (CRLF):");
    println!("    MyNKF -s input.txt > output_sjis.txt");
    println!();
    println!("  # Convert line endings to CRLF without changing encoding:");
    println!("    MyNKF -c input.txt > output_crlf.txt");
    println!();
    println!("  # Process input from pipeline (stdin):");
    println!("    type input.txt | MyNKF -w > output.txt");
    println!();
    println!("  # Batch guess encoding for multiple files using wildcards:");
    println!("    MyNKF -g *.txt");
}

/// 自動判定（Guess）モードにおける出力文字列をフォーマットします。
///
/// ファイル名（ファイル指定時）、判定エンコーディング、改行コード、
/// 論理行数（`--line` 指定時）、ファイルサイズ（`--size` 指定時）を適切に組み立てます。
fn format_guess_output(
    prefix: Option<&str>,
    guessed: Encoding,
    ending: LineEnding,
    lines: usize,
    size: usize,
    is_line: bool,
    is_size: bool,
) -> String {
    let mut parts = Vec::new();

    if guessed == Encoding::Unknown {
        parts.push("BINARY".to_string());
    } else {
        parts.push(format!("{} ({})", guessed.as_str(), ending.as_str()));
        if is_line {
            parts.push(format!("[{} lines]", lines));
        }
    }

    if is_size {
        parts.push(format!("[{}]", format_size(size)));
    }

    let joined = parts.join(" ");
    match prefix {
        Some(name) => format!("{}: {}", name, joined),
        None => joined,
    }
}

/// 入力バイト列に対し、指定されたオプションに従って文字コード変換または改行コード変換を適用します。
///
/// 文字コード指定（`-w`, `-s`, `-e`）がない場合は再エンコードを行わず、
/// バイト列レベルの高速改行置換（[`convert_line_endings_raw`]）を実行して元エンコーディングを維持します。
fn process_buffer(
    buffer: &[u8],
    has_enc_option: bool,
    to_enc: Encoding,
    force_lf: bool,
    force_crlf: bool,
    table: &[u16],
    unicode_to_jis: &HashMap<u16, u16>,
) -> Vec<u8> {
    if !has_enc_option {
        // エンコーディング指定なし（改行コード変換のみ、またはそのまま出力）
        if force_crlf {
            convert_line_endings_raw(buffer, true)
        } else if force_lf {
            convert_line_endings_raw(buffer, false)
        } else {
            buffer.to_vec()
        }
    } else {
        // エンコーディング変換
        let guessed = guess_encoding(buffer);
        let actual_crlf = if force_crlf {
            true
        } else if force_lf {
            false
        } else {
            to_enc == Encoding::Sjis
        };
        let unicode = decode_to_unicode(buffer, guessed, table);
        encode_from_unicode(&unicode, to_enc, unicode_to_jis, actual_crlf)
    }
}

/// CLI アプリケーションのエントリーポイント。
///
/// コマンドライン引数をパースし、指定されたファイル群または標準入力を処理して標準出力に結果を書き出します。
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut to_enc = Encoding::Utf8; // デフォルトUTF-8
    let mut has_enc_option = false;
    let mut is_guess = false;
    let mut is_line = false;
    let mut is_size = false;
    let mut raw_files = Vec::new();
    let mut force_lf = false;
    let mut force_crlf = false;

    // コマンドライン引数のシンプルなパース
    let mut skip = true;
    for arg in args.iter() {
        if skip {
            skip = false;
            continue; // 実行可能ファイル名はスキップ
        }
        if arg == "-h" {
            print_usage();
            return Ok(());
        } else if arg == "--help" {
            print_detailed_help();
            return Ok(());
        } else if arg == "--version" || arg == "--versio" || arg == "-v" {
            println!(
                "MyNKF v{}\nCopyright (c) 2026 tkshnkgwr\nLicense: MIT",
                env!("CARGO_PKG_VERSION")
            );
            return Ok(());
        } else if arg == "-w" || arg == "--utf8" {
            to_enc = Encoding::Utf8;
            has_enc_option = true;
        } else if arg == "-s" || arg == "--sjis" {
            to_enc = Encoding::Sjis;
            has_enc_option = true;
        } else if arg == "-e" || arg == "--euc" {
            to_enc = Encoding::EucJp;
            has_enc_option = true;
        } else if arg == "-g" || arg == "--guess" {
            is_guess = true;
        } else if arg == "--line" {
            is_line = true;
        } else if arg == "--size" {
            is_size = true;
        } else if arg == "-d" || arg == "--lf" {
            force_lf = true;
        } else if arg == "-c" || arg == "--crlf" {
            force_crlf = true;
        } else if arg.starts_with('-') {
            // 不明なオプションはヘルプを表示して終了
            eprintln!("Unknown option: {}", arg);
            print_usage();
            std::process::exit(1);
        } else {
            raw_files.push(arg.clone());
        }
    }

    // ワイルドカード展開
    let mut files = Vec::new();
    for rf in raw_files {
        if let Err(e) = expand_wildcard(&rf, &mut files) {
            eprintln!("Error expanding wildcard: {}", e);
            std::process::exit(1);
        }
    }

    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    if files.is_empty() {
        // 標準入力からの読込
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;

        if is_guess {
            let guessed = guess_encoding(&buffer);
            let size = buffer.len();
            let ending = detect_line_ending(&buffer);
            let lines = count_lines(&buffer);

            let info = format_guess_output(None, guessed, ending, lines, size, is_line, is_size);
            println!("{}", info);
        } else {
            let output = process_buffer(
                &buffer,
                has_enc_option,
                to_enc,
                force_lf,
                force_crlf,
                &table,
                &unicode_to_jis,
            );
            io::stdout().write_all(&output)?;
            io::stdout().flush()?;
        }
    } else {
        // ファイルからの読込
        for filename in &files {
            let mut file = match File::open(filename) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error opening file '{}': {}", filename, e);
                    continue;
                }
            };
            let mut buffer = Vec::new();
            if let Err(e) = file.read_to_end(&mut buffer) {
                eprintln!("Error reading file '{}': {}", filename, e);
                continue;
            }

            if is_guess {
                let guessed = guess_encoding(&buffer);
                let size = buffer.len();
                let ending = detect_line_ending(&buffer);
                let lines = count_lines(&buffer);

                let info = format_guess_output(
                    Some(filename),
                    guessed,
                    ending,
                    lines,
                    size,
                    is_line,
                    is_size,
                );
                println!("{}", info);
            } else {
                let output = process_buffer(
                    &buffer,
                    has_enc_option,
                    to_enc,
                    force_lf,
                    force_crlf,
                    &table,
                    &unicode_to_jis,
                );
                io::stdout().write_all(&output)?;
            }
        }
        io::stdout().flush()?;
    }

    Ok(())
}
