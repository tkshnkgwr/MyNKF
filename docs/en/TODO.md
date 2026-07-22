**English** | [日本語版](../ja/TODO.md)

# Task Roadmap (TODO.md)

This document maps out `MyNKF`'s current milestones ("Done"), pending works ("In Progress / Todo"), and future enhancement ideas ("Backlog").

---

## 1. Completed Milestones (Done)

### 1.1 Core Engine (`mynkf` - `src/lib.rs`)
- [x] **Auto Encoding Guess**: Predicts ASCII, UTF-8, EUC-JP, Shift_JIS, and BINARY with high accuracy.
- [x] **Newline Detection**: Identifies LF, CRLF, CR, MIXED, and NONE newline structures.
- [x] **Character Conversion**: Performs UTF-8 ⇆ EUC-JP ⇆ Shift_JIS conversions using static JIS X 0208 maps.
- [x] **Newline Normalization**: Automatically converts breaks and supports explicit forcing of LF / CRLF.
- [x] **Half-width Katakana Preservation**: Map Shift_JIS half-width Katakana to destination encodings without data loss.
- [x] **Fallback Handling**: Replaces unmapped characters with `??` instead of failing.
- [x] **Wildcard Expansion**: Expands Windows command-line path globs (`*` and `?`) natively with a 100-file safety limit.
- [x] **Cargo Features separation**: Introduced `cli` and `gui` compiler flags in `Cargo.toml` to optimize compile times and skip GUI downloads during CLI compilation.
- [x] **Tests Refactoring**: Extracted unit test cases out of `src/lib.rs` into `src/tests.rs` to lower core line count (from 986 to 786 lines).
- [x] **Automated Tests**: Validated logic using 17 core unit test cases.

### 1.2 CLI Utility (`mynkf` - `src/main.rs`)
- [x] **CLI Flags**: Supports `-w`, `-s`, `-e`, `-g`, `--line`, `--size`, `-d`, `-c`, `-h`, `-v`, and `--versio` parameters.
- [x] **Standard Streams (Pipes)**: Handles `stdin` buffer reads and pipes outputs to `stdout` when file paths are omitted.
- [x] **Batch File Runs**: Iterates and processes multiple glob-expanded files sequentially.

### 1.3 GUI Edition (`mynkf-gui` - `src/bin/mynkf-gui.rs`)
- [x] **Instance Mutex**: Leverages Win32 Named Mutex to prevent multi-launch overheads.
- [x] **Borderless Window Styling**: Eliminates native decorations and shadow overlays using direct DWM API calls.
- [x] **Custom Titlebar Dragging**: Employs dragging panels calling `ViewportCommand::StartDrag` for lag-free frame movement.
- [x] **System Font Loading**: Scans `C:\Windows\Fonts` for `meiryo.ttc` or other fallbacks to render Japanese typography.
- [x] **Batch Conversion Panels**: Adds files via drag & drop or file picker dialogs, shows predictions in grids, and saves in place.
- [x] **Live Conversion Playground**: Translates text buffers on the fly and decodes outputs mockingly to the clipboard.

---

## 2. Short-Term Tasks (In Progress / Todo)

- [ ] **Decouple Relative `common_lib` Dependency**:
  - The project references `../common_lib` via relative path bindings. Evaluate submodules or decouple operations to simplify CI setup.
- [ ] **Restore Window Position/Size State**:
  - Store application boundary bounds on exit and restore them on subsequent startups.
- [ ] **Enrich Live Conversion Playground Previews**:
  - Render byte layouts and mock encoding corruptions (mojibake previews) inside the UI before copying.
- [ ] **Verify Cross-Platform GUI Behaviors**:
  - Validate layouts on macOS/Linux and configure fallback system fonts to ensure consistent aesthetics.
- [ ] **Extend Test Coverage**:
  - Write test cases for `expand_wildcard` edge cases (e.g. invalid chars in paths) and isolate GUI logic for integration testing.

---

## 3. Long-Term Tasks (Backlog)

- [ ] **Support Additional Character Encodings**:
  - Implement conversion maps for email-legacy `JIS` (ISO-2022-JP), `UTF-16` (LE/BE), and `UTF-32`.
- [ ] **Explicit CR Line Ending Flag**:
  - Add `--cr` conversion flags to target older macOS structures (detection is already active).
- [ ] **WebAssembly (Wasm) Port of mynkf**:
  - Compile the core library into Wasm to substitute the Web desktop simulator's script conversion logic with actual native Rust libraries.
