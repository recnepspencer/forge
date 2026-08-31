use super::PersistentVector;

#[test]
fn reservation_stays_on_the_flat_ordinary_lane() {
    let mut ordinary = PersistentVector::<u64, 64>::new();
    ordinary.reserve_exclusive(4_096);
    assert!(ordinary
        .exclusive_capacity()
        .is_some_and(|capacity| capacity >= 4_096));
    ordinary.extend(0..4_096);

    let mut fork = ordinary.fork_persistent();
    assert!(ordinary.shares_storage_with(&fork));
    fork.reserve_exclusive(65_536);
    assert!(
        ordinary.shares_storage_with(&fork),
        "reservation must not flatten a genuinely shared fork"
    );

    fork.push_back(4_096);
    assert_eq!(ordinary.len(), 4_096);
    assert_eq!(fork.len(), 4_097);
    assert_eq!(fork.get(4_096), Some(&4_096));
}
