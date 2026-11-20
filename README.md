# MyNKF (Standard Library Edition)

[![Rust CI](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml)
[![Release](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://img.shields.io/badge/platform-Windows-blue.svg)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://img.shields.io/badge/rust-1.85%2B-orange.svg)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

**English** | [日本語版](README_JA.md)

A lightweight, high-performance Japanese character encoding conversion CLI utility modeled after the classic `nkf` (Network Kanji Filter), optimized specifically for Windows PCs with limited resources.

Its entire encoding detection, conversion, and glob expansion logic is implemented in **100% pure Rust** using only the standard library (`std`).
With **zero external crate dependencies**, `mynkf` eliminates supply-chain risks, compiles in seconds, and keeps an ultra-small binary footprint (~200–250 KB when stripped).

---

## ✨ Key Features

- **Zero External Dependencies**: Built exclusively with the Rust standard library (`std`). No third-party crates or external shared libraries required.
- **Complete JIS X 0208 Support**: Embeds a complete 94×94 JIS X 0208 mapping table (all 6,953 characters) for accurate roundtrip conversion across **UTF-8 ⇆ Shift_JIS ⇆ EUC-JP**.
- **High-Accuracy Auto Detection (`-g` / `--guess`)**: Detects `ASCII`, `UTF-8`, `Shift_JIS`, `EUC-JP`, and `BINARY`, alongside line endings (`LF`, `CRLF`, `CR`, `MIXED`, `NONE`), logical line counts (`--line`), and formatted file sizes (`--size`).
- **Lossless Line-Ending-Only Conversion**: When `-d` (`--lf`) or `-c` (`--crlf`) is specified without encoding flags, newline bytes are replaced directly without re-encoding—preserving the original encoding with zero risk of character corruption.
- **Half-Width Katakana & Safe Fallback**: Preserves half-width Katakana accurately and safely substitutes unmapped characters with `??` instead of panicking.
- **Native Wildcard Globbing & Pipes**: Expands `*` and `?` patterns natively on Windows shells (up to 100 files for safety) and supports `stdin`/`stdout` pipelines.
- **Two-Tier Help**: Concise summary (`-h`) and detailed manual with practical examples (`--help`).

---

## 🚀 Build & Installation

Requires **Rust 1.85+** (Edition 2024).

```powershell
git clone https://github.com/tkshnkgwr/MyNKF.git
cd MyNKF
cargo build --release
```

The standalone executable is generated at `target/release/mynkf.exe`.

---

## 📖 Usage

```powershell
mynkf [options] [file...]
```

### Supported Options

| Option | Alias                   | Description                                                                             |
| :----- | :---------------------- | :-------------------------------------------------------------------------------------- |
| `-w`   | `--utf8`                | Convert output to UTF-8 (default line ending: `LF`)                                     |
| `-s`   | `--sjis`                | Convert output to Shift_JIS (default line ending: `CRLF`)                               |
| `-e`   | `--euc`                 | Convert output to EUC-JP (default line ending: `LF`)                                    |
| `-g`   | `--guess`               | Guess character encoding and line ending of input                                       |
|        | `--line`                | Show logical line count in guess mode (ignored for `BINARY`)                            |
|        | `--size`                | Show formatted file size in guess mode (e.g., `[1.2 KB]`; shown for `BINARY` too)       |
| `-d`   | `--lf`                  | Force line endings to `LF`                                                              |
| `-c`   | `--crlf`                | Force line endings to `CRLF`                                                            |
| `-h`   |                         | Show concise help and exit                                                              |
|        | `--help`                | Show detailed help with usage examples and exit                                         |
| `-v`   | `--version`, `--versio` | Show version, copyright, and license information (`--versio` is an `nkf`-compat alias)  |

### Examples

```powershell
# Guess encoding and line ending
mynkf -g input.txt

# Guess encoding with logical line count and formatted file size
mynkf -g --line --size input.txt

# Convert a file to UTF-8 (LF)
mynkf -w input.txt > output_utf8.txt

# Convert a file to Shift_JIS (CRLF)
mynkf -s input.txt > output_sjis.txt

# Convert line endings to CRLF while preserving the original encoding
mynkf -c input.txt > output_crlf.txt

# Process via standard input / output pipeline
type input.txt | mynkf -w > output.txt

# Batch guess across multiple files using wildcards (up to 100 files)
mynkf -g *.txt
```

---

## 📚 Documentation

| Document                                          | Description                                                |
| :------------------------------------------------ | :--------------------------------------------------------- |
| [SPEC.md](docs/en/SPEC.md)                        | Functional specifications, CLI options, and codec behavior |
| [ARCHITECTURE.md](docs/en/ARCHITECTURE.md)        | System architecture, module structure, and data pipelines  |
| [DIAGRAM.md](docs/en/DIAGRAM.md)                  | Mermaid flowcharts for CLI execution and encoding guess    |
| [FOOTPRINTS.md](docs/en/FOOTPRINTS.md)            | Measured binary sizes, memory usage, and CPU performance   |
| [TESTING.md](docs/en/TESTING.md)                  | Test plan, automated unit tests, and verification logs     |
| [CHANGELOG.md](docs/en/CHANGELOG.md)              | Detailed version history                                   |
| [TODO.md](docs/en/TODO.md)                        | Completed milestones and future roadmap                    |
| [CONTRIBUTING.md](docs/en/CONTRIBUTING.md)        | Contribution guidelines and PR checklist                   |
| [RELEASE.md](docs/en/RELEASE.md)                  | Release and tagging workflow                               |
| [SECURITY.md](docs/en/SECURITY.md)                | Security design and vulnerability reporting policy         |
| [INSTRUCTIONS.md](docs/en/INSTRUCTIONS.md)        | Coding standards and AI developer guidelines               |

---

## 📄 License

This project is licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later) - Copyright (c) 2026 tkshnkgwr. See the [LICENSE](LICENSE) file for details.
