**English** | [日本語版](../ja/SPEC.md)

# Specification (SPEC.md)

This document defines the latest functional specifications for `MyNKF` and its Web simulator, optimized for low-resource environments.

---

## 1. Overview

`MyNKF` is a lightweight command-line utility that emulates the core functionality of the traditional Japanese character encoding conversion tool `nkf` (Network Kanji Filter) using 100% pure Rust standard library (`std`). It has no external crate dependencies, ensuring security, portability, and minimal resource usage (CPU, memory, and binary size).

A "Web Desktop Simulator" is also provided to test the CLI and file conversions on a web browser, mimicking a borderless, transparent Windows environment.

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
| `-h`   | `--help`                | Displays the help text and exits.                                                                  |
| `-v`   | `--version`, `--versio` | Displays version info (`v1.6.0`) and exits.                                                        |

> [!NOTE]
> `--versio` (missing the trailing `n`) is an alias maintained for backward compatibility with the original `nkf` behavior (front-matching command parsers and historical typo compatibility).

- **Behavior with No Encoding Options**:
  - If no encoding flags (`-w`, `-s`, `-e`) are passed, the detected encoding of the input is preserved in the output.
  - This allows options like `-d` (LF) or `-c` (CRLF) to normalize newline endings without altering the file's character encoding (consistent with native `nkf`).
- **Standard Input Fallback**:
  - If no file paths are provided, standard input (`stdin`) is read and processed output is directed to standard output (`stdout`).
- **Multi-file / Wildcard Support**:
  - Multiple file paths can be passed as arguments, which are processed sequentially.
  - **Wildcard globbing**: Characters like `*` and `?` are expanded natively by the application even on Windows shells (PowerShell/cmd.exe) that do not perform expansion.
  - **Limit**: If the total files exceed **100**, the process displays an error and aborts immediately.

---

## 4. Encoding Conversion and Detection Logic

### 4.1 Encoding Detection (Guess)

Analyzes bytes to determine character encodings based on character patterns (valid multi-byte sequences) in the following order of priority:

1. **ASCII**: All bytes are within the `0x00` to `0x7F` range.
2. **UTF-8**: Composed of valid UTF-8 sequences (2-4 bytes long, with proper continuation bytes `0x80` to `0xBF`).
3. **EUC-JP**: Fits EUC-JP multi-byte ranges (e.g., `0xA1..=0xFE`).
4. **Shift_JIS**: Fits Shift_JIS byte patterns (lead bytes `0x81..=0x9F` / `0xE0..=0xFC` and trail bytes `0x40..=0x7E` / `0x80..=0xFC`) or half-width Katakana (`0xA1..=0xDF`).
5. **BINARY**: Contains obvious control characters or fails to match any text encodings listed above.

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

- **JIS X 0208 Mapping Table**: Static array mapping lookup table embedded natively.
- **Half-width Katakana**: Shifts and preserves half-width Katakana bytes (`0xA1` to `0xDF`) correctly without corrupting or forcing full-width conversion.
- **Fallback**: Unmapped characters fallback to `??` (two question marks) to avoid process interruption.

---

## 5. Web Desktop Simulator Specs

The simulator running on Web browsers provides:

- **Design**: Compact layouts with bold headers, transparent window emulations, and top-most display behavior.
- **Drag & Drop**: Auto-detects and converts files dragged directly into the simulator window.
- **CLI Shell**: Emulates `MyNKF` commands inside a terminal-like environment.
- **Downloads**: Immediate download links for converted outputs.
- **Obsidian Export**: One-click copy/download of source codes in Markdown.
