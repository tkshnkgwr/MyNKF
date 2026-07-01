# MyNKF (標準ライブラリ限定版)

[![Rust CI](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml)
[![Release](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://img.shields.io/badge/platform-Windows-blue.svg)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://img.shields.io/badge/rust-1.85%2B-orange.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

👉 **[English Version is here](README.md)**

低スペック・リソース制限のある Windows 環境での超高速・低負荷な動作を最優先に設計された、伝統的な日本語文字コード変換ツール `nkf` (Network Kanji Filter) の軽量互換ユーティリティ（CLI版 ＆ GUI版）です。

**100% Rust 標準ライブラリのみ** で主要な文字コード判定・変換アルゴリズムが実装されています。
CLI版 (`mynkf`) は外部クレートへの依存が完全に排除され、コンパイル後のバイナリサイズを最小限（リリースビルド時 約200〜250KB）に抑えています。
GUI版 (`mynkf-gui`) は、`eframe`/`egui` フレームワークを使用した超軽量・高速常駐型のデスクトップGUIアプリで、外部クレートサイズを極力小さく保ちつつ Windows API の直接制御 (FFI) を行うことで極小サイズを実現しています。

## 主な機能

- **GUI版 (`mynkf-gui`) による一括操作**:
  - ファイルのドラッグ＆ドロップまたはファイルダイアログにより、現在の文字コード/改行コードをリアルタイム自動判定。
  - 出力文字コード・改行コードを選択し、一括で上書き変換可能。
  - テキストコピペ領域により、リアルタイムに文字コード・改行コードの変換結果をプレビューしクリップボードにコピー可能。
  - 多重起動を防止する Named Mutex を搭載し、無駄なプロセス常駐を防止。
  - Windows API (DWM) を直接制御してウィンドウの枠や影を完全に消した、スタイリッシュな枠なし・透過フラットデザイン。
- **文字コードの相互変換**:
  - `UTF-8` ⇆ `EUC-JP`
  - `Shift-JIS` ⇆ `EUC-JP`
  - `Shift-JIS` ⇆ `UTF-8`
- **自動文字コード判定 (`--guess` オプション)**:
  - 入力ストリームおよび複数ファイルの文字コードを自動判定します。非 `BINARY` ファイルの場合は、判定された改行コード（LF / CRLF / CR / MIXED / NONE）も併せて表示します（例: `UTF-8 (LF)`）。
  - さらに `--line` オプションを指定した場合、テキストファイルの論理行数も併記します（例: `UTF-8 (LF) [100 lines]`、`BINARY` の場合は表示されません）。
  - さらに `--size` オプションを指定した場合、フォーマットされたファイルサイズを併記します（例: `UTF-8 (LF) [1.2 KB]`、`BINARY` の場合もサイズは併記されます）。両方の併用も可能です。
- **システムヘルプ・バージョン情報**:
  - `-h` / `--help`: 詳細なコマンドラインヘルプおよび対応オプション一覧を表示します。
  - `-v` / `--version` / `--versio`: ユーティリティのバージョン情報（v1.5.0）を表示します（`--versio` は本家 `nkf` との完全な互換性を維持するためのエイリアスです）。
- **改行コードの自動・明示変換**:
  - EUC-JP・UTF-8変換時: 改行コードを `LF` に正規化。
  - Shift-JIS変換時: 改行コードを `CRLF` に正規化。
- **半角カタカナの完全保持**:
  - `0xA1`〜`0xDF` の半角カタカナ領域を全角変換せず、対象エンコードに合わせて透過的に変換・保持します。
- **外字・マッピング定義外文字のフォールバック**:
  - JIS X 0208 規格外の絵文字や特殊文字が検出された場合、エラーにせず安全に `??` (疑問符2つ) へ自動置換します。
- **標準入出力（パイプ）および複数ファイル指定に対応**:
  - 引数なしの場合は標準入力 (`stdin`) からのパイプ入力を受け取り、標準出力 (`stdout`) へストリーム転送。
  - 複数のファイルパスを引数に指定し、一括処理することが可能です。
  - Windows 環境（PowerShellやcmd.exe等）でもファイル指定にワイルドカード（`*` や `?`）を使用できます（自動で展開されます）。安全のため一度に処理できる上限は最大 **100ファイル** です。

## クイックスタート (ビルド方法)

Windows PC で本プログラムをビルド・使用するには以下の手順を行います：

```powershell
# プロジェクトディレクトリに移動
cd MyNKF

# テストの実行 (デグレ検証)
cargo test

# リリースビルド（CLI版 & GUI版の双方を極小・最適化ビルド）
cargo build --release
```

ビルドが完了すると、`target/release/` ディレクトリ内に以下のバイナリが生成されます。
- `mynkf.exe` (CLI版文字コードコンバータ)
- `mynkf-gui.exe` (GUI版文字コードコンバータ)

### 実行例

#### CLI版 (`mynkf`)

```powershell
# ヘルプ情報を表示する
cargo run --bin mynkf -- --help

# ファイルの文字コードを推測する (ファイルサイズも併記)
cargo run --bin mynkf -- --guess --size input.txt

# input.txt を Shift-JIS (CRLF) に変換してファイルへ書き出す
cargo run --bin mynkf -- -s input.txt > output_sjis.txt

# パイプライン連携 (標準入出力)
type input_utf8.txt | cargo run --bin mynkf -- -e > output_euc.txt
```

#### GUI版 (`mynkf-gui`)

```powershell
# GUI版を起動する
cargo run --bin mynkf-gui

# リリース用単体実行バイナリの起動
.\target\release\mynkf-gui.exe
```

## ライセンス

本プロジェクトは MIT ライセンスの下で提供されています。詳細は [LICENSE](LICENSE) ファイルをご覧ください。
