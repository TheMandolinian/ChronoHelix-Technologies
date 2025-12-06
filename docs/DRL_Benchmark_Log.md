# HashHelix DRL — Benchmark Log

This document records performance benchmarks for the Rust-based
HashHelix Deterministic Recurrence Ledger (DRL) engine.

---

## Benchmark 1 — WDTP+NER core (Rust, single-threaded)

- **Date:** 2025-12-05
- **Environment:** GitHub Codespaces (default 2-core VM, Rust stable)
- **Crate:** `drl-engine`
- **Command:**
  ```bash
  cargo run --example tps_bench --release
  cargo run --example tps_bench --release
Workload: 10,000,000 WDTP+NER steps

Result:

Elapsed: 1.167643 seconds

Throughput: 8,564,261 steps/second

Final term a_10_000_000 = -4,035,155

Notes:

This benchmark exercises only the core WDTP+NER recurrence.

Future benchmarks will compare:

Rust DRL vs Python DTL performance

Multi-threaded / batched execution

Higher-order engine layers (lanes, epochs, vault operations)
