**English** | [日本語版](../ja/INSTRUCTIONS.md)

# AI Developer Instructions (INSTRUCTIONS.md)

This document defines the coding styles, error-handling conventions, design rules, and interaction formats that AI agents (Gem) and developers must follow when modifying or extending `MyNKF` source code.

---

## 1. Naming Conventions

Follow standard Rust style guidelines.

| Target | Convention | Example |
| :--- | :--- | :--- |
| **Variables, functions, methods, parameters, modules** | `snake_case` (lowercase with underscores) | `guess_encoding`, `to_enc`, `utf8_score` |
| **Types, structures, enums, traits** | `PascalCase` (Capitalized words) | `Encoding`, `LineEnding`, `MyNkfGuiApp` |
| **Constants, static variables** | `SCREAMING_SNAKE_CASE` (uppercase with underscores) | `MAX_GLOB_FILES`, `JIS_TO_UNICODE_BASE64` |
| **Macros** | `snake_case!` | `todo!`, `matches!` |
| **Documents, files** | `UPPER_SNAKE_CASE` (uppercase with underscores) | `ARCHITECTURE.md`, `INSTRUCTIONS.md`, `README.md` |

- **Document Names**: Markdown document names in the workspace root and under the `docs/` folder must use uppercase snake case (e.g., `README.md`, `TESTING.md`).

---

## 2. Error-Handling Guidelines

MyNKF focuses on robust error handling to allow continuous execution under daemon/background setups.

- **Avoid Panic**: In library functions (`src/lib.rs`) and GUI entrypoints (`src/bin/mynkf-gui.rs`), try to avoid panicking functions like `unwrap()`, `expect()`, or `panic!`. Make sure file I/O or encoding errors do not crash the entire process.
- **Error Propagation**: Return `Result<T, E>` or `Option<T>` for operations that might fail, propagating handling to callers.
  - Return clear types like `Result<(), String>` or `Option<(u8, u8)>` inside libraries.
  - The CLI entrypoint (`src/main.rs`) returns `std::io::Result` from `fn main()`, prints descriptions via `eprintln!`, and terminates gracefully with code `1` in case of failure.
- **Fallback on Conversion Failures**: If a character fails to match encoding tables, substitute it with `??` (two question marks) instead of aborting the conversion.

---

## 3. Module Partitioning Guidelines

To minimize binary size and memory foot prints, keep modules clean:

- **`src/lib.rs` (Core Library)**:
  Aggregates pure functions dealing with buffer processing (heuristics detection, decoding/encoding tables, newline normalizations, wildcard expansions). Skips external crate dependencies entirely.
- **`src/main.rs` (CLI Entrypoint)**:
  Handles CLI-specific I/O (arguments parsing, terminal streams reading, print layouts).
- **`src/bin/mynkf-gui.rs` (GUI Entrypoint)**:
  Deals with immediate-mode layouts (`eframe`/`egui`), file picking integrations (`rfd`), and native Win32 FFI systems. Conditional compiler flag `#[cfg(feature = "gui")]` skips building this module when GUI feature is disabled.
- **Build Commands with Features**:
  Use the following commands to build specific binaries:
  - Test CLI only: `cargo build --no-default-features --features cli`
  - Test GUI only: `cargo build --no-default-features --features gui`
  - Full compile: `cargo build` (default)
- **Consolidation**: Shared logic across CLI/GUI must be extracted into `src/lib.rs`.

---

## 4. AI Interaction and Response Layouts

When proposing changes, AI agents (Gem) must comply with the following instructions:

- **Keep Explanations Short**: Focus descriptions on background architectures or technical reasoning. Avoid explaining obvious code changes.
- **Direct Copy-Paste Code Blocks**: Provide code snippets with exact filenames, target line ranges, and before/after contexts for smooth integration.
- **Japanese Rustdoc Comments**:
  When modifying or creating public APIs, ensure Japanese Rustdoc comments (`///` or `//!`) are kept in sync with implementation details. Avoid doc rot.
- **Auto-Documentation Rule**:
  Keep markdown documentations (e.g. `CHANGELOG.md`, `SPEC.md`) fully synchronized and update both `docs/en/` and `docs/ja/` copies simultaneously.
- **1,000-Line Limit**:
  If a single source file exceeds **1,000 lines** (or is expected to), propose refactorings (such as extracting testing code to `src/tests.rs` or module splitting) to keep the code clean.
