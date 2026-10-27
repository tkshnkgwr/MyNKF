# MyNKF (標準ライブラリ限定版)

[![Rust CI](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/ci.yml)
[![Release](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml/badge.svg)](https://github.com/tkshnkgwr/MyNKF/actions/workflows/release.yml)
[![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)](https://img.shields.io/badge/platform-Windows-blue.svg)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://img.shields.io/badge/rust-1.85%2B-orange.svg)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

[English README](README.md) | **日本語版**

低スペック・リソース制限のある Windows 環境での超高速・低負荷な動作を最優先に設計された、伝統的な日本語文字コード変換ツール `nkf` (Network Kanji Filter) の軽量互換 CLI ユーティリティです。

**100% Rust 標準ライブラリのみ** で主要な文字コード判定・変換アルゴリズムが実装されています。
外部クレートへの依存が完全に排除され、コンパイル後のバイナリサイズを最小限（リリースビルド時 約200〜250KB）に抑えています。

## 📄 ライセンス

本プロジェクトは GNU General Public License v3.0 or later (GPL-3.0-or-later) のもとで公開されています - Copyright (c) 2026 tkshnkgwr。詳細は [LICENSE](LICENSE) をご覧ください。
