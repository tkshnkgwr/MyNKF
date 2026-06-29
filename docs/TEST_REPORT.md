# テストレポート (TEST_REPORT.md)

本ドキュメントは、`MyNKF` の機能正確性、エッジケースへの対応、および Web シミュレータの整合性を検証するためのテスト項目、検証手順、およびセルフテストの結果を記録するものです。

---

## 1. テスト環境
- **OS環境**: Windows 11 / Windows 10
- **言語環境**: Rust 1.70+ / Node.js 18+
- **検証日**: 2026-06-29

---

## 2. コマンドライン・テストケースと実行結果

### 2.1 ヘルプおよびバージョンオプションの動作検証 (新規機能)

| テスト項目 | コマンド・操作手順 | 期待される結果 | 合否 |
| :--- | :--- | :--- | :--- |
| **ヘルプ表示 (`--help`)** | `MyNKF --help` または `MyNKF -h` | オプション一覧、使用例、バージョンを含むヘルプ表示が崩れずに出力されること。 | **合格** |
| **バージョン表示 (`--version`)** | `MyNKF --version` または `MyNKF -v` | バージョン番号 `MyNKF v1.1.0` が出力されて正常終了すること。 | **合格** |
| **バージョン別名表示 (`--versio`)** | `MyNKF --versio` | バージョン番号 `MyNKF v1.1.0` が出力されて正常終了すること。 | **合格** |

### 2.2 文字コード変換の正確性検証

| テスト項目 | コマンド・操作手順 | 期待される結果 | 合否 |
| :--- | :--- | :--- | :--- |
| **UTF-8 ⇆ Shift_JIS 変換** | `MyNKF -s input_utf8.txt > out.txt` | 日本語ひらがな、カタカナ、漢字が化けずに正しくSJISに変換されること。改行コードが `CRLF` になること。 | **合格** |
| **EUC-JP ⇆ UTF-8 変換** | `MyNKF -w input_euc.txt > out.txt` | EUC-JPの文書がUTF-8に正確に変換され、改行コードが `LF` になること。 | **合格** |
| **半角カタカナの保持** | `MyNKF -w sjis_with_kana.txt` | `ｱｲｳｴｵ` などの半角カタカナがマージされず、文字化けや全角への勝手な変換なしにUTF-8で正しく保持されること。 | **合格** |
| **未定義文字のフォールバック** | `MyNKF -s emoji_utf8.txt` | SJISに存在しない絵文字などが検出された際、エラー終了せず、該当箇所が安全に `??` に置換されて変換されること。 | **合格** |

### 2.3 文字コード自動判定 (`--guess`)

| テスト項目 | 入力ファイル | 期待される結果 | 合否 |
| :--- | :--- | :--- | :--- |
| **ASCII 判定** | `Hello World!` (制御文字なし) | `ASCII` | **合格** |
| **UTF-8 判定** | `こんにちは世界` (UTF-8保存) | `UTF-8` | **合格** |
| **Shift_JIS 判定** | `こんにちは世界` (SJIS保存) | `Shift_JIS` | **合格** |
| **EUC-JP 判定** | `こんにちは世界` (EUC-JP保存) | `EUC-JP` | **合格** |
| **BINARY 判定** | `0x00 0x01 0xFF 0x80` などの混合データ | `BINARY` | **合格** |

### 2.4 未知のオプションエラーの検証 (新規機能)

| テスト項目 | コマンド・操作手順 | 期待される結果 | 合否 |
| :--- | :--- | :--- | :--- |
| **未知のオプション指定** | `MyNKF --verison` または `MyNKF -x` | 標準エラー出力に `Error: Unknown option '--verison'` のようにエラーメッセージと使用方法を表示し、終了ステータス `1` で即時に終了すること（入力待ちにならないこと）。 | **合格** |

---

## 3. Webデスクトップシミュレータの結合テスト項目

シミュレータが動作する Web インターフェースの検証項目です。

1. **ドラッグ＆ドロップ機能**:
   - 任意のエンコードのテキストファイルをドラッグエリアにドロップした際、文字コードが即座に自動判定され、プレビューエリアにデコードされたテキストが表示されることを確認。 (合格)
2. **変換とダウンロード機能**:
   - 変換先コードに「Shift_JIS (CRLF)」「UTF-8 (LF)」「EUC-JP (LF)」を選択し、ダウンロードボタンを押下した際、選択したコードおよび改行コードでバイナリが生成され、ブラウザから安全にダウンロードできることを確認。 (合格)
3. **CLI擬似ターミナル**:
   - 画面の「コマンドライン実行」にて、`MyNKF -w sample.txt` や `MyNKF --guess sample.txt` などのコマンドを打ち込み、Enterを押下した際に、CLI仕様に適合したテキストがシミュレータ上に流れることを確認。 (合格)
   - `--help`, `--version`, `--versio` などのシステムオプションもシミュレータ上で期待通りのテキストが出力されることを確認。 (合格)

---

## 4. 自動単体テスト (`cargo test`)

コアロジックの正確性を保証するため、`src/main.rs` の末尾に 11 個のテストケースからなるテストモジュールを実装し、すべて合格しました。

### 4.1 テスト項目と結果
- **`test_guess_encoding_ascii`**: すべてが ASCII 範囲内のバイト列を `ASCII` と正しく判定すること。(合格)
- **`test_guess_encoding_utf8`**: UTF-8 特有バイトを含む長い日本語文章を `UTF-8` と正しく判定すること。(合格)
- **`test_guess_encoding_sjis`**: Shift_JIS でエンコードされた文章を `Shift_JIS` と正しく判定すること。(合格)
- **`test_guess_encoding_eucjp`**: EUC-JP 特有バイト（"日本語" in EUC-JP など）を `EUC-JP` と判定すること。(合格)
- **`test_guess_encoding_binary`**: 明らかなバイナリデータ（非テキスト）を `BINARY` と判定すること。(合格)
- **`test_sjis_to_eucjp_coords`**: Shift_JIS 座標から EUC-JP 座標への変換が正確に行われること。(合格)
- **`test_eucjp_to_sjis_coords`**: EUC-JP 座標から Shift_JIS 座標への変換が正確に行われること。※バグ修正済み(合格)
- **`test_conversion_utf8_to_sjis`**: UTF-8 文字列から Shift_JIS バイト列への相互変換および改行コード `CRLF` の強制変換が正常に動作すること。(合格)
- **`test_conversion_sjis_to_utf8`**: Shift_JIS バイト列から UTF-8 への復元が正常に動作すること。(合格)
- **`test_conversion_fallback`**: 変換先エンコーディングにマッピングを持たない外字（絵文字など）を `??` に安全にフォールバックすること。(合格)
- **`test_half_width_kana`**: 半角カタカナ（`ｱ` など）がエンコーディング変換時に切り詰め・文字化けせず、対応するコードポイントに正しくマッピングされること。(合格)

### 4.2 テスト実行ログ
```text
running 11 tests
test tests::test_sjis_to_eucjp_coords ... ok
test tests::test_guess_encoding_ascii ... ok
test tests::test_guess_encoding_binary ... ok
test tests::test_guess_encoding_eucjp ... ok
test tests::test_guess_encoding_sjis ... ok
test tests::test_guess_encoding_utf8 ... ok
test tests::test_eucjp_to_sjis_coords ... ok
test tests::test_conversion_sjis_to_utf8 ... ok
test tests::test_half_width_kana ... ok
test tests::test_conversion_utf8_to_sjis ... ok
test tests::test_conversion_fallback ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```
