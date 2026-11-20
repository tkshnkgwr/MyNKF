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

- **Zero External Dependencies (Pure Rust `std` Only)**: Implements custom JIS X 0208 mapping lookups, auto-detection heuristics, and wildcard glob expansion natively using only the Rust standard library. With zero third-party crates or external shared libraries, the project builds standalone in seconds.

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
│   ├── lib.rs                       # Facade module (coordinates and re-exports submodules)
│   ├── types.rs                     # Type definitions (Encoding, LineEnding)
│   ├── newline.rs                   # Line ending detection & line counting (detect_line_ending, count_lines)
│   ├── glob.rs                      # Wildcard glob expansion & formatting (wildcard_match, expand_wildcard, format_size)
│   ├── table.rs                     # JIS X 0208 mapping table & coordinate math (load_jis_table, sjis_to_eucjp, etc.)
│   ├── guess.rs                     # Character code auto-detection engine (guess_encoding)
│   ├── convert.rs                   # Unicode decode, encode & raw newline replacement (decode_to_unicode, encode_from_unicode, etc.)
│   ├── main.rs                      # CLI entrypoint & I/O control
│   └── tests.rs                     # Unit test modules (mynkf::tests)
├── Cargo.toml                       # Dependencies and release profiles
├── CHANGELOG.md                     # Changelog navigation
├── README.md                        # English global guide
└── README_JA.md                     # Japanese global guide
```

### Design Intent

1. **Separation of Concerns & Facade Pattern (`src/lib.rs` and Submodules)**:
   Following the Single Responsibility Principle, responsibilities are cleanly decoupled into encoding detection (`guess.rs`), JIS mapping (`table.rs`), codec logic (`convert.rs`), newline control (`newline.rs`), file globbing (`glob.rs`), and core types (`types.rs`). `lib.rs` acts as a facade module re-exporting all items via `pub use`, maintaining 100% public API backward compatibility while keeping each source file under 300 lines.

2. **Ultra-Lightweight CLI Footprint (`src/main.rs`)**:
   The CLI entrypoint relies solely on `mynkf::*` and the Rust standard library, achieving zero external dependencies and a binary footprint under 250 KB.

---

## 4. Data Flow and Module Interactions

### 4.1 CLI Data Pipeline

When CLI version (`mynkf`) starts, data flows through the following pipeline:

```text
[CLI Arguments Parse] (main.rs)
   ├─► -h / --help / -v ──► [Displays help / version and exits immediately]
   │    * -h: concise summary, --help: detailed manual & examples
   ▼
[Wildcard Glob Expansion] (lib.rs::expand_wildcard)
   │  * Limits maximum files to 100
   ▼
[Retrieve Input Buffer] (via files or stdin stream)
   │
   ├─► [No Encoding Flags Specified (Newline conversion only)]
   │      │
   │      ▼ (lib.rs::convert_line_endings_raw)
   │    Replaces newline bytes directly without re-encoding (zero corruption risk)
   │      │
   │      ▼
   │    [Output Transfer] (to stdout)
   │
   └─► [Encoding Flags Specified or Guess Mode (--guess)]
          │
          ▼
        [Auto Encoding Guess] (lib.rs::guess_encoding)
          │  * Evaluates byte constraints for ASCII/UTF-8/EUC-JP/Shift_JIS/BINARY
          ▼
        [Decode to Unicode Intermediate] (lib.rs::decode_to_unicode)
          │  * Builds Unicode chars (Vec<char>) via JIS map. Replaces unmapped chars with '??'
          ▼
        [Newline Normalize & Encode] (lib.rs::encode_from_unicode)
          │  * Normalizes breaks to LF/CRLF and encodes into output bytes
          ▼
        [Output Transfer] (to stdout)
```

### 4.2 Encoding Guess Heuristics

Walks input buffers and determines the encoding using the following steps and scoring:

1. **Empty input**: Treated as `ASCII`.
2. **Control character check (`BINARY`)**: Immediately returns `BINARY` (`Encoding::Unknown`) if null bytes (`0x00`) or binary-specific control bytes are detected.
3. **ASCII**: Returns `ASCII` if all bytes fall within `0x00..=0x7F`.
4. **UTF-8**: Returns `UTF-8` if strict validation via `std::str::from_utf8` succeeds.
5. **Shift_JIS / EUC-JP Scoring**: Evaluates syntax validity and weighted JIS X 0208 Ku-Ten range scores (Ku 1–84, Ten 1–94). If only one is valid, it is chosen; if both are valid, the higher score wins (`Shift_JIS` on ties).
6. **BINARY**: Returns `BINARY` (`Encoding::Unknown`) if neither encoding is valid.
