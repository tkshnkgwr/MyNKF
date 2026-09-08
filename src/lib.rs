//! # MyNKF コアライブラリ (`mynkf`)
//!
//! 日本語文字コードの自動判定および、UTF-8・Shift_JIS・EUC-JP 相互の文字コード・改行コード変換を提供する軽量ライブラリです。
//!
//! ## 設計方針
//! - **標準ライブラリ限定 (Zero External Dependencies)**:
//!   サードパーティ製クレートに一切依存せず、Rust標準ライブラリ（`std`）のみで完結しています。
//! - **安全・堅牢なマッピング**:
//!   JIS X 0208（全6,953文字）の完全マッピングテーブルを内包し、外字・未定義文字の安全なフォールバック（`??`）を備えています。
//! - **バイトレベルの改行変換**:
//!   エンコード指定なしの改行変換ではバイト列を直接置換（`convert_line_endings_raw`）し、再エンコードによる文字化けリスクを完全排除しています。
//!
//! ## 使用例
//! ```rust
//! use mynkf::{guess_encoding, detect_line_ending, Encoding, LineEnding};
//!
//! let data = "こんにちは世界\n".as_bytes();
//! let enc = guess_encoding(data);
//! let ending = detect_line_ending(data);
//!
//! assert_eq!(enc, Encoding::Utf8);
//! assert_eq!(ending, LineEnding::Lf);
//! ```

pub mod convert;
pub mod glob;
pub mod guess;
pub mod newline;
pub mod table;
pub mod types;

pub use convert::*;
pub use glob::*;
pub use guess::*;
pub use newline::*;
pub use table::*;
pub use types::*;

#[cfg(test)]
mod tests;
