use drl_engine::lanes::{Lane, LaneSet};

#[test]
fn laneset_basic_behavior() {
    let mut set = LaneSet::new();
    assert!(set.is_empty());

    let lane = Lane::new(0, "primary");
    set.add_lane(lane);

    assert_eq!(set.len(), 1);
    assert!(!set.is_empty());
}
