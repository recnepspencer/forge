use super::*;

#[test]
fn reconstruction_capacity_remains_typed() {
    let denial = snapshot_capacity_reconstruction_denial(29);
    assert_eq!(
        denial.kind(),
        WorthQueryConditionalRuntimeInstallationDenialKind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots: 29,
        }
    );
}
