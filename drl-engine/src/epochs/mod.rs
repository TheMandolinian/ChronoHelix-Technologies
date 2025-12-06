//! Epoch subsystem for the HashHelix DRL engine.
//!
//! Epochs group ranges of recurrence steps into verifiable bundles.
//! Higher layers can attach commitments, Merkle-like hashes, etc.
//!
//! DRL v0.1: scaffolding only — no commitments or hashing yet.

pub mod epoch;

pub use epoch::Epoch;

/// EpochSet represents a collection of epochs.
#[derive(Debug, Clone)]
pub struct EpochSet {
    pub epochs: Vec<Epoch>,
}

impl EpochSet {
    /// Create an empty set of epochs.
    pub fn new() -> Self {
        Self { epochs: Vec::new() }
    }

    /// Add an epoch to the set.
    pub fn add_epoch(&mut self, epoch: Epoch) {
        self.epochs.push(epoch);
    }

    /// Return how many epochs are tracked.
    pub fn len(&self) -> usize {
        self.epochs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.epochs.is_empty()
    }
}
