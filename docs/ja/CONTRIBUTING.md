[English](../en/CONTRIBUTING.md) | **日本語版**

# 貢献ガイドライン (CONTRIBUTING.md)

`MyNKF` プロジェクトへの貢献に興味を持っていただきありがとうございます！
本ドキュメントでは、バグ報告、機能提案、プルリクエスト提出時のガイドラインを説明します。

---

## 1. 開発方針と重要原則

開発を行う際は、以下の基本方針を遵守してください。

1. **外部依存ゼロ (CLI版)**:
   - CLI版のビルドを軽量に保つため、CLI用の処理およびライブラリコア（`src/lib.rs`）は常に Rust 標準ライブラリ (`std`) のみで機能を実装します。
   - `Cargo.toml` の `[dependencies]` にオプショナルでない外部クレートを追加しないでください。
2. **GUIのオプショナル依存とWin32制御の隔離**:
   - GUI版（`mynkf-gui`）に必要な依存は、`gui` フィーチャーを介したオプショナルな依存関係（`optional = true`）として定義します。
   - Windows 固有処理（Win32 APIを直接呼び出す FFI 等）には条件付きコンパイル `#[cfg(target_os = "windows")]` を使用し、他OSでのビルドエラーを防いでください。
3. **多言語ドキュメントの同期**:
   - 仕様変更や機能追加を行う際は、`docs/ja/` および `docs/en/` の両方の対応ドキュメントを更新してください。

---

## 2. 開発環境のセットアップ

本プロジェクトは、隣接するディレクトリにある `common_lib` に依存しています。

1. **リポジトリのクローン**:
   ```bash
   # 親フォルダ上で並列にクローンします
   git clone https://github.com/tkshnkgwr/common_lib.git
   git clone https://github.com/tkshnkgwr/MyNKF.git
   cd MyNKF
   ```
2. **動作確認 (CLI)**:
   ```bash
   cargo run --bin mynkf -- --help
   ```
3. **動作確認 (GUI)**:
   ```bash
   cargo run --bin mynkf-gui
   ```

---

## 3. コミットおよびプルリクエスト手順

### コミットメッセージの規約
コミットメッセージには Conventional Commits 形式を使用してください：

- `feat:` 新機能の追加
- `fix:` バグ修正
- `docs:` ドキュメントの変更
- `refactor:` リファクタリング
- `perf:` パフォーマンス改善
- `test:` テストの追加・修正
- `chore:` ビルドスクリプトや設定の変更

### プルリクエスト作成前のチェックリスト
プルリクエストを送信する前に、以下のコマンドを実行し全て合格することを確認してください：

- [ ] `cargo test` （ユニットテスト合格）
- [ ] `cargo clippy --all-targets -- -D warnings` （静的解析の警告ゼロ）
- [ ] `cargo fmt --check` （コードフォーマット準拠）
- [ ] `cargo doc --no-deps --document-private-items` （ドキュメントビルドエラーゼロ）
