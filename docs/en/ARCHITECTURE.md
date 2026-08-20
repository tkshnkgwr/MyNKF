**English** | [日本語版](../ja/ARCHITECTURE.md)

# Architecture Design (ARCHITECTURE.md)

This document describes the design philosophy, technical stack, directory structure design, and data flows between modules for `MyNKF` optimized for low-resource environments.

---

## 1. System Overview and Goals

`MyNKF` is a lightweight CLI utility that emulates the major features of the traditional Japanese character encoding converter `nkf` (Network Kanji Filter) using the Rust language.

### Core Goals

- **Minimal Resource Footprint**: Restricts CPU and memory utilization to a minimum to run efficiently in low-resource environments.
- **High Portability**: Cuts down external library dependencies, packaging the application into a single, standalone executable for easy deployment on Windows 10/11.
- **Logic Consolidation**: Packages core encoding detection and conversion engines as a library (`mynkf`), ensuring maintainability and code reuse.

---

## 2. Technical Stack

To minimize binary sizes and runtime overheads, we utilize a strictly selected technical stack.

### 2.1 Language and Edition

- **Rust (Edition 2024, v1.85+)**: Utilizes Edition 2024 features (such as `let_chains`) to maintain secure and readable codebases.

### 2.2 Dependencies

- **Zero External Dependencies (Pure Rust `std` Only)**: Implements custom JIS X 0208 mapping lookups, auto-detection heuristics, and shell expansions natively using only the Rust standard library. Zero third-party crate dependencies mean ultra-lightweight and lightning-fast compilations.
- **Out-of-workspace Dependency**:
  - `common_lib` (Referenced via relative path `../common_lib`): Shared library for shared operations.

### 2.3 Optimization Settings

The release profile `[profile.release]` in `Cargo.toml` applies the following parameters to ensure minimum binary sizes:

- `opt-level = 'z'` (Optimize for size)
- `lto = true` (Link-Time Optimization)
- `codegen-units = 1` (Integrates compilation units)
- `panic = 'abort'` (Disables stack unwinding to reduce overhead)
- `strip = true` (Removes debug symbols)

---

## 3. Architecture and Directory Intent

The codebase is organized as follows to separate concerns:

```text
MyNKF/
├── .agents/
│   └── AGENTS.md                    # AI agent instructions and guidelines
├── .github/workflows/
│   ├── ci.yml                       # Continuous integration test & build verification
│   └── release.yml                  # Automated release upon pushing tags
├── docs/
│   ├── en/
│   │   ├── ARCHITECTURE.md          # [This Document] Architecture overview
│   │   ├── DIAGRAM.md               # Mermaid system architecture diagrams
│   │   ├── FOOTPRINTS.md            # Memory and binary footprint telemetry
│   │   ├── INSTRUCTIONS.md          # Coding style guide for AI assistants
│   │   ├── PROJECT_TEMPLATE_GUIDE.md# Project initialization and templating guide
│   │   ├── SPEC.md                  # Detailed functional specification
│   │   ├── TESTING.md               # Testing procedures and logs
│   │   ├── RELEASE.md               # Release checklist and procedures
│   │   ├── CONTRIBUTING.md          # Contribution guidelines
│   │   ├── SECURITY.md              # Security policies
│   │   ├── TODO.md                  # Task tracker
│   │   └── CHANGELOG.md             # Changelog source
│   └── ja/
│       └── ...                      # Japanese documentation set
├── src/
│   ├── lib.rs                       # Core logic library (mynkf)
│   ├── main.rs                      # CLI entrypoint & I/O control
│   └── tests.rs                     # Unit test modules (mynkf::tests)
├── Cargo.toml                       # Dependencies and release profiles
├── CHANGELOG.md                     # Changelog navigation
├── README.md                        # English global guide
└── README_JA.md                     # Japanese global guide
```

### Design Intent

1. **Centralization of Core Logic (`src/lib.rs`)**:
   Encoding auto-detection, Unicode intermediate decoding, multi-encoding generation, line ending normalizations, and wildcard resolutions are consolidated inside `lib.rs`.

2. **Ultra-Lightweight CLI Footprint (`src/main.rs`)**:
   The CLI entrypoint relies solely on `lib.rs` and the Rust standard library, achieving zero external dependencies and a binary footprint under 250 KB.

---

## 4. Data Flow and Module Interactions

### 4.1 CLI Data Pipeline

When CLI version (`mynkf`) starts, data flows through the following pipeline:

```text
[CLI Arguments Parse] (main.rs)
   │
   ▼
[Wildcard Glob Expansion] (lib.rs::expand_wildcard)
   │  * Limits maximum files to 100
   ▼
[Retrieve Input Buffer] (via files or stdin stream)
   │
   ▼
[Auto Encoding Guess] (lib.rs::guess_encoding)
   │  * Evaluates byte constraints for ASCII/UTF-8/EUC-JP/Shift_JIS/BINARY
   ▼
[Decode to Unicode Intermediate] (lib.rs::decode_to_unicode)
   │  * Builds Unicode chars (Vec<char>) via JIS map. Replaces unknown bytes with '?'
   ▼
[Newline Normalize & Encode] (lib.rs::encode_from_unicode)
   │  * Normalizes breaks to LF/CRLF and encodes into output bytes
   ▼
[Output Transfer] (to stdout or file write)
```

### 4.2 Encoding Guess Heuristics

Walks input buffers to evaluate grammar scores:

1. **ASCII**: All bytes fall into `0x00`..=`0x7F`.
2. **UTF-8**: Complies with UTF-8 byte syntax (proper lead and continuation bytes).
3. **EUC-JP**: Complies with EUC-JP multi-byte ranges.
4. **Shift_JIS**: Complies with Shift_JIS double-bytes or half-width Katakana (`0xA1`..=`0xDF`).
5. **BINARY**: Violates grammar rules or contains control characters.
Priority: `ASCII` ＞ `UTF-8` ＞ `EUC-JP` ＞ `Shift_JIS` ＞ `BINARY`. The highest match score dictates the final prediction.
