# システム構成図 (DIAGRAM.md)

本ドキュメントでは、`MyNKF` の処理機構、および Web デスクトップシミュレータにおけるデータフローを Mermaid ダイアグラムを用いて表現します。

---

## 1. 全体データフロー (CLI ユーティリティ)

CLI コマンドが実行された際の、引数解析から入力判定・コード変換、出力に及ぶパイプラインです。

```mermaid
graph TD
    A[開始: コマンド実行] --> B{引数解析}
    
    %% 引数解析分岐
    B -->|"-h / --help"| C[ヘルプ表示 & 終了]
    B -->|"-v / --version / --versio"| D[バージョン情報表示 & 終了]
    B -->|変換オプション + ファイル / 標準入力| E{入力ソース判別}
    
    %% 入力判別
    E -->|ファイル名指定あり| F[ファイル読み込み: バイナリバッファ]
    E -->|なし (標準入力)| G[stdinストリーム読み込み: バイナリバッファ]
    
    %% 自動判定 or 変換
    F --> H{--guess / -g オプション?}
    G --> H
    
    H -->|YES| I[文字コード判定ロジック: Guess]
    H -->|NO| J[変換処理: Map & Replace]
    
    %% 文字コード判定
    I --> K[エンコーディング名を標準出力]
    
    %% 変換処理
    J --> L[改行コード正規化 LF/CRLF]
    L --> M[外字/未定義文字フォールバック '??']
    M --> N[標準出力 stdout または上書き]
    
    K --> O[終了]
    N --> O
```

---

## 2. 文字コード自動判定アルゴリズム (Guess Flow)

バイトシーケンスの特徴から、どのエンコーディングであるかを特定する処理フローです。

```mermaid
graph TD
    Start[バイト配列入力] --> CheckASCII{すべて 0x00 ..= 0x7F ?}
    
    CheckASCII -->|YES| ReturnASCII[ASCII 判定]
    CheckASCII -->|NO| CheckUTF8{UTF-8 の文法規則に適合するか?}
    
    CheckUTF8 -->|YES| ReturnUTF8[UTF-8 判定]
    CheckUTF8 -->|NO| CheckEUC{EUC-JP の文字範囲に適合するか?}
    
    CheckEUC -->|YES| ReturnEUC[EUC-JP 判定]
    CheckEUC -->|NO| CheckSJIS{Shift_JIS (半角カナ含む) に適合するか?}
    
    CheckSJIS -->|YES| ReturnSJIS[Shift_JIS 判定]
    CheckSJIS -->|NO| ReturnBinary[BINARY 判定]
```

---

## 3. Webデスクトップシミュレータ ＆ Obsidian連携 構造

React アプリケーションと Rust ソースコードのエクスポートプロセスの連携構造です。

```mermaid
graph TD
    subgraph Browser_Environment [Web ブラウザ環境 (Vite + React)]
        A[App.tsx] --> B[DesktopSimulator.tsx]
        A --> C[ObsidianDocs.tsx]
        
        subgraph Simulator_Components [シミュレータ構成要素]
            B --> B1[CLI Terminal Simulator]
            B --> B2[File Drag & Drop Area]
            B --> B3[Visual Simulation: Topmost Window / Win11 Frame]
        end
        
        subgraph Export_Engine [Obsidian連携エクスポート]
            C --> C1[1. CHANGELOG エクスポート]
            C --> C2[2. SPEC.md 仕様書エクスポート]
            C --> C3[3. Rust src/main.rs 全ソース]
            C --> C4[4. TEST_REPORT テスト結果]
            C --> C5[5. DIAGRAM.md システム構成図]
        end
    end

    B2 -->|アップロードされたファイル| B1
    C1 -->|Clipboard Copier / ZIP Download| User[ユーザーのObsidian]
    C2 -->|Clipboard Copier / ZIP Download| User
    C3 -->|Clipboard Copier / ZIP Download| User
    C4 -->|Clipboard Copier / ZIP Download| User
    C5 -->|Clipboard Copier / ZIP Download| User
```
