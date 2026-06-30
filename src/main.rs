// =========================================================================
// MyNKF (Standard Library Only Edition)
// Compiled Size: ~250KB (stripped release build)
// CLI Executable Entry Point
// =========================================================================

#![allow(dead_code, unused_variables, unused_mut)]

use std::env;
use std::io::{self, Read, Write};
use std::fs::File;
use std::collections::HashMap;

use mynkf::*;

fn print_usage() {
    println!("MyNKF [Rust Standard Library Edition] v{}", env!("CARGO_PKG_VERSION"));
    println!("Usage: MyNKF [options] [file...]");
    println!("Options:");
    println!("  -w               Convert output to UTF-8 (LF)");
    println!("  -s               Convert output to Shift-JIS (CRLF)");
    println!("  -e               Convert output to EUC-JP (LF)");
    println!("  -g, --guess      Guess the character encoding of the input");
    println!("  --line           Show line count in guess mode (ignored for BINARY)");
    println!("  --size           Show formatted file size in guess mode");
    println!("  -d               Force Line Endings as LF");
    println!("  -c               Force Line Endings as CRLF");
    println!("  -h, --help       Show this help information");
    println!("  -v, --version    Show version information");
    println!("  --versio         Show version information (alias)");
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
        } else if arg == "-w" {
            to_enc = Encoding::Utf8;
            has_enc_option = true;
        } else if arg == "-s" {
            to_enc = Encoding::Sjis;
            has_enc_option = true;
        } else if arg == "-e" {
            to_enc = Encoding::EucJp;
            has_enc_option = true;
        } else if arg == "-g" || arg == "--guess" {
            is_guess = true;
        } else if arg == "--line" {
            is_line = true;
        } else if arg == "--size" {
            is_size = true;
        } else if arg == "-d" {
            force_lf = true;
        } else if arg == "-c" {
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

            let mut info = format!("STDIN: {}", guessed.as_str());
            if guessed != Encoding::Unknown {
                if is_size {
                    info = format!("{} ({})", info, format_size(size));
                }
                if is_line {
                    info = format!("{} ({} lines, {})", info, lines, ending.as_str());
                } else {
                    info = format!("{} ({})", info, ending.as_str());
                }
            } else {
                if is_size {
                    info = format!("{} ({})", info, format_size(size));
                }
            }
            println!("{}", info);
        } else {
            let guessed = guess_encoding(&buffer);
            let target_enc = if has_enc_option {
                to_enc
            } else {
                if guessed == Encoding::Unknown {
                    Encoding::Utf8
                } else {
                    guessed
                }
            };
            let unicode = decode_to_unicode(&buffer, guessed, &table);
            let actual_crlf = if force_crlf {
                true
            } else if force_lf {
                false
            } else {
                target_enc == Encoding::Sjis
            };
            let output = encode_from_unicode(&unicode, target_enc, &unicode_to_jis, actual_crlf);
            io::stdout().write_all(&output)?;
            io::stdout().flush()?;
        }
    } else {
        // ファイルからの読込
        for filename in files {
            let mut file = match File::open(&filename) {
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

                let mut info = format!("{}: {}", filename, guessed.as_str());
                if guessed != Encoding::Unknown {
                    if is_size {
                        info = format!("{} ({})", info, format_size(size));
                    }
                    if is_line {
                        info = format!("{} ({} lines, {})", info, lines, ending.as_str());
                    } else {
                        info = format!("{} ({})", info, ending.as_str());
                    }
                } else {
                    if is_size {
                        info = format!("{} ({})", info, format_size(size));
                    }
                }
                println!("{}", info);
            } else {
                let guessed = guess_encoding(&buffer);
                let target_enc = if has_enc_option {
                    to_enc
                } else {
                    if guessed == Encoding::Unknown {
                        Encoding::Utf8
                    } else {
                        guessed
                    }
                };
                let unicode = decode_to_unicode(&buffer, guessed, &table);
                let actual_crlf = if force_crlf {
                    true
                } else if force_lf {
                    false
                } else {
                    target_enc == Encoding::Sjis
                };
                let output = encode_from_unicode(&unicode, target_enc, &unicode_to_jis, actual_crlf);
                io::stdout().write_all(&output)?;
            }
        }
        io::stdout().flush()?;
    }

    Ok(())
}
