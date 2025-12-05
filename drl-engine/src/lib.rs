use std::f64::consts::PI;

/// Compute the next WDTP term under NER.
/// 
/// Seed: a₁ = 1 (handled by the sequence helper)
/// Recurrence for n ≥ 2:
/// aₙ = floor( n * sin( (aₙ₋₁ + π/n) mod 2π ) ) + 1
pub fn wdtp_next(prev: i64, n: u64) -> i64 {
    assert!(n >= 2, "n must be ≥ 2 for the recurrence");

    let two_pi = 2.0 * PI;

    // Numerical Evaluation Rule (NER):
    // phase must be reduced modulo 2π with a deterministic op.
    let phase = ((prev as f64) + PI / (n as f64)).rem_euclid(two_pi);

    let value = (n as f64 * phase.sin()).floor() as i64 + 1;
    value
}

/// Generate the first `len` WDTP terms (including a₁ = 1).
pub fn wdtp_sequence(len: usize) -> Vec<i64> {
    assert!(len >= 1, "sequence length must be at least 1");

    let mut seq = Vec::with_capacity(len);
    let mut a: i64 = 1;
    seq.push(a); // a₁

    for n in 2..=len as u64 {
        a = wdtp_next(a, n);
        seq.push(a);
    }

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

        #[test]
    fn first_32_terms_match_python_reference() {
        // Canonical WDTP+NER sequence for n = 1..32 from the Python DTL engine
        let expected: [i64; 32] = [
            1,   2,   1,   4,  -4,   2,   5,  -6,
            6,   1,  11, -11,  13,   9,   4, -13,
           -4,  12,  -7, -10,   9,   7,  18, -15,
          -18,  22,  -3,  -7, -16,   6,  -5,  32,
        ];

        let seq = wdtp_sequence(32);
        assert_eq!(seq, expected);
    }


    #[test]
    fn deterministic_replay_is_identical() {
        let s1 = wdtp_sequence(100);
        let s2 = wdtp_sequence(100);
        assert_eq!(s1, s2);
    }
}

