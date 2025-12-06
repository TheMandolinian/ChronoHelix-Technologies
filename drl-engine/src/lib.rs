use std::f64::consts::PI;

pub mod engine;
pub mod lanes;
pub mod epochs;
pub mod relics;
pub mod vault;
pub mod compression;
pub mod api;

// Re-export the core engine type at the crate root for external users/tests.
pub use crate::engine::DrlEngine;

/// HashHelix Deterministic Recurrence Ledger (DRL) Engine
/// Institutional Rust counterpart to the public Python DTL engine.
///
/// This engine computes the WDTP+NER recurrence deterministically.
/// Higher layers (lanes, epochs, vaults) will attach to this struct.
pub struct Engine;

impl Engine {
    /// Creates a new engine instance (placeholder for config later)
    pub fn new() -> Self {
        Engine
    }

    /// Computes n terms of the WDTP+NER recurrence.
    ///
    /// Returns a Vec<i64> of length n_terms.
    /// The first term is always a₁ = 1.
    pub fn sequence(&self, n_terms: usize) -> Vec<i64> {
        wdtp_sequence(n_terms)
    }
}

/// Core WDTP+NER deterministic recurrence.
/// Private, engine-facing function.
pub fn wdtp_sequence(n_terms: usize) -> Vec<i64> {
    let tau: f64 = 2.0 * PI;
    let mut seq: Vec<i64> = Vec::with_capacity(n_terms);

    let mut a: f64 = 1.0;
    seq.push(1);

    for n in 2..=n_terms {
        let phase = (a + PI / (n as f64)) % tau;
        a = ( (n as f64) * phase.sin() ).floor() + 1.0;
        seq.push(a as i64);
    }

    seq
}

// ----------------------------- //
// Tests                        //
// ----------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_32_terms_match_python_reference() {
        let expected: [i64; 32] = [
            1, 2, 1, 4, -4, 2, 5, -6, 6, 1, 11, -11, 13, 9, 4, -13,
            -4, 12, -7, -10, 9, 7, 18, -15, -18, 22, -3, -7, -16, 6, -5, 32,
        ];

        let eng = Engine::new();
        let seq = eng.sequence(32);
        assert_eq!(seq, expected);
    }

    #[test]
    fn deterministic_replay_is_identical() {
        let eng = Engine::new();
        let s1 = eng.sequence(100);
        let s2 = eng.sequence(100);
        assert_eq!(s1, s2);
    }
}

/// Compute the next WDTP+NER term given the previous term and step index.
/// This is the *canonical* DRL engine primitive.
pub fn wdtp_step(prev: i64, n: usize) -> i64 {
    use std::f64::consts::PI;

    // Numerical Evaluation Rule: reduce phase mod 2π deterministically
    let phase = ((prev as f64) + (PI / n as f64)) % (2.0 * PI);

    let value = (n as f64 * phase.sin()).floor() as i64 + 1;
    value
}
