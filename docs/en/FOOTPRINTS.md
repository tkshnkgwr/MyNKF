**English** | [日本語版](../ja/FOOTPRINTS.md)

# Resource Footprint (FOOTPRINTS.md)

This document records the measured and estimated resource footprints (binary sizes, memory consumption, and CPU overheads) of `MyNKF` under resource-constrained environments (e.g., low-end Windows PCs).

---

## 1. Binary Size

By implementing custom character tables and heuristics without linking heavy third-party crates, MyNKF maintains an exceptionally small binary size.

| Compile Configuration | Windows Executable Size (.exe) | Linux Executable Size | Notes |
|:---|:---|:---|:---|
| `cargo build` (Debug) | ~3.2 MB | ~2.8 MB | Includes debug symbols |
| `cargo build --release` | ~290 KB | ~260 KB | Optimization enabled |
| `cargo build --release` + `strip = true` | ~210 KB | ~180 KB | Unused symbols stripped |
| `cargo build --release` + profile tuning (2026-06-29) | **~170 KB** (v1.2.0: 169KB) | **~140 KB** | Measured results after applying full profile options |
| `cargo build --release` + glob/size flags (v1.4.0) | **~215 KB** (Measured: 214KB) | - | Size increased due to custom wildcard expansion and formatting |
| `cargo build --release` + CLI unification / egui removal (v1.6.0) | **~203 KB** | - | Pure std CLI after completely removing GUI functionality and external dependencies |

### Recommended Settings for Size Optimization (`Cargo.toml`)
Add the following configuration to `Cargo.toml` when building production releases:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-Time Optimization
codegen-units = 1   # Disable parallel code generation for maximum optimizations
panic = "abort"     # Disable stack unwinding
strip = true        # Automatically strip debug symbols
```

---

## 2. Memory Consumption (Memory Usage)

### 2.1 CLI Utility
Since the data processing logic utilizes streams, memory allocations are kept low even when converting large files.

- **Static Memory (Base Overhead)**: ~500 KB to 1 MB (OS thread and Rust runtime requirements).
- **Dynamic Memory (During processing)**:
  - 10 MB input file: Peak private memory remains under 15 MB.
  - Generally, only input byte arrays and conversion output buffers are held, consuming `file size * 1.5 to 2.0` transient buffers.

### 2.2 Web Desktop Simulator
The browser-based simulator relies on lightweight React states.
- **Refresh Rates**: Configured at 1-second intervals to minimize GPU/CPU wakes, saving battery on mobile or low-spec systems.

---

## 3. CPU Overhead and Performance

- **Conversion Speeds**:
  - Utilizes array mapping indexes. A 100 MB text file translates in seconds, even on low-end CPUs (like Core i3).
- **Disk I/O**:
  - Leverages `std::io::BufReader` / `std::io::BufWriter` block allocations to suppress excessive system calls, operating smoothly on older HDD storage devices.
