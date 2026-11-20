[English](../en/DIAGRAM.md) | **日本語版**

# システム構成図 (DIAGRAM.md)

本ドキュメントでは、`MyNKF` CLI ユーティリティの処理パイプラインおよび文字コード自動判定フローを Mermaid ダイアグラムで表現します。

---

## 1. 全体データフロー (CLI ユーティリティ)

CLI コマンドが実行された際の、引数解析から入力取得、自動判定または文字コード・改行コード変換、標準出力までの処理パイプラインです。

```mermaid
graph TD
    A["開始: コマンド実行"] --> B{"引数解析"}

    %% 引数解析分岐
    B -->|"-h / --help"| C["ヘルプ表示 & 正常終了"]
    B -->|"-v / --version / --versio"| D["バージョン情報表示 & 正常終了"]
    B -->|"不明なオプション"| E["エラー出力 & 終了コード 1"]
    B -->|"オプション + ファイル / 標準入力"| F["ワイルドカード展開 (最大100件)"]

    %% 入力判別
    F --> G{"入力ソース判別"}
    G -->|"ファイル指定あり"| H["ファイル読み込み: バイナリバッファ"]
    G -->|"指定なし (標準入力)"| I["stdin 読み込み: バイナリバッファ"]

    %% 自動判定 or 変換
    H --> J{"--guess / -g モード?"}
    I --> J

    J -->|"YES"| K["自動判定: guess_encoding / detect_line_ending / count_lines"]
    K --> L["判定結果・行数・サイズを stdout 出力"]

    J -->|"NO"| M{"文字コード指定 (-w/-s/-e) あり?"}
    M -->|"NO (改行変換のみ/そのまま)"| N["バイト列直接改行置換: convert_line_endings_raw"]
    M -->|"YES"| O["自動判定 -> Unicode デコード -> 改行正規化 & エンコード"]

    N --> P["標準出力 (stdout)"]
    O --> P

    L --> Q["終了"]
    P --> Q
```

---

## 2. 文字コード自動判定アルゴリズム (Guess Flow)

バイトシーケンスの制御文字チェック、厳格な UTF-8 検証、および JIS X 0208 区点スコアリングによりエンコーディングを特定する処理フローです。

```mermaid
graph TD
    Start["バイト配列入力"] --> CheckEmpty{"空データ (0 バイト)?"}
    CheckEmpty -->|"YES"| ReturnASCII["ASCII 判定"]
    CheckEmpty -->|"NO"| CheckCtrl{"ヌル文字 / 制御文字を含む?"}

    CheckCtrl -->|"YES"| ReturnBinary["BINARY 判定"]
    CheckCtrl -->|"NO"| CheckASCII{"全バイトが 0x00..=0x7F?"}

    CheckASCII -->|"YES"| ReturnASCII
    CheckASCII -->|"NO"| CheckUTF8{"std::str::from_utf8 に適合?"}

    CheckUTF8 -->|"YES"| ReturnUTF8["UTF-8 判定"]
    CheckUTF8 -->|"NO"| CheckJIS{"Shift_JIS / EUC-JP 構文検証 & 区点スコア比較"}

    CheckJIS -->|"Shift_JIS のみ妥当 または 双方妥当で SJIS >= EUC"| ReturnSJIS["Shift_JIS 判定"]
    CheckJIS -->|"EUC-JP のみ妥当 または 双方妥当で EUC > SJIS"| ReturnEUC["EUC-JP 判定"]
    CheckJIS -->|"どちらも不適合"| ReturnBinary
```
