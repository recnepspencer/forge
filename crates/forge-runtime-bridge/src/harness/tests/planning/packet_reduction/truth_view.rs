#[test]
fn bridge_bulk_packet_set_tracks_truth_view_materialization_packets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("bulk workload should plan");

    assert_eq!(planned.packet_set().truth_view_packets().len(), 2);
    assert_eq!(
        planned
            .packet_set()
            .truth_view_packets()
            .iter()
            .map(|packet| packet.source_snapshot())
            .collect::<Vec<_>>(),
        vec!["snapshot-a", "snapshot-a"]
    );
    assert_eq!(
        planned
            .packet_set()
            .truth_view_packets()
            .iter()
            .map(|packet| packet.snapshot_read_count())
            .sum::<usize>(),
        2
    );
    assert_eq!(planned.packet_set().counters().bulk_packet_count(), 5);
}

#[test]
fn bridge_bulk_reduction_artifact_carries_truth_view_materializations() {
    let left_source = InMemoryRelationalBridgeSource::default();
    left_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    left_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let left_runtime = build_runtime(
        left_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let right_source = InMemoryRelationalBridgeSource::default();
    right_source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    right_source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let right_runtime = build_runtime(
        right_source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let left = left_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("left bulk workload should plan");
    let right = right_runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("right bulk workload should plan");

    assert_eq!(
        left.execution_plan()
            .reduced_artifact()
            .reduced_truth_views()
            .len(),
        1
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().reduced_truth_views(),
        right.execution_plan().reduced_artifact().reduced_truth_views()
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().reduced_truth_views()[0].source_snapshot(),
        "snapshot-a"
    );
    assert_eq!(
        left.execution_plan().reduced_artifact().reduced_truth_views()[0].snapshot_read_count(),
        1
    );
}

use super::*;
