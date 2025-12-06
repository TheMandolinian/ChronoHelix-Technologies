//! HOT vault placeholder for HashHelix DRL.
//!
//! Intended for fast, low-latency storage tiers.
//! DRL v0.1: metadata-only placeholder.

use super::{Vault, VaultTier};

#[derive(Debug, Clone)]
pub struct HotVault {
    pub name: String,
}

impl HotVault {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Vault for HotVault {
    fn tier(&self) -> VaultTier {
        VaultTier::Hot
    }

    fn name(&self) -> &str {
        &self.name
    }
}
