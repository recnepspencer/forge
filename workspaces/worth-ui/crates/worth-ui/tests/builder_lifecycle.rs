use worth_ui::facade::app::WorthUi;
use worth_ui::facade::diagnostics::{CapabilitySnapshot, SnapshotMetrics};

#[test]
fn equivalent_builder_inputs_freeze_to_equivalent_snapshots() {
    let left = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let right = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");

    assert_equivalent_empty_snapshots(left.capabilities(), right.capabilities());
}

#[test]
fn hidden_global_registration_does_not_affect_snapshot() {
    let first_builder = WorthUi::app();
    let second_builder = WorthUi::app();

    let second_snapshot = second_builder
        .freeze()
        .expect("application preparation should succeed");
    let first_snapshot = first_builder
        .freeze()
        .expect("application preparation should succeed");

    assert_equivalent_empty_snapshots(
        first_snapshot.capabilities(),
        second_snapshot.capabilities(),
    );
}

fn assert_equivalent_empty_snapshots(left: &CapabilitySnapshot, right: &CapabilitySnapshot) {
    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
    assert_eq!(left.digest().as_u64(), right.digest().as_u64());
    assert_empty_snapshot_metrics(left.metrics());
    assert_empty_snapshot_metrics(right.metrics());
    assert!(left.registered_capabilities().is_empty());
    assert!(right.registered_capabilities().is_empty());
    assert_eq!(left.registered_capabilities().registered_family_count(), 0);
    assert_eq!(right.registered_capabilities().registered_family_count(), 0);
    assert_eq!(left.registered_capabilities().total_width(), 0);
    assert_eq!(right.registered_capabilities().total_width(), 0);
}

fn assert_empty_snapshot_metrics(metrics: SnapshotMetrics) {
    assert_eq!(metrics.registered_family_count(), 0);
    assert_eq!(metrics.total_width(), 0);
}
