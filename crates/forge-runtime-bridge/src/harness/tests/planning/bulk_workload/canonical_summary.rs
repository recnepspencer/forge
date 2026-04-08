#[test]
fn bridge_bulk_canonical_workload_request_carries_canonical_member_sets() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
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

    assert_eq!(planned.canonical_request().workload_identity(), planned.workload_identity());
    assert_eq!(planned.canonical_request().route_members().len(), 2);
    assert_eq!(planned.canonical_request().subscription_slice_members().len(), 2);
    assert_eq!(planned.canonical_request().truth_view_members().len(), 2);
    assert_eq!(planned.canonical_request().commit_members().len(), 2);
    assert_eq!(planned.canonical_request().snapshot_members().len(), 2);
    assert_eq!(planned.canonical_request().branch_members().len(), 1);
}

#[test]
fn bridge_bulk_normalized_summary_derives_shared_workload_facts_once() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
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

    assert_eq!(planned.normalized_summary().workload_identity(), planned.workload_identity());
    assert_eq!(planned.normalized_summary().route_count(), 2);
    assert_eq!(planned.normalized_summary().subscription_slice_count(), 2);
    assert_eq!(planned.normalized_summary().snapshot_read_count(), 2);
    assert_eq!(planned.normalized_summary().truth_view_member_count(), 2);
    assert_eq!(planned.normalized_summary().continuity_member_count(), 0);
    assert_eq!(planned.normalized_summary().branch_scope_count(), 1);
    assert_eq!(planned.normalized_summary().snapshot_scope_count(), 2);
}

#[test]
fn bridge_bulk_execution_plan_falls_back_to_serial_for_single_route_workload() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![BridgeBulkWorkloadSegment::new(
            BridgeRouteRequest::for_commit("commit-a"),
        )]))
        .expect("single-route bulk workload should plan");

    assert_eq!(
        planned.execution_plan().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::SerialRequired
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::BelowMinWorkloadWidth
    );
    assert_eq!(
        planned.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::SerialOnly
    );
    assert_eq!(
        planned.execution_plan().legality_decision().reason(),
        BridgeParallelLegalityReason::BelowMinWorkloadWidth
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::NotApplicable
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().reason(),
        BridgeParallelProfitabilityReason::SerialOnlyWorkload
    );
    assert_eq!(
        planned
            .execution_plan()
            .legality_proof()
            .admitted_partitions()
            .partitions()
            .len(),
        0
    );
    assert_eq!(
        planned.execution_plan().reduced_artifact().reduction_input_count(),
        2
    );
    assert_eq!(
        planned.execution_plan().reduced_artifact().reduction_output_count(),
        2
    );
    assert!(
        planned
            .execution_plan()
            .legality_proof()
            .disjoint_packet_regions()
            .regions()
            .is_empty()
    );
    assert_eq!(planned.packet_set().routing_packets().len(), 1);
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert_eq!(planned.packet_set().reduction_packets().len(), 1);
    assert_eq!(planned.execution_plan().counters().bulk_serial_required_count(), 1);
    assert_eq!(
        planned.execution_plan().counters().bulk_parallel_profitable_count(),
        0
    );
    assert!(planned.execution_plan().planning_failures().is_empty());
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::BelowMinWorkloadWidth
    );
}

use super::*;
