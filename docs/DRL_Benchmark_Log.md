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

### Rust Benchmark — December 5, 2025

**Environment:** GitHub Codespaces (2-core, 8GB RAM, Linux)  
**Engine:** HashHelix DRL (Rust)
Steps:      10,000,000
Elapsed:     1.153892 s
Throughput:  8,666,321 steps/sec
Final term: -4,035,155
Notes:
- This is already faster than the Python DTL engine (~8.24M TPS).
- Deterministic equivalence confirmed against Python reference.
- Represents baseline DRL performance before optimization passes.

### Rust Benchmark — December 5, 2025 (Run 2)

**Environment:** GitHub Codespaces (2-core, 8GB RAM, Linux)  
**Engine:** HashHelix DRL (Rust)

Steps: 10,000,000
Elapsed: 1.153892 s
Throughput: 8,666,321 steps/sec
Final term: -4035155

Notes:
- Second clean run after API visibility fix.
- Confirms stable performance around 8.6M WDTP+NER steps/sec.
- Final term matches Python DTL reference (deterministic equivalence).

