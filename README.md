# ChronoHelix-Technologies — HashHelix DRL

**HashHelix DRL (Deterministic Recurrence Ledger)**  
Private Rust implementation of the HashHelix ledger for business and institutional use.

This repo is the **private / economy-side** counterpart to the public HashHelix engine:

- Public engine (DTL): `TheMandolinian/HashHelix-Ledger`  
  – **HashHelix DTL** = Deterministic Temporal Ledger (Python reference engine)  
- Private engine (DRL): `TheMandolinian/ChronoHelix-Technologies`  
  – **HashHelix DRL** = Deterministic Recurrence Ledger (Rust implementation + business stack)

The **recurrence and laws** are anchored in the public DTL repo.  
This repo focuses on **Rust performance, private apps, and institutional deployments.**

---

## Current Components

### 1. `drl-engine` (Rust crate)

Core Rust crate for the HashHelix DRL engine:

- Implements the **WDTP+NER** primitive (Waresback Deterministic Temporal Primitive + Numerical Evaluation Rule).
- Generates the same sequence as the public Python reference engine.
- Uses 64-bit integer state for high-N stability.
- Designed to be embedded into higher layers (lanes, epochs, vaults, apps) later.

#### Determinism / Equivalence

The crate currently provides:

- **First-32-term equivalence test** against the Python reference values:
  - `[1, 2, 1, 4, -4, 2, 5, -6, 6, 1, 11, -11, 13, 9, 4, -13, -4, 12, -7, -10, 9, 7, 18, -15, -18, 22, -3, -7, -16, 6, -5, 32]`
- **Deterministic replay test**:
  - Running the same (seed, steps) pair twice must produce **bit-for-bit identical** results.

These tests are the first building block for a full **Stage 12 equivalence harness**  
(Python ⇆ Rust ⇆ WASM) that will live in both repos later.

---

## Performance Benchmark (Rust DRL vs. Python DTL)

The `drl-engine` crate includes an example benchmark:

- Example: `examples/tps_bench.rs`
- Workload: **10,000,000 WDTP+NER steps**
- Result (on GitHub Codespaces, 2025-12-05):

> **Rust DRL**: ~8.56 million steps per second  
> Command: `cargo run --example tps_bench --release`

The Python DTL engine will be benchmarked in its own repo using an equivalent harness,  
so we can maintain a clean **DTL vs DRL** comparison without mixing codebases.

---

## How to Build & Test

From the repo root:

```bash
cd drl-engine

# Run the Rust unit tests (determinism + first 32 terms)
cargo test

# Run the WDTP+NER performance benchmark (10M steps)
cargo run --example tps_bench --release
