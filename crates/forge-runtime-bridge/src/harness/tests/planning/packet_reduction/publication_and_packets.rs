#[test]
fn bridge_bulk_reduction_collapses_duplicate_publications_deterministically() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("duplicate bulk workload should plan");
    let reduced_artifact = planned.execution_plan().reduced_artifact();

    assert_eq!(reduced_artifact.reduction_input_count(), 2);
    assert_eq!(reduced_artifact.reduction_output_count(), 2);
    assert_eq!(reduced_artifact.counters().bulk_reduction_input_count(), 2);
    assert_eq!(reduced_artifact.reduced_truth_views().len(), 1);
    assert_eq!(reduced_artifact.reduced_publications().len(), 1);
    assert_eq!(
        reduced_artifact.reduced_publications()[0]
            .reduced_route_identities()
            .len(),
        2
    );
    assert_eq!(
        reduced_artifact.reduced_publications()[0].invalidation_target_count(),
        2
    );
}

#[test]
fn bridge_bulk_reduction_artifact_is_stable_across_input_order() {
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
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

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
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("left workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("right workload should plan");

    assert_eq!(
        left.execution_plan().reduced_artifact().digest(),
        right.execution_plan().reduced_artifact().digest()
    );
    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_publications()[0]
            .publication_identity(),
        right
            .execution_plan()
            .reduced_artifact()
            .reduced_publications()[0]
            .publication_identity()
    );
    assert_eq!(left.packet_set().digest(), right.packet_set().digest());
}

#[test]
fn bridge_bulk_packet_set_is_stable_across_input_order() {
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
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

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
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("left workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("right workload should plan");

    assert_eq!(left.packet_set().digest(), right.packet_set().digest());
    assert_eq!(
        left.packet_set()
            .routing_packets()
            .iter()
            .map(|packet| packet.packet_identity())
            .collect::<Vec<_>>(),
        right
            .packet_set()
            .routing_packets()
            .iter()
            .map(|packet| packet.packet_identity())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        left.packet_set()
            .reduction_packets()
            .iter()
            .map(|packet| packet.reduced_target_identity())
            .collect::<Vec<_>>(),
        right
            .packet_set()
            .reduction_packets()
            .iter()
            .map(|packet| packet.reduced_target_identity())
            .collect::<Vec<_>>()
    );
}

#[test]
fn bridge_bulk_packet_reduction_collapses_duplicate_slice_targets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("duplicate workload should plan");

    assert_eq!(planned.packet_set().routing_packets().len(), 2);
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(planned.packet_set().reduction_packets().len(), 1);
    assert_eq!(
        planned.packet_set().reduction_packets()[0].reduction_family(),
        "publication"
    );
    assert_eq!(
        planned.packet_set().reduction_packets()[0].reduced_target_scope(),
        planned.packet_set().routing_packets()[0].subscription_slice_identity()
    );
    assert_eq!(planned.packet_set().counters().bulk_packet_count(), 4);
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget
    );
    assert_eq!(
        planned
            .execution_plan()
            .locality_footprint()
            .publication_scope_count(),
        1
    );
}

#[test]
fn bridge_bulk_packet_set_emits_fallback_packets_for_fallback_admitted_slices() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![surface_fallback_registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        ]))
        .expect("fallback workload should plan");

    assert_eq!(planned.packet_set().routing_packets().len(), 1);
    assert_eq!(planned.packet_set().fallback_packets().len(), 1);
    assert_eq!(
        planned.packet_set().fallback_packets()[0].fallback_class(),
        "surface"
    );
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(planned.packet_set().counters().bulk_packet_count(), 4);
    assert_eq!(
        planned
            .execution_plan()
            .reduced_artifact()
            .reduced_fallbacks()
            .len(),
        1
    );
    assert_eq!(
        planned
            .execution_plan()
            .reduced_artifact()
            .reduction_output_count(),
        3
    );
}

use super::*;
