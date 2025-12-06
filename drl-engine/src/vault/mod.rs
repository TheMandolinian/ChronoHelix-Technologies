//! Vault subsystem for the HashHelix DRL engine.
//!
//! Vaults are storage tiers attached to the temporal engine:
//! - HOT: fast, low-latency, smaller retention
//! - WARM: balanced
//! - COLD: long-term, slower
//!
//! DRL v0.1: interface scaffolding only, no real storage yet.

pub mod hot;
pub mod warm;
pub mod cold;

pub use hot::HotVault;
pub use warm::WarmVault;
pub use cold::ColdVault;

/// Vault tier classification for DRL storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultTier {
    Hot,
    Warm,
    Cold,
}

/// Common interface for all vault types.
///
/// In DRL v0.1 this is metadata-only; real storage APIs will be layered later.
pub trait Vault {
    /// Return the tier (HOT / WARM / COLD).
    fn tier(&self) -> VaultTier;

    /// Human-readable name or label for the vault.
    fn name(&self) -> &str;
}
