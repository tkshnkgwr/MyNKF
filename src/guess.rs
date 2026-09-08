//! 文字コード自動判定モジュール。

use crate::table::sjis_to_eucjp;
use crate::types::Encoding;

/// 与えられたバイト列の文字コード（ASCII, UTF-8, Shift_JIS, EUC-JP, BINARY）を自動判定します。
///
/// 判定は以下の優先度およびアルゴリズムで行われます：
/// 1. ヌル文字（`0x00`）等の制御文字が含まれる場合: [`Encoding::Unknown`] (BINARY)
/// 2. 全バイトが `0x7F` 以下の場合: [`Encoding::Ascii`]
/// 3. UTF-8 のマルチバイト文法に完全適合する場合: [`Encoding::Utf8`]
/// 4. JIS区点範囲（1〜84区、1〜94点）の重み付けスコア比較による Shift_JIS と EUC-JP の判定
///
/// # 引数
///
/// * `bytes` - 判定対象のバイト列
///
/// # 戻り値
///
/// 判定された [`Encoding`] を返します。
///
/// # 例
/// ```rust
/// use mynkf::{guess_encoding, Encoding};
///
/// assert_eq!(guess_encoding(b"Hello"), Encoding::Ascii);
/// assert_eq!(guess_encoding("こんにちは".as_bytes()), Encoding::Utf8);
/// assert_eq!(guess_encoding(&[0x00, 0xFF]), Encoding::Unknown);
/// ```
pub fn guess_encoding(bytes: &[u8]) -> Encoding {
    if bytes.is_empty() {
        return Encoding::Ascii;
    }

    // 制御文字（ヌル文字やバイナリ特有の制御文字）が含まれている場合は即座に BINARY (Unknown)
    for &b in bytes {
        if b == 0x00
            || (b < 0x09 && b != 0x07)
            || b == 0x0B
            || b == 0x0C
            || (0x0E..=0x19).contains(&b)
            || (0x1C..=0x1F).contains(&b)
        {
            return Encoding::Unknown;
        }
    }

    // すべて ASCII 範囲内であれば ASCII
    if bytes.iter().all(|&b| b < 0x80) {
        return Encoding::Ascii;
    }

    // UTF-8 の厳格なバリデーション
    if std::str::from_utf8(bytes).is_ok() {
        return Encoding::Utf8;
    }

    // Shift_JIS と EUC-JP の詳細検証
    let mut sjis_valid = true;
    let mut sjis_chars = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b < 0x80 {
            i += 1;
        } else if (0xA1..=0xDF).contains(&b) {
            // 半角カナ
            sjis_chars += 1;
            i += 1;
        } else if (0x81..=0x9F).contains(&b) || (0xE0..=0xFC).contains(&b) {
            if i + 1 < bytes.len() {
                let b2 = bytes[i + 1];
                if let Some((e1, e2)) = sjis_to_eucjp(b, b2) {
                    let ku = e1.saturating_sub(0xA0);
                    let ten = e2.saturating_sub(0xA0);
                    if (1..=84).contains(&ku) && (1..=94).contains(&ten) {
                        sjis_chars += 10;
                    } else {
                        sjis_chars += 2;
                    }
                    i += 2;
                } else if (0x40..=0x7E).contains(&b2) || (0x80..=0xFC).contains(&b2) {
                    sjis_chars += 1;
                    i += 2;
                } else {
                    sjis_valid = false;
                    break;
                }
            } else {
                sjis_valid = false;
                break;
            }
        } else {
            sjis_valid = false;
            break;
        }
    }

    let mut euc_valid = true;
    let mut euc_chars = 0usize;
    let mut j = 0;
    while j < bytes.len() {
        let b = bytes[j];
        if b < 0x80 {
            j += 1;
        } else if b == 0x8E {
            // 半角カナ (SS2)
            if j + 1 < bytes.len() && (0xA1..=0xDF).contains(&bytes[j + 1]) {
                euc_chars += 1;
                j += 2;
            } else {
                euc_valid = false;
                break;
            }
        } else if b == 0x8F {
            // 補助漢字 (SS3)
            if j + 2 < bytes.len()
                && (0xA1..=0xFE).contains(&bytes[j + 1])
                && (0xA1..=0xFE).contains(&bytes[j + 2])
            {
                euc_chars += 2;
                j += 3;
            } else {
                euc_valid = false;
                break;
            }
        } else if (0xA1..=0xFE).contains(&b) {
            if j + 1 < bytes.len() && (0xA1..=0xFE).contains(&bytes[j + 1]) {
                let ku = b.saturating_sub(0xA0);
                let ten = bytes[j + 1].saturating_sub(0xA0);
                if (1..=84).contains(&ku) && (1..=94).contains(&ten) {
                    euc_chars += 10;
                } else {
                    euc_chars += 2;
                }
                j += 2;
            } else {
                euc_valid = false;
                break;
            }
        } else {
            euc_valid = false;
            break;
        }
    }

    if sjis_valid && !euc_valid {
        return Encoding::Sjis;
    }
    if !sjis_valid && euc_valid {
        return Encoding::EucJp;
    }
    if sjis_valid && euc_valid {
        if euc_chars > sjis_chars {
            return Encoding::EucJp;
        } else {
            return Encoding::Sjis;
        }
    }

    Encoding::Unknown
}
