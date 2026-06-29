# CHANGELOG

すべての変更履歴をここに記録します。

## [1.1.1] - 2026-06-29

### 追加
- **自動単体テスト**:
  - `src/main.rs` に 11 個の自動単体テスト（`cargo test`）を実装しました。
  - 文字コード自動判定（guess）、座標変換、文字コード変換、フォールバック、半角カタカナの挙動をカバー。
- **エディタ設定ファイル**:
  - コードフォーマットやエンコーディング（BOMなしUTF-8、改行LF）を統一するため、`.editorconfig` および `.vscode/settings.json` を新規作成しました。
- **CI/CD 自動化設定**:
  - プッシュ・PR時に自動テストとビルド検証を行う `.github/workflows/ci.yml` を追加。
  - タグプッシュ時に Windows 向け CLI バイナリをパッケージングし自動リリースする `.github/workflows/release.yml` を追加.
  - 依存ライブラリの週次アップデートを追従する `.github/dependabot.yml` を追加。

### 修正
- **EUC-JP -> Shift_JIS 座標変換バグの修正**:
  - `eucjp_to_sjis` 関数において、第1バイトと第2バイトの座標変換定数（それぞれ `0x70` を `0x80` へ修正、`ten + 0x7D` などを正しいJIS座標の対応式へ修正）の誤りを正し、正確な Shift_JIS 出力が行われるように修正しました。

---

## [1.1.0] - 2026-06-29

### 追加
- **ヘルプ・バージョン表示オプション**:
  - シミュレータの CLI および Rust 実装コード (`src/components/ObsidianDocs.tsx`) 双方に `--help`, `-h`, `--version`, `--versio`, `-v` オプションを追加しました。
- **各種ドキュメント類の整備**:
  - `README.md`、`README.ja.md`、`CHANGELOG.md`、`docs/SPEC.md`、`docs/DIAGRAM.md`、`docs/FOOTPRINTS.md`、`docs/TEST_REPORT.md` の作成。
- **多言語対応とリンクの整備**:
  - `README.md`（英語）と `README.ja.md`（日本語）のルート配置および相互リンク。

---

## [1.0.0] - 2026-06-28

### 追加
- **NKF-Win Rust 互換ユーティリティの初期リリース**:
  - 100% Rust 標準ライブラリのみで実装された `nkf` 互換ツール。
  - UTF-8、Shift_JIS、EUC-JP の相互変換、自動判定（--guess）をサポート。
- **Webデスクトップシミュレータの実装**:
  - Windows環境を模したGUIシミュレータ（常に手前に表示、タイトルバー非表示、背景透過をエミュレート）。
  - ドラッグ＆ドロップによるファイルアップロード、変換結果の即時ダウンロード、CLIでのコマンド実行機能。
- **Obsidian連携用ドキュメントエクスポート**:
  - Rustの全ソースコード、ライセンス、仕様書などをObsidianにそのまま貼り付け可能な形式で出力・コピー・エクスポートする機能。
