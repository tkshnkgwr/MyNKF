**English** | [日本語版](../ja/DIAGRAM.md)

# System Diagram (DIAGRAM.md)

This document visualizes the execution pipeline and encoding auto-detection flow of the `MyNKF` CLI utility using Mermaid diagrams.

---

## 1. Overall Data Flow (CLI Utility)

The end-to-end pipeline from CLI argument parsing through glob expansion, input reading, encoding detection or conversion, and standard output.

```mermaid
graph TD
    A["Start: Run Command"] --> B{"Parse CLI Arguments"}

    %% Argument branches
    B -->|"-h / --help"| C["Print Help & Exit 0"]
    B -->|"-v / --version / --versio"| D["Print Version & Exit 0"]
    B -->|"Unknown option"| E["Print Error & Exit 1"]
    B -->|"Options + Files / stdin"| F["Wildcard Expansion (Max 100 files)"]

    %% Input source
    F --> G{"Input Source"}
    G -->|"File paths specified"| H["Read File: Byte Buffer"]
    G -->|"No files (stdin)"| I["Read stdin: Byte Buffer"]

    %% Guess vs Conversion
    H --> J{"--guess / -g mode?"}
    I --> J

    J -->|"YES"| K["Detect: guess_encoding / detect_line_ending / count_lines"]
    K --> L["Print Encoding / Lines / Size to stdout"]

    J -->|"NO"| M{"Encoding flag (-w/-s/-e) set?"}
    M -->|"NO (Newline only / pass-through)"| N["Raw Byte Newline Replace: convert_line_endings_raw"]
    M -->|"YES"| O["Guess -> Decode to Unicode -> Normalize & Encode"]

    N --> P["Write to stdout"]
    O --> P

    L --> Q["End"]
    P --> Q
```

---

## 2. Encoding Auto-Detection Algorithm (Guess Flow)

The heuristic process identifying encodings through control-byte checks, strict UTF-8 validation, and JIS X 0208 Ku-Ten scoring.

```mermaid
graph TD
    Start["Input Byte Slice"] --> CheckEmpty{"Empty (0 bytes)?"}
    CheckEmpty -->|"YES"| ReturnASCII["ASCII"]
    CheckEmpty -->|"NO"| CheckCtrl{"Contains null / binary control bytes?"}

    CheckCtrl -->|"YES"| ReturnBinary["BINARY"]
    CheckCtrl -->|"NO"| CheckASCII{"All bytes in 0x00..=0x7F?"}

    CheckASCII -->|"YES"| ReturnASCII
    CheckASCII -->|"NO"| CheckUTF8{"Valid via std::str::from_utf8?"}

    CheckUTF8 -->|"YES"| ReturnUTF8["UTF-8"]
    CheckUTF8 -->|"NO"| CheckJIS{"Validate Shift_JIS & EUC-JP + Compare Ku-Ten Scores"}

    CheckJIS -->|"SJIS valid only OR both valid with SJIS >= EUC"| ReturnSJIS["Shift_JIS"]
    CheckJIS -->|"EUC valid only OR both valid with EUC > SJIS"| ReturnEUC["EUC-JP"]
    CheckJIS -->|"Neither valid"| ReturnBinary
```
