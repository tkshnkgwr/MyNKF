**English** | [日本語版](../ja/SECURITY.md)

# Security Policy (SECURITY.md)

This document describes the security designs, supported releases, and vulnerability reporting procedures for the `MyNKF` project.

---

## 1. Security Architecture and Principles

`MyNKF` is designed to guarantee high safety and credibility based on the following principles:

1. **Zero External Dependencies (Pure Rust `std` Only)**:
   - Since the utility compiles without linking any third-party crates, it is immune to supply-chain attacks and vulnerabilities originating from external dependencies.
2. **Memory Safety**:
   - Leverages Rust's strong compile-time ownership checks and memory safety rules to eliminate memory safety issues (such as buffer overflows or dangling pointers).
3. **Local Execution**:
   - The tool performs operations completely locally without establishing network connections or telemetry calls.

---

## 2. Supported Versions

Security updates are provided for the following release targets:

| Version                             | Support Status |
| :---------------------------------- | :------------: |
| Latest Release (`v1.6.x` and above) | ✅ Supported   |
| Legacy Releases                     | ❌ Unsupported |

---

## 3. Reporting Vulnerabilities

If you find a security vulnerability within `MyNKF`, do not open a public Github Issue. Instead, please follow the steps below:

1. **Contact Point**:
   - Reach out directly to the repository maintainer or email the security contact address.
2. **Details to Include**:
   - Affected `MyNKF` versions and OS runtime environments.
   - Detailed descriptions of the vulnerability and reproduction steps (POC code or command examples).
3. **Response Timeline**:
   - We will review reports immediately, develop and verify patches, and publish fixed versions as quickly as possible.
