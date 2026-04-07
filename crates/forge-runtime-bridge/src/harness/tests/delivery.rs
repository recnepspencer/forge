use forge_harness::facade::{ExecutionProfile, ExecutionRequest, HarnessRunner, MutationBatch, ScenarioPlan};
use forge_harness::runtime::HarnessAdapter;

use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeDeliveryErrorKind,
    BridgePreparationMode, BridgeRouteRequest, RuntimeBridgeBuilder, TruthSnapshotIdentity,
};

use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessMutation};
use crate::harness::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink, SnapshotFixture,
};
use super::support::{
    build_runtime, build_runtime_with_aspects, committed_patch, field_aspect_registration,
    field_slice_snapshot, registration, snapshot,
    RejectingSignalSink, CountingSnapshotReaderPool,
};

#[test]
fn bridge_snapshot_delivery_remains_stable_after_newer_truth_arrives() {
    let runner = HarnessRunner::new(BridgeHarnessAdapter);
    let fixture = ScenarioPlan::new(
        "bridge-snapshot-stability",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let mutation = MutationBatch::new("publish-newer-truth")
        .push(BridgeHarnessMutation::PublishCommittedPatch(committed_patch(
            "commit-b",
            "patch-b",
            "snapshot-b",
            "name",
        )))
        .push(BridgeHarnessMutation::PublishSnapshot(snapshot("snapshot-b", "bob")));
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let bundle = runner
        .execute_core(&fixture, Some(&mutation), &request, &profile)
        .expect("bridge snapshot-stability execution should succeed");

    assert_eq!(bundle.run.summary["snapshot_identity"], "snapshot-a");
}

#[test]
fn bridge_delivery_keeps_preplanned_snapshot_after_newer_truth_arrives_during_delivery() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(
        source.clone(),
        sink.clone(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan from the original committed artifact");

    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));

    let result = runtime
        .deliver_invalidation(route)
        .expect("bridge should deliver the preplanned route against its original snapshot");

    assert_eq!(result.result_summary().snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(result.receipt().snapshot_identity().as_str(), "snapshot-a");
    let delivered = sink
        .last_delivery()
        .expect("bridge sink should record the delivered artifact");
    assert_eq!(delivered.delivery.source_snapshot().as_str(), "snapshot-a");
}

#[test]
fn bridge_prepares_signal_evaluation_with_snapshot_context_without_sink_delivery() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source, sink.clone(), vec![registration()]);

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan the route");
    let evaluation = runtime
        .prepare_signal_evaluation(route)
        .expect("bridge should prepare signal evaluation");

    assert_eq!(evaluation.snapshot().snapshot_identity().as_str(), "snapshot-a");
    assert!(sink.last_delivery().is_none());
}

#[test]
fn bridge_prepared_signal_evaluation_keeps_preplanned_snapshot_after_newer_truth_arrives() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source.clone(), sink, vec![registration()]);

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan the route");

    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));

    let evaluation = runtime
        .prepare_signal_evaluation(route)
        .expect("bridge should prepare signal evaluation");

    assert_eq!(evaluation.snapshot().snapshot_identity().as_str(), "snapshot-a");
}

#[test]
fn bridge_snapshot_identity_mismatch_fails_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-snapshot-mismatch",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(
                snapshot("snapshot-a", "alice")
                    .with_read_result_identity(TruthSnapshotIdentity::new("snapshot-bad")),
            ),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let profile = ExecutionProfile::development("development");

    let mut session = adapter.create_runtime().expect("bridge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("bridge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("bridge harness load fixture");
    let error = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect_err("bridge execution should fail on snapshot identity mismatch");

    assert!(format!("{error}").to_ascii_lowercase().contains("snapshot"));
    let failure_record = session
        .runtime
        .as_ref()
        .expect("bridge runtime")
        .diagnostics()
        .last_failure_record()
        .expect("bridge failure record");
    assert!(failure_record.detail().contains("Snapshot read returned"));
    assert_eq!(failure_record.counters().snapshot_identity_mismatch_count(), 1);
}

#[test]
fn bridge_snapshot_contract_rejects_missing_required_reads() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(SnapshotFixture::new(
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![],
    ));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan before validating snapshot reads");

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("bridge should reject incomplete snapshot read results");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::SnapshotReadContractViolation);
    assert!(error.to_string().contains("returned 0 records"));
}

#[test]
fn bridge_delivery_fails_when_newer_truth_arrives_without_required_snapshot() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    let runtime = build_runtime(
        source.clone(),
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
        .expect("bridge should plan the route");

    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));

    let error = runtime
        .deliver_invalidation(route)
        .expect_err("delivery should still require the original planned snapshot");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::SnapshotAcquisitionFailure);
    assert!(error.to_string().contains("snapshot-a"));
}

#[test]
fn bridge_snapshot_reader_pool_is_used_when_configured() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let pool = CountingSnapshotReaderPool::new(source.clone());
    let runtime = RuntimeBridgeBuilder::new()
        .with_relational_source(source)
        .with_snapshot_reader_pool(pool.clone())
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .build()
        .expect("bridge runtime should build with a snapshot reader pool");

    runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
                .expect("bridge should plan the route"),
        )
        .expect("bridge delivery should succeed");

    assert_eq!(pool.acquire_count(), 1);
    assert_eq!(pool.release_count(), 1);
}

#[test]
fn bridge_sink_rejection_records_failure_diagnostics_with_slice_identity() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(field_slice_snapshot("snapshot-a", "alice"));
    let runtime = build_runtime_with_aspects(
        source,
        RejectingSignalSink,
        vec![registration()],
        vec![field_aspect_registration()],
    );

    let route = runtime
        .plan_committed_patch(BridgeRouteRequest::for_commit("commit-a"))
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

#[test]
fn bridge_bulk_delivery_keeps_preplanned_snapshots_after_newer_truth_arrives() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source.clone(), sink.clone(), vec![registration()]);

    let plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bridge should plan the bulk workload");

    source.insert_committed_patch(committed_patch("commit-c", "patch-c", "snapshot-c", "name"));
    source.insert_snapshot(snapshot("snapshot-c", "charlie"));

    let result = runtime
        .deliver_bulk_workload_plan(plan)
        .expect("bridge should deliver the preplanned bulk workload");

    assert_eq!(result.summary().selected_mode(), BridgePreparationMode::ParallelPreparation);
    assert_eq!(result.summary().delivered_route_count(), 2);
    assert_eq!(result.route_results().len(), 2);
    assert_eq!(
        result.route_results()[0]
            .result_summary()
            .snapshot_identity()
            .as_str(),
        "snapshot-a"
    );
    assert_eq!(
        result.route_results()[1]
            .result_summary()
            .snapshot_identity()
            .as_str(),
        "snapshot-b"
    );
    assert_eq!(
        sink.deliveries()
            .iter()
            .map(|delivery| delivery.delivery.source_snapshot().as_str())
            .collect::<Vec<_>>(),
        vec!["snapshot-a", "snapshot-b"]
    );
}

#[test]
fn bridge_bulk_delivery_accepts_replayed_canonical_bulk_plan() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let sink = RecordingSignalBridgeSink::default();
    let runtime = build_runtime(source, sink.clone(), vec![registration()]);

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bridge should plan the bulk workload before replay-backed delivery");
    let canonical = runtime.canonicalize_bulk_workload_plan(&planned);
    let replayed = runtime
        .replay_canonical_bulk_plan_record(&canonical)
        .expect("bridge should replay the canonical bulk plan before delivery");

    let result = runtime
        .deliver_bulk_workload_plan(replayed)
        .expect("bridge should deliver a replayed canonical bulk plan");

    assert_eq!(
        result.summary().canonical_planning_identity(),
        planned.canonical_planning_identity()
    );
    assert_eq!(
        result.summary().execution_plan_digest(),
        planned.execution_plan().digest()
    );
    assert_eq!(
        result.summary().reduced_artifact_digest(),
        planned.execution_plan().reduced_artifact().digest()
    );
    assert_eq!(result.summary().counters(), planned.execution_plan().counters());
    assert_eq!(result.summary().delivered_route_count(), 2);
    assert_eq!(result.summary().delivered_target_count(), 2);
    assert_eq!(sink.deliveries().len(), 2);
}

#[test]
fn bridge_bulk_delivery_is_stable_across_input_order() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    left_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let left_sink = RecordingSignalBridgeSink::default();
    let left_runtime = build_runtime(left_source, left_sink.clone(), vec![registration()]);

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    right_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let right_sink = RecordingSignalBridgeSink::default();
    let right_runtime = build_runtime(right_source, right_sink.clone(), vec![registration()]);

    let left_plan = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("left bulk workload should plan");
    let right_plan = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("right bulk workload should plan");

    let left_result = left_runtime
        .deliver_bulk_workload_plan(left_plan)
        .expect("left bulk workload should deliver");
    let right_result = right_runtime
        .deliver_bulk_workload_plan(right_plan)
        .expect("right bulk workload should deliver");

    assert_eq!(left_result.summary(), right_result.summary());
    assert_eq!(
        left_result
            .route_results()
            .iter()
            .map(|result| result.result_summary().route_identity().clone())
            .collect::<Vec<_>>(),
        right_result
            .route_results()
            .iter()
            .map(|result| result.result_summary().route_identity().clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        left_sink
            .deliveries()
            .iter()
            .map(|delivery| delivery.delivery.invalidation_identity().clone())
            .collect::<Vec<_>>(),
        right_sink
            .deliveries()
            .iter()
            .map(|delivery| delivery.delivery.invalidation_identity().clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn bridge_bulk_delivery_replay_matches_original_serial_fallback_path() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let original_sink = RecordingSignalBridgeSink::default();
    let original_runtime = build_runtime(source.clone(), original_sink.clone(), vec![registration()]);

    let original_plan = original_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("original serial-fallback bulk workload should plan");
    let canonical = original_runtime.canonicalize_bulk_workload_plan(&original_plan);
    let original_result = original_runtime
        .deliver_bulk_workload_plan(original_plan)
        .expect("original serial-fallback bulk workload should deliver");

    let replay_sink = RecordingSignalBridgeSink::default();
    let replay_runtime = build_runtime(source, replay_sink.clone(), vec![registration()]);
    let replayed_plan = replay_runtime
        .replay_canonical_bulk_plan_record(&canonical)
        .expect("replayed serial-fallback bulk workload should reconstruct");
    let replayed_result = replay_runtime
        .deliver_bulk_workload_plan(replayed_plan)
        .expect("replayed serial-fallback bulk workload should deliver");

    assert_eq!(original_result.summary(), replayed_result.summary());
    assert_eq!(
        original_result.summary().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        original_result
            .route_results()
            .iter()
            .map(|result| result.result_summary().invalidation_identity().clone())
            .collect::<Vec<_>>(),
        replayed_result
            .route_results()
            .iter()
            .map(|result| result.result_summary().invalidation_identity().clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        original_sink
            .deliveries()
            .iter()
            .map(|delivery| delivery.delivery.invalidation_identity().clone())
            .collect::<Vec<_>>(),
        replay_sink
            .deliveries()
            .iter()
            .map(|delivery| delivery.delivery.invalidation_identity().clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn bridge_bulk_delivery_rejects_invalid_parallel_upgrade_for_rejected_plan() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let rejected_plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("shared-truth-view workload should plan before rejection certification")
        .with_selected_mode_for_test(BridgePreparationMode::ParallelPreparation);

    let error = runtime
        .deliver_bulk_workload_plan(rejected_plan)
        .expect_err("bulk delivery should reject a plan that upgrades a rejected class");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::BulkDeliveryRejected);
    assert!(error.to_string().contains("selected mode"));
}
