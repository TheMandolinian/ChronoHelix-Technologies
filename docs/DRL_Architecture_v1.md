# HashHelix DRL Architecture — Version 1

**Project:** ChronoHelix-Technologies  
**Layer:** HashHelix DRL (Deterministic Recurrence Ledger)  
**Language:** Rust

## 1. Purpose

This document defines the initial architecture of the HashHelix DRL engine in Rust.
DRL is the private, institutional implementation that must remain deterministically
equivalent to the public HashHelix DTL engine (Python) under WDTP + NER.

Engine-only scope:

- No tokenomics
- No business logic
- No private economy layer

## 2. Core Modules (Current Layout)

- `engine/`
  - `core.rs` — `DrlEngine` (stateful WDTP+NER engine)
  - `builder.rs` — `DrlEngineBuilder` for future configuration
- `lanes/`
  - `lane.rs` — placeholder for chiral / logical lanes
- `epochs/`
  - `epoch.rs` — placeholder for epoch bundling and commitments
- `relics/`
  - `mod.rs` — placeholder for Temporal Relic types
- `vault/`
  - `hot.rs` / `warm.rs` / `cold.rs` — placeholders for storage tiers
- `compression/`
  - `mod.rs` — placeholder for deterministic compression
- `api/`
  - `public.rs` — public API surface (version, stable entrypoints)

## 3. Engine Primitive (WDTP + NER)

The engine is governed by the Waresback Deterministic Temporal Primitive (WDTP)
with the Numerical Evaluation Rule (NER):

- Seed: `a₁ = 1`
- Recurrence implemented in Rust as:

```rust
pub fn wdtp_step(prev: i64, n: usize) -> i64;
```

`DrlEngine` wraps this primitive and tracks:

- `last_term: i64` (current `a_n`)
- `step: usize` (current `n`, starting at 1)

## 4. Deterministic Guarantees

- DRL must match DTL outputs for all tested `n` under WDTP+NER.
- `tests/equivalence.rs` verifies the first 1000 terms by default.
- Floating point behaviour is locked by NER (phase reduced modulo `2π`).

## 5. Planned Extensions (Not Yet Implemented)

- Lane configuration and chiral lane sets
- Epoch bundling with Merkle-style commitments
- Vault backends for HOT / WARM / COLD tiers
- Deterministic compression pipeline
- Stable public APIs for external systems

This document will be versioned as the architecture evolves.

