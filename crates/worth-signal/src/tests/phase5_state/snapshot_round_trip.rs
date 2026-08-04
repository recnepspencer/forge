use crate::facade::{
    mark_dirty, LineageRecordKind, NodeEvaluationResult, NodeId, ReplayEventKind, SignalGraph,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn graph_snapshot_round_trip_restores_versions_and_emits_restore_replay_and_lineage() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_v1 =
        |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_output_identity("artifact-v1"))
        };
    let mut source_v2 =
        |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
                .with_output_identity("artifact-v2"))
        };
    let mut dependent_compute = |_id: NodeId, graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    let snapshot = graph.capture_snapshot();
    let replay_len_before = graph.replay_events().len();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    assert_eq!(
        graph
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2
    );

    graph.restore_snapshot(&snapshot).unwrap();

    assert_eq!(
        graph
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1
    );
    assert!(
        graph.replay_events().len() > replay_len_before,
        "restore should append replay-visible state transitions"
    );
    assert!(
        graph.replay_events().iter().any(|event| {
            event.kind == ReplayEventKind::SnapshotRestored
                && event.snapshot_id == Some(snapshot.meta.snapshot_id)
        }),
        "restore should emit a snapshot-restored replay event"
    );
    assert!(
        graph.observe().lineage_records().iter().any(|record| {
            matches!(
                record.kind,
                LineageRecordKind::SnapshotRestore { snapshot_id, .. }
                    if snapshot_id == snapshot.meta.snapshot_id
            )
        }),
        "restore should emit lineage-visible restore records under the active runtime policy"
    );
    let around_restore = graph
        .observe()
        .replay_around_snapshot(snapshot.meta.snapshot_id);
    assert!(
        around_restore
            .iter()
            .any(|event| event.kind == ReplayEventKind::SnapshotRestored),
        "snapshot-centered replay inspection should include the restore event"
    );
}
