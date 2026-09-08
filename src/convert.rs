//! 文字コードデコード・エンコードおよび改行直接変換モジュール。

use crate::table::{eucjp_to_sjis, sjis_to_eucjp};
use crate::types::Encoding;
use std::collections::HashMap;

/// バイト列の改行コード（LF, CRLF, CR）を、指定された改行コード（CRLF または LF）に直接変換します。
///
/// 文字コードの再デコード・再エンコードを経由せず、バイト列レベルで直接改行文字を置換するため、
/// 外字・絵文字・半角カナなどのデータ欠落や破損のリスクが一切ありません。
///
/// # 引数
///
/// * `bytes` - 変換対象のバイト列
/// * `actual_crlf` - 改行コードをCRLFにするか（`true`で `\r\n` (CRLF)、`false`で `\n` (LF)）
///
/// # 戻り値
///
/// 改行コード変換後のバイト列 `Vec<u8>` を返します。
///
/// # 例
/// ```rust
/// use mynkf::convert_line_endings_raw;
///
/// let crlf_data = convert_line_endings_raw(b"line1\nline2\n", true);
/// assert_eq!(crlf_data, b"line1\r\nline2\r\n");
///
/// let lf_data = convert_line_endings_raw(b"line1\r\nline2\r\n", false);
/// assert_eq!(lf_data, b"line1\nline2\n");
/// ```
pub fn convert_line_endings_raw(bytes: &[u8], actual_crlf: bool) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes.len() + bytes.len() / 10);
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' {
            if actual_crlf {
                result.push(b'\r');
                result.push(b'\n');
            } else {
                result.push(b'\n');
            }
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 2;
            } else {
                i += 1;
            }
        } else if bytes[i] == b'\n' {
            if actual_crlf {
                result.push(b'\r');
                result.push(b'\n');
            } else {
                result.push(b'\n');
            }
            i += 1;
        } else {
            result.push(bytes[i]);
            i += 1;
        }
    }
    result
}

/// 指定されたバイト列を指定された文字コードからUnicode（`Vec<char>`）にデコードします。
///
/// JIS X 0208 の Unicode マッピングテーブルを参照して、Shift_JIS や EUC-JP をデコードします。
/// マッピングに存在しない未定義文字や外字は `'?'` にフォールバック置換されます。
///
/// # 引数
///
/// * `bytes` - デコード対象のバイト列
/// * `from_enc` - デコード元の文字コード ([`Encoding`])
/// * `table` - JIS-Unicodeマッピングテーブル（[`load_jis_table`](crate::load_jis_table) の戻り値）
///
/// # 戻り値
///
/// 変換された Unicode 文字配列 `Vec<char>` を返します。
///
/// # 例
/// ```rust
/// use mynkf::{decode_to_unicode, load_jis_table, Encoding};
///
/// let table = load_jis_table();
/// let sjis_bytes = &[0x82, 0xA0]; // 「あ」
/// let chars = decode_to_unicode(sjis_bytes, Encoding::Sjis, &table);
/// assert_eq!(chars, vec!['あ']);
/// ```
pub fn decode_to_unicode(bytes: &[u8], from_enc: Encoding, table: &[u16]) -> Vec<char> {
    let mut chars = Vec::new();
    let mut i = 0;

    match from_enc {
        Encoding::Ascii | Encoding::Unknown => {
            for &b in bytes {
                chars.push(b as char);
            }
        }
        Encoding::Utf8 => {
            let s = String::from_utf8_lossy(bytes);
            chars = s.chars().collect();
        }
        Encoding::Sjis => {
            while i < bytes.len() {
                let b1 = bytes[i];
                if b1 < 0x80 {
                    chars.push(b1 as char);
                    i += 1;
                } else if (0xA1..=0xDF).contains(&b1) {
                    let code = 0xFF61 + (b1 as u32 - 0xA1);
                    chars.push(std::char::from_u32(code).unwrap_or('?'));
                    i += 1;
                } else if (0x81..=0x9F).contains(&b1) || (0xE0..=0xFC).contains(&b1) {
                    if i + 1 < bytes.len() {
                        let b2 = bytes[i + 1];
                        if let Some((e1, e2)) = sjis_to_eucjp(b1, b2) {
                            let ku = e1 - 0xA0;
                            let ten = e2 - 0xA0;
                            let idx = ((ku as usize - 1) * 94) + (ten as usize - 1);
                            if idx < table.len() && table[idx] != 0 {
                                chars.push(std::char::from_u32(table[idx] as u32).unwrap_or('?'));
                            } else {
                                chars.push('?');
                            }
                        } else {
                            chars.push('?');
                        }
                        i += 2;
                    } else {
                        chars.push('?');
                        i += 1;
                    }
                } else {
                    chars.push('?');
                    i += 1;
                }
            }
        }
        Encoding::EucJp => {
            while i < bytes.len() {
                let b1 = bytes[i];
                if b1 < 0x80 {
                    chars.push(b1 as char);
                    i += 1;
                } else if b1 == 0x8E {
                    if i + 1 < bytes.len() {
                        let b2 = bytes[i + 1];
                        if (0xA1..=0xDF).contains(&b2) {
                            let code = 0xFF61 + (b2 as u32 - 0xA1);
                            chars.push(std::char::from_u32(code).unwrap_or('?'));
                        } else {
                            chars.push('?');
                        }
                        i += 2;
                    } else {
                        chars.push('?');
                        i += 1;
                    }
                } else if b1 == 0x8F {
                    chars.push('?');
                    chars.push('?');
                    i += 3;
                } else if (0xA1..=0xFE).contains(&b1) {
                    if i + 1 < bytes.len() {
                        let b2 = bytes[i + 1];
                        let ku = b1 - 0xA0;
                        let ten = b2 - 0xA0;
                        let idx = ((ku as usize - 1) * 94) + (ten as usize - 1);
                        if idx < table.len() && table[idx] != 0 {
                            chars.push(std::char::from_u32(table[idx] as u32).unwrap_or('?'));
                        } else {
                            chars.push('?');
                        }
                        i += 2;
                    } else {
                        chars.push('?');
                        i += 1;
                    }
                } else {
                    chars.push('?');
                    i += 1;
                }
            }
        }
    }
    chars
}

/// Unicode文字スライス（`chars`）を指定された文字コード（UTF-8, Shift_JIS, EUC-JP）のバイト列にエンコードします。
///
/// 各エンコーディングへの変換ルール：
/// - **改行文字（`\r`, `\n`, `\r\n`）**: `actual_crlf` に従って一括正規化（`true`で `\r\n`、`false`で `\n`）。
/// - **半角カタカナ（`0xFF61..=0xFF9F`）**: 各文字コードの半角カナ領域（SJIS: `0xA1..=0xDF`、EUC-JP: `0x8E 0xA1..=0xDF`）に正確にマッピング。
/// - **JIS X 0208 定義外文字・外字**: エラーにせず安全に `??` (0x3F 0x3F) にフォールバック置換。
///
/// # 引数
///
/// * `chars` - エンコード対象の文字スライス
/// * `to_enc` - エンコード先の文字コード ([`Encoding`])
/// * `unicode_to_jis` - UnicodeコードポイントからJISインデックスへのマッピング辞書
/// * `actual_crlf` - 改行コードをCRLFにするか（`true`でCRLF、`false`でLF）
///
/// # 戻り値
///
/// エンコードされたバイト列 `Vec<u8>` を返します。
///
/// # 例
/// ```rust
/// use std::collections::HashMap;
/// use mynkf::{encode_from_unicode, load_jis_table, Encoding};
///
/// let table = load_jis_table();
/// let mut unicode_to_jis = HashMap::new();
/// for (idx, &uni) in table.iter().enumerate() {
///     if uni != 0 {
///         unicode_to_jis.insert(uni, idx as u16);
///     }
/// }
///
/// let chars: Vec<char> = "あ\n".chars().collect();
/// let sjis_bytes = encode_from_unicode(&chars, Encoding::Sjis, &unicode_to_jis, true);
/// assert_eq!(sjis_bytes, vec![0x82, 0xA0, 0x0D, 0x0A]);
/// ```
pub fn encode_from_unicode(
    chars: &[char],
    to_enc: Encoding,
    unicode_to_jis: &HashMap<u16, u16>,
    actual_crlf: bool,
) -> Vec<u8> {
    let mut bytes = Vec::new();

    let mut normalized_chars = Vec::new();
    let mut skip_next = false;
    for i in 0..chars.len() {
        if skip_next {
            skip_next = false;
            continue;
        }
        let c = chars[i];
        if c == '\r' {
            normalized_chars.push('\n');
            if i + 1 < chars.len() && chars[i + 1] == '\n' {
                skip_next = true;
            }
        } else {
            normalized_chars.push(c);
        }
    }

    for &c in &normalized_chars {
        if c == '\n' {
            if actual_crlf {
                bytes.push(0x0D);
                bytes.push(0x0A);
            } else {
                bytes.push(0x0A);
            }
            continue;
        }

        let uni = c as u32;
        if uni < 0x80 {
            bytes.push(uni as u8);
        } else if (0xFF61..=0xFF9F).contains(&uni) {
            let k_byte = (uni - 0xFF61 + 0xA1) as u8;
            if to_enc == Encoding::Sjis {
                bytes.push(k_byte);
            } else if to_enc == Encoding::EucJp {
                bytes.push(0x8E);
                bytes.push(k_byte);
            } else {
                let mut buf = [0; 4];
                let s = c.encode_utf8(&mut buf);
                bytes.extend_from_slice(s.as_bytes());
            }
        } else {
            if to_enc == Encoding::Utf8 {
                let mut buf = [0; 4];
                let s = c.encode_utf8(&mut buf);
                bytes.extend_from_slice(s.as_bytes());
            } else {
                let uni_u16 = uni as u16;
                if let Some(&idx) = unicode_to_jis.get(&uni_u16) {
                    let ku = (idx / 94) + 1;
                    let ten = (idx % 94) + 1;
                    let e1 = (ku + 0xA0) as u8;
                    let e2 = (ten + 0xA0) as u8;

                    if to_enc == Encoding::EucJp {
                        bytes.push(e1);
                        bytes.push(e2);
                    } else if to_enc == Encoding::Sjis {
                        let (s1, s2) = eucjp_to_sjis(e1, e2);
                        bytes.push(s1);
                        bytes.push(s2);
                    }
                } else {
                    bytes.push(0x3F);
                    bytes.push(0x3F);
                }
            }
        }
    }
    bytes
}
