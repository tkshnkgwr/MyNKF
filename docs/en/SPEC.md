**English** | [日本語版](../ja/SPEC.md)

# Specification (SPEC.md)

This document defines the latest functional specifications for the `MyNKF` CLI utility, optimized for low-resource environments.

---

## 1. Overview

`MyNKF` is a lightweight command-line utility that emulates the core functionality of the traditional Japanese character encoding conversion tool `nkf` (Network Kanji Filter) using 100% pure Rust standard library (`std`). It has no external crate dependencies, ensuring security, portability, and minimal resource usage (CPU, memory, and binary size).

---

## 2. Operating Environment and Constraints

- **Target OS**: Windows 10 / 11 (designed for low-resource PCs, cross-compilation is supported).
- **Dependencies**: Built with zero external crates (Rust standard library `std` only).
- **Binary Size Targets**: ~200 KB to 250 KB (release build with `strip`).
- **Memory Footprint**: Under a few megabytes (using stream/buffered processing).
- **File Limit**:
  - To prevent excessive CPU/Memory exhaustion, the maximum number of files processed at once (including expanded wildcards) is limited to **100**. Exceeding this limit yields an error and terminates execution.

---

## 3. CLI Command Specifications

### 3.1 Syntax

```powershell
MyNKF [options] [file...]
```

### 3.2 Supported Options

| Option | Alias                   | Description                                                                                        |
| :----- | :---------------------- | :------------------------------------------------------------------------------------------------- |
| `-w`   | `--utf8`                | Converts output to UTF-8. Newline is normalized to `LF`.                                           |
| `-s`   | `--sjis`                | Converts output to Shift_JIS. Newline is normalized to `CRLF`.                                     |
| `-e`   | `--euc`                 | Converts output to EUC-JP. Newline is normalized to `LF`.                                          |
| `-g`   | `--guess`               | Detects and displays the character encoding of files or standard input.                            |
|        | `--line`                | Appends logical line count in guess mode (ignored for `BINARY`).                                   |
|        | `--size`                | Appends formatted file size in guess mode (e.g., `[1.2 KB]`, `[100 B]`; printed for `BINARY` too). |
| `-d`   | `--lf`                  | Forces newline characters to `LF`.                                                                 |
| `-c`   | `--crlf`                | Forces newline characters to `CRLF`.                                                               |
| `-h`   |                         | Displays concise help text and exits.                                                              |
|        | `--help`                | Displays detailed help text with examples and exits.                                               |
| `-v`   | `--version`, `--versio` | Displays version info (`v1.6.5`), copyright, and license, then exits.                              |

> [!NOTE]
> `--versio` (missing the trailing `n`) is an alias maintained for backward compatibility with the original `nkf` behavior (front-matching command parsers and historical typo compatibility).

- **Behavior with No Encoding Options**:
  - If no encoding flags (`-w`, `-s`, `-e`) are passed, raw byte-level newline replacement (`convert_line_endings_raw`) is applied without re-encoding, preserving the input encoding completely.
  - This allows options like `-d` (LF) or `-c` (CRLF) to normalize newline endings without altering the file's character encoding (consistent with native `nkf`).
- **Repeated Flag Precedence**:
  - If conflicting flags such as `-w`/`-s`/`-e` or `-d`/`-c` are passed multiple times, the last specified flag wins.
- **Standard Input Fallback**:
  - If no file paths are provided, standard input (`stdin`) is read and processed output is directed to standard output (`stdout`).
- **Multi-file / Wildcard Support & Exit Code**:
  - Multiple file paths can be passed as arguments, which are processed sequentially. If any file fails to open or read, an error is written to `stderr`, remaining files continue to be processed, and the process exits with status code `1`.
  - **Wildcard globbing**: Characters like `*` and `?` are expanded natively by the application even on Windows shells (PowerShell/cmd.exe) that do not perform expansion.
  - **Limit**: If the total files exceed **100**, the process displays an error and aborts immediately with exit code `1`.

---

## 4. Encoding Conversion and Detection Logic

### 4.1 Encoding Detection (Guess)

Analyzes bytes to determine character encodings using the following steps and scoring:

1. **Empty input**: Treated as `ASCII`.
2. **Control character check (`BINARY`)**: Immediately returns `BINARY` (`Encoding::Unknown`) if null bytes (`0x00`) or binary-specific control bytes are detected.
3. **ASCII**: Returns `ASCII` if all bytes are within `0x00..=0x7F`.
4. **UTF-8**: Returns `UTF-8` if strict validation via `std::str::from_utf8` passes.
5. **Shift_JIS / EUC-JP Scoring**: Evaluates syntax validity and weighted JIS X 0208 Ku-Ten range scores (Ku 1–84, Ten 1–94) for both Shift_JIS (including half-width Katakana `0xA1..=0xDF`) and EUC-JP (including SS2 `0x8E` / SS3 `0x8F`). If both are valid, the higher score wins (`Shift_JIS` on ties).
6. **BINARY**: Returns `BINARY` if the input does not match any supported text encoding.

#### Guess Output Formats

Output changes depending on input type and active options:

- **`BINARY` files**:
  Newline and line counts are not displayed (size option is supported).

  - Standard input: `BINARY`
  - Standard input with `--size`: `BINARY [1.2 KB]`
  - File input: `file.bin: BINARY`
  - File input with `--size`: `file.bin: BINARY [1.2 KB]`
- **Text files**:
  Appends detected newline format (`LF` / `CRLF` / `CR` / `MIXED` / `NONE`). Displays line counts with `--line` and file size with `--size`.

  - Standard input: `UTF-8 (LF)`
  - Standard input with `--line`: `UTF-8 (LF) [123 lines]`
  - Standard input with `--size`: `UTF-8 (LF) [1.2 KB]`
  - Standard input with both: `UTF-8 (LF) [123 lines] [1.2 KB]`
  - File input: `file.txt: UTF-8 (LF)`
  - File input with `--line`: `file.txt: UTF-8 (LF) [123 lines]`
  - File input with `--size`: `file.txt: UTF-8 (LF) [1.2 KB]`
  - File input with both: `file.txt: UTF-8 (LF) [123 lines] [1.2 KB]`

### 4.2 Encoding Table and Fallback

- **JIS X 0208 Mapping Table**: Complete 6,953-character JIS X 0208 mapping table embedded natively in Base64 form.
- **Half-width Katakana**: Preserves half-width Katakana (`0xA1..=0xDF` in Shift_JIS) accurately across UTF-8 and EUC-JP without forcing full-width conversion.
- **Fallback**: Unmapped characters, emojis, and non-BMP characters (`U+10000` and above) safely substitute with `??` (two question marks) without panicking.
