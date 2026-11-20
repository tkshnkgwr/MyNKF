**English** | [日本語版](../ja/TODO.md)

# Task Roadmap (TODO.md)

This document maps out `MyNKF`'s current milestones ("Done"), pending works ("In Progress / Todo"), and future enhancement ideas ("Backlog").

---

## 1. Completed Milestones (Done)

### 1.1 Core Engine (`mynkf` - `src/lib.rs`)

- [x] **Auto Encoding Guess**: Predicts ASCII, UTF-8, EUC-JP, Shift_JIS, and BINARY with high accuracy (control char checks, strict UTF-8 validation, JIS coordinate scoring).
- [x] **Newline Detection**: Identifies LF, CRLF, CR, MIXED, and NONE newline structures.
- [x] **Character Conversion**: Performs UTF-8 ⇆ EUC-JP ⇆ Shift_JIS conversions using complete 94x94 JIS X 0208 maps (all 6,953 characters).
- [x] **Newline Normalization**: Automatically converts breaks via raw byte conversion (`convert_line_endings_raw`) and supports explicit forcing of LF / CRLF.
- [x] **Half-width Katakana Preservation**: Map Shift_JIS half-width Katakana to destination encodings without data loss.
- [x] **Fallback Handling**: Replaces unmapped characters with `??` instead of failing.
- [x] **Wildcard Expansion**: Expands Windows command-line path globs (`*` and `?`) natively with a 100-file safety limit.
- [x] **Tests Refactoring**: Extracted unit test cases out of `src/lib.rs` into `src/tests.rs`.
- [x] **Domain Modular Refactoring**: Decomposed monolithic `src/lib.rs` (~1,000 lines) into cohesive domain submodules (`types`, `newline`, `glob`, `table`, `guess`, `convert`) with a re-export facade in `src/lib.rs`, reducing every file to under 310 lines.
- [x] **Automated Tests**: Validated logic and edge cases using 25 unit tests (+ 14 doctests).
- [x] **Complete egui Removal & CLI Unification**: Removed unnecessary GUI logic and dependencies, optimizing MyNKF as a pure std CLI app.
- [x] **Complete Removal of Unused `common_lib` Dependency**: Removed the unused `../common_lib` relative path dependency from `Cargo.toml` and CI/CD workflows, achieving true zero-external-dependency standalone builds.

### 1.2 CLI Utility (`mynkf` - `src/main.rs`)

- [x] **CLI Flags**: Supports `-w`, `-s`, `-e`, `-g`, `--line`, `--size`, `-d`, `-c`, `-h`, `--help`, `-v`, and `--versio` parameters (last-flag-wins precedence).
- [x] **Two-Tier Help**: Separated output between concise help (`-h`) and comprehensive manual with examples (`--help`).
- [x] **Standard Streams (Pipes)**: Handles `stdin` buffer reads and pipes outputs to `stdout` when file paths are omitted.
- [x] **Batch File Runs**: Iterates and processes multiple glob-expanded files sequentially (exits with status code `1` on file I/O errors).
- [x] **RustDoc & Documentation Polish**: Comprehensive doc comments with working doctests (14 doctests passing) across all types and functions, with full synchronization of documentation.

---

## 2. Short-Term Tasks (In Progress / Todo)

- [ ] **Extend Test Coverage**:
  - Add CLI integration tests covering `stdin`/`stdout` pipelines, non-zero exit codes on file errors, and conflicting flag combinations (`-d` / `-c`).

---

## 3. Long-Term Tasks (Backlog)

- [ ] **Support Additional Character Encodings**:
  - Implement conversion maps for email-legacy `JIS` (ISO-2022-JP), `UTF-16` (LE/BE), and `UTF-32`.
- [ ] **Explicit CR Line Ending Flag**:
  - Add `--cr` conversion flags to target older macOS structures (detection is already active).
- [ ] **WebAssembly (Wasm) Port of mynkf**:
  - Make the core library Wasm-compatible so browser environments can execute the exact same native Rust detection and conversion logic.
