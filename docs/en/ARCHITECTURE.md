**English** | [日本語版](../ja/ARCHITECTURE.md)

# Architecture Design (ARCHITECTURE.md)

This document describes the design philosophy, technical stack, directory structure design, and data flows between modules for `MyNKF` (both CLI and GUI versions) optimized for low-resource environments.

---

## 1. System Overview and Goals

`MyNKF` is a lightweight utility that emulates the major features of the traditional Japanese character encoding converter `nkf` (Network Kanji Filter) using the Rust language.

### Core Goals
- **Minimal Resource Footprint**: Restricts CPU and memory utilization to a minimum to run efficiently in low-resource environments.
- **High Portability**: Cuts down external library dependencies, packaging the application into single, standalone executables for easy deployment on Windows 10/11.
- **Logic Consolidation**: Packages core encoding detection and conversion engines as a library (`mynkf`), shared by both CLI (`mynkf`) and GUI (`mynkf-gui`) editions.

---

## 2. Technical Stack

To minimize binary sizes and runtime overheads, we utilize a strictly selected technical stack.

### 2.1 Language and Edition
- **Rust (Edition 2024, v1.85+)**: Utilizes Edition 2024 features (such as `let_chains`) to maintain secure and readable codebases.

### 2.2 Dependencies and Cargo Features
- **Cargo Features Separation (`[features]`)**:
  - `default = ["cli", "gui"]`
  - `cli = []`: Activates the CLI entry point. Zero external crate dependencies (Rust standard `std` only).
  - `gui = ["dep:eframe", "dep:egui", "dep:rfd"]`: Activates the GUI entry point and required graphic/dialog crates.
- **CLI Edition (`mynkf`)**: **Zero external dependencies (pure Rust `std` only)**. Implements custom JIS X 0208 mapping lookups, auto-detection heuristics, and shell expansions natively. Running `cargo build --no-default-features --features cli` skips all GUI dependencies, delivering ultra-lightweight and lightning-fast compilations.
- **GUI Edition (`mynkf-gui`)**:
  - `eframe` / `egui` (v0.35, optional): Immediate-mode GUI frameworks for layout rendering.
  - `rfd` (v0.17, optional): Cross-platform file picker dialog.
  - **Win32 API FFI bindings**: Instead of importing heavy libraries like `windows-sys`, it defines necessary system calls (`kernel32`, `user32`, `dwmapi`) directly via `extern "system"` to optimize compile times.
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

```
MyNKF/
├── .agents/
│   └── AGENTS.md                    # Rules & guidelines for AI agents (Gem)
├── .github/workflows/
│   ├── ci.yml                       # GitHub Actions CI for tests (skips on markdown-only edits)
│   └── release.yml                  # Auto-release CI creating Windows ZIPs on tags
├── docs/
│   ├── ja/
│   │   ├── ARCHITECTURE.md          # Architecture Design (Japanese)
│   │   ├── DIAGRAM.md               # System Diagram (Japanese)
│   │   ├── FOOTPRINTS.md            # Footprint measurements (Japanese)
│   │   ├── INSTRUCTIONS.md          # AI Coding Instructions (Japanese)
│   │   ├── PROJECT_TEMPLATE_GUIDE.md# Project initialization guide (Japanese)
│   │   ├── SPEC.md                  # Detail specification (Japanese)
│   │   ├── TESTING.md               # Testing guide (Japanese)
│   │   ├── RELEASE.md               # Release guide (Japanese)
│   │   ├── CONTRIBUTING.md          # Contribution guidelines (Japanese)
│   │   ├── SECURITY.md              # Security policies (Japanese)
│   │   ├── TODO.md                  # Roadmap task management (Japanese)
│   │   └── CHANGELOG.md             # Real changelog logs (Japanese)
│   └── en/
│       ├── ARCHITECTURE.md          # ARCHITECTURE (English)
│       ├── DIAGRAM.md               # DIAGRAM (English)
│       ├── FOOTPRINTS.md            # FOOTPRINTS (English)
│       ├── INSTRUCTIONS.md          # INSTRUCTIONS (English)
│       ├── PROJECT_TEMPLATE_GUIDE.md# PROJECT_TEMPLATE_GUIDE (English)
│       ├── SPEC.md                  # SPEC (English)
│       ├── TESTING.md               # TESTING (English)
│       ├── RELEASE.md               # RELEASE (English)
│       ├── CONTRIBUTING.md          # CONTRIBUTING (English)
│       ├── SECURITY.md              # SECURITY (English)
│       ├── TODO.md                  # TODO (English)
│       └── CHANGELOG.md             # CHANGELOG (English)
├── src/
│   ├── bin/
│   │   └── mynkf-gui.rs             # GUI app entry point & UI drawing
│   ├── lib.rs                       # Core encoding library (mynkf)
│   ├── main.rs                      # CLI entry point & filesystem I/O
│   └── tests.rs                     # Unit tests module (mynkf::tests)
├── Cargo.toml                       # Build profiles & dependency lists
├── CHANGELOG.md                     # Changelog navigation links
├── README.md                        # English global guide
└── README_JA.md                     # Japanese global guide
```

### Design Intentions
1. **Consolidated Logic (`src/lib.rs`)**:
   Maintains core encoding heuristics, Unicode intermediate conversions, custom encoding tables, line endings normalizations, and wildcard expansions. This ensures uniform CLI/GUI behavior and eliminates double maintenance.
2. **Lean CLI (`src/main.rs`)**:
   Purely handles console args parsing, filesystem/stream I/O, and stdout formatting. Since it only links against `lib.rs` with zero external crate dependencies, the binary compiles instantly and remains extremely small (~250 KB).
3. **Isolated GUI (`src/bin/mynkf-gui.rs`)**:
   Confines heavy UI-related graphical libraries to a single binary. Native OS configurations (such as transparent overlays and mutex controls) are isolated using conditional compilation (`#[cfg(target_os = "windows")]`).

---

## 4. Data Flow and Module Interactions

### 4.1 CLI Data Pipeline

When CLI version (`mynkf`) starts, data flows through the following pipeline:

```
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

### 4.2 GUI Event and Interaction Loops

The GUI edition (`mynkf-gui`) uses immediate-mode loops linking frame updates against Windows events.

```
[Start: Create Named Mutex] (win32::CreateMutexW)
   │  * Aborts immediately (exit 0) if another process is active
   ▼
[Dynamic Font Loading] (Checks C:\Windows\Fonts for meiryo.ttc, etc.)
   │  * Avoids tofu boxes on localized systems
   ▼
[Initial Draw Frame: Frame Shadow/Border Removal] (win32::DwmSetWindowAttribute)
   │  * Deactivates standard system decorations and shadow buffers
   ▼
┌───────────────────┴───────────────────┐
▼                                       ▼
[Batch Files Convert Tab]               [Text Conversion Playground]
- Resolves paths via D&D or picker      - Monitors text area buffers
- Auto guesses file info via mynkf      - Simulates encoding/decoding live
- Performs batch overwrite saves        - Copies simulated bytes to clipboard
```

### 4.3 Encoding Guess Heuristics
Walks input buffers to evaluate grammar scores:
1. **ASCII**: All bytes fall into `0x00`..=`0x7F`.
2. **UTF-8**: Complies with UTF-8 byte syntax (proper lead and continuation bytes).
3. **EUC-JP**: Complies with EUC-JP multi-byte ranges.
4. **Shift_JIS**: Complies with Shift_JIS double-bytes or half-width Katakana (`0xA1`..=`0xDF`).
5. **BINARY**: Violates grammar rules or contains control characters.
Priority: `ASCII` ＞ `UTF-8` ＞ `EUC-JP` ＞ `Shift_JIS` ＞ `BINARY`. The highest match score dictates the final prediction.
