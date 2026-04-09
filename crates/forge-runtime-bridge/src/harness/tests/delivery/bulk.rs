use super::*;

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

    assert_eq!(
        result.summary().selected_mode(),
        BridgePreparationMode::ParallelPreparation
    );
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
    assert_eq!(
        result.summary().counters(),
        planned.execution_plan().counters()
    );
    assert_eq!(result.summary().delivered_route_count(), 2);
    assert_eq!(result.summary().delivered_target_count(), 2);
    assert_eq!(sink.deliveries().len(), 2);
}

#[test]
fn bridge_bulk_delivery_is_stable_across_input_order() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch(
        "commit-a",
        "patch-a",
        "snapshot-a",
        "name",
    ));
    left_source.insert_committed_patch(committed_patch(
        "commit-b",
        "patch-b",
        "snapshot-b",
        "name",
    ));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    left_source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let left_sink = RecordingSignalBridgeSink::default();
    let left_runtime = build_runtime(left_source, left_sink.clone(), vec![registration()]);

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch(
        "commit-a",
        "patch-a",
        "snapshot-a",
        "name",
    ));
    right_source.insert_committed_patch(committed_patch(
        "commit-b",
        "patch-b",
        "snapshot-b",
        "name",
    ));
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
    let original_runtime =
        build_runtime(source.clone(), original_sink.clone(), vec![registration()]);

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
        .expect("shared-truth-view workload should plan before rejection certification");

    let error = crate::delivery::validate_bulk_delivery_mode(
        rejected_plan.execution_plan(),
        BridgePreparationMode::ParallelPreparation,
    )
    .expect_err("bulk delivery should reject a plan that upgrades a rejected class");

    assert_eq!(error.kind(), BridgeDeliveryErrorKind::BulkDeliveryRejected);
    assert!(error.to_string().contains("selected mode"));
}
