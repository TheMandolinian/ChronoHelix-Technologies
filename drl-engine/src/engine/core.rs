use crate::wdtp_step;

/// Core DRL engine type.
///
/// This wraps the WDTP+NER recurrence in a stateful engine.
pub struct DrlEngine {
    /// Last term produced by the engine (a_n).
    pub last_term: i64,
    /// Current step index n (so next step is n+1).
    pub step: usize,
}

impl DrlEngine {
    /// Create a new engine in its initial state.
    ///
    /// WDTP seed: a₁ = 1, step = 1.
    pub fn new() -> Self {
        DrlEngine {
            last_term: 1,
            step: 1,
        }
    }

    /// Advance the engine by one step and return the new term.
    pub fn step(&mut self) -> i64 {
        // Next index is step + 1 (we start at n=1).
        let next = wdtp_step(self.last_term, self.step + 1);

        self.step += 1;
        self.last_term = next;

        next
    }

    /// Advance the engine by n steps and return the last term.
    pub fn step_n(&mut self, n: usize) -> i64 {
        for _ in 0..n {
            self.step();
        }
        self.last_term
    }
}
