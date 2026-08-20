//! MyNKF CLI (標準ライブラリのみ使用版)
//!
//! コマンドラインからファイルの文字コード検出および変換を行うツールです。
//! リリースビルドのバイナリサイズは約250KB（ストリップ後）と非常に軽量です。

#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

use mynkf::*;

fn print_usage() {
    println!(
        "MyNKF [Rust Standard Library Edition] v{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Usage: MyNKF [options] [file...]");
    println!("Options:");
    println!("  -w, --utf8       Convert output to UTF-8 (LF)");
    println!("  -s, --sjis       Convert output to Shift-JIS (CRLF)");
    println!("  -e, --euc        Convert output to EUC-JP (LF)");
    println!("  -g, --guess      Guess the character encoding of the input");
    println!("  --line           Show line count in guess mode (ignored for BINARY)");
    println!("  --size           Show formatted file size in guess mode");
    println!("  -d, --lf         Force Line Endings as LF");
    println!("  -c, --crlf       Force Line Endings as CRLF");
    println!("  -h, --help       Show this help information");
    println!("  -v, --version    Show version information");
    println!("  --versio         Show version information (alias)");
}

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
        if arg == "--help" || arg == "-h" {
            print_usage();
            return Ok(());
        } else if arg == "--version" || arg == "--versio" || arg == "-v" {
            println!("MyNKF v{}", env!("CARGO_PKG_VERSION"));
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
