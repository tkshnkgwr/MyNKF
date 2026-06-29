# MyNKF (Standard Library Edition)

👉 **[日本語版のドキュメントはこちら (Japanese Version is here)](README.ja.md)**

A lightweight, high-performance command-line utility that clones the classic Japanese character encoding conversion tool `nkf` (Network Kanji Filter), optimized specifically for Windows PCs with limited resources.

Built in **100% pure Rust** using only the standard library (`std`) to keep the binary size extremely small (~250 KB when stripped) and ensure zero external dependency vulnerabilities.

## Features

- **Character Encoding Conversion**:
  - `UTF-8` ⇆ `EUC-JP`
  - `Shift-JIS` ⇆ `EUC-JP`
  - `Shift-JIS` ⇆ `UTF-8`
- **Automatic Encoding Detection (`--guess` Option)**:
  - Scans files or standard input streams to determine their encoding. For non-`BINARY` files, it also appends the detected newline format (LF / CRLF / CR / MIXED / NONE) (e.g., `UTF-8 (LF)`).
  - When `--line` is specified, it also displays the logical line count of the text file (e.g., `UTF-8 (LF) [100 lines]`; ignored for `BINARY`).
  - When `--size` is specified, it displays the formatted file size (e.g., `UTF-8 (LF) [1.2 KB]`; printed for `BINARY` files too). You can combine both options.
- **System Options**:
  - `-h`, `--help` displays a comprehensive CLI manual.
  - `-v`, `--version` or `--versio` (maintained for backward compatibility with `nkf`) displays the exact utility version.
- **Newline Normalization**:
  - Auto-converts to `CRLF` when outputting to `Shift-JIS`, and to `LF` for `EUC-JP` or `UTF-8`.
- **Half-width Katakana Preservation**:
  - Safely maps half-width Katakana bytes (`0xA1..=0xDF`) without corrupting or converting them to full-width.
- **Foreign/Unmapped Characters**:
  - Safely falls back to `??` (two question marks) when a character cannot be represented in the destination mapping.
- **Pipes and Multi-file Processing**:
  - Seamlessly supports `stdin` and `stdout` piping as well as batch-processing multiple files specified via command-line arguments.
  - Supports wildcard file specifications (`*` and `?`) natively (glob expansion) even on Windows shells like PowerShell or cmd.exe. The maximum number of processed files is limited to **100** for safety.

## Quick Start (Rust)

To compile and run this tool locally on Windows:

```bash
# Clone or copy the source code to main.rs
cargo new MyNKF --bin
cd MyNKF

# Replace src/main.rs with the provided Rust code.
# Build stripped release version for minimal binary size
cargo build --release
```

### Usage Examples

```powershell
# Display help information
MyNKF --help

# Display utility version
MyNKF --version

# Guess the encoding of a file
MyNKF --guess input.txt

# Convert input.txt to Shift-JIS and write to stdout
MyNKF -s input.txt > output_sjis.txt

# Pipe support
type input_utf8.txt | MyNKF -e > output_euc.txt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
