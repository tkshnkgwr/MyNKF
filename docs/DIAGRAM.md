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

---

## 4. デスクトップGUIアプリ (`mynkf-gui`) 内部イベントループと制御

eframe/egui アプリケーションのライフサイクル、Win32 APIを直接制御する直接 FFI 呼び出し、およびユーザーインターフェースイベントの流れです。

```mermaid
graph TD
    subgraph mynkf-gui [mynkf-gui アプリケーション]
        Start[起動] --> PreventDouble[1. 二重起動防止: Named Mutex]
        PreventDouble -->|既に Mutex 存在| Exit[即座に正常終了 exit 0]
        PreventDouble -->|新規起動| Init[2. アプリケーション初期化: CreationContext]
        
        Init --> LoadFonts[3. Windowsシステム日本語フォントの自動走査・動的ロード]
        LoadFonts --> StartLoop[4. eframe イベントループ開始]
        
        subgraph WindowControl [ウィンドウの描画とネイティブ制御]
            StartLoop --> FirstFrame{初回フレーム?}
            FirstFrame -->|YES| HideShadow[5. Win32 FFI: DwmSetWindowAttribute 影・枠完全削除]
            FirstFrame -->|NO| DrawWindow[6. egui: CentralPanel 角丸・外枠描画]
            HideShadow --> DrawWindow
        end
        
        subgraph CustomHeader [カスタムタイトルバー制御]
            DrawWindow --> Header[7. 自作タイトルバーヘッダー]
            Header -->|ドラッグ検知| WindowDrag[8. ViewportCommand::StartDragOS ウィンドウ移動処理]
            Header -->|X ボタンクリック| WindowClose[9. ViewportCommand::Close ウィンドウ閉じる]
            Header -->|最小化ボタン| WindowMin[10. ViewportCommand::Minimized ウィンドウ最小化]
        end
        
        subgraph TabPanel [タブ機能とデータフロー]
            DrawWindow --> TabSelect{タブ選択状態}
            TabSelect -->|ファイル一括変換| TabFile[11. ファイル一括変換パネル]
            TabSelect -->|テキスト直接変換| TabText[12. テキスト直接変換パネル]
            
            TabFile -->|ドラッグ＆ドロップ| Dnd[13. dropped_files のファイルパス取得]
            TabFile -->|ファイルダイアログ| Picker[14. rfd::FileDialog からパス取得]
            Dnd --> FileList[15. 変換リスト追加 & mynkf ライブラリで文字コード・改行自動解析]
            Picker --> FileList
            FileList -->|変換実行ボタン| LibConv[16. mynkf ライブラリで文字/改行変換 & 上書き保存]
            
            TabText -->|文字入力| Realtime[17. 入力テキスト保持]
            Realtime -->|コピーボタン| Clip[18. mynkf で出力文字コードのバイト列を模擬デコードしクリップボード転送]
        end
    end
```
```
