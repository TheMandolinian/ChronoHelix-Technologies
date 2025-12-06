use drl_engine::vault::{ColdVault, HotVault, WarmVault, Vault, VaultTier};

#[test]
fn vault_tiers_are_reported_correctly() {
    let hot = HotVault::new("hot-0");
    let warm = WarmVault::new("warm-0");
    let cold = ColdVault::new("cold-0");

    assert_eq!(hot.tier(), VaultTier::Hot);
    assert_eq!(warm.tier(), VaultTier::Warm);
    assert_eq!(cold.tier(), VaultTier::Cold);
}

#[test]
fn vault_names_are_preserved() {
    let hot = HotVault::new("primary-hot");
    assert_eq!(hot.name(), "primary-hot");
}
