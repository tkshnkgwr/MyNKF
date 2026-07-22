**English** | [日本語版](../ja/CHANGELOG.md)

# CHANGELOG

All notable changes to this project will be documented in this file.

## [1.5.6] - 2026-07-21

### Changed
- **Refactoring & Module Separation for Code Size Constraint**:
  - Added a 1,000-line limit policy inside `.agents/AGENTS.md`.
  - Extracted unit tests (approx. 200 lines) from `src/lib.rs` to a dedicated `src/tests.rs` module, shrinking `src/lib.rs` line count from 986 to 786 lines to improve readability.
- **Cargo Features for Compilation Optimization**:
  - Introduced `cli` and `gui` configurations in `[features]` section of `Cargo.toml`.
  - Configured graphical dependencies (`eframe`, `egui`, `rfd`) as optional (`optional = true`) to skip downloading/compiling them during CLI-only builds.
  - Configured `required-features = ["cli"]` for `mynkf` and `required-features = ["gui"]` for `mynkf-gui` binaries.

---

## [1.5.5] - 2026-07-16

### Added
- **System Architecture Design (`docs/ARCHITECTURE.md`)**:
  - Outlined system flows, tech stacks, directory intents, and Mermaid data pipelines.
- **AI Coding Instructions (`docs/INSTRUCTIONS.md`)**:
  - Standardized coding styles, naming conventions, error-handling conventions, and AI interaction templates.
- **Roadmap Task Board (`docs/TODO.md`)**:
  - Organized milestones into Done, Todo, and Backlog grids.

### Changed
- **Unified Document Naming Scheme**:
  - Modified markdown file names under `docs/` and root to use uppercase snake case.
  - Renamed `README.ja.md` to `README_JA.md`.
  - Renamed `docs/project_template_guide.md` to `docs/PROJECT_TEMPLATE_GUIDE.md`.
  - Refactored `.agents/AGENTS.md` rules and links to sync with document name updates.

---

## [1.5.4] - 2026-07-14

### Added
- **Japanese Rustdoc Comments**:
  - Added Japanese Rustdoc documentation (`///` or `//!`) to all public APIs inside `src/lib.rs`.
  - Replaced English comments with Japanese descriptions inside `src/main.rs` and `src/bin/mynkf-gui.rs`.
- **AI Guidelines**:
  - Added instructions to `.agents/AGENTS.md` to keep Rustdocs in sync with code updates.

---

## [1.5.3] - 2026-07-13

### Fixed
- **GitHub Actions CI/CD Configuration**:
  - Configured `paths-ignore` in `ci.yml` to skip test/build runners on markdown-only changes.
- **AI Guideline updates (AGENTS.md)**:
  - Exempted markdown-only changes from triggering mandatory CHANGELOG updates and CI runs.

---

## [1.5.2] - 2026-07-08

### Fixed
- **Stable GitHub Workflows & Preconditions**:
  - Reverted checkout actions version from `@v7` to stable `@v4` and release actions from `@v3` to `@v2`.
  - Added clone steps for adjacent `common_lib` dependency (`../common_lib`) before builds, using `${{ secrets.PAT || github.token }}` fallbacks for private repo tokens.
  - Unified default run paths to `MyNKF` to align subdirectory contexts.
  - Corrected ZIP target paths in release workflows to `MyNKF/target/release/MyNKF-windows-x64.zip` and fixed binary name casing bugs.
  - Documented workspace checkouts preconditions in README files.
- **eframe / egui 0.35.0 Upgrades**:
  - Migrated `App` entrypoints to `App::ui` following egui updates.
  - Replaced `Rounding` with `CornerRadius` configurations.
  - Changed `Margin::same` arguments to match integer declarations.
  - Rewrote `ui.allocate_ui_at_rect` via `UiBuilder` scopes.
  - Migrated `Frame::none()` to `Frame::new()`.
  - Replaced clipboard integrations with `ctx.copy_text` functions.

---

## [1.5.1] - 2026-06-30

### Fixed
- **Clippy Warnings for Rust 1.96 / Edition 2024**:
  - Simplified double `if let` blocks inside `add_file_paths` using Edition 2024 `&& let` (let_chains) formats.
  - Refactored redundant newline `match` blocks using `matches!` macros.
  - Suppressed Win32 API legacy naming warnings via `#[allow(clippy::upper_case_acronyms)]`.
  - Resolved 22 clippy warnings (such as replacing ranges with `.contains()` checks, collapsing conditions, and flattening iterators) inside `src/lib.rs`.
- **Badges and Documentation Updates**:
  - Embedded badges showing GitHub Action statuses, Windows supports, Rust 1.85+ recommendations, and MIT licenses to README headers.
- **Synchronized Version Flags**:
  - Updated Cargo version definitions to `1.5.1`.
  - Replaced hardcoded version strings inside `src/main.rs` with `env!("CARGO_PKG_VERSION")`.

---

## [1.5.0] - 2026-06-29

### Added
- **GUI Edition (`mynkf-gui`)**:
  - Introduced borderless desktop GUI conversions built on `eframe`/`egui`.
  - Prevents multi-launch operations using Windows Named Mutex objects.
  - Completely hides native OS window decorations (borders/shadows) using DWM FFI calls.
  - Built custom drag handles and close/minimize buttons.
  - Supports dragging files onto UI interfaces to parse and batch overwrite encodings.
  - Provides quick conversions with copy-to-clipboard actions.
- **Core Library Extractions (`src/lib.rs`)**:
  - Extracted encoding routines into a shared library module (`mynkf`).
- **Direct FFI Windows Bindings**:
  - Removed dependency on heavy `windows-sys` targets in GUI builds.

### Fixed
- **Newline Normalization (Preserving Source Encoding)**:
  - Fixes default output conversion behaviors. If no encoding flags are set, MyNKF now normalizes breaks (LF/CRLF) while retaining the original file encoding (matching standard `nkf` rules).
- **GUI Titlebar Operations**:
  - Corrected overlay collision issues where drag-areas blocked titlebar close/minimize buttons.
- **Shared Version Indicators**:
  - Synchronized versions across CLI and GUI targets using `env!("CARGO_PKG_VERSION")`.
- **Cleanup unused arguments**:
  - Removed dead `table` parameters from encoding functions.

---

## [1.4.0] - 2026-06-29

### Added
- **`--size` option (File sizes)**:
  - Appends formatted sizes (e.g. `[1.2 KB]`) inside auto guess outputs, working on both text and `BINARY` targets.

---

## [1.3.0] - 2026-06-29

### Added
- **Native Wildcard Expansion**:
  - Expands `*` and `?` paths natively on Windows commandlines.
- **Safety Queue Limit**:
  - Aborts execution if total file paths exceed 100 to protect low-spec environments.

---

## [1.2.0] - 2026-06-29

### Added
- **Newline Guess Outputs**:
  - Appends break formats (e.g., `UTF-8 (LF)`) on guess outcomes.
- **`--line` Option**:
  - Appends text logical line counts (e.g., `[120 lines]`) inside guess modes.

---

## [1.1.1] - 2026-06-29

### Added
- **Unit Tests**:
  - Created 11 test scenarios inside CLI modules.
- **Workspace Standards**:
  - Standardized workspaces using `.editorconfig` and VS Code setups.
- **Automated workflows**:
  - Integrated CI, Release, and Dependabot automation files.

### Fixed
- **Corrected Coordinate Mappings**:
  - Resolved offset bugs in EUC-JP to Shift_JIS coordinate math.
- **Brand Alignments**:
  - Renamed all binary and documentation references to `MyNKF`.
- **Unknown Options Errors**:
  - Exits with error codes when unsupported flags are passed.
- **Documented Compatibility Aliases**:
  - Documented legacy `--versio` alias options in README files.

---

## [1.1.0] - 2026-06-29

### Added
- **Help / Version Flags**:
  - Added `-h`, `--help`, `-v`, `--version`, and `--versio` behaviors.
- **Drafted Documents**:
  - Drafted SPEC, DIAGRAM, FOOTPRINTS, and TEST_REPORT documentations.
- **Multi-lingual support**:
  - Configured root README files.

---

## [1.0.0] - 2026-06-28

### Added
- **Initial release of MyNKF**:
  - 100% Rust std implementation of Japanese encoding filters.
  - Supports UTF-8, Shift_JIS, and EUC-JP conversions.
- **Web Simulators**:
  - Emulates Windows GUI experiences inside browsers.
- **Obsidian Exports**:
  - One-click copy/paste exports of files in Markdown.
