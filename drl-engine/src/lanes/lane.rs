//! A single logical lane in the HashHelix DRL engine.
//!
//! Lanes will eventually support:
//! - chiral configurations
//! - temporal roles
//! - lane IDs
//! - linkage to epochs
//!
//! DRL v0.1: placeholder only.

#[derive(Debug, Clone)]
pub struct Lane {
    pub id: usize,
    pub name: String,
}

impl Lane {
    /// Create a new lane with an ID and name.
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
