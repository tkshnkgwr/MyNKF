**English** | [日本語版](../ja/FOOTPRINTS.md)

# Resource Footprint (FOOTPRINTS.md)

This document records the measured and estimated resource footprints (binary sizes, memory consumption, and CPU overhead) of `MyNKF` under resource-constrained environments (e.g., low-end Windows PCs).

---

## 1. Binary Size

By eliminating all external crate and shared library dependencies and building solely on the Rust standard library (`std`), `MyNKF` maintains an exceptionally small standalone executable.

| Compile Configuration                                                   | Windows Executable Size (.exe)       | Linux Executable Size | Notes                                                                               |
| :---------------------------------------------------------------------- | :----------------------------------- | :-------------------- | :---------------------------------------------------------------------------------- |
| `cargo build` (Debug)                                                   | ~3.2 MB                              | ~2.8 MB               | Includes debug symbols                                                              |
| `cargo build --release`                                                 | ~290 KB                              | ~260 KB               | Optimization enabled                                                                |
| `cargo build --release` + `strip = true`                                | ~210 KB                              | ~180 KB               | Unused symbols stripped                                                             |
| `cargo build --release` + profile tuning (2026-06-29)                   | **~170 KB** (v1.2.0: 169 KB)         | **~140 KB**           | Measured after applying full release profile options                                |
| `cargo build --release` + glob/size flags (v1.4.0)                      | **~215 KB** (Measured: 214 KB)       | -                     | Size increase due to custom wildcard expansion and size formatting                  |
| `cargo build --release` + CLI unification / egui removal (v1.6.0)       | **~203 KB**                          | -                     | Pure std CLI after removing GUI functionality and external dependencies             |
| `cargo build --release` + full JIS table & newline tuning (v1.6.1)      | **~207 KB** (Measured: 207 KB)       | -                     | Maintains tiny binary size even with complete JIS X 0208 table (6,953 chars)        |
| `cargo build --release` + `common_lib` removal & zero deps (2026-11-20) | **~214 KB** (Measured: 219,136 B)    | -                     | Latest measurement after removing unused `common_lib`, two-tier help, and bug fixes |

### Release Size Optimization Settings (`Cargo.toml`)

Production builds apply the following `[profile.release]` configuration:

```toml
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Link-Time Optimization
codegen-units = 1   # Disable parallel code generation for maximum optimization
panic = 'abort'     # Disable stack unwinding
strip = true        # Automatically strip debug symbols
```

---

## 2. Memory Consumption (Memory Usage)

- **Static Memory (Base Overhead)**: ~500 KB to 1 MB (OS thread and Rust runtime baseline).
- **Dynamic Memory (During Processing)**:
  - Primarily holds the input byte buffer and converted output buffer, keeping transient memory around `file size × 1.5 to 2.0`.
  - In `--guess` mode and newline-only conversion mode, JIS table decoding and reverse `HashMap` construction are skipped completely to avoid unnecessary allocations.

---

## 3. CPU Overhead and Performance

- **Conversion Speed**:
  - Uses O(1) array indexing for decoding, O(1) `HashMap` lookups for encoding, and direct byte-slice replacement for newline-only conversion, enabling fast throughput even on low-end CPUs.
- **Disk I/O**:
  - Performs bulk buffer reads (`File::read_to_end`) and writes (`io::stdout().write_all`) to avoid excessive syscall overhead.
