**English** | [日本語版](../ja/DIAGRAM.md)

# System Diagram (DIAGRAM.md)

This document represents the execution flow and data processing structures of `MyNKF` and its Web desktop simulator using Mermaid diagrams.

---

## 1. Overall Data Flow (CLI Utility)

The pipeline showing command execution, arguments parsing, input checks, conversion, and stdout output.

```mermaid
graph TD
    A[Start: Run Command] --> B{Arguments Parse}
    
    %% Branch
    B -->|"-h / --help"| C[Print Help & Exit]
    B -->|"-v / --version / --versio"| D[Print Version & Exit]
    B -->|Flags + Files / stdin| E{Detect Input Source}
    
    %% Input Source
    E -->|File paths specified| F[Read File: Binary Buffer]
    E -->|No files | G[Read stdin stream: Binary Buffer]
    
    %% Heuristics vs Conversion
    F --> H{--guess / -g flag active?}
    G --> H
    
    H -->|YES| I[Detection logic: Guess]
    H -->|NO| J[Conversion logic: Map & Replace]
    
    %% Heuristics
    I --> K[Print encoding string to stdout]
    
    %% Conversion
    J --> L[Normalize Newlines: LF / CRLF]
    L --> M[Fallback foreign characters to '??']
    M --> N[Output to stdout or overwrite files]
    
    K --> O[End]
    N --> O
```

---

## 2. Encoding Auto-Detection Algorithm (Guess Flow)

The heuristic process identifying file encodings by scanning byte structures.

```mermaid
graph TD
    Start[Input Byte Array] --> CheckASCII{All bytes 0x00 ..= 0x7F ?}
    
    CheckASCII -->|YES| ReturnASCII[ASCII Prediction]
    CheckASCII -->|NO| CheckUTF8{Matches UTF-8 grammar rules?}
    
    CheckUTF8 -->|YES| ReturnUTF8[UTF-8 Prediction]
    CheckUTF8 -->|NO| CheckEUC{Matches EUC-JP character range?}
    
    CheckEUC -->|YES| ReturnEUC[EUC-JP Prediction]
    CheckEUC -->|NO| CheckSJIS{Matches Shift_JIS including Kana?}
    
    CheckSJIS -->|YES| ReturnSJIS[Shift_JIS Prediction]
    CheckSJIS -->|NO| ReturnBinary[BINARY Prediction]
```

---

## 3. Web Desktop Simulator & Obsidian Integration

The integration setup of the React app and Rust document exporter.

```mermaid
graph TD
    subgraph Browser_Environment [Web Browser (Vite + React)]
        A[App.tsx] --> B[DesktopSimulator.tsx]
        A --> C[ObsidianDocs.tsx]
        
        subgraph Simulator_Components [Simulator Layouts]
            B --> B1[CLI Terminal Simulator]
            B --> B2[File Drag & Drop Area]
            B --> B3[Visual Simulation: Topmost Window / Win11 Frame]
        end
        
        subgraph Export_Engine [Obsidian Integration Exporter]
            C --> C1[1. Export CHANGELOG]
            C --> C2[2. Export SPEC.md Specifications]
            C --> C3[3. Export Rust src/main.rs code]
            C --> C4[4. Export TESTING report]
            C --> C5[5. Export DIAGRAM.md diagrams]
        end
    end

    B2 -->|Dropped Files| B1
    C1 -->|Clipboard Copier / ZIP Download| User[User Obsidian]
    C2 -->|Clipboard Copier / ZIP Download| User
    C3 -->|Clipboard Copier / ZIP Download| User
    C4 -->|Clipboard Copier / ZIP Download| User
    C5 -->|Clipboard Copier / ZIP Download| User
```


