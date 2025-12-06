//! A single Epoch in the HashHelix DRL engine.
//!
//! An Epoch is a logical bundle of consecutive steps in the WDTP+NER sequence.
//! Later, we can attach commitment hashes, metadata, and verification proofs.
//!
//! DRL v0.1: metadata-only placeholder.

#[derive(Debug, Clone)]
pub struct Epoch {
    /// Epoch index (0-based or 1-based by convention; here we use 0-based).
    pub id: usize,
    /// Inclusive start step index (n_start).
    pub start_step: usize,
    /// Inclusive end step index (n_end).
    pub end_step: usize,
}

impl Epoch {
    /// Create a new epoch with the given ID and range.
    pub fn new(id: usize, start_step: usize, end_step: usize) -> Self {
        Self {
            id,
            start_step,
            end_step,
        }
    }

    /// Return how many steps are contained in this epoch.
    pub fn span(&self) -> usize {
        if self.end_step >= self.start_step {
            self.end_step - self.start_step + 1
        } else {
            0
        }
    }
}
