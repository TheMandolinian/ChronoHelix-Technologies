//! COLD vault placeholder for HashHelix DRL.
//!
//! Intended for long-term, slower storage tiers.
//! DRL v0.1: metadata-only placeholder.

use super::{Vault, VaultTier};

#[derive(Debug, Clone)]
pub struct ColdVault {
    pub name: String,
}

impl ColdVault {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Vault for ColdVault {
    fn tier(&self) -> VaultTier {
        VaultTier::Cold
    }

    fn name(&self) -> &str {
        &self.name
    }
}
