use crate::facade::BridgeRouteRequest;
use crate::facade::TruthSnapshotIdentity;

use super::super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration, surface_widening_registration,
};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

#[test]
fn coarse_and_fine_routes_remain_parity_safe_for_shared_scope() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    left_source.insert_snapshot(field_slice_snapshot(
        TruthSnapshotIdentity::new("snapshot-a"),
        "alice",
    ));
    let left_runtime = build_runtime_with_aspects(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    right_source.insert_snapshot(field_slice_snapshot(
        TruthSnapshotIdentity::new("snapshot-a"),
        "alice",
    ));
    let right_runtime = build_runtime_with_aspects(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![surface_widening_registration()],
        vec![field_aspect_registration()],
    );

    let left_result = left_runtime
        .deliver_invalidation(
            left_runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit(
                    crate::facade::TruthCommitIdentity::new("commit-a"),
                ))
                .expect("left route should plan"),
        )
        .expect("left route should deliver");
    let right_result = right_runtime
        .deliver_invalidation(
            right_runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit(
                    crate::facade::TruthCommitIdentity::new("commit-a"),
                ))
                .expect("right route should plan"),
        )
        .expect("right route should deliver");

    assert_eq!(
        left_result.result_summary().subscription_slice_identity(),
        right_result.result_summary().subscription_slice_identity()
    );
    assert_eq!(left_result.result_summary().subscription_slice_count(), 1);
    assert_eq!(right_result.result_summary().subscription_slice_count(), 1);
}
