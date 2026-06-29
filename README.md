# NKF-Win Rust Utility (Standard Library Edition)

👉 **[日本語版のドキュメントはこちら (Japanese Version is here)](README.ja.md)**

A lightweight, high-performance command-line utility that clones the classic Japanese character encoding conversion tool `nkf` (Network Kanji Filter), optimized specifically for Windows PCs with limited resources.

Built in **100% pure Rust** using only the standard library (`std`) to keep the binary size extremely small (~250 KB when stripped) and ensure zero external dependency vulnerabilities.

## Features

- **Character Encoding Conversion**:
  - `UTF-8` ⇆ `EUC-JP`
  - `Shift-JIS` ⇆ `EUC-JP`
  - `Shift-JIS` ⇆ `UTF-8`
- **Automatic Encoding Detection (`--guess` Option)**:
  - Scans files or standard input streams to determine if they are `UTF-8`, `Shift_JIS`, `EUC-JP`, `ASCII`, or `BINARY`.
- **System Options**:
  - `-h`, `--help` displays a comprehensive CLI manual.
  - `-v`, `--version` or `--versio` displays the exact utility version.
- **Newline Normalization**:
  - Auto-converts to `CRLF` when outputting to `Shift-JIS`, and to `LF` for `EUC-JP` or `UTF-8`.
- **Half-width Katakana Preservation**:
  - Safely maps half-width Katakana bytes (`0xA1..=0xDF`) without corrupting or converting them to full-width.
- **Foreign/Unmapped Characters**:
  - Safely falls back to `??` (two question marks) when a character cannot be represented in the destination mapping.
- **Pipes and Multi-file Processing**:
  - Seamlessly supports `stdin` and `stdout` piping as well as batch-processing multiple files specified via command-line arguments.

## Quick Start (Rust)

To compile and run this tool locally on Windows:

```bash
# Clone or copy the source code to main.rs
cargo new nkf-win --bin
cd nkf-win

# Replace src/main.rs with the provided Rust code.
# Build stripped release version for minimal binary size
cargo build --release
```

### Usage Examples

```powershell
# Display help information
nkf-win --help

# Display utility version
nkf-win --version

# Guess the encoding of a file
nkf-win --guess input.txt

# Convert input.txt to Shift-JIS and write to stdout
nkf-win -s input.txt > output_sjis.txt

# Pipe support
type input_utf8.txt | nkf-win -e > output_euc.txt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
