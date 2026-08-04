use crate::facade::{
    mark_dirty, EvaluationCondition, LineageRecordKind, NodeEvaluationResult, NodeState,
    ReplayEventKind, SignalGraph, SignalRuntimePolicy, SnapshotRestoreLineageMode,
};
use crate::tests::support::{
    evaluate, evaluate_on_demand, version_ab, GraphDependencyBatchExt, ASPECT_A,
};

#[test]
fn threshold_flap_storm_with_on_demand_and_restore_keeps_replay_coherent() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(
        SignalRuntimePolicy::development()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::PerNode),
    );
    let source = graph.node().output_identity().build();
    let threshold = graph
        .node()
        .condition(EvaluationCondition::DeltaThreshold(2.0))
        .output_identity()
        .build();
    let deferred = graph.node().on_demand().output_identity().build();
    graph
        .append_dependency(threshold, source, ASPECT_A)
        .unwrap();
    graph
        .append_dependency(deferred, threshold, ASPECT_A)
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)).with_output_identity("base"))
    })
    .unwrap();
    evaluate(&mut graph, threshold, &mut |_id, graph| {
        let version = graph.get_entry(source).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version).with_output_identity("threshold-base"))
    })
    .unwrap();
    evaluate_on_demand(&mut graph, deferred, &mut |_id, graph| {
        let version = graph.get_entry(threshold).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version).with_output_identity("deferred-base"))
    })
    .unwrap();
    let snapshot = graph.capture_snapshot();

    for version in [11_u64, 12, 11, 13, 12, 14, 13, 15] {
        mark_dirty(&mut graph, source, ASPECT_A).unwrap();
        evaluate(&mut graph, source, &mut |_id, _graph| {
            Ok(NodeEvaluationResult::from_version(version_ab(version, 0))
                .with_output_identity(format!("source-{version}")))
        })
        .unwrap();
        evaluate(&mut graph, threshold, &mut |_id, graph| {
            let current = graph.get_entry(source).unwrap().get_aspect_version();
            Ok(NodeEvaluationResult::from_version(current)
                .with_output_identity(format!("threshold-{version}")))
        })
        .unwrap();
        if version % 2 == 0 {
            evaluate_on_demand(&mut graph, deferred, &mut |_id, graph| {
                let current = graph.get_entry(threshold).unwrap().get_aspect_version();
                Ok(NodeEvaluationResult::from_version(current)
                    .with_output_identity(format!("deferred-{version}")))
            })
            .unwrap();
        }
    }

    graph.restore_snapshot(&snapshot).unwrap();
    assert!(
        graph.replay_events().iter().any(|frame| {
            frame.kind == ReplayEventKind::SnapshotRestored
                && frame.snapshot_id == Some(snapshot.meta.snapshot_id)
        }),
        "restore should append a snapshot-restored replay event after threshold flap churn"
    );
    let explanation = graph.observe().explain(deferred).unwrap();
    assert_eq!(explanation.state, NodeState::Clean);
    assert!(
        graph
            .observe()
            .lineage_for_node(deferred)
            .iter()
            .any(|record| matches!(record.kind, LineageRecordKind::SnapshotRestore { .. })),
        "on-demand node should preserve restore lineage after threshold flap storm"
    );
}
