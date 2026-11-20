//! コアライブラリ (mynkf) の単体テストモジュール
//!
//! 文字コード判定、マッピング変換、改行制御、ワイルドカード展開などの網羅的なテストを実施します。

use super::*;
use std::collections::HashMap;

/// ASCII テキストの文字コード判定テスト
#[test]
fn test_guess_encoding_ascii() {
    let data = b"Hello, World!";
    assert_eq!(guess_encoding(data), Encoding::Ascii);
}

/// UTF-8 日本語テキストの文字コード判定テスト
#[test]
fn test_guess_encoding_utf8() {
    let data = "日本語の文字コード判定テスト用のテキストです。".as_bytes();
    assert_eq!(guess_encoding(data), Encoding::Utf8);
}

/// Shift_JIS 日本語テキストの文字コード判定テスト
#[test]
fn test_guess_encoding_sjis() {
    let data = &[0x82, 0xB1, 0x82, 0xF1, 0x82, 0x49, 0x82, 0x61, 0x82, 0x6F];
    assert_eq!(guess_encoding(data), Encoding::Sjis);
}

/// EUC-JP 日本語テキストの文字コード判定テスト
#[test]
fn test_guess_encoding_eucjp() {
    let data = &[0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC];
    assert_eq!(guess_encoding(data), Encoding::EucJp);
}

/// バイナリデータの文字コード判定テスト (Unknown / BINARY)
#[test]
fn test_guess_encoding_binary() {
    let data = &[0x00, 0x01, 0xff, 0x02];
    assert_eq!(guess_encoding(data), Encoding::Unknown);
}

/// Shift_JIS から EUC-JP への区点バイト値直接変換テスト
#[test]
fn test_sjis_to_eucjp_coords() {
    let res = sjis_to_eucjp(0x82, 0xA0);
    assert_eq!(res, Some((0xA4, 0xA2)));
}

/// EUC-JP から Shift_JIS への区点バイト値直接変換テスト
#[test]
fn test_eucjp_to_sjis_coords() {
    let res = eucjp_to_sjis(0xA4, 0xA2);
    assert_eq!(res, (0x82, 0xA0));
}

/// UTF-8 から Shift_JIS へのエンコードおよび CRLF 改行正規化テスト
#[test]
fn test_conversion_utf8_to_sjis() {
    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    let input_chars: Vec<char> = "あ\nい".chars().collect();
    let encoded = encode_from_unicode(&input_chars, Encoding::Sjis, &unicode_to_jis, true);

    let expected = vec![0x82, 0xA0, 0x0D, 0x0A, 0x82, 0xA2];
    assert_eq!(encoded, expected);
}

/// Shift_JIS から UTF-8 へのデコードテスト
#[test]
fn test_conversion_sjis_to_utf8() {
    let table = load_jis_table();
    let sjis_bytes = vec![0x82, 0xA0, 0x0D, 0x0A, 0x82, 0xA2];
    let decoded_chars = decode_to_unicode(&sjis_bytes, Encoding::Sjis, &table);

    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }
    let encoded_utf8 = encode_from_unicode(&decoded_chars, Encoding::Utf8, &unicode_to_jis, false);
    let output_str = String::from_utf8(encoded_utf8).unwrap();
    assert_eq!(output_str, "あ\nい");
}

/// JIS X 0208 範囲外文字（絵文字等）のフォールバック (??) テスト
#[test]
fn test_conversion_fallback() {
    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    let input_chars: Vec<char> = "😀".chars().collect();
    let encoded = encode_from_unicode(&input_chars, Encoding::Sjis, &unicode_to_jis, false);
    assert_eq!(encoded, b"??");
}

/// 半角カタカナの保持・エンコードテスト (Shift_JIS / EUC-JP)
#[test]
fn test_half_width_kana() {
    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    let input_chars: Vec<char> = "ｱ".chars().collect();

    let encoded_sjis = encode_from_unicode(&input_chars, Encoding::Sjis, &unicode_to_jis, false);
    assert_eq!(encoded_sjis, vec![0xB1]);

    let encoded_euc = encode_from_unicode(&input_chars, Encoding::EucJp, &unicode_to_jis, false);
    assert_eq!(encoded_euc, vec![0x8E, 0xB1]);
}

/// 改行コード検出（LF, CRLF, CR, MIXED, NONE）テスト
#[test]
fn test_detect_line_ending() {
    assert_eq!(detect_line_ending(b"hello\nworld"), LineEnding::Lf);
    assert_eq!(detect_line_ending(b"hello\r\nworld"), LineEnding::Crlf);
    assert_eq!(detect_line_ending(b"hello\rworld"), LineEnding::Cr);
    assert_eq!(
        detect_line_ending(b"hello\nworld\r\ntest"),
        LineEnding::Mixed
    );
    assert_eq!(detect_line_ending(b"helloworld"), LineEnding::None);
}

/// 論理行数カウント（空データ、末尾改行あり/なし、複数改行種）テスト
#[test]
fn test_count_lines() {
    assert_eq!(count_lines(b""), 0);
    assert_eq!(count_lines(b"hello"), 1);
    assert_eq!(count_lines(b"hello\n"), 1);
    assert_eq!(count_lines(b"hello\nworld"), 2);
    assert_eq!(count_lines(b"hello\r\nworld\n"), 2);
}

/// ワイルドカードマッチング（*, ?）テスト
#[test]
fn test_wildcard_match() {
    assert!(wildcard_match("*.txt", "hello.txt"));
    assert!(wildcard_match("a*.txt", "apple.txt"));
    assert!(wildcard_match("a?c.txt", "abc.txt"));
    assert!(!wildcard_match("a?c.txt", "abbc.txt"));
    assert!(wildcard_match("*", "anything"));
}

/// 通常のワイルドカードファイル展開テスト
#[test]
fn test_expand_wildcard_normal() {
    use std::fs::File;
    let temp_dir = std::env::temp_dir().join("mynkf_test_wildcard_normal");
    std::fs::remove_dir_all(&temp_dir).ok();
    std::fs::create_dir_all(&temp_dir).unwrap();

    let p1 = temp_dir.join("temp_normal_1.txt");
    let p2 = temp_dir.join("temp_normal_2.txt");
    File::create(&p1).unwrap();
    File::create(&p2).unwrap();

    let pattern = temp_dir.join("temp_normal_*.txt");
    let mut files = Vec::new();
    let res = expand_wildcard(pattern.to_str().unwrap(), &mut files);
    assert!(res.is_ok());
    assert!(files.contains(&p1.to_string_lossy().into_owned()));
    assert!(files.contains(&p2.to_string_lossy().into_owned()));
    assert_eq!(files.len(), 2);

    std::fs::remove_dir_all(&temp_dir).ok();
}

/// ワイルドカード展開の上限件数（100件）超過エラー検出テスト
#[test]
fn test_glob_limit_exceeded() {
    use std::fs::File;
    let temp_dir = std::env::temp_dir().join("mynkf_test_glob_limit");
    std::fs::remove_dir_all(&temp_dir).ok();
    std::fs::create_dir_all(&temp_dir).unwrap();

    for i in 0..=100 {
        let path = temp_dir.join(format!("temp_test_limit_{}.txt", i));
        File::create(&path).unwrap();
    }

    let pattern = temp_dir.join("temp_test_limit_*.txt");
    let mut files = Vec::new();
    let res = expand_wildcard(pattern.to_str().unwrap(), &mut files);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err(),
        format!("Maximum limit of {} files exceeded.", MAX_GLOB_FILES)
    );

    std::fs::remove_dir_all(&temp_dir).ok();
}

/// ファイルサイズ整形関数（format_size）の単位変換テスト
#[test]
fn test_format_size() {
    assert_eq!(format_size(0), "0 B");
    assert_eq!(format_size(500), "500 B");
    assert_eq!(format_size(1024), "1.0 KB");
    assert_eq!(format_size(2048 + 512), "2.5 KB");
    assert_eq!(format_size(1024 * 1024), "1.0 MB");
    assert_eq!(
        format_size(1024 * 1024 * 1024 * 3 + 1024 * 1024 * 1024 / 2),
        "3.5 GB"
    );
}

/// 漢字・ひらがな・カタカナ・記号の Shift_JIS ラウンドトリップ変換テスト
#[test]
fn test_kanji_roundtrip() {
    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    let test_str = "日本語の文字コード変換テスト。漢字、ひらがな、カタカナ、記号！";
    let input_chars: Vec<char> = test_str.chars().collect();

    // UTF-8 -> SJIS
    let sjis_bytes = encode_from_unicode(&input_chars, Encoding::Sjis, &unicode_to_jis, false);
    let guessed_sjis = guess_encoding(&sjis_bytes);
    assert_eq!(guessed_sjis, Encoding::Sjis);

    // SJIS -> Unicode
    let decoded_chars = decode_to_unicode(&sjis_bytes, Encoding::Sjis, &table);
    let result_str: String = decoded_chars.into_iter().collect();
    assert_eq!(result_str, test_str);
}

/// 漢字・ひらがな・カタカナ・記号の EUC-JP ラウンドトリップ変換テスト
#[test]
fn test_eucjp_roundtrip() {
    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    let test_str = "日本語の文字コード変換テスト。漢字、ひらがな、カタカナ、記号！";
    let input_chars: Vec<char> = test_str.chars().collect();

    // UTF-8 -> EUC-JP
    let euc_bytes = encode_from_unicode(&input_chars, Encoding::EucJp, &unicode_to_jis, false);
    let guessed_euc = guess_encoding(&euc_bytes);
    assert_eq!(guessed_euc, Encoding::EucJp);

    // EUC-JP -> Unicode
    let decoded_chars = decode_to_unicode(&euc_bytes, Encoding::EucJp, &table);
    let result_str: String = decoded_chars.into_iter().collect();
    assert_eq!(result_str, test_str);
}

/// バイトレベル直接改行コード変換（convert_line_endings_raw）テスト
#[test]
fn test_convert_line_endings_raw() {
    // CRLF -> LF
    let crlf_data = b"line1\r\nline2\r\nline3";
    let lf_result = convert_line_endings_raw(crlf_data, false);
    assert_eq!(lf_result, b"line1\nline2\nline3");

    // LF -> CRLF
    let lf_data = b"line1\nline2\nline3";
    let crlf_result = convert_line_endings_raw(lf_data, true);
    assert_eq!(crlf_result, b"line1\r\nline2\r\nline3");

    // CR -> LF
    let cr_data = b"line1\rline2\rline3";
    let lf_from_cr = convert_line_endings_raw(cr_data, false);
    assert_eq!(lf_from_cr, b"line1\nline2\nline3");

    // CR -> CRLF
    let crlf_from_cr = convert_line_endings_raw(cr_data, true);
    assert_eq!(crlf_from_cr, b"line1\r\nline2\r\nline3");
}

/// ヌル文字（0x00）を含むバイナリの自動判定テスト (Unknown / BINARY)
#[test]
fn test_guess_encoding_binary_with_null() {
    let mut text_with_null = "こんにちは".as_bytes().to_vec();
    text_with_null.push(0x00);
    assert_eq!(guess_encoding(&text_with_null), Encoding::Unknown);
}

/// サロゲート外文字（U+10000以上）が下位16ビット切り捨てで別文字に化けないことの検証テスト
#[test]
fn test_encode_non_bmp_no_truncation() {
    let table = load_jis_table();
    let mut unicode_to_jis = HashMap::new();
    for (idx, &uni) in table.iter().enumerate() {
        if uni != 0 {
            unicode_to_jis.insert(uni, idx as u16);
        }
    }

    // U+13042 は下位16ビットが 0x3042（ひらがな「あ」）だが、BMP外なので ?? にフォールバックすべき
    let non_bmp = char::from_u32(0x13042).unwrap();
    let sjis = encode_from_unicode(&[non_bmp], Encoding::Sjis, &unicode_to_jis, false);
    assert_eq!(sjis, b"??");

    let euc = encode_from_unicode(&[non_bmp], Encoding::EucJp, &unicode_to_jis, false);
    assert_eq!(euc, b"??");
}

/// EUC-JP デコード時に不正な第2バイトが含まれてもパニックせず安全に置換されることのテスト
#[test]
fn test_decode_eucjp_invalid_second_byte() {
    let table = load_jis_table();
    // 0xA4 の後に EUC-JP 範囲外の 0x20（スペース）が続く不正シーケンス
    let invalid_euc = &[0xA4, 0x20, b'A'];
    let decoded = decode_to_unicode(invalid_euc, Encoding::EucJp, &table);
    assert_eq!(decoded, vec!['?', ' ', 'A']);
}

/// EUC-JP 補助漢字（0x8F）のデコードおよび末尾途切れの安全性テスト
#[test]
fn test_decode_eucjp_ss3_truncated() {
    let table = load_jis_table();
    let ss3_full = &[0x8F, 0xA1, 0xA1, b'X'];
    assert_eq!(
        decode_to_unicode(ss3_full, Encoding::EucJp, &table),
        vec!['?', 'X']
    );

    let ss3_truncated = &[0x8F, 0xA1];
    assert_eq!(
        decode_to_unicode(ss3_truncated, Encoding::EucJp, &table),
        vec!['?']
    );
}

/// ワイルドカード未一致フォールバック時にもファイル数上限チェックが機能することのテスト
#[test]
fn test_expand_wildcard_no_match_limit() {
    let mut files: Vec<String> = (0..MAX_GLOB_FILES)
        .map(|i| format!("dummy_{}.txt", i))
        .collect();
    let res = expand_wildcard("nonexistent_pattern_*.zzz", &mut files);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err(),
        format!("Maximum limit of {} files exceeded.", MAX_GLOB_FILES)
    );
}
