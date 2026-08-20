//! MyNKFライブラリモジュール
//!
//! 文字コード自動検出および文字コード・改行コード変換のコアロジックを提供します。

use std::collections::HashMap;

/// サポートされている文字コードを表す列挙型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// ASCIIコード
    Ascii,
    /// UTF-8コード
    Utf8,
    /// Shift_JISコード
    Sjis,
    /// EUC-JPコード
    EucJp,
    /// 判定不可（バイナリなど）
    Unknown,
}

impl Encoding {
    /// 文字コードに対応する文字列表記を返します。
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

/// 改行コードの種類を表す列挙型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// LF (Linux/macOS)
    Lf,
    /// CRLF (Windows)
    Crlf,
    /// CR (古いMac)
    Cr,
    /// 複数の改行コードが混在
    Mixed,
    /// 改行なし
    None,
}

impl LineEnding {
    /// 改行コードに対応する文字列表記を返します。
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

/// 与えられたバイト列から改行コードの種類を検出します。
///
/// # 引数
///
/// * `bytes` - 判定対象のバイト列
///
/// # 戻り値
///
/// 検出された `LineEnding` を返します。
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

/// 与えられたバイト列の行数をカウントします。
///
/// 各種改行コード（LF, CRLF, CR）を考慮してカウントします。空のバイト列の場合は 0 を返します。
///
/// # 引数
///
/// * `bytes` - カウント対象のバイト列
///
/// # 戻り値
///
/// 行数を返します。
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

/// ワイルドカード展開時に一度に処理できる最大ファイル数。
pub const MAX_GLOB_FILES: usize = 100;

/// ワイルドカードパターン（`*` や `?`）が指定されたテキストにマッチするか判定します。
///
/// # 引数
///
/// * `pattern` - ワイルドカードパターン（例: `*.txt`, `a?c.txt`）
/// * `text` - マッチング対象のテキスト
///
/// # 戻り値
///
/// マッチした場合は `true`、そうでない場合は `false` を返します。
pub fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    let mut p_idx = 0;
    let mut t_idx = 0;
    let mut p_star = None;
    let mut t_star = None;

    while t_idx < text_chars.len() {
        if p_idx < pattern_chars.len()
            && (pattern_chars[p_idx] == '?' || pattern_chars[p_idx] == text_chars[t_idx])
        {
            p_idx += 1;
            t_idx += 1;
        } else if p_idx < pattern_chars.len() && pattern_chars[p_idx] == '*' {
            p_star = Some(p_idx);
            t_star = Some(t_idx);
            p_idx += 1;
        } else if let Some(star) = p_star {
            p_idx = star + 1;
            t_star = Some(t_star.unwrap() + 1);
            t_idx = t_star.unwrap();
        } else {
            return false;
        }
    }

    while p_idx < pattern_chars.len() && pattern_chars[p_idx] == '*' {
        p_idx += 1;
    }

    p_idx == pattern_chars.len()
}

/// 指定された引数（ワイルドカードを含む場合があるパス）を展開し、ファイルリストに追加します。
///
/// パターンマッチによりファイル名部分が一致するローカルファイルを探して `files` に格納します。
/// 最大ファイル数 `MAX_GLOB_FILES` を超えた場合はエラーを返します。
///
/// # 引数
///
/// * `arg` - 展開対象のパスパターン
/// * `files` - 展開されたファイル名が追加されるベクター
///
/// # 戻り値
///
/// 成功した場合は `Ok(())`、ファイル数上限超過やディレクトリの読み込みに失敗した場合は `Err` を返します。
pub fn expand_wildcard(arg: &str, files: &mut Vec<String>) -> Result<(), String> {
    if !arg.contains('*') && !arg.contains('?') {
        files.push(arg.to_string());
        if files.len() > MAX_GLOB_FILES {
            return Err(format!(
                "Maximum limit of {} files exceeded.",
                MAX_GLOB_FILES
            ));
        }
        return Ok(());
    }

    let path = std::path::Path::new(arg);
    let parent_dir = path.parent().unwrap_or_else(|| std::path::Path::new(""));
    let file_pattern = match path.file_name().and_then(|f| f.to_str()) {
        Some(p) => p,
        None => {
            files.push(arg.to_string());
            return Ok(());
        }
    };

    let dir_to_read = if parent_dir.as_os_str().is_empty() {
        std::path::Path::new(".")
    } else {
        parent_dir
    };

    let entries = match std::fs::read_dir(dir_to_read) {
        Ok(e) => e,
        Err(err) => {
            return Err(format!(
                "Failed to read directory '{:?}': {}",
                dir_to_read, err
            ));
        }
    };

    let mut matched_any = false;
    for entry in entries.flatten() {
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if file_type.is_file()
            && let Some(name_str) = entry.file_name().to_str()
            && wildcard_match(&file_pattern.to_lowercase(), &name_str.to_lowercase())
        {
            let matched_path = if parent_dir.as_os_str().is_empty() {
                std::path::PathBuf::from(entry.file_name())
            } else {
                parent_dir.join(entry.file_name())
            };
            if let Some(path_str) = matched_path.to_str() {
                files.push(path_str.to_string());
                matched_any = true;
                if files.len() > MAX_GLOB_FILES {
                    return Err(format!(
                        "Maximum limit of {} files exceeded.",
                        MAX_GLOB_FILES
                    ));
                }
            }
        }
    }

    if !matched_any {
        files.push(arg.to_string());
    }

    Ok(())
}

/// 指定されたバイト数を読みやすい形式（KB, MB, GB）に整形します。
///
/// # 引数
///
/// * `bytes` - 整形対象のバイト数
///
/// # 戻り値
///
/// 整形されたサイズの文字列を返します。
pub fn format_size(bytes: usize) -> String {
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;
    let bytes_f = bytes as f64;

    if bytes_f >= gb {
        format!("{:.1} GB", bytes_f / gb)
    } else if bytes_f >= mb {
        format!("{:.1} MB", bytes_f / mb)
    } else if bytes_f >= kb {
        format!("{:.1} KB", bytes_f / kb)
    } else {
        format!("{} B", bytes)
    }
}

/// バイト列の改行コード（LF, CRLF, CR）を、指定された改行コード（CRLF または LF）に変換します。
///
/// 文字コードの再エンコードを行わないため、文字欠落やデータ破壊のリスクがありません。
///
/// # 引数
///
/// * `bytes` - 変換対象のバイト列
/// * `actual_crlf` - 改行コードをCRLFにするか（`true`でCRLF、`false`でLF）
///
/// # 戻り値
///
/// 改行コード変換後のバイト列 `Vec<u8>` を返します。
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

pub const JIS_TO_UNICODE_BASE64: &str = "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WeZbjFuYW9tecl55YKNhH2FjYb5j22ViZ9FoU2j6az5rU2xXbyJvl29FdLB1GHbjdwt6/3uhfCF96X82f/CAnYJmg56Js4rMjKuQhJRRlZOVkZWilmWX05koghhOOFQrXLhdzHOpdkx3PFypf+uNC5bBmBGYVJhYTwFPDlNxVZxWaFf6WUdbCVvEXJBeDF5+X8xj7mc6Zddl4mcfaMtoxGpfXjBrxWwXbH11f3lIW2N6AH0AX72Jj4oYjLSNd47Mjx2Y4poOmzxOgFB9UQBZk1ucYi9igGTsazpyoHWReUd/qYf7iryLcGOsg8qXoFQJVANVq2hUaliKcHgnZ3WezVN0W6KBGoZQkAZOGE5FTsdPEVPKVDhbrl8TYCVlUWc9bEJscmzjcHh0A3p2eq57CH0afP59ZmXncltTu1xFXehi0mLgYxluIIZaijGN3ZL4bwF5pptaTqhOq06sT5tPoFDRUUd69lFxUfZTVFMhU39T61WsWINc4V83X0pgL2BQYG1jH2VZaktswXLCcu1374D4gQWCCIVOkPeT4Zf/mVeaWk7wUd1cLWaBaW1cQGbyaXVziWhQfIFQxVLkV0dd/pMmZaRrI2s9dDR5gXm9e0t9yoK5g8yIf4lfizmP0ZHRVB+SgE5dUDZT5VM6ctdzlnfpguaOr5nGmciZ0lF3YRqGXlWwenpQdlvTkEeWhU4yatuR51xRXEhjmHqfbJOXdI9heqpxipaIfIJoF35waFGTbFLyVBuFq4oTf6SOzZDhU2aIiHlBT8JQvlIRUURVU1ctc+pXi1lRX2JfhGB1YXZhZ2GpY7JkOmVsZm9oQm4TdWZ6PXz7fUx9mX5Lf2uDDoNKhs2KCIpji2aO/ZganY+CuI/Om+hSh2IfZINvwJaZaEFQkWsgbHpvVHp0fVCIQIojZwhO9lA5UCZQZVF8UjhSY1WnVw9YBVrMXvphsmH4YvNjcmkcailyfXKscy54FHhvfXl3DICpiYuLGYzijtKQY5N1lnqYVZoTnnhRQ1OfU7Nee18mbhtukHOEc/59Q4I3igCK+pZQTk5QC1PkVHxW+lnRW2Rd8V6rXydiOGVFZ69uVnLQfMqItIChgOGD8IZOioeN6JI3lseYZ58TTpROkk8NU0hUSVQ+Wi9fjF+hYJ9op2qOdFp4gYqeiqSLd5GQTl6byU6kT3xPr1AZUBZRSVFsUp9SuVL+U5pT41QRVA5ViVdRV6JZfVtUW11bj13lXedd9154XoNeml63XxhgUmFMYpdi2GOnZTtmAmZDZvRnbWghaJdpy2xfbSptaW4vbp11MnaHeGx6P3zgfQV9GH1efbGAFYADgK+AsYFUgY+CKoNSiEyIYYsbjKKM/JDKkXWScXg/kvyVpJZNmAWZmZrYnTtSW1KrU/dUCFjVYvdv4Ixqj1+euVFLUjtUSlb9ekCRd51gntJzRG8JgXB1EV/9YNqaqHLbj7xrZJgDTspW8FdkWL5aWmBoYcdmD2YGaDlosW33ddV9OoJum0JOm09QU8lVBl1vXeZd7mf7bJl0c3gCilCTlojfV1Bep2MrULVQrFGNZwBUyVheWbtbsF9pYk1joWg9a3NuCHB9kcdygHgVeCZ5bWWOfTCD3IjBjwmWm1JkVyhnUH9qjKFRtFdClipYOmmKgLRUsl0OV/x4lZ36T1xSSlSLZD5mKGcUZ/V6hHtWfSKTL2hcm617OVMZUYpSN1vfYvZkrmTmZy1ruoWpltF2kJvWY0yTBpurdr9mUk4JUJhTwlxxYOhkkmVjaF9x5nPKdSN7l36ChpWLg4zbkXiZEGWsZqtri07VTtRPOk9/UjpT+FPyVeNW21jrWctZyVn/W1BcTV4CXitf12AdYwdlL1tcZa9lvWXoZ51rYmt7bA9zRXlJecF8+H0ZfSuAooECgfOJlopeimmKZoqMiu6Mx4zclsyY/GtvTotPPE+NUVBbV1v6YUhjAWZCayFuy2y7cj50vXXUeMF5OoAMgDOB6oSUj55sUJ5/Xw+LWJ0revqO+FuNlutOA1PxV/dZMVrJW6RgiW5/bwZ1vozqW5+FAHvgUHJn9IKdXGGFSn4egg5RmVwEY2iNZmWccW55Pn0XgAWLHY7KkG6Gx5CqUB9S+lw6Z1NwfHI1kUyRyJMrguVbwl8xYPlOO1PWW4hiS2cxa4py6XPgei6Ba42jkVKZllESU9dUalv/Y4hqOX2slwBW2lPOVGhbl1wxXd5P7mEBYv5tMnnAect9Qn5Nf9KB7YIfhJCIRolyi5COdI8vkDGRS5FslsaRnE7AT09RRVNBX5NiDmfUbEFuC3NjfiaRzZKDU9RZGVu/bdF5XX4ufJtYfnGfUfqIU4/wT8pc+2Yld6x644Icmf9Rxl+qZexpb2uJbfNulm9kdv59FF3hkHWRh5gGUeZSHWJAZpFm2W4aXrZ90n9yZviFr4X3ivhSqVPZWXNej1+QYFWS5JZkULdRH1LdUyBTR1PsVOhVRlUxVhdZaFm+WjxbtVwGXA9cEVwaXoReil7gX3Bif2KEYttjjGN3ZgdmDGYtZnZnfmiiah9qNWy8bYhuCW5YcTxxJnFndcd3AXhdeQF5ZXnweuB7EXynfTmAloPWhIuFSYhdiPOKH4o8ilSKc4xhjN6RpJJmk36UGJacl5hOCk4ITh5OV1GXUnBXzlg0WMxbIl44YMVk/mdhZ1ZtRHK2dXN6Y4S4i3KRuJMgVjFX9Jj+Yu1pDWuWce1+VIB3gnKJ5pjfh1WPsVw7TzhP4U+1VQdaIFvdW+lfw2FOYy9lsGZLaO5pm214bfF1M3W5dx95XnnmfTOB44KvhaqJqoo6jquPm5Aykd2XB066TsFSA1h1WOxcC3UaXD2BTooKj8WWY5dteyWKz5gIkWJW81OokBdUOVeCXiVjqGw0cIp3YXyLf+CIcJBCkVSTEJMYlo90XprEXQddaWVwZ6KNqJbbY25nSWkZg8WYF5bAiP5vhGR6W/hOFnAsdV1mL1HEUjZS4lnTX4FgJ2IQZT9ldGYfZnRo8mgWa2NuBXJydR9223y+gFZY8Ij9iX+KoIqTisuQHZGSl1KXWWWJeg6BBpa7Xi1g3GIaZaVmFGeQd/N6TXxNfj6BCoysjWSN4Y5feKlSB2LZY6VkQmKYii16g3vAiqyW6n12ggyHSU7ZUUhTQ1NgW6NcAlwWXd1iJmJHZLBoE2g0bMltRW0XZ9NvXHFOcX1ly3p/e6192n5Kf6iBeoIbgjmFpopujM6N9ZB4kHeSrZKRlYObrlJNVYRvOHE2UWh5hX5VgbN8zlZMWFFcqGOqZv5m/Wlactl1j3WOeQ55VnnffJd9IH1EhgeKNJY7kGGfIFDnUnVTzFPiUAlVqljuWU9yPVuLXGRTHWDjYPNjXGODYz9ju2TNZelm+V3jac1p/W8VceVOiXXpdvh6k3zffc99nIBhg0mDWIRshLyF+4jFjXCQAZBtk5eXHJoSUM9Yl2GOgdOFNY0IkCBPw1B0UkdTc2BvY0lnX24sjbOQH0/XXF6MymXPfZpTUoiWUXZjw1tYW2tcCmQNZ1GQXE7WWRpZKmxwilFVPlgVWaVg8GJTZ8GCNWlVlkCZxJooT1NYBlv+gBBcsV4vX4VgIGFLYjRm/2zwbt6AzoF/gtSIi4y4kACQLpaKntub207jU/BZJ3sskY2YTJ35bt1wJ1NTVURbhWJYYp5i02yib+90IooXlDhvwYr+gzhR54b4U+pT6U9GkFSPsFlqgTFd/Xrqj79o2ow3cvicSGo9irBOOVNYVgZXZmLFY6Jl5mtObeFuW3Ctd+1673uqfbuAPYDGhsuKlZNbVuNYx18+Za1mlmqAa7V1N4rHUCR35VcwXxtgZWZ6bGB19Hoaf26B9IcYkEWZs3vJdVx6+XtRhMSQEHnpepKDNlrhd0BOLU7yW5lf4GK9Zjxn8WzohmuId4o7kU6S85nQahdwJnMqgueEV4yvTgFRRlHLVYtb9V4WXjNegV8UXzVfa1+0YfJjEWaiZx1vbnJSdTp3OoB0gTmBeId2ir+K3I2FjfOSmpV3mAKc5VLFY1d29GcVbIhzzYzDk66Wc20lWJxpDmnMj/2TmnXbkBpYWmgCY7Rp+09Dbyxn2I+7hSZ9tJNUaT9vcFdqWPdbLH0scipUCpHjnbROrU9OUFxQdVJDjJ5USFgkW5peHV6VXq1e918fYIxitWM6Y9Bor2xAeId5jnoLfeCCR4oCiuaORJATkLiRLZHYnw5s5WRYZOJldW70doR7G5Bpk9FuulTyX7lkpI9Nj+2SRFF4WGtZKVxVXpdt+36PdRyMvI7imFtwuU8da79vsXUwlvtRTlQQWDVYV1msXGBfkmWXZ1xuIXZ7g9+M7ZAUkP2TTXgleDpSql6mVx9ZdGASUBJRWlGsUc1SAFUQWFRYWFlXW5Vc9l2LYLxilWQtZ3FoQ2i8aN92123Ybm9tm3BvcchfU3XYeXd7SXtUe1J81n1xUjCEY4VpheSKDosEjEaOD5ADkA+UGZZ2mC2aMJXYUM1S1VQMWAJcDmGnZJ5tHnezeuWA9IQEkFOShVzgnQdTP1+XX7NtnHJ5d2N5v3vka9Jy7IqtaANqYVH4eoFpNFxKnPaC61vFkUlwHlZ4XG9gx2VmbIyMWpBBmBNUUWbHkg1ZSJCjUYVOTVHqhZmLDnBYY3qTS2limbR+BHV3U1dpYI7fluNsXU6MXDxfEI/pUwKM0YCJhnle/2XlTnNRZVmCXD+X7k77WYpfzYqNb+F5sHliW+eEcXMrcbFedF/1Y3tkmnHDfJhOQ178TktX3FaiYKlvw30NgP2BM4G/j7KJl4akXfRiimStiYdnd2zibT50Nng0WkZ/dYKtmaxP817DYt1jkmVXZ292w3JMgMyAuo8pkU1QDVf5WpJohWlzcWRy/Yy3WPKM4JZqkBmHf3nkd+eEKU8vUmVTWmLNZ89synZ9e5R8lYI2hYSP62bdbyByBn4bg6uZwZ6mUf17sXhye7iAh3tIauheYYCMdVF1YFFrkmJujHZ6kZea6k8Qf3BinHtPlaWc6VZ6WFmG5Ja8TzRSJFNKU81T214GZCxlkWd/bD5sTnJIcq9z7XVUfkGCLIXpjKl7xJHGcWmYEpjvYz1maXVqduR40IVDhu5TKlNRVCZZg16HX3xgsmJJYnliq2WQa9RszHWydq54kXnYfct/d4CliKuKuYy7kH+XXpjbagt8OFCZXD5frmeHa9h0NXcJf46fO2fKehdTOXWLmu1fZoGdg/GAmF88X8V1YntGkDxoZ1nrWpt9EHZ+iyxP9V9qahlsN28CdOJ5aIhoilWMeV7fY891xXnSgteTKJLyhJyG7ZwtVMFfbGWMbVxwFYynjNOYO2VPdPZODU7YV+BZK1pmW8xRqF4DXpxgFmJ2ZXdlp2ZubW5yNnsmgVCBmoKZi1yMoIzmjXSWHJZET65kq2tmgh6EYYVqkOhcAWlTmKiEeoVXTw9Sb1+pXkVnDXmPgXmJB4mGbfVfF2JVbLhOz3Jpm5JSBlQ7VnRYs2GkYm5xGllufIl83n0blvBlh4BeThlPdVF1WEBeY15zXwpnxE4mhT2ViZZbfHOYAVD7WMF2VninUiV3pYURe4ZQT1kJckd7x33oj7qP1JBNT79SyVopXwGXrU/dgheS6lcDY1VraXUriNyPFHpCUt9Yk2FVYgpmrmvNfD+D6VAjT/hTBVRGWDFZSVudXPBc710pXpZisWNnZT5luWcLbNVs4XD5eDJ+K4DegrOEDITshwKJEooqjEqQppLSmP2c851sTk9OoVCNUlZXSlmoXj1f2F/ZYj9mtGcbZ9Bo0lGSfSGAqoGoiwCMjIy/kn6WMlQgmCxTF1DVU1xYqGSyZzRyZ3dmekaR5lLDbKFrhlgAXkxZVGcsf/tR4XbGZGl46JtUnrtXy1m5ZidnmmvOVOlp2V5VgZxnlZuqZ/6cUmhdTqZP41PIYrlnK2yrj8RPrX5tnr9OB2FiboBvK4UTVHNnKptFXfN7lVysW8aHHG5KhNF6FIEIWZl8jWwRdyBS2VkicSFyX3fblyedYWkLWn9aGFGlVA1UfWYOdt+P95KYnPRZ6nJdbsVRTWjJfb997JdinrpkeGohgwJZhFtfa9tzG3byfbKAF4SZUTJnKJ7Zdu5nYlL/mQVcJGI7fH6MsFVPYLZ9C5WAUwFOX1G2WRxyOoA2kc5fJXfiU4RfeX0EhayKM46Nl1Zn84WulFNhCWEIbLl2UortjzhVL09RUSpSx1PLW6VefWCgYYJj1mcJZ9puZ22MczZzN3UxeVCI1YqYkEqQkZD1lsSHjVkVTohPWU4OiomPP5gQUK1efFmWW7leuGPaY/pkwWbcaUpp2G0LbrZxlHUoeq9/ioAAhEmEyYmBiyGOCpBlln2ZCmF+YpFrMmyDbXR/zH/8bcB/hYe6iPhnZYOxmDyW920bfWGEPZFqTnFTdV1QawRv64XNhi2Jp1IpVA9cZWdOaKh0BnSDdeKIz4jhkcyW4pZ4X4tzh3rLhE5joHVlUoltQW6cdAl1WXhrfJKWhnrcn41PtmFuZcWGXE6GTq5Q2k4hUcxb7mWZaIFtvHMfdkJ3rXocfOeCb4rSkHyRz5Z1mBhSm33RUCtTmGeXbctx0HQzgeiPKpajnFeen3RgWEFtmX0vmF5O5E82T4tRt1KxXbpgHHOyeTyC05I0lreW9pcKnpefYmama3RSF1KjcMiIwl7JYEthkG8jcUl8Pn30gG+E7pAjkyxUQptvatNwiYzCje+XMlK0WkFeyl8EZxdpfGmUbWpvD3Jicvx77YABgH6HS5DOUW2ek3mEgIuTMorWUC1UjIpxa2qMxIEHYNFnoJ3yTplOmJwQimuFwYVoaQBufniXgVUAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAF8MThBOFU4qTjFONk48Tj9OQk5WTlhOgk6FjGtOioISXw1Ojk6eTp9OoE6iTrBOs062Ts5OzU7ETsZOwk7XTt5O7U7fTvdPCU9aTzBPW09dT1dPR092T4hPj0+YT3tPaU9wT5FPb0+GT5ZRGE/UT99Pzk/YT9tP0U/aT9BP5E/lUBpQKFAUUCpQJVAFTxxP9lAhUClQLE/+T+9QEVAGUENQR2cDUFVQUFBIUFpQVlBsUHhQgFCaUIVQtFCyUMlQylCzUMJQ1lDeUOVQ7VDjUO5Q+VD1UQlRAVECURZRFVEUURpRIVE6UTdRPFE7UT9RQFFSUUxRVFFievhRaVFqUW5RgFGCVthRjFGJUY9RkVGTUZVRllGkUaZRolGpUapRq1GzUbFRslGwUbVRvVHFUclR21HghlVR6VHtUfBR9VH+UgRSC1IUUg5SJ1IqUi5SM1I5Uk9SRFJLUkxSXlJUUmpSdFJpUnNSf1J9Uo1SlFKSUnFSiFKRj6iPp1KsUq1SvFK1UsFSzVLXUt5S41LmmO1S4FLzUvVS+FL5UwZTCHU4Uw1TEFMPUxVTGlMjUy9TMVMzUzhTQFNGU0VOF1NJU01R1lNeU2lTblkYU3tTd1OCU5ZToFOmU6VTrlOwU7ZTw3wSltlT32b8ce5T7lPoU+1T+lQBVD1UQFQsVC1UPFQuVDZUKVQdVE5Uj1R1VI5UX1RxVHdUcFSSVHtUgFR2VIRUkFSGVMdUolS4VKVUrFTEVMhUqFSrVMJUpFS+VLxU2FTlVOZVD1UUVP1U7lTtVPpU4lU5VUBVY1VMVS5VXFVFVVZVV1U4VTNVXVWZVYBUr1WKVZ9Ve1V+VZhVnlWuVXxVg1WpVYdVqFXaVcVV31XEVdxV5FXUVhRV91YWVf5V/VYbVflWTlZQcd9WNFY2VjJWOFZrVmRWL1ZsVmpWhlaAVopWoFaUVo9WpVauVrZWtFbCVrxWwVbDVsBWyFbOVtFW01bXVu5W+VcAVv9XBFcJVwhXC1cNVxNXGFcWVcdXHFcmVzdXOFdOVztXQFdPV2lXwFeIV2FXf1eJV5NXoFezV6RXqlewV8NXxlfUV9JX01gKV9ZX41gLWBlYHVhyWCFYYlhLWHBrwFhSWD1YeViFWLlYn1irWLpY3li7WLhYrljFWNNY0VjXWNlY2FjlWNxY5FjfWO9Y+lj5WPtY/Fj9WQJZClkQWRtoplklWSxZLVkyWThZPnrSWVVZUFlOWVpZWFliWWBZZ1lsWWlZeFmBWZ1PXk+rWaNZslnGWehZ3FmNWdlZ2lolWh9aEVocWglaGlpAWmxaSVo1WjZaYlpqWppavFq+Wstawlq9WuNa11rmWula1lr6WvtbDFsLWxZbMlrQWypbNls+W0NbRVtAW1FbVVtaW1tbZVtpW3Bbc1t1W3hliFt6W4Bbg1umW7hbw1vHW8lb1FvQW+Rb5lviW95b5VvrW/Bb9lvzXAVcB1wIXA1cE1wgXCJcKFw4XDlcQVxGXE5cU1xQXE9bcVxsXG5OYlx2XHlcjFyRXJRZm1yrXLtctly8XLdcxVy+XMdc2VzpXP1c+lztXYxc6l0LXRVdF11cXR9dG10RXRRdIl0aXRldGF1MXVJdTl1LXWxdc112XYddhF2CXaJdnV2sXa5dvV2QXbddvF3JXc1d013SXdZd213rXfJd9V4LXhpeGV4RXhteNl43XkReQ15AXk5eV15UXl9eYl5kXkdedV52XnqevF5/XqBewV7CXshe0F7PXtZe417dXtpe217iXuFe6F7pXuxe8V7zXvBe9F74Xv5fA18JX11fXF8LXxFfFl8pXy1fOF9BX0hfTF9OXy9fUV9WX1dfWV9hX21fc193X4Nfgl9/X4pfiF+RX4dfnl+ZX5hfoF+oX61fvF/WX/tf5F/4X/Ff3WCzX/9gIWBgYBlgEGApYA5gMWAbYBVgK2AmYA9gOmBaYEFgamB3YF9gSmBGYE1gY2BDYGRgQmBsYGtgWWCBYI1g52CDYJpghGCbYJZgl2CSYKdgi2DhYLhg4GDTYLRf8GC9YMZgtWDYYU1hFWEGYPZg92EAYPRg+mEDYSFg+2DxYQ1hDmFHYT5hKGEnYUphP2E8YSxhNGE9YUJhRGFzYXdhWGFZYVpha2F0YW9hZWFxYV9hXWFTYXVhmWGWYYdhrGGUYZphimGRYathrmHMYcphyWH3Ychhw2HGYbphy395Yc1h5mHjYfZh+mH0Yf9h/WH8Yf5iAGIIYgliDWIMYhRiG2IeYiFiKmIuYjBiMmIzYkFiTmJeYmNiW2JgYmhifGKCYolifmKSYpNilmLUYoNilGLXYtFiu2LPYv9ixmTUYshi3GLMYspiwmLHYptiyWMMYu5i8WMnYwJjCGLvYvVjUGM+Y01kHGNPY5ZjjmOAY6tjdmOjY49jiWOfY7Vja2NpY75j6WPAY8Zj42PJY9Jj9mPEZBZkNGQGZBNkJmQ2ZR1kF2QoZA9kZ2RvZHZkTmUqZJVkk2SlZKlkiGS8ZNpk0mTFZMdku2TYZMJk8WTngglk4GThYqxk42TvZSxk9mT0ZPJk+mUAZP1lGGUcZQVlJGUjZStlNGU1ZTdlNmU4dUtlSGVWZVVlTWVYZV5lXWVyZXhlgmWDi4plm2WfZatlt2XDZcZlwWXEZcxl0mXbZdll4GXhZfFncmYKZgNl+2dzZjVmNmY0ZhxmT2ZEZklmQWZeZl1mZGZnZmhmX2ZiZnBmg2aIZo5miWaEZphmnWbBZrlmyWa+ZrxmxGa4ZtZm2mbgZj9m5mbpZvBm9Wb3Zw9nFmceZyZnJ5c4Zy5nP2c2Z0FnOGc3Z0ZnXmdgZ1lnY2dkZ4lncGepZ3xnameMZ4tnpmehZ4Vnt2fvZ7Rn7GezZ+lnuGfkZ95n3WfiZ+5nuWfOZ8Zn52qcaB5oRmgpaEBoTWgyaE5os2graFloY2h3aH9on2iPaK1olGidaJtog2quaLlodGi1aKBoumkPaI1ofmkBaMppCGjYaSJpJmjhaQxozWjUaOdo1Wk2aRJpBGjXaONpJWj5aOBo72koaSppGmkjaSFoxml5aXdpXGl4aWtpVGl+aW5pOWl0aT1pWWkwaWFpXmldaYFpammyaa5p0Gm/acFp02m+ac5b6GnKad1pu2nDaadqLmmRaaBpnGmVabRp3mnoagJqG2n/awpp+WnyaedqBWmxah5p7WoUaetqCmoSasFqI2oTakRqDGpyajZqeGpHamJqWWpmakhqOGoiapBqjWqgaoRqomqjapeGF2q7asNqwmq4arNqrGreatFq32qqatpq6mr7awWGFmr6axJrFpsxax9rOGs3dtxrOZjua0drQ2tJa1BrWWtUa1trX2tha3hreWt/a4BrhGuDa41rmGuVa55rpGuqa6trr2uya7Frs2u3a7xrxmvLa9Nr32vsa+tr82vvnr5sCGwTbBRsG2wkbCNsXmxVbGJsamyCbI1smmyBbJtsfmxobHNskmyQbMRs8WzTbL1s12zFbN1srmyxbL5sumzbbO9s2WzqbR+ITW02bSttPW04bRltNW0zbRJtDG1jbZNtZG1abXltWW2ObZVv5G2FbfluFW4KbbVtx23mbbhtxm3sbd5tzG3obdJtxW36bdlt5G3Vbept7m4tbm5uLm4ZbnJuX24+biNua24rbnZuTW4fbkNuOm5ObiRu/24dbjhugm6qbphuyW63btNuvW6vbsRusm7UbtVuj26lbsJun29BbxFwTG7sbvhu/m8/bvJvMW7vbzJuzG8+bxNu92+Gb3pveG+Bb4Bvb29bb/NvbW+Cb3xvWG+Ob5Fvwm9mb7Nvo2+hb6RvuW/Gb6pv32/Vb+xv1G/Yb/Fv7m/bcAlwC2/6cBFwAXAPb/5wG3Aab3RwHXAYcB9wMHA+cDJwUXBjcJlwknCvcPFwrHC4cLNwrnDfcMtw3XDZcQlw/XEccRlxZXFVcYhxZnFicUxxVnFscY9x+3GEcZVxqHGscddxuXG+cdJxyXHUcc5x4HHscedx9XH8cflx/3INchByG3Ioci1yLHIwcjJyO3I8cj9yQHJGcktyWHJ0cn5ygnKBcodyknKWcqJyp3K5crJyw3LGcsRyznLScuJy4HLhcvly91APcxdzCnMccxZzHXM0cy9zKXMlcz5zTnNPnthzV3Nqc2hzcHN4c3Vze3N6c8hzs3POc7tzwHPlc+5z3nSidAV0b3Qlc/h0MnQ6dFV0P3RfdFl0QXRcdGl0cHRjdGp0dnR+dIt0nnSndMp0z3TUc/F04HTjdOd06XTudPJ08HTxdPh093UEdQN1BXUMdQ51DXUVdRN1HnUmdSx1PHVEdU11SnVJdVt1RnVadWl1ZHVndWt1bXV4dXZ1hnWHdXR1inWJdYJ1lHWadZ11pXWjdcJ1s3XDdbV1vXW4dbx1sXXNdcp10nXZdeN13nX+df91/HYBdfB1+nXydfN2C3YNdgl2H3YndiB2IXYidiR2NHYwdjt2R3ZIdkZ2XHZYdmF2YnZodml2anZndmx2cHZydnZ2eHZ8doB2g3aIdot2jnaWdpN2mXaadrB2tHa4drl2unbCds121nbSdt524Xbldud26oYvdvt3CHcHdwR3KXckdx53JXcmdxt3N3c4d0d3Wndod2t3W3dld393fnd5d453i3eRd6B3nnewd7Z3uXe/d7x3vXe7d8d3zXfXd9p33Hfjd+53/HgMeBJ5JnggeSp4RXiOeHR4hnh8eJp4jHijeLV4qniveNF4xnjLeNR4vni8eMV4ynjseOd42nj9ePR5B3kSeRF5GXkseSt5QHlgeVd5X3laeVV5U3l6eX95inmdeaefS3mqea55s3m5ebp5yXnVeed57HnheeN6CHoNehh6GXogeh95gHoxejt6Pno3ekN6V3pJemF6Ynppn516cHp5en16iHqXepV6mHqWeql6yHqwerZ6xXrEer+Qg3rHesp6zXrPetV603rZetp63XrheuJ65nrtevB7AnsPewp7Bnszexh7GXseezV7KHs2e1B7ensEe017C3tMe0V7dXtle3R7Z3twe3F7bHtue517mHufe417nHuae4t7knuPe117mXvLe8F7zHvPe7R7xnvde+l8EXwUe+Z75XxgfAB8B3wTe/N793wXfA179nwjfCd8KnwffDd8K3w9fEx8Q3xUfE98QHxQfFh8X3xkfFZ8ZXxsfHV8g3yQfKR8rXyifKt8oXyofLN8snyxfK58uXy9fMB8xXzCfNh80nzcfOKbO3zvfPJ89Hz2fPp9Bn0CfRx9FX0KfUV9S30ufTJ9P301fUZ9c31WfU59cn1ofW59T31jfZN9iX1bfY99fX2bfbp9rn2jfbV9x329fat+PX2ifa993H24fZ99sH3Yfd195H3efft98n3hfgV+Cn4jfiF+En4xfh9+CX4LfiJ+Rn5mfjt+NX45fkN+N34yfjp+Z35dflZ+Xn5Zflp+eX5qfml+fH57foN91X59j65+f36Ifol+jH6SfpB+k36UfpZ+jn6bfpx/OH86f0V/TH9Nf05/UH9Rf1V/VH9Yf19/YH9of2l/Z394f4J/hn+Df4h/h3+Mf5R/nn+df5p/o3+vf7J/uX+uf7Z/uItxf8V/xn/Kf9V/1H/hf+Z/6X/zf/mY3IAGgASAC4ASgBiAGYAcgCGAKIA/gDuASoBGgFKAWIBagF+AYoBogHOAcoBwgHaAeYB9gH+AhICGgIWAm4CTgJqArVGQgKyA24DlgNmA3YDEgNqA1oEJgO+A8YEbgSmBI4EvgUuWi4FGgT6BU4FRgPyBcYFugWWBZoF0gYOBiIGKgYCBgoGggZWBpIGjgV+Bk4GpgbCBtYG+gbiBvYHAgcKBuoHJgc2B0YHZgdiByIHagd+B4IHngfqB+4H+ggGCAoIFggeCCoINghCCFoIpgiuCOIIzgkCCWYJYgl2CWoJfgmSCYoJogmqCa4IugnGCd4J4gn6CjYKSgquCn4K7gqyC4YLjgt+C0oL0gvOC+oOTgwOC+4L5gt6DBoLcgwmC2YM1gzSDFoMygzGDQIM5g1CDRYMvgyuDF4MYg4WDmoOqg5+DooOWgyODjoOHg4qDfIO1g3ODdYOgg4mDqIP0hBOD64POg/2EA4PYhAuDwYP3hAeD4IPyhA2EIoQgg72EOIUGg/uEbYQqhDyFWoSEhHeEa4SthG6EgoRphEaELIRvhHmENYTKhGKEuYS/hJ+E2YTNhLuE2oTQhMGExoTWhKGFIYT/hPSFF4UYhSyFH4UVhRSE/IVAhWOFWIVIhUGGAoVLhVWFgIWkhYiFkYWKhaiFbYWUhZuF6oWHhZyFd4V+hZCFyYW6hc+FuYXQhdWF3YXlhdyF+YYKhhOGC4X+hfqGBoYihhqGMIY/hk1OVYZUhl+GZ4ZxhpOGo4aphqqGi4aMhraGr4bEhsaGsIbJiCOGq4bUht6G6Ybsht+G24bvhxKHBocIhwCHA4b7hxGHCYcNhvmHCoc0hz+HN4c7hyWHKYcah2CHX4d4h0yHTod0h1eHaIduh1mHU4djh2qIBYeih5+Hgoevh8uHvYfAh9CW1oerh8SHs4fHh8aHu4fvh/KH4IgPiA2H/of2h/eIDofSiBGIFogViCKIIYgxiDaIOYgniDuIRIhCiFKIWYheiGKIa4iBiH6Inoh1iH2ItYhyiIKIl4iSiK6ImYiiiI2IpIiwiL+IsYjDiMSI1IjYiNmI3Yj5iQKI/Ij0iOiI8okEiQyJCokTiUOJHokliSqJK4lBiUSJO4k2iTiJTIkdiWCJXolmiWSJbYlqiW+JdIl3iX6Jg4mIiYqJk4mYiaGJqYmmiayJr4myibqJvYm/icCJ2oncid2J54n0ifiKA4oWihCKDIobih2KJYo2ikGKW4pSikaKSIp8im2KbIpiioWKgoqEiqiKoYqRiqWKpoqaiqOKxIrNisKK2orrivOK54rkivGLFIrgiuKK94reituLDIsHixqK4YsWixCLF4sgizOXq4smiyuLPosoi0GLTItPi06LSYtWi1uLWotri1+LbItvi3SLfYuAi4yLjouSi5OLlouZi5qMOoxBjD+MSIxMjE6MUIxVjGKMbIx4jHqMgoyJjIWMioyNjI6MlIx8jJhiHYytjKqMvYyyjLOMroy2jMiMwYzkjOOM2oz9jPqM+40EjQWNCo0HjQ+NDY0Qn06NE4zNjRSNFo1njW2NcY1zjYGNmY3Cjb6Nuo3PjdqN1o3MjduNy43qjeuN343jjfyOCI4Jjf+OHY4ejhCOH45CjjWOMI40jkqOR45JjkyOUI5IjlmOZI5gjiqOY45VjnaOco58joGOh46FjoSOi46KjpOOkY6UjpmOqo6hjqyOsI7GjrGOvo7FjsiOy47bjuOO/I77juuO/o8KjwWPFY8SjxmPE48cjx+PG48MjyaPM487jzmPRY9Cjz6PTI9Jj0aPTo9Xj1yPYo9jj2SPnI+fj6OPrY+vj7eP2o/lj+KP6o/vkIeP9JAFj/mP+pARkBWQIZANkB6QFpALkCeQNpA1kDmP+JBPkFCQUZBSkA6QSZA+kFaQWJBekGiQb5B2lqiQcpCCkH2QgZCAkIqQiZCPkKiQr5CxkLWQ4pDkYkiQ25ECkRKRGZEykTCRSpFWkViRY5FlkWmRc5FykYuRiZGCkaKRq5GvkaqRtZG0kbqRwJHBkcmRy5HQkdaR35HhkduR/JH1kfaSHpH/khSSLJIVkhGSXpJXkkWSSZJkkkiSlZI/kkuSUJKckpaSk5KbklqSz5K5kreS6ZMPkvqTRJMukxmTIpMakyOTOpM1kzuTXJNgk3yTbpNWk7CTrJOtk5STuZPWk9eT6JPlk9iTw5Pdk9CTyJPklBqUFJQTlAOUB5QQlDaUK5Q1lCGUOpRBlFKURJRblGCUYpRelGqSKZRwlHWUd5R9lFqUfJR+lIGUf5WClYeVipWUlZaVmJWZlaCVqJWnla2VvJW7lbmVvpXKb/aVw5XNlcyV1ZXUldaV3JXhleWV4pYhliiWLpYvlkKWTJZPlkuWd5Zcll6WXZZflmaWcpZslo2WmJaVlpeWqpanlrGWspawlrSWtpa4lrmWzpbLlsmWzYlNltyXDZbVlvmXBJcGlwiXE5cOlxGXD5cWlxmXJJcqlzCXOZc9lz6XRJdGl0iXQpdJl1yXYJdkl2aXaFLSl2uXcZd5l4WXfJeBl3qXhpeLl4+XkJecl6iXppejl7OXtJfDl8aXyJfLl9yX7Z9Pl/J635f2l/WYD5gMmDiYJJghmDeYPZhGmE+YS5hrmG+YcJhxmHSYc5iqmK+YsZi2mMSYw5jGmOmY65kDmQmZEpkUmRiZIZkdmR6ZJJkgmSyZLpk9mT6ZQplJmUWZUJlLmVGZUplMmVWZl5mYmaWZrZmumbyZ35nbmd2Z2JnRme2Z7pnxmfKZ+5n4mgGaD5oFmeKaGZormjeaRZpCmkCaQ5o+mlWaTZpbmleaX5pimmWaZJppmmuaapqtmrCavJrAms+a0ZrTmtSa3prfmuKa45rmmu+a65rumvSa8Zr3mvubBpsYmxqbH5simyObJZsnmyibKZsqmy6bL5sym0SbQ5tPm02bTptRm1ibdJuTm4ObkZuWm5ebn5ugm6ibtJvAm8qbuZvGm8+b0ZvSm+Ob4pvkm9Sb4Zw6m/Kb8ZvwnBWcFJwJnBOcDJwGnAicEpwKnAScLpwbnCWcJJwhnDCcR5wynEacPpxanGCcZ5x2nHic55zsnPCdCZ0InOudA50GnSqdJp2vnSOdH51EnRWdEp1BnT+dPp1GnUidXZ1enWSdUZ1QnVmdcp2JnYedq51vnXqdmp2knamdsp3EncGdu524nbqdxp3PncKd2Z3Tnfid5p3tne+d/Z4anhueHp51nnmefZ6Bnoiei56MnpKelZ6Rnp2epZ6pnrieqp6tl2GezJ7Ons+e0J7Untye3p7dnuCe5Z7onu+e9J72nvee+Z77nvye/Z8Hnwh2t58VnyGfLJ8+n0qfUp9Un2OfX59gn2GfZp9nn2yfap93n3Kfdp+Vn5yfoFgvaceQWXRkUdxxmQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

/// 簡易的なBase64デコーダです。
///
/// # 引数
///
/// * `s` - デコード対象のBase64文字列
///
/// # 戻り値
///
/// デコードされたバイト列を返します。
pub fn decode_base64(s: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;
    for c in s.chars() {
        let val = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' => 62,
            '/' => 63,
            _ => continue,
        };
        buffer = (buffer << 6) | val;
        bits += 6;
        while bits >= 8 {
            bits -= 8;
            bytes.push((buffer >> bits) as u8);
        }
    }
    bytes
}

/// JISとUnicodeのマッピングテーブル（94x94の配列に相当）を構築します。
///
/// # 戻り値
///
/// 各インデックスがJISコードポイントに対応し、値がUnicodeコードポイントである `Vec<u16>` を返します。
pub fn load_jis_table() -> Vec<u16> {
    let raw_bytes = decode_base64(JIS_TO_UNICODE_BASE64);
    let mut table = Vec::with_capacity(raw_bytes.len() / 2);
    for chunk in raw_bytes.chunks_exact(2) {
        let code_point = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        table.push(code_point);
    }
    table
}

/// 与えられたバイト列の文字コード（UTF-8, Shift_JIS, EUC-JP, ASCII）を自動検出します。
///
/// # 引数
///
/// * `bytes` - 判定対象 of バイト列
///
/// # 戻り値
///
/// 検出された `Encoding` を返します。検出できなかった場合は `Encoding::Unknown` を返します。
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

/// Shift_JISの文字コード（2バイト）をEUC-JPの文字コードに変換します。
///
/// # 引数
///
/// * `s1` - Shift_JISの第1バイト
/// * `s2` - Shift_JISの第2バイト
///
/// # 戻り値
///
/// 変換に成功した場合は `Some((e1, e2))`、無効なバイトの場合は `None` を返します。
pub fn sjis_to_eucjp(s1: u8, s2: u8) -> Option<(u8, u8)> {
    let s1_val = s1 as i32;
    let s2_val = s2 as i32;
    let temp1 = if (0x81..=0x9F).contains(&s1_val) {
        s1_val - 0x81
    } else if (0xE0..=0xFC).contains(&s1_val) {
        s1_val - 0xE0 + 31
    } else {
        return None;
    };
    let temp2 = if (0x40..=0x7E).contains(&s2_val) {
        s2_val - 0x40
    } else if (0x80..=0xFC).contains(&s2_val) {
        s2_val - 0x80 + 63
    } else {
        return None;
    };
    let ku = temp1 * 2 + if temp2 < 94 { 1 } else { 2 };
    let ten = if temp2 < 94 {
        temp2 + 1
    } else {
        temp2 - 94 + 1
    };
    let e1 = ku + 0xA0;
    let e2 = ten + 0xA0;
    if (0xA1..=0xFE).contains(&e1) && (0xA1..=0xFE).contains(&e2) {
        Some((e1 as u8, e2 as u8))
    } else {
        None
    }
}

/// EUC-JPの文字コード（2バイト）をShift_JISの文字コードに変換します。
///
/// # 引数
///
/// * `e1` - EUC-JPの第1バイト
/// * `e2` - EUC-JPの第2バイト
///
/// # 戻り値
///
/// 変換後のShift_JISコード `(s1, s2)` を返します。
pub fn eucjp_to_sjis(e1: u8, e2: u8) -> (u8, u8) {
    let ku = e1 as i32 - 0xA0;
    let ten = e2 as i32 - 0xA0;
    let s1 = if ku % 2 == 1 {
        (ku + 1) / 2 + 0x80
    } else {
        ku / 2 + 0x80
    };
    let s1 = if s1 >= 0xA0 { s1 + 0x40 } else { s1 };

    let s2 = if ku % 2 == 1 {
        if ten >= 64 { ten + 0x40 } else { ten + 0x3F }
    } else {
        ten + 0x9E
    };
    (s1 as u8, s2 as u8)
}

/// 指定されたバイト列を指定された文字コードからUnicode（`Vec<char>`）にデコードします。
///
/// マッピングテーブルを使用して、Shift_JISやEUC-JPからの変換を行います。
///
/// # 引数
///
/// * `bytes` - デコード対象のバイト列
/// * `from_enc` - デコード元の文字コード
/// * `table` - JIS-Unicodeマッピングテーブル
///
/// # 戻り値
///
/// 変換された `Vec<char>` を返します。
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

/// Unicodeの文字スライス（`chars`）を指定された文字コードのバイト列にエンコードします。
///
/// # 引数
///
/// * `chars` - エンコード対象の文字スライス
/// * `to_enc` - エンコード先の文字コード
/// * `unicode_to_jis` - UnicodeからJISへのマッピングハッシュマップ
/// * `actual_crlf` - 改行コードをCRLFにするか（`true`でCRLF、`false`でLF）
///
/// # 戻り値
///
/// エンコードされたバイト列 `Vec<u8>` を返します。
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

#[cfg(test)]
mod tests;
