**English** | [日本語版](../ja/TESTING.md)

# Test Plan & Verification Report (TESTING.md)

This document records test cases, verification checklists, and automated test results validating the correctness and edge-case handling of `MyNKF`.

---

## 1. Test Environment

- **Operating System**: Windows 11 / Windows 10
- **Toolchain**: Rust 1.85+ / Edition 2024
- **Execution Date**: 2026-11-20

---

## 2. Command-Line Tests and Results

### 2.1 Help and Version Flags

| Test Item                         | Command / Operations            | Expected Result                                                            | Pass/Fail |
| :-------------------------------- | :------------------------------ | :------------------------------------------------------------------------- | :-------- |
| **Concise Help (`-h`)**           | `mynkf -h`                      | Prints concise usage and option summary with a pointer to `--help`.        | **Pass**  |
| **Detailed Help (`--help`)**      | `mynkf --help`                  | Prints detailed options, notes, and comprehensive examples.                | **Pass**  |
| **Version Details (`--version`)** | `mynkf --version` or `mynkf -v` | Outputs version (v1.6.5), copyright, and license, then terminates cleanly. | **Pass**  |
| **Version Alias (`--versio`)**    | `mynkf --versio`                | Outputs version (v1.6.5), copyright, and license, then terminates cleanly. | **Pass**  |

### 2.2 Encoding Conversion Accuracy

| Test Item               | Command / Operations                | Expected Result                                                                                           | Pass/Fail |
| :---------------------- | :---------------------------------- | :-------------------------------------------------------------------------------------------------------- | :-------- |
| **UTF-8 <-> Shift_JIS** | `mynkf -s input_utf8.txt > out.txt` | Japanese text translates cleanly without Mojibake. Newline normalizes to `CRLF`.                          | **Pass**  |
| **EUC-JP <-> UTF-8**    | `mynkf -w input_euc.txt > out.txt`  | Documents written in EUC-JP convert to UTF-8. Newline normalizes to `LF`.                                 | **Pass**  |
| **Half-width Katakana** | `mynkf -w sjis_with_kana.txt`       | Half-width Katakana characters (like `ｱ`) are preserved in UTF-8 without converting to full-width.        | **Pass**  |
| **Fallback Handling**   | `mynkf -s emoji_utf8.txt`           | Unsupported characters (emojis, non-BMP chars) substitute safely with `??` without panicking or aliasing. | **Pass**  |

### 2.3 Heuristic Guessing (`--guess`)

| Test Item           | Input Content                                      | Expected Result | Pass/Fail |
| :------------------ | :------------------------------------------------- | :-------------- | :-------- |
| **ASCII guess**     | `Hello World!` (plain text)                        | `ASCII`         | **Pass**  |
| **UTF-8 guess**     | UTF-8 encoded Japanese characters                  | `UTF-8`         | **Pass**  |
| **Shift_JIS guess** | Shift_JIS encoded Japanese characters              | `Shift_JIS`     | **Pass**  |
| **EUC-JP guess**    | EUC-JP encoded Japanese characters                 | `EUC-JP`        | **Pass**  |
| **BINARY guess**    | Arbitrary binary byte arrays (`0x00`, `0xFF` etc.) | `BINARY`        | **Pass**  |

### 2.4 Error Handling

| Test Item            | Command / Operations            | Expected Result                                                                                    | Pass/Fail |
| :------------------- | :------------------------------ | :------------------------------------------------------------------------------------------------- | :-------- |
| **Invalid Flags**    | `mynkf --verison` or `mynkf -x` | Outputs `Unknown option: --verison` to stderr and terminates immediately with exit code `1`.       | **Pass**  |
| **Nonexistent File** | `mynkf -g nonexistent.txt`      | Outputs file open error message to stderr and terminates with exit code `1`.                       | **Pass**  |

---

## 3. Automated Unit Tests & Doctests (`cargo test`)

**25 unit test cases** in `src/tests.rs` and **14 doctests** across the public library APIs (39 total) are implemented and passing.

### 3.1 Unit Test Cases

- **`test_guess_encoding_ascii`**: Verifies ASCII strings predict as `ASCII`. (Pass)
- **`test_guess_encoding_utf8`**: Verifies UTF-8 Japanese arrays predict as `UTF-8`. (Pass)
- **`test_guess_encoding_sjis`**: Verifies Shift_JIS Japanese arrays predict as `Shift_JIS`. (Pass)
- **`test_guess_encoding_eucjp`**: Verifies EUC-JP Japanese arrays predict as `EUC-JP`. (Pass)
- **`test_guess_encoding_binary`**: Verifies invalid byte sets predict as `BINARY`. (Pass)
- **`test_guess_encoding_binary_with_null`**: Verifies inputs containing null bytes predict as `BINARY` (`Unknown`). (Pass)
- **`test_sjis_to_eucjp_coords`**: Translates SJIS coordinates to EUC-JP. (Pass)
- **`test_eucjp_to_sjis_coords`**: Translates EUC-JP coordinates to SJIS. (Pass)
- **`test_conversion_utf8_to_sjis`**: Converts UTF-8 chars into SJIS CRLF byte streams. (Pass)
- **`test_conversion_sjis_to_utf8`**: Reconstitutes Shift_JIS bytes back to UTF-8. (Pass)
- **`test_kanji_roundtrip`**: Validates UTF-8 ⇆ Shift_JIS roundtrip for Japanese kanji, hiragana, katakana, and punctuation. (Pass)
- **`test_eucjp_roundtrip`**: Validates UTF-8 ⇆ EUC-JP roundtrip for Japanese kanji, hiragana, katakana, and punctuation. (Pass)
- **`test_conversion_fallback`**: Safe fallback replacement (`??`) for unmapped characters. (Pass)
- **`test_encode_non_bmp_no_truncation`**: Ensures non-BMP code points (e.g. `U+13042`) fall back to `??` instead of truncating to 16-bit BMP characters. (Pass)
- **`test_decode_eucjp_invalid_second_byte`**: Ensures malformed EUC-JP second bytes do not underflow/panic and fall back safely. (Pass)
- **`test_decode_eucjp_ss3_truncated`**: Ensures EUC-JP SS3 (`0x8F`) sequences and truncated endings are handled safely. (Pass)
- **`test_half_width_kana`**: Checks half-width Katakana mappings across SJIS and EUC-JP. (Pass)
- **`test_convert_line_endings_raw`**: Checks raw byte-level CRLF/LF/CR break replacements without re-encoding. (Pass)
- **`test_count_lines`**: Checks logical line counting. (Pass)
- **`test_detect_line_ending`**: Identifies LF, CRLF, CR, MIXED, and NONE breaks. (Pass)
- **`test_wildcard_match`**: Tests custom glob match helper functions. (Pass)
- **`test_expand_wildcard_normal`**: Tests local glob path resolutions inside an isolated temporary directory. (Pass)
- **`test_glob_limit_exceeded`**: Aborts if glob expansion exceeds 100 files. (Pass)
- **`test_expand_wildcard_no_match_limit`**: Enforces the 100-file limit even on unmatched wildcard fallbacks. (Pass)
- **`test_format_size`**: Validates file size abbreviations. (Pass)

### 3.2 Test Execution Output (2026-11-20)

```text
running 25 tests
test tests::test_convert_line_endings_raw ... ok
test tests::test_conversion_fallback ... ok
test tests::test_conversion_sjis_to_utf8 ... ok
test tests::test_conversion_utf8_to_sjis ... ok
test tests::test_count_lines ... ok
test tests::test_decode_eucjp_invalid_second_byte ... ok
test tests::test_decode_eucjp_ss3_truncated ... ok
test tests::test_detect_line_ending ... ok
test tests::test_encode_non_bmp_no_truncation ... ok
test tests::test_eucjp_roundtrip ... ok
test tests::test_eucjp_to_sjis_coords ... ok
test tests::test_expand_wildcard_no_match_limit ... ok
test tests::test_expand_wildcard_normal ... ok
test tests::test_format_size ... ok
test tests::test_glob_limit_exceeded ... ok
test tests::test_guess_encoding_ascii ... ok
test tests::test_guess_encoding_binary ... ok
test tests::test_guess_encoding_binary_with_null ... ok
test tests::test_guess_encoding_eucjp ... ok
test tests::test_guess_encoding_sjis ... ok
test tests::test_guess_encoding_utf8 ... ok
test tests::test_half_width_kana ... ok
test tests::test_kanji_roundtrip ... ok
test tests::test_sjis_to_eucjp_coords ... ok
test tests::test_wildcard_match ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

Doc-tests mynkf
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s
```

---

## 4. Workspace Quality Audits

Local verification commands executed with all checks passing:

| Check                | Command                                        | Status                                            | Pass/Fail |
| :------------------- | :--------------------------------------------- | :------------------------------------------------ | :-------- |
| **Type Check**       | `cargo check`                                  | 0 warnings, 0 errors (1.34s)                      | **Pass**  |
| **Formatting**       | `cargo fmt --check`                            | Exit code 0, compliant with `rustfmt`             | **Pass**  |
| **Clippy Checks**    | `cargo clippy --all-targets -- -D warnings`    | 0 warnings, 0 errors                              | **Pass**  |
| **Automated Tests**  | `cargo test`                                   | 25 unit tests + 14 doctests passed                | **Pass**  |
| **Doc Compilations** | `cargo doc --no-deps --document-private-items` | HTML docs compiled with 0 warnings                | **Pass**  |
| **Release Build**    | `cargo build --release`                        | Built cleanly (`mynkf.exe`: 219,136 B / ~214 KB)  | **Pass**  |
