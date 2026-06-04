use crate::facade::{BridgeDeliveryErrorKind, BridgeRouteRequest, TruthSnapshotIdentity};
use crate::harness::fixtures::InMemoryRelationalBridgeSource;

use super::super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration, RejectingSignalSink,
};

#[test]
fn bridge_sink_rejection_records_failure_diagnostics_with_slice_identity() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch(
        crate::facade::TruthCommitIdentity::new("commit-a"),
        crate::facade::TruthPatchIdentity::new("patch-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid harness field key"),
    ));
    source.insert_snapshot(field_slice_snapshot(
        TruthSnapshotIdentity::new("snapshot-a"),
        "alice",
    ));
    let runtime = build_runtime_with_aspects(
        source,
        RejectingSignalSink,
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new("commit-a"),
        ))
        .expect("route should plan before sink rejection");
    let expected_slice_identity = route
        .lowering_summary()
        .subscription_slice_identity()
        .clone();

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("delivery should surface the sink rejection");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::SignalSinkRejection);
    let failure = runtime
        .diagnostics()
        .last_failure_record()
        .expect("sink rejection should be recorded in diagnostics");
    assert_eq!(
        failure.subscription_slice_identity().map(|id| id.as_str()),
        Some(expected_slice_identity.as_str())
    );
    assert!(failure.invalidation_identity().is_some());
}
