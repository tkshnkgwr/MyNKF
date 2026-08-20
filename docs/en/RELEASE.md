**English** | [日本語版](../ja/RELEASE.md)

# Release Guide (RELEASE.md)

This document describes the manual steps and release checklists required to update version numbers and publish release tags for the `MyNKF` repository.

---

## 1. Pre-release Checklists

Ensure the codebase and related documentations satisfy the latest quality guidelines before building.

1. **Verify Code Audits**:

   ```bash
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt --check
   cargo doc --no-deps --document-private-items
   ```

2. **Review Documentations**:
   - `docs/ja/CHANGELOG.md` and `docs/en/CHANGELOG.md` reflect the release items.
   - `docs/ja/FOOTPRINTS.md` and `docs/en/FOOTPRINTS.md` contain updated binary size/memory footprints.

---

## 2. Version Updates

1. **Bump Version in `Cargo.toml`**:
   Update the `version` field:

   ```toml
   [package]
   name = "MyNKF"
   version = "X.Y.Z" # New version code
   ```

2. **Synchronize `Cargo.lock`**:

   ```bash
   cargo check
   ```

3. **Update Version References**:
   - Bump version tags inside `README.md` and `README_JA.md` descriptions.

---

## 3. Build and Tagging

1. **Test Compile Production Releases**:
   Compile local release bins to verify both CLI and GUI builds:

   ```bash
   cargo build --release
   ```

2. **Git Commit & Tag**:

   ```bash
   git add .
   git commit -m "chore: release vX.Y.Z"
   git tag -a vX.Y.Z -m "Release version X.Y.Z"
   ```

3. **Push to Remote Repository**:

   ```bash
   git push origin main --tags
   ```

---

## 4. Release Validations

- Confirm GitHub Actions (Release / Rust CI) workflows compile successfully.
- Verify GitHub Release tags are published automatically and include compiled binary artifacts (such as `MyNKF-windows-x64.zip`).
