**English** | [日本語版](../ja/PROJECT_TEMPLATE_GUIDE.md)

# Project Setup Template Guide (PROJECT_TEMPLATE_GUIDE.md)

This document describes configuration templates and setups to quickly standardize development workspaces, automate builds and test runners, and publish release tags for Rust desktop/CLI utilities.

Refer to this guide when initializing sibling projects in the future.

---

## 1. Development Editor Configurations

### 1.1 `.editorconfig`

Aligns formatting behaviors across different text editors (forcing LF endings, BOM-less UTF-8, indentation sizes).

**Path**: `.editorconfig` (Workspace Root)

```ini
# EditorConfig is awesome: https://EditorConfig.org

root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true
indent_style = space
indent_size = 4

[*.{md,yml,yaml}]
indent_size = 2

[*.rs]
indent_size = 4
```

### 1.2 `.vscode/settings.json`

Specifies configurations for VS Code developers. Triggers formatters on file saves, aligning text rules against EditorConfig definitions.

**Path**: `.vscode/settings.json`

```json
{
  "editor.formatOnSave": true,
  "editor.trimTrailingWhitespace": true,
  "editor.insertSpaces": true,
  "editor.tabSize": 4,
  "files.insertFinalNewline": true,
  "files.eol": "\n",
  "files.encoding": "utf8",
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[markdown]": {
    "editor.tabSize": 2,
    "editor.wordWrap": "on"
  },
  "[yaml]": {
    "editor.tabSize": 2
  }
}
```

---

## 2. GitHub Actions CI/CD Workflows

### 2.1 Continuous Integration (`ci.yml`)

Triggers build validation and runs unit tests automatically on Windows runners when commits are pushed or PRs are opened. Implements Swatinem cache to optimize execution.

**Path**: `.github/workflows/ci.yml`

```yaml
name: Rust CI

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Run cargo test and cargo build (Windows)
    runs-on: windows-latest

    steps:

    - name: Checkout repository
      uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Rust cache
      uses: Swatinem/rust-cache@v2

    - name: Run cargo test
      run: cargo test --verbose

    - name: Run cargo build
      run: cargo build --release --verbose
```

### 2.2 Continuous Delivery / Automated Releases (`release.yml`)

When release version tags (e.g. `v0.2.1`) are pushed to GitHub, this workflow automatically compiles release versions for CLI and GUI targets, archives them in a single ZIP, and deploys them to GitHub Releases.

**Path**: `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:

      - 'v*'

permissions:
  contents: write

jobs:
  build-release:
    name: Build & Release (Windows)
    runs-on: windows-latest

    steps:

      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      # 1. Build CLI release binary

      - name: Build CLI release
        run: cargo build --release --verbose

      # 2. Extract CLI Binary

      - name: Package CLI binary
        shell: pwsh
        run: |
          New-Item -ItemType Directory -Force -Path target/dist
          Copy-Item -Path target/release/<YOUR_APP_NAME>.exe -Destination target/dist/<YOUR_APP_NAME>.exe -Force

      # 3. Build GUI release binary (if gui feature is defined)

      - name: Build GUI release
        run: cargo build --release --features gui --verbose

      # 4. Extract GUI Binary and Rename

      - name: Package GUI binary
        shell: pwsh
        run: |
          Copy-Item -Path target/release/<YOUR_APP_NAME>.exe -Destination target/dist/<YOUR_APP_NAME>-gui.exe -Force

      # 5. Compress both into a single ZIP archive

      - name: Archive production binaries
        shell: pwsh
        run: |
          Compress-Archive -Path target/dist/<YOUR_APP_NAME>.exe, target/dist/<YOUR_APP_NAME>-gui.exe -DestinationPath target/dist/<YOUR_APP_NAME>-windows-x64.zip -Force

      # 6. Push ZIP Asset to GitHub Releases

      - name: Create GitHub Release and Upload Asset
        uses: softprops/action-gh-release@v2
        with:
          files: target/dist/<YOUR_APP_NAME>-windows-x64.zip
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

*Note: Replace `<YOUR_APP_NAME>` with the actual name of your application defined in the `name` field of `Cargo.toml` (e.g. `bunka`).*

---

## 3. Automated Dependency Updates

### 3.1 `dependabot.yml`

Configures Dependabot to check weekly for updates to Cargo dependencies or active GitHub Action tasks, automatically opening pull requests when updates are resolved.

**Path**: `.github/dependabot.yml`

```yaml
version: 2
updates:
  # GitHub Actions updates

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"

  # Cargo (Rust) updates

  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
```

---

## 4. Cargo Release Optimizations (`Cargo.toml`)

Parameters added to optimize release binary sizes and memory footprints:

**Path**: End of `Cargo.toml`

```toml
[profile.release]
opt-level = 'z'       # Optimizes for minimum binary sizes
lto = true            # Link-Time Optimization across crates
codegen-units = 1     # Restricts code generation to single threads for inline optimizations
panic = 'abort'       # Aborts immediately on panic, removing unwinding code blocks
strip = true          # Strips debug and symbol table data completely
```

---

## 5. Standard Dependency Configurations (`Cargo.toml`)

Template dependency settings to prevent multi-launch threads and handle borderless graphics overlays via immediate-mode frames:

**Path**: `[dependencies]` / `[features]` sections in `Cargo.toml`

```toml
[dependencies]

# eframe (egui core graphics library)

eframe = { version = "0.35.0", optional = true }

# windows bindings (Win32 Mutex structures)

windows = {
    version = "0.62.0",
    features = [
        "Win32_System_Threading",
        "Win32_Foundation",
        "Win32_Security"
    ],
    optional = true
}

# winapi bindings (additional system calls)

winapi = { version = "0.3.9", features = ["winuser", "windef"], optional = true }

[features]
default = []
gui = ["dep:eframe", "dep:windows", "dep:winapi"]
```
