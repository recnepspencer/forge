use super::super::{
    commit_a, committed_patch, mismatched_snapshot, patch_a, registration, snapshot, snapshot_a,
};
use crate::facade::{BridgeDeliveryErrorKind, BridgeFailureClass, TruthSnapshotIdentity};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;
use worth_harness::facade::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
use worth_harness::runtime::HarnessAdapter;

#[test]
fn bridge_snapshot_identity_mismatch_fails_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-snapshot-mismatch",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch(
                commit_a(),
                patch_a(),
                snapshot_a(),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(
                snapshot(snapshot_a(), "alice").with_read_result_identity(mismatched_snapshot()),
            ),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target(
        "deliver-commit-a",
        BridgeHarnessTargetId::committed_route(commit_a()),
    );
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let _error = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect_err("bridge execution should fail on snapshot identity mismatch");

    let failure_record = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_failure_record()
        .expect("bridge failure record");
    assert_eq!(
        failure_record.failure_class(),
        &BridgeFailureClass::Delivery(BridgeDeliveryErrorKind::SnapshotIdentityMismatch)
    );
    assert_eq!(
        failure_record
            .context()
            .snapshot_identity()
            .and_then(TruthSnapshotIdentity::relational_snapshot_parts),
        Some(crate::facade::RelationalBridgeSnapshotIdentityParts::new(
            1, 1
        ))
    );
    assert_eq!(
        failure_record.counters().snapshot_identity_mismatch_count(),
        1
    );
}
