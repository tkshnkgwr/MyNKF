**English** | [日本語版](../ja/CONTRIBUTING.md)

# Contributing Guidelines (CONTRIBUTING.md)

Thank you for your interest in contributing to the `MyNKF` project!
This document outlines the guidelines for reporting bugs, proposing features, and submitting pull requests.

---

## 1. Development Philosophies and Principles

Please adhere to the following principles when contributing to this project:

1. **Zero External Dependencies**:
   - To keep the utility lightweight and compile-time fast, the CLI entrypoint and the core library (`src/lib.rs`) must remain strictly within the boundaries of the Rust standard library (`std`).
   - Do not add third-party crates to the `[dependencies]` section in `Cargo.toml`.
2. **Synchronized Multi-lingual Documentation**:
   - When introducing specification modifications or feature extensions, update documentations under both `docs/ja/` and `docs/en/` concurrently.

---

## 2. Environment Setup

This project depends on the shared `common_lib` repository.

1. **Clone Repositories**:

   ```bash
   # Clone both repositories side-by-side in the same parent directory
   git clone https://github.com/tkshnkgwr/common_lib.git
   git clone https://github.com/tkshnkgwr/MyNKF.git
   cd MyNKF
   ```

2. **Verify CLI**:

   ```bash
   cargo run --bin mynkf -- --help
   ```

---

## 3. Commit and PR Workflow

### Commit Messages

Use the Conventional Commits format for your commit messages:

- `feat:` Adds new features
- `fix:` Bug fixes
- `docs:` Changes to documentation
- `refactor:` Refactoring code layout
- `perf:` Performance optimizations
- `test:` Adding or updating unit tests
- `chore:` Maintenance changes to build scripts or settings

### Pull Request Checklist

Before opening a pull request, run the following commands locally and ensure everything passes with zero errors:

- [ ] `cargo test` (Ensure all unit tests pass)
- [ ] `cargo clippy --all-targets -- -D warnings` (Ensure clippy reports zero warnings)
- [ ] `cargo fmt --check` (Ensure formatting complies)
- [ ] `cargo doc --no-deps --document-private-items` (Ensure documentation builds without warnings)
