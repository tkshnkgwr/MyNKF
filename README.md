# MyNKF (Standard Library Edition)

[![Rust CI](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml)
[![Release](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://img.shields.io/badge/platform-Windows-blue.svg)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://img.shields.io/badge/rust-1.85%2B-orange.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**English** | [日本語版](README_JA.md)

A lightweight, high-performance Japanese character encoding conversion CLI utility modeled after the classic `nkf` (Network Kanji Filter), optimized specifically for Windows PCs with limited resources.

Its core encoding detection and conversion logic is implemented in **100% pure Rust** using only the standard library (`std`).
The CLI version (`mynkf`) has zero external dependency vulnerabilities and maintains an extremely small binary footprint (~250 KB when stripped).

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

## Prerequisites

This project depends on a shared library `common_lib` located in the parent directory (`../common_lib`). Before compiling or testing this project, make sure to clone `common_lib` into the same parent directory:

```bash
# Example directory structure:

# workspace/

# ├── MyNKF/      (this repository)

# └── common_lib/  (dependency repository)

# Run this in the parent directory of MyNKF

git clone https://github.com/tkshnkgwr/common_lib.git
```

## Quick Start (Rust)

To compile and run this tool locally on Windows:

```bash
# Navigate to the repository

cd MyNKF

# Run tests to verify logic integrity

cargo test

# Build stripped release version

cargo build --release
```

Once compilation completes, the executable will be available under `target/release/`:

- `mynkf.exe` (CLI character converter)

### Usage Examples

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

## 📚 Documentation

Detailed documentation is available in the `docs/en/` directory:

- [Specifications](docs/en/SPEC.md)
- [Architecture Design](docs/en/ARCHITECTURE.md)
- [System Diagram & Flows](docs/en/DIAGRAM.md)
- [Resource Footprint Measurements](docs/en/FOOTPRINTS.md)
- [AI Developer Instructions](docs/en/INSTRUCTIONS.md)
- [Testing Guide & Reports](docs/en/TESTING.md)
- [Release Guide](docs/en/RELEASE.md)
- [Contributing Guidelines](docs/en/CONTRIBUTING.md)
- [Security Policy](docs/en/SECURITY.md)
- [Task Roadmap](docs/en/TODO.md)
- [Changelog](docs/en/CHANGELOG.md)

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
