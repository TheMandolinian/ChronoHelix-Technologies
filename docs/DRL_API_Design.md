# HashHelix DRL API Design — Draft v1

**Scope:** Engine-only Rust API for institutional use.  
**Layer:** DRL (Deterministic Recurrence Ledger)

## 1. Design Goals

- Small, stable API surface  
- Deterministic behaviour under WDTP + NER  
- Easy to audit and test  
- Clear separation between:
  - Core engine (`DrlEngine`)
  - Legacy sequence facade (`Engine`)
  - Future lanes / epochs / vault abstractions

## 2. Core Types (Current)

### 2.1 `DrlEngine`

Defined in `engine/core.rs`, re-exported at the crate root:

```rust
pub struct DrlEngine {
    pub last_term: i64,
    pub step: usize,
}

impl DrlEngine {
    pub fn new() -> Self;
    pub fn step(&mut self) -> i64;
    pub fn step_n(&mut self, n: usize) -> i64;
}
```

Semantics:

- `new()` starts at `a₁ = 1`, `step = 1`  
- `step()` computes the next recurrence term  
- `step_n(k)` advances the recurrence `k` times  

### 2.2 `Engine` (Legacy Facade)

Defined in `src/lib.rs`:

```rust
pub struct Engine;

impl Engine {
    pub fn new() -> Self;
    pub fn sequence(&self, n_terms: usize) -> Vec<i64>;
}
```

This provides an easy sequence generator and maintains compatibility with
early Python-aligned tests.  
`DrlEngine` is the canonical engine type going forward.

## 3. Public Functions

At crate root (`lib.rs`):

```rust
pub use crate::engine::DrlEngine;

pub fn wdtp_step(prev: i64, n: usize) -> i64;
pub fn wdtp_sequence(n_terms: usize) -> Vec<i64>;
```

These two recurrence primitives form the stable public API for v1.

## 4. Testing & Equivalence

Current test suite:

- `lib.rs`:
  - `first_32_terms_match_python_reference`
  - `deterministic_replay_is_identical`
- `tests/equivalence.rs`:
  - `engine_matches_wdtp_reference_for_first_1000`

Any public-API change must:

- Preserve deterministic equivalence with the DTL engine  
- Keep all tests passing  
- Add new tests for any new public-facing functionality  

## 5. Future API Surface (Outline)

Planned structural components:

- **Lane API**
  - Lane identifiers, chiral lane sets, lane mapping logic
- **Epoch API**
  - Epoch bundles, commitments, epoch-level state
- **Vault API**
  - HOT / WARM / COLD storage abstractions, unified trait layer
- **Compression API**
  - Deterministic relic encoding/decoding

Each will be added incrementally with documentation and full test coverage.
