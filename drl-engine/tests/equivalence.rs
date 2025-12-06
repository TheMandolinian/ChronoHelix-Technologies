use drl_engine::{DrlEngine, wdtp_step};

#[test]
fn engine_matches_wdtp_reference_for_first_1000() {
    let mut engine = DrlEngine::new();
    let mut prev = 1i64;

    // WDTP seed: a₁ = 1
    assert_eq!(engine.last_term, 1);

    for n in 2..=1000 {
        let expected = wdtp_step(prev, n);
        let got = engine.step();

        assert_eq!(
            got, expected,
            "Mismatch at step {}: engine={} expected={}",
            n, got, expected
        );

        prev = expected;
    }
}
