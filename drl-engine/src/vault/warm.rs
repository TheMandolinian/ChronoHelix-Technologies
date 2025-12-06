//! WARM vault placeholder for HashHelix DRL.
//!
//! Intended for balanced storage tiers.
//! DRL v0.1: metadata-only placeholder.

use super::{Vault, VaultTier};

#[derive(Debug, Clone)]
pub struct WarmVault {
    pub name: String,
}

impl WarmVault {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Vault for WarmVault {
    fn tier(&self) -> VaultTier {
        VaultTier::Warm
    }

    fn name(&self) -> &str {
        &self.name
    }
}
