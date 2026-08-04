use crate::facade::{
    mark_dirty, ChangedRegion, NodeEvaluationResult, NodeId, NodeState, OutputChange, SignalGraph,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A, ASPECT_B};

#[test]
fn output_identity_unchanged_suppresses_downstream_propagation() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("artifact"))
    };
    let mut source_v2_same_identity = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("artifact"))
    };
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Dirty);

    evaluate(&mut graph, source, &mut source_v2_same_identity).unwrap();

    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
    let explanation = graph.observe().explain(source).unwrap();
    assert_eq!(explanation.output_change, Some(OutputChange::Unchanged));
    assert!(explanation.propagation_suppressed);
    assert_eq!(
        graph
            .observe()
            .metrics()
            .evaluation
            .suppressed_downstream_propagations,
        1
    );
}

#[test]
fn output_identity_suppression_does_not_hide_other_real_upstream_changes() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().output_identity().build();
    let source_b = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source_a, ASPECT_A)
        .unwrap();
    graph
        .append_dependency(dependent, source_b, ASPECT_B)
        .unwrap();

    let mut source_a_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("artifact-a"))
    };
    let mut source_a_v2_same_identity = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("artifact-a"))
    };
    let mut source_b_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut source_b_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 2));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 10)))
    };

    evaluate(&mut graph, source_a, &mut source_a_v1).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source_a, ASPECT_A).unwrap();
    mark_dirty(&mut graph, source_b, ASPECT_B).unwrap();
    evaluate(&mut graph, source_a, &mut source_a_v2_same_identity).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v2).unwrap();

    assert_ne!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn continuity_token_match_does_not_hide_real_output_identity_change() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_output_identity("artifact-a")
            .with_continuity_token("stable-lineage"))
    })
    .unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_output_identity("artifact-b")
            .with_continuity_token("stable-lineage"))
    })
    .unwrap();

    let explanation = graph.observe().explain(source).unwrap();
    assert_eq!(
        explanation.output_change,
        Some(OutputChange::Replaced),
        "a continuity-token match must not erase a real output identity change"
    );
}

#[test]
fn changed_regions_flow_into_trace_and_explanation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().partitioned_output().build();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing-panel").with_detail("rib-12")))
    };

    evaluate(&mut graph, node, &mut compute).unwrap();

    let explanation = graph.observe().explain(node).unwrap();
    assert_eq!(explanation.changed_regions.len(), 1);
    assert_eq!(
        explanation
            .historical_artifact_record
            .as_ref()
            .map(|record| record.runtime.changed_partition_count())
            .unwrap(),
        1
    );
    assert_eq!(
        graph
            .observe()
            .metrics()
            .invalidation
            .partition_aware_recomputations,
        1
    );
}
