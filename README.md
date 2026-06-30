# MyNKF (Standard Library Edition)

[![Rust CI](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml)
[![Release](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml)

👉 **[日本語版のドキュメントはこちら (Japanese Version is here)](README.ja.md)**

A lightweight, high-performance Japanese character encoding conversion utility (CLI & GUI editions) modeled after the classic `nkf` (Network Kanji Filter), optimized specifically for Windows PCs with limited resources.

Its core encoding detection and conversion logic is implemented in **100% pure Rust** using only the standard library (`std`).
The CLI version (`mynkf`) has zero external dependency vulnerabilities and maintains an extremely small binary footprint (~250 KB when stripped).
The GUI version (`mynkf-gui`) leverages `eframe`/`egui` to provide an ultra-lightweight desktop application that directly calls Win32 APIs (DwmSetWindowAttribute and CreateMutexW via direct FFI) for a modern, borderless window styling with a minimal binary size and compile-time overhead.

## Features

- **GUI Version (`mynkf-gui`) Batch Operations**:
  - Drag and drop files directly onto the UI to auto-detect their encoding and newline format instantly.
  - Choose destination encoding/newline options and apply batch conversions to files in place.
  - Built-in multi-launch prevention (Named Mutex) to avoid wasted background processes.
  - Custom borderless, transparent frame design using direct Win32 FFI for dwmapi controls.
  - Text playground for copying, pasting, and converting text with live preview and clipboard integrations.
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
  - `-v`, `--version` or `--versio` (maintained for backward compatibility with `nkf`) displays the exact utility version (`v1.5.0`).
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
# Navigate to the repository
cd MyNKF

# Run tests to verify logic integrity
cargo test

# Build stripped release version for both CLI & GUI binaries
cargo build --release
```

Once compilation completes, the following executables will be available under `target/release/`:
- `mynkf.exe` (CLI character converter)
- `mynkf-gui.exe` (GUI desktop character converter)

### Usage Examples

#### CLI Edition (`mynkf`)

```powershell
# Display help information
cargo run --bin mynkf -- --help

# Guess file encoding (displays size too)
cargo run --bin mynkf -- --guess --size input.txt

# Convert input.txt to Shift-JIS and write to a file
cargo run --bin mynkf -- -s input.txt > output_sjis.txt

# Pipe support
type input_utf8.txt | cargo run --bin mynkf -- -e > output_euc.txt
```

#### GUI Edition (`mynkf-gui`)

```powershell
# Run GUI version directly
cargo run --bin mynkf-gui

# Launch final built GUI binary
.\target\release\mynkf-gui.exe
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
