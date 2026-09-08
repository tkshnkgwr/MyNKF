//! 改行コードの検出および行数カウントモジュール。

use crate::types::LineEnding;

/// 与えられたバイト列から改行コードの種類を検出します。
///
/// バイト列内を走査し、`\n` (LF)、`\r\n` (CRLF)、`\r` (CR) の出現パターンを判定します。
/// 複数種類の改行コードが検出された場合は [`LineEnding::Mixed`]、改行が一切ない場合は [`LineEnding::None`] を返します。
///
/// # 引数
///
/// * `bytes` - 判定対象のバイト列
///
/// # 戻り値
///
/// 検出された [`LineEnding`] を返します。
///
/// # 例
/// ```rust
/// use mynkf::{detect_line_ending, LineEnding};
///
/// assert_eq!(detect_line_ending(b"foo\r\nbar\r\n"), LineEnding::Crlf);
/// assert_eq!(detect_line_ending(b"foo\nbar\n"), LineEnding::Lf);
/// assert_eq!(detect_line_ending(b"hello world"), LineEnding::None);
/// ```
pub fn detect_line_ending(bytes: &[u8]) -> LineEnding {
    let mut has_lf = false;
    let mut has_crlf = false;
    let mut has_cr = false;

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                has_crlf = true;
                i += 2;
            } else {
                has_cr = true;
                i += 1;
            }
        } else if bytes[i] == b'\n' {
            has_lf = true;
            i += 1;
        } else {
            i += 1;
        }
    }

    match (has_lf, has_crlf, has_cr) {
        (true, false, false) => LineEnding::Lf,
        (false, true, false) => LineEnding::Crlf,
        (false, false, true) => LineEnding::Cr,
        (false, false, false) => LineEnding::None,
        _ => LineEnding::Mixed,
    }
}

/// 与えられたバイト列の論理行数をカウントします。
///
/// 各種改行コード（LF, CRLF, CR）に対応し、末尾に改行コードがない行も1行としてカウントします。
/// 空のバイト列の場合は `0` を返します。
///
/// # 引数
///
/// * `bytes` - カウント対象のバイト列
///
/// # 戻り値
///
/// カウントされた論理行数を返します。
///
/// # 例
/// ```rust
/// use mynkf::count_lines;
///
/// assert_eq!(count_lines(b""), 0);
/// assert_eq!(count_lines(b"line 1\nline 2\n"), 2);
/// assert_eq!(count_lines(b"line 1\r\nline 2"), 2);
/// ```
pub fn count_lines(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut i = 0;
    let mut ends_with_newline = false;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            count += 1;
            ends_with_newline = true;
            i += 1;
        } else if bytes[i] == b'\r' {
            count += 1;
            ends_with_newline = true;
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 2;
            } else {
                i += 1;
            }
        } else {
            ends_with_newline = false;
            i += 1;
        }
    }
    if !ends_with_newline {
        count += 1;
    }
    count
}
