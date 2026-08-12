**English** | [日本語版](../ja/TESTING.md)

# Test Plan & Verification Report (TESTING.md)

This document records validation reports, test checklists, and execution logs to ensure encoding heuristics, conversion maps, simulator functionalities, and native desktop window controls work correctly.

---

## 1. Test Environment
- **Operating System**: Windows 11 / Windows 10
- **Runtimes**: Rust 1.70+ / Node.js 18+
- **Execution Date**: 2026-07-22

---

## 2. Command-Line Tests and Results

### 2.1 Help and Version Flags

| Test Item | Command / Operations | Expected Result | Pass/Fail |
| :--- | :--- | :--- | :--- |
| **Help Details (`--help`)** | `MyNKF --help` or `MyNKF -h` | Formatted lists of CLI usage, parameters, and version flags print cleanly. | **Pass** |
| **Version Details (`--version`)** | `MyNKF --version` or `MyNKF -v` | Outputs `MyNKF v1.1.0` and terminates with exit code 0. | **Pass** |
| **Version Alias (`--versio`)** | `MyNKF --versio` | Outputs `MyNKF v1.1.0` and terminates with exit code 0. | **Pass** |

### 2.2 Encoding Conversion Accuracy

| Test Item | Command / Operations | Expected Result | Pass/Fail |
| :--- | :--- | :--- | :--- |
| **UTF-8 ⇆ Shift_JIS** | `MyNKF -s input_utf8.txt > out.txt` | Japanese text translates cleanly without Mojibake. Newline normalizes to `CRLF`. | **Pass** |
| **EUC-JP ⇆ UTF-8** | `MyNKF -w input_euc.txt > out.txt` | Documents written in EUC-JP convert to UTF-8. Newline normalizes to `LF`. | **Pass** |
| **Half-width Katakana** | `MyNKF -w sjis_with_kana.txt` | Half-width Katakana characters (like `ｱ`) are preserved in UTF-8 without converting to full-width. | **Pass** |
| **Fallback Handling** | `MyNKF -s emoji_utf8.txt` | Unsupported characters (like emojis) substitute safely with `??` instead of panicking. | **Pass** |

### 2.3 Heuristic Guessing (`--guess`)

| Test Item | Input Content | Expected Result | Pass/Fail |
| :--- | :--- | :--- | :--- |
| **ASCII guess** | `Hello World!` (plain text) | `ASCII` | **Pass** |
| **UTF-8 guess** | UTF-8 encoded Japanese characters | `UTF-8` | **Pass** |
| **Shift_JIS guess** | Shift_JIS encoded Japanese characters | `Shift_JIS` | **Pass** |
| **EUC-JP guess** | EUC-JP encoded Japanese characters | `EUC-JP` | **Pass** |
| **BINARY guess** | Arbitrary binary byte arrays (`0x00`, `0xFF` etc.) | `BINARY` | **Pass** |

### 2.4 Error Codes and Invalid Flags

| Test Item | Command / Operations | Expected Result | Pass/Fail |
| :--- | :--- | :--- | :--- |
| **Invalid Flags** | `MyNKF --verison` or `MyNKF -x` | Outputs `Error: Unknown option` to stderr and terminates immediately with exit code `1`. | **Pass** |

---

## 3. Web Desktop Simulator Integration Checklist

Validating browser-based simulation features:

1. **File Drag & Drop**:
   - Dropping target files onto the simulator zone immediately parses metadata and reveals text streams. (Pass)
2. **Download Functions**:
   - Converts buffers into Shift_JIS (CRLF), UTF-8 (LF), or EUC-JP (LF) binary blobs, triggering local browser file downloads. (Pass)
3. **CLI Terminal**:
   - Inputting `MyNKF -w sample.txt` or `MyNKF --guess sample.txt` inside CLI simulators feeds stdout outputs onto console logs. (Pass)

---

## 4. Automated Tests (`cargo test`)

17 unit test cases are implemented inside `src/tests.rs` to validate the core conversion logic:

### 4.1 Test Cases
- **`test_guess_encoding_ascii`**: Verifies ASCII strings predict as `ASCII`. (Pass)
- **`test_guess_encoding_utf8`**: Verifies UTF-8 Japanese arrays predict as `UTF-8`. (Pass)
- **`test_guess_encoding_sjis`**: Verifies Shift_JIS Japanese arrays predict as `Shift_JIS`. (Pass)
- **`test_guess_encoding_eucjp`**: Verifies EUC-JP Japanese arrays predict as `EUC-JP`. (Pass)
- **`test_guess_encoding_binary`**: Verifies invalid byte sets predict as `BINARY`. (Pass)
- **`test_sjis_to_eucjp_coords`**: Translates SJIS coordinates to EUC-JP. (Pass)
- **`test_eucjp_to_sjis_coords`**: Translates EUC-JP coordinates to SJIS. (Pass)
- **`test_conversion_utf8_to_sjis`**: Converts UTF-8 chars into SJIS CRLF byte streams. (Pass)
- **`test_conversion_sjis_to_utf8`**: Reconstitutes Shift_JIS bytes back to UTF-8. (Pass)
- **`test_conversion_fallback`**: Safe fallback replacement for unmapped characters. (Pass)
- **`test_half_width_kana`**: Checks half-width Katakana mappings across SJIS and EUC-JP. (Pass)
- **`test_count_lines`**: Checks line counting functions. (Pass)
- **`test_detect_line_ending`**: Identifies LF, CRLF, CR, MIXED, and NONE breaks. (Pass)
- **`test_wildcard_match`**: Tests custom glob match helper functions. (Pass)
- **`test_expand_wildcard_normal`**: Tests local glob path resolutions. (Pass)
- **`test_glob_limit_exceeded`**: Aborts if glob paths exceed 100 paths. (Pass)
- **`test_format_size`**: Validates file size abbreviations. (Pass)

### 4.2 Test Execution Output
```text
running 17 tests
test tests::test_eucjp_to_sjis_coords ... ok
test tests::test_guess_encoding_ascii ... ok
test tests::test_count_lines ... ok
test tests::test_detect_line_ending ... ok
test tests::test_format_size ... ok
test tests::test_guess_encoding_binary ... ok
test tests::test_guess_encoding_eucjp ... ok
test tests::test_guess_encoding_sjis ... ok
test tests::test_guess_encoding_utf8 ... ok
test tests::test_conversion_fallback ... ok
test tests::test_sjis_to_eucjp_coords ... ok
test tests::test_wildcard_match ... ok
test tests::test_conversion_sjis_to_utf8 ... ok
test tests::test_conversion_utf8_to_sjis ... ok
test tests::test_half_width_kana ... ok
test tests::test_expand_wildcard_normal ... ok
test tests::test_glob_limit_exceeded ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s
```

---

## 5. Workspace Quality Audits

Local checks executed to ensure code quality:

| Check | Command | Status | Pass/Fail |
| :--- | :--- | :--- | :--- |
| **Formatting** | `cargo fmt --check` | Exit code 0, no trailing whitespaces | **Pass** |
| **Clippy checks** | `cargo clippy --all-targets -- -D warnings` | 0 warnings, 0 errors | **Pass** |
| **Tests runner** | `cargo test` | 17/17 tests completed successfully | **Pass** |
| **Doc compilations** | `cargo doc --no-deps --document-private-items` | HTML documents compile without errors | **Pass** |
