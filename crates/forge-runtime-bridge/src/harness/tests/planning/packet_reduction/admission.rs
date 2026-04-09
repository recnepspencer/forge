#[test]
fn bridge_bulk_execution_plan_rejects_parallel_preparation_for_shared_truth_view_targets() {
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
        .expect("shared truth-view workload should plan");

    assert_eq!(
        planned.execution_plan().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::ParallelPreparationRejected
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedTruthViewMaterializationTarget
    );
    assert_eq!(
        planned.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::ParallelPreparationIllegal
    );
    assert_eq!(
        planned.execution_plan().legality_decision().reason(),
        BridgeParallelLegalityReason::SharedTruthViewMaterializationTarget
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::NotApplicable
    );
    assert_eq!(
        planned
            .execution_plan()
            .counters()
            .bulk_parallel_preparation_rejected_count(),
        1
    );
    assert_eq!(
        planned
            .execution_plan()
            .counters()
            .bulk_parallel_fallback_to_serial_count(),
        0
    );
    assert_eq!(planned.execution_plan().planning_failures().len(), 1);
    assert_eq!(
        planned.execution_plan().planning_failures()[0].kind(),
        BridgeBulkPlanningFailureKind::InvalidParallelAdmissionBasis
    );
    assert_eq!(planned.packet_set().truth_view_packets().len(), 1);
    assert!(planned
        .execution_plan()
        .legality_proof()
        .admitted_partitions()
        .partitions()
        .is_empty());
    assert!(planned
        .execution_plan()
        .legality_proof()
        .disjoint_packet_regions()
        .regions()
        .is_empty());
}

#[test]
fn bridge_bulk_execution_plan_rejects_parallel_preparation_for_continuity_remap_workloads() {
    let source = InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let lineage_context = BridgeLineageContext::new(BridgeContinuityAuthorityBasis::new(
        crate::facade::TruthBranchIdentity::new("main"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let planned = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context.clone()),
                ),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b"))
                .with_mapping_context(
                    BridgeMappingContext::default().with_lineage_context(lineage_context),
                ),
        ]))
        .expect("continuity remap workload should plan");

    assert_eq!(
        planned.execution_plan().selected_mode(),
        BridgePreparationMode::Serial
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::ParallelPreparationRejected
    );
    assert_eq!(
        planned.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::ContinuityRemapRequiresSerialPreparation
    );
    assert_eq!(
        planned.execution_plan().legality_decision().class(),
        BridgeParallelLegalityClass::ParallelPreparationIllegal
    );
    assert_eq!(
        planned.execution_plan().legality_decision().reason(),
        BridgeParallelLegalityReason::ContinuityRemapRequiresSerialPreparation
    );
    assert_eq!(
        planned.execution_plan().profitability_decision().class(),
        BridgeParallelProfitabilityClass::NotApplicable
    );
    assert_eq!(
        planned
            .execution_plan()
            .counters()
            .bulk_parallel_preparation_rejected_count(),
        1
    );
    assert_eq!(planned.execution_plan().planning_failures().len(), 1);
    assert_eq!(
        planned.execution_plan().planning_failures()[0].kind(),
        BridgeBulkPlanningFailureKind::InvalidParallelAdmissionBasis
    );
    assert!(planned
        .execution_plan()
        .legality_proof()
        .admitted_partitions()
        .partitions()
        .is_empty());
    assert!(planned
        .execution_plan()
        .legality_proof()
        .disjoint_packet_regions()
        .regions()
        .is_empty());
}

use super::*;
