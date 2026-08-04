use crate::data::dependency::DependencySnapshot;
use crate::facade::{
    mark_dirty, NodeEvaluationResult, NodeState, SignalGraph, SnapshotArtifactRestoreMode,
    SnapshotDependencyRestoreMode, SnapshotRestoreCoarseReason, SnapshotRestoreIntent,
    SnapshotStateRestoreMode,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn restore_snapshot_rejects_seed_recomputation_intent_before_mutation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().output_identity().build();

    evaluate(&mut graph, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("seed-test"))
    })
    .unwrap();
    let snapshot = graph.capture_snapshot();

    mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    let err = graph
        .restore_snapshot_with_intent(
            &snapshot,
            SnapshotRestoreIntent {
                state: SnapshotStateRestoreMode::RewindActiveState,
                artifacts: SnapshotArtifactRestoreMode::RestoreCapturedRetention,
                dependency_state: SnapshotDependencyRestoreMode::SeedRecomputationFromSnapshot,
            },
        )
        .unwrap_err();

    assert!(
        err.to_string().contains("SeedRecomputationFromSnapshot"),
        "unsupported recomputation-seed restore intent should fail explicitly"
    );
    assert_eq!(
        graph.get_state(node).unwrap(),
        NodeState::Dirty,
        "unsupported restore intent must fail before mutating operational graph state"
    );
}

#[test]
fn snapshot_restore_plan_reports_shared_delta_and_coarse_requirements() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let target = graph.node().build();
    graph.append_dependency(target, source, ASPECT_A).unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(
            NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_output_identity("plan-source"),
        )
    })
    .unwrap();
    evaluate(&mut graph, target, &mut |_id, graph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    })
    .unwrap();

    let snapshot = graph.capture_snapshot();

    let mut updated_snapshot = DependencySnapshot::empty();
    updated_snapshot.record(source, ASPECT_A, 9, None);
    graph.set_dep_snapshot(target, updated_snapshot).unwrap();

    let plan = graph
        .plan_snapshot_restore(&snapshot, SnapshotRestoreIntent::restore_runtime_truth())
        .unwrap();

    assert_eq!(plan.shared_node_count(), 2);
    assert_eq!(plan.current_only_node_count(), 0);
    assert_eq!(plan.snapshot_only_node_count(), 0);
    assert_eq!(plan.dependency_snapshot_delta_node_count(), 1);
    assert_eq!(
        plan.checkpoint_restore_batch()
            .classified()
            .target_nodes()
            .as_slice()
            .len(),
        1
    );
    assert!(plan.coarse_replacement_required());
    assert!(plan
        .coarse_reasons()
        .contains(&SnapshotRestoreCoarseReason::EntryStateRewind));
    assert!(plan
        .coarse_reasons()
        .contains(&SnapshotRestoreCoarseReason::DiagnosticsHistoryRestore));
    assert!(
        !plan
            .coarse_reasons()
            .contains(&SnapshotRestoreCoarseReason::NodeSetDifference),
        "shared-node-only restore planning should not claim node-set mismatch when node sets still align"
    );
}
