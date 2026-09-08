//! 文字コードおよび改行コードの型定義モジュール。

/// サポートされている文字エンコーディングを表す列挙型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// ASCII (7ビット英数字・記号)
    Ascii,
    /// UTF-8 (可変長マルチバイト)
    Utf8,
    /// Shift_JIS / Windows-31J (CP932互換領域含む)
    Sjis,
    /// EUC-JP (拡張UNIXコード)
    EucJp,
    /// 判定不能またはバイナリデータ
    Unknown,
}

impl Encoding {
    /// 文字エンコーディングの識別名文字列（`"ASCII"`, `"UTF-8"`, `"Shift_JIS"`, `"EUC-JP"`, `"BINARY"`）を返します。
    ///
    /// # 例
    /// ```rust
    /// use mynkf::Encoding;
    ///
    /// assert_eq!(Encoding::Utf8.as_str(), "UTF-8");
    /// assert_eq!(Encoding::Unknown.as_str(), "BINARY");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Encoding::Ascii => "ASCII",
            Encoding::Utf8 => "UTF-8",
            Encoding::Sjis => "Shift_JIS",
            Encoding::EucJp => "EUC-JP",
            Encoding::Unknown => "BINARY",
        }
    }
}

/// 検出された改行コードの種類を表す列挙型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// LF (`\n`, Linux / macOS / Unix系)
    Lf,
    /// CRLF (`\r\n`, Windows系)
    Crlf,
    /// CR (`\r`, 従来のClassic Mac OS系)
    Cr,
    /// 複数の異なる改行コードが混在
    Mixed,
    /// 改行文字が存在しない
    None,
}

impl LineEnding {
    /// 改行コードの識別名文字列（`"LF"`, `"CRLF"`, `"CR"`, `"MIXED"`, `"NONE"`）を返します。
    ///
    /// # 例
    /// ```rust
    /// use mynkf::LineEnding;
    ///
    /// assert_eq!(LineEnding::Crlf.as_str(), "CRLF");
    /// assert_eq!(LineEnding::Lf.as_str(), "LF");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Lf => "LF",
            LineEnding::Crlf => "CRLF",
            LineEnding::Cr => "CR",
            LineEnding::Mixed => "MIXED",
            LineEnding::None => "NONE",
        }
    }
}
