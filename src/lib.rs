// =========================================================================
// MyNKF Library Module
// Contains core encoding detection and conversion logic
// =========================================================================

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Ascii,
    Utf8,
    Sjis,
    EucJp,
    Unknown,
}

impl Encoding {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    Crlf,
    Cr,
    Mixed,
    None,
}

impl LineEnding {
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

pub const MAX_GLOB_FILES: usize = 100;

pub fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    let mut p_idx = 0;
    let mut t_idx = 0;
    let mut p_star = None;
    let mut t_star = None;

    while t_idx < text_chars.len() {
        if p_idx < pattern_chars.len() && (pattern_chars[p_idx] == '?' || pattern_chars[p_idx] == text_chars[t_idx]) {
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

pub fn expand_wildcard(arg: &str, files: &mut Vec<String>) -> Result<(), String> {
    if !arg.contains('*') && !arg.contains('?') {
        files.push(arg.to_string());
        if files.len() > MAX_GLOB_FILES {
            return Err(format!("Maximum limit of {} files exceeded.", MAX_GLOB_FILES));
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
            return Err(format!("Failed to read directory '{:?}': {}", dir_to_read, err));
        }
    };

    let mut matched_any = false;
    for entry in entries {
        if let Ok(entry) = entry {
            let file_type = match entry.file_type() {
                Ok(t) => t,
                Err(_) => continue,
            };
            if file_type.is_file() {
                if let Some(name_str) = entry.file_name().to_str() {
                    if wildcard_match(&file_pattern.to_lowercase(), &name_str.to_lowercase()) {
                        let matched_path = if parent_dir.as_os_str().is_empty() {
                            std::path::PathBuf::from(entry.file_name())
                        } else {
                            parent_dir.join(entry.file_name())
                        };
                        if let Some(path_str) = matched_path.to_str() {
                            files.push(path_str.to_string());
                            matched_any = true;
                            if files.len() > MAX_GLOB_FILES {
                                return Err(format!("Maximum limit of {} files exceeded.", MAX_GLOB_FILES));
                            }
                        }
                    }
                }
            }
        }
    }

    if !matched_any {
        files.push(arg.to_string());
    }

    Ok(())
}

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

pub const JIS_TO_UNICODE_BASE64: &str = "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FsUp9SuVL+U5pT41QRVA5ViVdRV6JZfVtUW11bj13lXedd9154XoNeml63XxhgUmFMYpdi2GOnZTtmAmZDZvRnbWghaJdpy2xfbSptaW4vbp11MnaHeGx6P3zgfQV9GH1efbGAFYADgK+AsYFUgY+CKoNSiEyIYYsbjKKM/JDKkXWScXg/kvyVpJZNmAWZmZrYnTtSW1KrU/dUCFjVYvdv4Ixqj1+euVFLUjtUSlb9ekCRd51gntJzRG8JgXB1EV/9YNqaqHLbj7xrZJgDTspW8FdkWL5aWmBoYcdmD2YGaDlosW33ddV9OoJum0JOm09QU8lVBl1vXeZd7mf7bJl0c3gCilCTlojfV1Bep2MrULVQrFGNZwBUyVheWbtbsF9pYk1joWg9a3NuCHB9kcdygHgVeCZ5bWWOfTCD3IjBjwmWm1JkVyhnUH9qjKFRtFdClipYOmmKgLRUsl0OV/x4lZ36T1xSSlSLZD5mKGcUZ/V6hHtWfSKTL2hcm617OVMZUYpSN1vfYvZkrmTmZy1ruoWpltF2kJvWY0yTBpurdr9mUk4JUJhTwlxxYOhkkmVjaF9x5nPKdSN7l36ChpWLg4zbkXiZEGWsZqtri07VTtRPOk9/UjpT+FPyVeNW21jrWctZyVn/W1BcTV4CXitf12AdYwdlL1tcZa9lvWXoZ51rYmt7bA9zRXlJecF8+H0ZfSuAooECgfOJlopeimmKZoqMiu6Mx4zclsyY/GtvTotPPE+NUVBbV1v6YUhjAWZCayFuy2y7cj50vXXUeMF5OoAMgDOB6oSUj55sUJ5/Xw+LWJ0revqO+FuNlutOA1PxV/dZMVrJW6RgiW5/bwZ1vozqW5+FAHvgUHJn9IKdXGGFSn4egg5RmVwEY2iNZmWccW55Pn0XgAWLHY7KkG6Gx5CqUB9S+lw6Z1NwfHI1kUyRyJMrguVbwl8xYPlOO1PWW4hiS2cxa4py6XPgei6Ba42jkVKZllESU9dUalv/Y4hqOX2slwBW2lPOVGhbl1wxXd5P7mEBYv5tMnnAect9Qn5Nf9KB7YIfhJCIRolyi5COdI8vkDGRS5FslsaRnE7AT09RRVNBX5NiDmfUbEFuC3NjfiaRzZKDU9RZGVu/bdF5XX4ufJtYfnGfUfqIU4/wT8pc+2Yld6x644Icmf9Rxl+qZexpb2uJbfNulm9kdv59FF3hkHWRh5gGUeZSHWJAZpFm2W4aXrZ90n9yZviFr4X3ivhSqVPZWXNej1+QYFWS5JZkULdRH1LdUyBTR1PsVOhVRlUxVhdZaFm+WjxbtVwGXA9cEVwaXoReil7gX3Bif2KEYttjjGN3ZgdmDGYtZnZnfmiiah9qNWy8bYhuCW5YcTxxJnFndcd3AXhdeQF5ZXnweuB7EXynfTmAloPWhIuFSYhdiPOKH4o8ilSKc4xhjN6RpJJmk36UGJacl5hOCk4ITh5OV1GXUnBXzlg0WMxbIl44YMVk/mdhZ1ZtRHK2dXN6Y4S4i3KRuJMgVjFX9Jj+Yu1pDWuWce1+VIB3gnKJ5pjfh1WPsVw7TzhP4U+1VQdaIFvdW+lfw2FOYy9lsGZLaO5pm214bfF1M3W5dx95XnnmfTOB44KvhaqJqoo6jquPm5Aykd2XB066TsFSA1h1WOxcC3UaXD2BTooKj8WWY5dteyWKz5gIkWJW81OokBdUOVeCXiVjqGw0cIp3YXyLf+CIcJBCkVSTEJMYlo90XprEXQddaWVwZ6KNqJbbY25nSWkZg8WYF5bAiP5vhGR6W/hOFnAsdV1mL1HEUjZS4lnTX4FgJ2IQZT9ldGYfZnRo8mgWa2NuBXJydR9223y+gFZY8Ij9iX+KoIqTisuQHZGSl1KXWWWJeg6BBpa7Xi1g3GIaZaVmFGeQd/N6TXxNfj6BCoysjWSN4Y5feKlSB2LZY6VkQmKYii16g3vAiqyW6n12ggyHSU7ZUUhTQ1NgW6NcAlwWXd1iJmJHZLBoE2g0bMltRW0XZ9NvXHFOcX1ly3p/e6192n5Kf6iBeoIbgjmFpopujM6N9ZB4kHeSrZKRlYObrlJNVYRvOHE2UWh5hX5VgbN8zlZMWFFcqGOqZv5m/Wlactl1j3WOeQ55VnnffJd9IH1EhgeKNJY7kGGfIFDnUnVTzFPiUAlVqljuWU9yPVuLXGRTHWDjYPNjXGODYz9ju2TNZelm+V3jac1p/W8VceVOiXXpdvh6k3zffc99nIBhg0mDWIRshLyF+4jFjXCQAZBtk5eXHJoSUM9Yl2GOgdOFNY0IkCBPw1B0UkdTc2BvY0lnX24sjbOQH0/XXF6MymXPfZpTUoiWUXZjw1tYW2tcCmQNZ1GQXE7WWRpZKmxwilFVPlgVWaVg8GJTZ8GCNWlVlkCZxJooT1NYBlv+gBBcsV4vX4VgIGFLYjRm/2zwbt6AzoF/gtSIi4y4kACQLpaKntub207jU/BZJ3sskY2YTJ35bt1wJ1NTVURbhWJYYp5i02yib+90IooXlDhvwYr+gzhR54b4U+pT6U9GkFSPsFlqgTFd/Xrqj79o2ow3cvicSGo9irBOOVNYVgZXZmLFY6Jl5mtObeFuW3Ctd+1673uqfbuAPYDGhsuKlZNbVuNYx18+Za1mlmqAa7V1N4rHUCR35VcwXxtgZWZ6bGB19Hoaf26B9IcYkEWZs3vJdVx6+XtRhMSQEHnpepKDNlrhd0BOLU7yW5lf4GK9Zjxn8WzohmuId4o7kU6S85nQahdwJnMqgueEV4yvTgFRRlHLVYtb9V4WXjNegV8UXzVfa1+0YfJjEWaiZx1vbnJSdT33OoB0gTmBeId2ir+K3I2FjfOSmpV3mAKc5VLFY1d29GcVbIhzzYzDk66Wc20lWJxpDmnMj/2TmnXbkBpYWmgCY7Rp+09Dbyxn2I+7hSZ9tJNUaT9vcFdqWPdbLH0scipUCpHjnbROrU9OUFxQdVJDjJ5USFgkW5peHV6VXq1e918fYIxitWM6Y9Bor2xAeId5jnoLfeCCR4oCiuaORJATkLiRLZHYnw5s5WRYZOJldW70doR7G5Bpk9FuulTyX7lkpI9Nj+2SRFF4WGtZKVxVXpdt+36PdRyMvI7imFtwuU8da79vsXUwlvtRTlQQWDVYV1msXGBfkmWXZ1xuIXZ7g9+M7ZAUkP2TTXgleDpSql6mVx9ZdGASUBJRWlGsUc1SAFUQWFRYWFlXW5Vc9l2LYLxilWQtZ3FoQ2i8aN92123Ybm9tm3BvcchfU3XYeXd7SXtUe1J81n1xUjCEY4VpheSKDosEjEaOD5ADkA+UGZZ2mC2aMJXYUM1S1VQMWAJcDmGnZJ5tHnezeuWA9IQEkFOShVzgnQdTP1+XX7NtnHJ5d2N5v3vka9Jy7IqtaANqYVH4eoFpNFxKnPaC61vFkUlwHlZ4XG9gx2VmbIyMWpBBmBNUUWbHkg1ZSJCjUYVOTVHqhZmLDnBYY3qTS2limbR+BHV3U1dpYI7fluNsXU6MXDxfEI/pUwKM0YCJhnle/2XlTnNRZVmCXD+X7k77WYpfzYqNb+F5sHliW+eEcXMrcbFedF/1Y3tkmnHDfJhOQ178TktX3FaiYKlvw30NgP2BM4G/j7KJl4akXfRiimStiYdnd2zibT50Nng0WkZ/dYKtmaxP817DYt1jkmVXZ292w3JMgMyAuo8pkU1QDVf5WpJohWlzcWRy/Yy3WPKM4JZqkBmHf3nkd+eEKU8vUmVTWmLNZ89synZ9e5R8lYI2hYSP62bdbyByBn4bg6uZwZ6mUf17sXhye7iAh3tIauheYYCMdVF1YFFrkmJujHZ6kZea6k8Qf3BinHtPlaWc6VZ6WFmG5Ja8TzRSJFNKU81T214GZCxlkWd/bD5sTnJIcq9z7XVUfkGCLIXpjKl7xJHGcWmYEpjvYz1maXVqduR40IVDhu5TKlNRVCZZg16HX3xgsmJJYnliq2WQa9RszHWydq54kXnYfct/d4CliKuKuYy7kH+XXpjbagt8OFCZXD5frmeHa9h0NXcJf46fO2fKehdTOXWLmu1fZoGdg/GAmF88X8V1YntGkDxoZ1nrWpt9EHZ+iyxP9V9qahlsN28CdOJ5aIhoilWMeV7fY891xXnSgteTKJLyhJyG7ZwtVMFfbGWMbVxwFYynjNOYO2VPdPZODU7YV+BZK1pmW8xRqF4DXpxgFmJ2ZXdlp2ZubW5yNnsmgVCBmoKZi1yMoIzmjXSWHJZET65kq2tmgh6EYYVqkOhcAWlTmKiEeoVXTw9Sb1+pXkVnDXmPgXmJB4mGbfVfF2JVbLhOz3Jpm5JSBlQ7VnRYs2GkYm5xGllufIl83n0ag=";

// 簡易Base64デコーダ
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

// マッピングテーブルの構築
pub fn load_jis_table() -> Vec<u16> {
    let raw_bytes = decode_base64(JIS_TO_UNICODE_BASE64);
    let mut table = Vec::with_capacity(raw_bytes.len() / 2);
    for chunk in raw_bytes.chunks_exact(2) {
        let code_point = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        table.push(code_point);
    }
    table
}

// 文字コード判定ステートマシン
pub fn guess_encoding(bytes: &[u8]) -> Encoding {
    if bytes.is_empty() {
        return Encoding::Ascii;
    }
    if bytes.iter().all(|&b| b < 0x80) {
        return Encoding::Ascii;
    }

    let mut is_utf8 = true;
    let mut is_sjis = true;
    let mut is_euc = true;

    let mut utf8_score = 0;
    let mut sjis_score = 0;
    let mut euc_score = 0;

    let mut utf8_needed = 0;
    let mut sjis_needed = 0;
    let mut euc_needed = 0;

    for &b in bytes {
        // UTF-8 判定
        if is_utf8 {
            if utf8_needed > 0 {
                if (b & 0xC0) == 0x80 {
                    utf8_needed -= 1;
                    if utf8_needed == 0 { utf8_score += 2; }
                } else {
                    is_utf8 = false;
                }
            } else {
                if b < 0x80 {
                    // ASCII
                } else if (b & 0xE0) == 0xC0 {
                    utf8_needed = 1;
                } else if (b & 0xF0) == 0xE0 {
                    utf8_needed = 2;
                } else if (b & 0xF8) == 0xF0 {
                    utf8_needed = 3;
                } else {
                    is_utf8 = false;
                }
            }
        }

        // Shift_JIS 判定
        if is_sjis {
            if sjis_needed > 0 {
                if (b >= 0x40 && b <= 0x7E) || (b >= 0x80 && b <= 0xFC) {
                    sjis_needed = 0;
                    sjis_score += 2;
                } else {
                    is_sjis = false;
                }
            } else {
                if b < 0x80 {
                    // ASCII
                } else if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xFC) {
                    sjis_needed = 1;
                } else if b >= 0xA1 && b <= 0xDF {
                    sjis_score += 1;
                } else {
                    is_sjis = false;
                }
            }
        }

        // EUC-JP 判定
        if is_euc {
            if euc_needed > 0 {
                if b >= 0xA1 && b <= 0xFE {
                    euc_needed -= 1;
                    if euc_needed == 0 { euc_score += 2; }
                } else {
                    is_euc = false;
                }
            } else {
                if b < 0x80 {
                    // ASCII
                } else if b == 0x8E {
                    euc_needed = 1;
                } else if b == 0x8F {
                    euc_needed = 2;
                } else if b >= 0xA1 && b <= 0xFE {
                    euc_needed = 1;
                } else {
                    is_euc = false;
                }
            }
        }
    }

    if utf8_needed > 0 { is_utf8 = false; }
    if sjis_needed > 0 { is_sjis = false; }
    if euc_needed > 0 { is_euc = false; }

    if is_utf8 && !is_sjis && !is_euc { return Encoding::Utf8; }
    if !is_utf8 && is_sjis && !is_euc { return Encoding::Sjis; }
    if !is_utf8 && !is_sjis && is_euc { return Encoding::EucJp; }

    let max_score = utf8_score.max(sjis_score).max(euc_score);
    if max_score == 0 {
        if is_utf8 { return Encoding::Utf8; }
        if is_sjis { return Encoding::Sjis; }
        if is_euc { return Encoding::EucJp; }
        return Encoding::Unknown;
    }

    if is_utf8 && utf8_score == max_score { return Encoding::Utf8; }
    if is_sjis && sjis_score == max_score { return Encoding::Sjis; }
    if is_euc && euc_score == max_score { return Encoding::EucJp; }

    Encoding::Unknown
}

pub fn sjis_to_eucjp(s1: u8, s2: u8) -> Option<(u8, u8)> {
    let s1_val = s1 as i32;
    let s2_val = s2 as i32;
    let temp1 = if s1_val >= 0x81 && s1_val <= 0x9F {
        s1_val - 0x81
    } else if s1_val >= 0xE0 && s1_val <= 0xFC {
        s1_val - 0xE0 + 31
    } else {
        return None;
    };
    let temp2 = if s2_val >= 0x40 && s2_val <= 0x7E {
        s2_val - 0x40
    } else if s2_val >= 0x80 && s2_val <= 0xFC {
        s2_val - 0x80 + 63
    } else {
        return None;
    };
    let ku = temp1 * 2 + if temp2 < 94 { 1 } else { 2 };
    let ten = if temp2 < 94 { temp2 + 1 } else { temp2 - 94 + 1 };
    let e1 = ku + 0xA0;
    let e2 = ten + 0xA0;
    if e1 >= 0xA1 && e1 <= 0xFE && e2 >= 0xA1 && e2 <= 0xFE {
        Some((e1 as u8, e2 as u8))
    } else {
        None
    }
}

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
                } else if b1 >= 0xA1 && b1 <= 0xDF {
                    let code = 0xFF61 + (b1 as u32 - 0xA1);
                    chars.push(std::char::from_u32(code).unwrap_or('?'));
                    i += 1;
                } else if (b1 >= 0x81 && b1 <= 0x9F) || (b1 >= 0xE0 && b1 <= 0xFC) {
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
                        if b2 >= 0xA1 && b2 <= 0xDF {
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
                } else if b1 >= 0xA1 && b1 <= 0xFE {
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
        } else if uni >= 0xFF61 && uni <= 0xFF9F {
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
mod tests {
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
        assert_eq!(detect_line_ending(b"hello\nworld\r\ntest"), LineEnding::Mixed);
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
        assert_eq!(res.unwrap_err(), format!("Maximum limit of {} files exceeded.", MAX_GLOB_FILES));

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
        assert_eq!(format_size(1024 * 1024 * 1024 * 3 + 1024 * 1024 * 1024 / 2), "3.5 GB");
    }
}
