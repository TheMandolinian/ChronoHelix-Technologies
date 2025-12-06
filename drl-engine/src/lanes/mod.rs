//! Lane subsystem for the HashHelix DRL engine.
//!
//! Lanes are higher-level logical/temporal channels that run *on top* of
//! the core WDTP+NER recurrence. They do NOT modify engine determinism.
//!
//! In DRL v0.1, lanes are scaffolding only — no execution logic yet.

pub mod lane;

pub use lane::Lane;

/// LaneSet represents a collection of logical/chiral lanes.
/// This is scaffolding for the future chiral engine (2-lane or 21-lane).
#[derive(Debug, Clone)]
pub struct LaneSet {
    pub lanes: Vec<Lane>,
}

impl LaneSet {
    /// Create an empty LaneSet.
    pub fn new() -> Self {
        Self { lanes: Vec::new() }
    }

    /// Add a lane to the set.
    pub fn add_lane(&mut self, lane: Lane) {
        self.lanes.push(lane);
    }

    /// Return how many lanes exist.
    pub fn len(&self) -> usize {
        self.lanes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lanes.is_empty()
    }
}
