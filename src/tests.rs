//! コアライブラリ (mynkf) の単体テストモジュール

use super::*;

#[test]
fn test_guess_encoding_ascii() {
    let data = b"Hello, World!";
    assert_eq!(guess_encoding(data), Encoding::Ascii);
}

#[test]
fn test_guess_encoding_utf8() {
    let data = "日本語の文字コード判定テスト用のテキストです。".as_bytes();
    assert_eq!(guess_encoding(data), Encoding::Utf8);
}

#[test]
fn test_guess_encoding_sjis() {
    let data = &[0x82, 0xB1, 0x82, 0xF1, 0x82, 0x49, 0x82, 0x61, 0x82, 0x6F];
    assert_eq!(guess_encoding(data), Encoding::Sjis);
}

#[test]
fn test_guess_encoding_eucjp() {
    let data = &[0xC6, 0xFC, 0xCB, 0xDC, 0xB8, 0xEC];
    assert_eq!(guess_encoding(data), Encoding::EucJp);
}

#[test]
fn test_guess_encoding_binary() {
    let data = &[0x00, 0x01, 0xff, 0x02];
    assert_eq!(guess_encoding(data), Encoding::Unknown);
}

#[test]
fn test_sjis_to_eucjp_coords() {
    let res = sjis_to_eucjp(0x82, 0xA0);
    assert_eq!(res, Some((0xA4, 0xA2)));
}

#[test]
fn test_eucjp_to_sjis_coords() {
    let res = eucjp_to_sjis(0xA4, 0xA2);
    assert_eq!(res, (0x82, 0xA0));
}

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

#[test]
fn test_count_lines() {
    assert_eq!(count_lines(b""), 0);
    assert_eq!(count_lines(b"hello"), 1);
    assert_eq!(count_lines(b"hello\n"), 1);
    assert_eq!(count_lines(b"hello\nworld"), 2);
    assert_eq!(count_lines(b"hello\r\nworld\n"), 2);
}

#[test]
fn test_wildcard_match() {
    assert!(wildcard_match("*.txt", "hello.txt"));
    assert!(wildcard_match("a*.txt", "apple.txt"));
    assert!(wildcard_match("a?c.txt", "abc.txt"));
    assert!(!wildcard_match("a?c.txt", "abbc.txt"));
    assert!(wildcard_match("*", "anything"));
}

#[test]
fn test_expand_wildcard_normal() {
    use std::fs::File;
    let p1 = "temp_normal_1.txt";
    let p2 = "temp_normal_2.txt";
    File::create(p1).unwrap();
    File::create(p2).unwrap();

    let mut files = Vec::new();
    let res = expand_wildcard("temp_normal_*.txt", &mut files);
    assert!(res.is_ok());
    assert!(files.contains(&p1.to_string()));
    assert!(files.contains(&p2.to_string()));
    assert_eq!(files.len(), 2);

    std::fs::remove_file(p1).ok();
    std::fs::remove_file(p2).ok();
}

#[test]
fn test_glob_limit_exceeded() {
    use std::fs::File;
    let mut created_paths = Vec::new();
    for i in 0..=100 {
        let path_str = format!("temp_test_limit_{}.txt", i);
        File::create(&path_str).unwrap();
        created_paths.push(path_str);
    }

    let mut files = Vec::new();
    let res = expand_wildcard("temp_test_limit_*.txt", &mut files);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err(),
        format!("Maximum limit of {} files exceeded.", MAX_GLOB_FILES)
    );

    for path in created_paths {
        std::fs::remove_file(path).ok();
    }
}

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
