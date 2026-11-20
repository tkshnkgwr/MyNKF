# MyNKF (標準ライブラリ限定版)

[![Rust CI](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml)
[![Release](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://img.shields.io/badge/platform-Windows-blue.svg)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://img.shields.io/badge/rust-1.85%2B-orange.svg)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

[English README](README.md) | **日本語版**

低スペック・リソース制限のある Windows 環境での超高速・低負荷な動作を最優先に設計された、伝統的な日本語文字コード変換ツール `nkf` (Network Kanji Filter) の軽量互換 CLI ユーティリティです。

文字コード判定・変換・ワイルドカード展開のすべてのロジックを **100% Rust 標準ライブラリ (`std`) のみ** で実装しています。
**外部クレートへの依存を完全に排除**しているため、サプライチェーンリスクがゼロであり、ビルドも数秒で完了し、リリースバイナリサイズも極小（約 200〜250 KB）に抑えています。

---

## ✨ 主な特徴

- **外部依存ゼロ（100% `std` のみ）**: サードパーティ製クレートや外部共有ライブラリを一切使用せず、単体で完結・ビルド可能です。
- **JIS X 0208 完全対応**: 全 6,953 文字（94×94 区点）の Unicode マッピングテーブルをバイナリに内包し、**UTF-8 ⇆ Shift_JIS ⇆ EUC-JP** の正確な相互変換を実現します。
- **高精度な自動判定 (`-g` / `--guess`)**: `ASCII` / `UTF-8` / `Shift_JIS` / `EUC-JP` / `BINARY` の判定に加え、改行コード（`LF` / `CRLF` / `CR` / `MIXED` / `NONE`）、論理行数（`--line`）、ファイルサイズ（`--size`）を表示できます。
- **無劣化の改行コード単独変換**: 文字コードオプションを指定せず `-d` (`--lf`) または `-c` (`--crlf`) のみを指定した場合、再エンコードを行わずバイト列レベルで直接改行置換を行い、元のエンコーディングを完全に保持します。
- **半角カタカナ保持 ＆ 安全フォールバック**: 半角カタカナを欠損なく変換し、JIS 範囲外の文字や絵文字はパニックせず安全に `??` に置換します。
- **Windows ワイルドカード展開 ＆ パイプ対応**: `*` や `?` を自前で展開（安全上限 100 ファイル）し、`stdin` / `stdout` のパイプライン処理にも対応しています。
- **二段階ヘルプ**: 簡潔な構文要約（`-h`）と、詳細な仕様・実践例付きマニュアル（`--help`）を使い分けられます。

---

## 🚀 ビルド・インストール

**Rust 1.85+**（Edition 2024）が必要です。

```powershell
git clone https://github.com/tkshnkgwr/MyNKF.git
cd MyNKF
cargo build --release
```

ビルド完了後、`target/release/mynkf.exe` に単体実行バイナリが生成されます。

---

## 📖 使い方

```powershell
mynkf [options] [file...]
```

### サポートするオプション

| オプション | エイリアス              | 説明                                                                               |
| :--------- | :---------------------- | :--------------------------------------------------------------------------------- |
| `-w`       | `--utf8`                | 出力を UTF-8 に変換する（既定改行コード: `LF`）                                    |
| `-s`       | `--sjis`                | 出力を Shift_JIS に変換する（既定改行コード: `CRLF`）                              |
| `-e`       | `--euc`                 | 出力を EUC-JP に変換する（既定改行コード: `LF`）                                   |
| `-g`       | `--guess`               | 入力の文字コードと改行コードを自動判定して表示する                                 |
|            | `--line`                | 自動判定モード時に論理行数を併記する（`BINARY` では非表示）                        |
|            | `--size`                | 自動判定モード時にファイルサイズを併記する（例: `[1.2 KB]`、`BINARY` でも表示）    |
| `-d`       | `--lf`                  | 改行コードを強制的に `LF` に変換する                                               |
| `-c`       | `--crlf`                | 改行コードを強制的に `CRLF` に変換する                                             |
| `-h`       |                         | 簡潔なヘルプを表示して終了する                                                     |
|            | `--help`                | 詳細なヘルプおよび使用例を表示して終了する                                         |
| `-v`       | `--version`, `--versio` | バージョン・著作権・ライセンス情報を表示する（`--versio` は本家 `nkf` 互換別名）   |

### 使用例

```powershell
# ファイルの文字コードと改行コードを判定
mynkf -g input.txt

# 論理行数とファイルサイズを併記して判定
mynkf -g --line --size input.txt

# UTF-8 (LF) に変換して保存
mynkf -w input.txt > output_utf8.txt

# Shift_JIS (CRLF) に変換して保存
mynkf -s input.txt > output_sjis.txt

# 文字コードを維持したまま改行コードのみ CRLF に変換
mynkf -c input.txt > output_crlf.txt

# 標準入出力パイプラインで変換
type input.txt | mynkf -w > output.txt

# ワイルドカードで複数ファイルを一括判定（最大 100 ファイル）
mynkf -g *.txt
```

---

## 📚 ドキュメント一覧

| ドキュメント                                | 概要                                                   |
| :------------------------------------------ | :----------------------------------------------------- |
| [SPEC.md](docs/ja/SPEC.md)                  | 機能仕様書（CLI オプション・判定/変換ロジック）        |
| [ARCHITECTURE.md](docs/ja/ARCHITECTURE.md)  | アーキテクチャ設計書（モジュール構成・データフロー）   |
| [DIAGRAM.md](docs/ja/DIAGRAM.md)            | システム構成図（Mermaid フローチャート）               |
| [FOOTPRINTS.md](docs/ja/FOOTPRINTS.md)      | リソースフットプリント（バイナリサイズ・メモリ・CPU）  |
| [TESTING.md](docs/ja/TESTING.md)            | テスト計画・検証報告書（単体テスト・品質検証結果）     |
| [CHANGELOG.md](docs/ja/CHANGELOG.md)        | 詳細な変更履歴                                         |
| [TODO.md](docs/ja/TODO.md)                  | 実装済み機能・タスク管理・ロードマップ                 |
| [CONTRIBUTING.md](docs/ja/CONTRIBUTING.md)  | 貢献ガイドライン・環境構築・PR チェックリスト          |
| [RELEASE.md](docs/ja/RELEASE.md)            | バージョン更新・リリース手順書                         |
| [SECURITY.md](docs/ja/SECURITY.md)          | セキュリティポリシー・脆弱性報告手順                   |
| [INSTRUCTIONS.md](docs/ja/INSTRUCTIONS.md)  | 開発規約・AI コーディング指示書                        |

---

## 📄 ライセンス

本プロジェクトは GNU General Public License v3.0 or later (GPL-3.0-or-later) のもとで公開されています - Copyright (c) 2026 tkshnkgwr。詳細は [LICENSE](LICENSE) をご覧ください。
