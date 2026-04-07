use crate::facade::BridgeRouteRequest;

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};
use super::support::{
    build_runtime_with_aspects, committed_patch, field_aspect_registration, field_slice_snapshot,
    registration,
};
use crate::facade::{
    BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewSelector, HistoricalEvaluationDeclaration,
    SnapshotReadPacket, TruthBranchIdentity, TruthCommitIdentity,
};

#[test]
fn bridge_counters_expose_digest_input_bytes() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("route should plan before digest budget capture"),
        )
        .expect("delivery should succeed before digest budget capture");

    assert!(result.counters().digest_computation_count() >= 8);
    assert!(result.counters().digest_input_bytes() > 0);
}

#[test]
fn historical_evaluation_counters_capture_selector_branch_and_materialization_width() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("main"),
            TruthCommitIdentity::new("commit-a"),
        ),
        BridgeReplayMode::Enabled,
        runtime.policy().diagnostics_tier(),
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let observation = runtime
        .materialize_truth_view_observation(
            runtime
                .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
                .expect("historical declaration should plan"),
        )
        .expect("historical declaration should materialize");
    let record = runtime.canonicalize_historical_evaluation_record(&observation);

    assert_eq!(record.counters().truth_view_selector_count(), 1);
    assert_eq!(record.counters().historical_truth_view_count(), 1);
    assert_eq!(record.counters().branch_truth_view_count(), 0);
    assert_eq!(record.counters().planned_truth_view_packet_count(), 1);
    assert_eq!(record.counters().resolved_truth_view_policy_count(), 1);
    assert_eq!(record.counters().materialized_truth_view_count(), 1);
    assert_eq!(record.counters().truth_view_decision_log_count(), 1);
    assert_eq!(record.counters().selector_width(), 1);
    assert_eq!(record.counters().branch_width(), 1);
    assert_eq!(record.counters().commit_envelope_materialization_count(), 1);
    assert_eq!(record.counters().direct_snapshot_materialization_count(), 0);
    assert_eq!(record.counters().branch_head_materialization_count(), 0);
}
