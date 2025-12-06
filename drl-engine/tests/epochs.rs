use drl_engine::epochs::{Epoch, EpochSet};

#[test]
fn epoch_span_is_computed_correctly() {
    let e = Epoch::new(0, 1, 100);
    assert_eq!(e.span(), 100);
}

#[test]
fn epochset_basic_behavior() {
    let mut set = EpochSet::new();
    assert!(set.is_empty());

    let e = Epoch::new(0, 1, 10);
    set.add_epoch(e);

    assert_eq!(set.len(), 1);
    assert!(!set.is_empty());
}
