[English](../en/TESTING.md) | **日本語版**

# テスト計画・報告書 (TESTING.md)

本ドキュメントは、`MyNKF` の機能正確性およびエッジケースへの対応を検証するためのテスト項目、検証手順、および自動テストの実行結果を記録するものです。

---

## 1. テスト環境

- **OS 環境**: Windows 11 / Windows 10
- **言語環境**: Rust 1.85+ / Edition 2024
- **検証日**: 2026-11-20

---

## 2. コマンドライン・テストケースと実行結果

### 2.1 ヘルプおよびバージョンオプションの動作検証

| テスト項目                          | コマンド・操作手順                  | 期待される結果                                                               | 合否     |
| :---------------------------------- | :---------------------------------- | :--------------------------------------------------------------------------- | :------- |
| **簡易ヘルプ表示 (`-h`)**           | `mynkf -h`                          | 構文と主要オプション一覧、`--help` への案内が簡潔に出力されること。          | **合格** |
| **詳細ヘルプ表示 (`--help`)**       | `mynkf --help`                      | オプション詳細、注意事項、使用例を含む詳細ヘルプが崩れずに出力されること。   | **合格** |
| **バージョン表示 (`--version`)**    | `mynkf --version` または `mynkf -v` | バージョン情報（v1.6.5、著作権、ライセンス）が出力されて正常終了すること。   | **合格** |
| **バージョン別名表示 (`--versio`)** | `mynkf --versio`                    | バージョン情報（v1.6.5、著作権、ライセンス）が出力されて正常終了すること。   | **合格** |

### 2.2 文字コード変換の正確性検証

| テスト項目                     | コマンド・操作手順                  | 期待される結果                                                                                               | 合否     |
| :----------------------------- | :---------------------------------- | :----------------------------------------------------------------------------------------------------------- | :------- |
| **UTF-8 <-> Shift_JIS 変換**   | `mynkf -s input_utf8.txt > out.txt` | 日本語ひらがな、カタカナ、漢字が化けずに正しく SJIS に変換されること。改行コードが `CRLF` になること。       | **合格** |
| **EUC-JP <-> UTF-8 変換**      | `mynkf -w input_euc.txt > out.txt`  | EUC-JP の文書が UTF-8 に正確に変換され、改行コードが `LF` になること。                                       | **合格** |
| **半角カタカナの保持**         | `mynkf -w sjis_with_kana.txt`       | `ｱｲｳｴｵ` などの半角カタカナが全角へ勝手変換されず、UTF-8 で正しく保持されること。                             | **合格** |
| **未定義文字のフォールバック** | `mynkf -s emoji_utf8.txt`           | SJIS に存在しない絵文字やサロゲート外文字が検出された際、別文字に化けず安全に `??` に置換されて変換されること。 | **合格** |

### 2.3 文字コード自動判定 (`--guess`)

| テスト項目         | 入力ファイル                           | 期待される結果 | 合否     |
| :----------------- | :------------------------------------- | :------------- | :------- |
| **ASCII 判定**     | `Hello World!` (制御文字なし)          | `ASCII`        | **合格** |
| **UTF-8 判定**     | `こんにちは世界` (UTF-8 保存)          | `UTF-8`        | **合格** |
| **Shift_JIS 判定** | `こんにちは世界` (SJIS 保存)           | `Shift_JIS`    | **合格** |
| **EUC-JP 判定**    | `こんにちは世界` (EUC-JP 保存)         | `EUC-JP`       | **合格** |
| **BINARY 判定**    | `0x00 0x01 0xFF 0x80` などの混合データ | `BINARY`       | **合格** |

### 2.4 エラーハンドリングの検証

| テスト項目               | コマンド・操作手順                  | 期待される結果                                                                                                                                         | 合否     |
| :----------------------- | :---------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- | :------- |
| **未知のオプション指定** | `mynkf --verison` または `mynkf -x` | 標準エラー出力に `Unknown option: --verison` と使用方法を表示し、終了ステータス `1` で即時に終了すること（入力待ちにならないこと）。                   | **合格** |
| **存在しないファイル**   | `mynkf -g nonexistent.txt`          | 標準エラー出力にファイルオープン失敗メッセージを表示し、終了ステータス `1` で終了すること。                                                            | **合格** |

---

## 3. 自動単体テスト ＆ ドキュメントテスト (`cargo test`)

コアロジックおよび境界ケースの正確性を保証するため、`src/tests.rs` に **25 件の単体テスト**、および各モジュールの RustDoc に **14 件の Doctest**（合計 39 件）を実装し、全件合格を確認しました。

### 3.1 単体テスト項目一覧

- **`test_guess_encoding_ascii`**: すべてが ASCII 範囲内のバイト列を `ASCII` と正しく判定すること。(合格)
- **`test_guess_encoding_utf8`**: UTF-8 特有バイトを含む日本語文章を `UTF-8` と正しく判定すること。(合格)
- **`test_guess_encoding_sjis`**: Shift_JIS でエンコードされた文章を `Shift_JIS` と正しく判定すること。(合格)
- **`test_guess_encoding_eucjp`**: EUC-JP 特有バイトを `EUC-JP` と正しく判定すること。(合格)
- **`test_guess_encoding_binary`**: 明らかなバイナリデータを `BINARY` と判定すること。(合格)
- **`test_guess_encoding_binary_with_null`**: テキスト中にヌルバイトが含まれる場合に `BINARY` (`Unknown`) と判定すること。(合格)
- **`test_sjis_to_eucjp_coords`**: Shift_JIS 座標から EUC-JP 座標への変換が正確に行われること。(合格)
- **`test_eucjp_to_sjis_coords`**: EUC-JP 座標から Shift_JIS 座標への変換が正確に行われること。(合格)
- **`test_conversion_utf8_to_sjis`**: UTF-8 文字列から Shift_JIS バイト列への変換および `CRLF` 強制変換が正常に動作すること。(合格)
- **`test_conversion_sjis_to_utf8`**: Shift_JIS バイト列から UTF-8 への復元が正常に動作すること。(合格)
- **`test_kanji_roundtrip`**: 漢字・ひらがな・カタカナ・記号の UTF-8 ⇆ Shift_JIS ラウンドトリップ変換が文字化けなく行われること。(合格)
- **`test_eucjp_roundtrip`**: 漢字・ひらがな・カタカナ・記号の UTF-8 ⇆ EUC-JP ラウンドトリップ変換が文字化けなく行われること。(合格)
- **`test_conversion_fallback`**: 絵文字等の未定義文字が `??` に安全にフォールバックされること。(合格)
- **`test_encode_non_bmp_no_truncation`**: サロゲート外文字（`U+13042` 等）が下位 16 ビット切り捨てで別文字に化けず `??` にフォールバックされること。(合格)
- **`test_decode_eucjp_invalid_second_byte`**: EUC-JP デコード時に不正な第 2 バイトが含まれてもアンダーフロー・パニックせず安全に置換されること。(合格)
- **`test_decode_eucjp_ss3_truncated`**: EUC-JP 補助漢字（`0x8F`）および末尾途切れ入力がパニックせず安全に処理されること。(合格)
- **`test_half_width_kana`**: 半角カタカナ（`ｱ` 等）が Shift_JIS / EUC-JP へ正確にマッピングされること。(合格)
- **`test_convert_line_endings_raw`**: 再エンコードを行わずに CRLF / LF / CR の改行コード置換が安全に行われること。(合格)
- **`test_count_lines`**: テキストデータの論理行数を正しくカウントすること。(合格)
- **`test_detect_line_ending`**: LF / CRLF / CR / MIXED / NONE をバイト列から正しく判別すること。(合格)
- **`test_wildcard_match`**: グロブの簡易ワイルドカード比較が正しく機能すること。(合格)
- **`test_expand_wildcard_normal`**: 一時ディレクトリ内のファイルに対してワイルドカード展開が機能すること。(合格)
- **`test_glob_limit_exceeded`**: 100 ファイル上限を超えた際に適切にエラーを返すこと。(合格)
- **`test_expand_wildcard_no_match_limit`**: ワイルドカード未一致フォールバック時にも 100 ファイル上限チェックが機能すること。(合格)
- **`test_format_size`**: バイトサイズを B / KB / MB / GB 単位に正しくフォーマットすること。(合格)

### 3.2 テスト実行ログ (2026-11-20)

```text
running 25 tests
test tests::test_convert_line_endings_raw ... ok
test tests::test_conversion_fallback ... ok
test tests::test_conversion_sjis_to_utf8 ... ok
test tests::test_conversion_utf8_to_sjis ... ok
test tests::test_count_lines ... ok
test tests::test_decode_eucjp_invalid_second_byte ... ok
test tests::test_decode_eucjp_ss3_truncated ... ok
test tests::test_detect_line_ending ... ok
test tests::test_encode_non_bmp_no_truncation ... ok
test tests::test_eucjp_roundtrip ... ok
test tests::test_eucjp_to_sjis_coords ... ok
test tests::test_expand_wildcard_no_match_limit ... ok
test tests::test_expand_wildcard_normal ... ok
test tests::test_format_size ... ok
test tests::test_glob_limit_exceeded ... ok
test tests::test_guess_encoding_ascii ... ok
test tests::test_guess_encoding_binary ... ok
test tests::test_guess_encoding_binary_with_null ... ok
test tests::test_guess_encoding_eucjp ... ok
test tests::test_guess_encoding_sjis ... ok
test tests::test_guess_encoding_utf8 ... ok
test tests::test_half_width_kana ... ok
test tests::test_kanji_roundtrip ... ok
test tests::test_sjis_to_eucjp_coords ... ok
test tests::test_wildcard_match ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

Doc-tests mynkf
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s
```

---

## 4. ローカル品質検証結果

ローカル環境にて以下の検証コマンドを実行し、すべての項目に合格しました。

| 検証項目                   | 実行コマンド                                   | 結果                                             | 合否     |
| :------------------------- | :--------------------------------------------- | :----------------------------------------------- | :------- |
| **構文・型チェック**       | `cargo check`                                  | 警告・エラーなし（1.34s）                        | **合格** |
| **コードフォーマット検証** | `cargo fmt --check`                            | 差分なく正常終了                                 | **合格** |
| **静的解析 (Clippy)**      | `cargo clippy --all-targets -- -D warnings`    | 警告・エラーゼロで正常終了                       | **合格** |
| **自動テスト ＆ Doctest**  | `cargo test`                                   | 単体テスト 25 件 + Doctest 14 件 すべて通過      | **合格** |
| **ドキュメントビルド検証** | `cargo doc --no-deps --document-private-items` | 警告・エラーゼロで正常終了                       | **合格** |
| **リリースビルド**         | `cargo build --release`                        | 正常終了（`mynkf.exe`: 219,136 bytes / 約 214 KB） | **合格** |
