use super::runtime_world::{build_runtime, Tier};
use crate::facade::{
    mark_dirty, DependencyMode, DirtyPropagation, EvaluationTrigger, NodeId, SignalGraph,
    TierPolicy, UpstreamCause, VersionComparatorPolicy,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn explain_reports_changed_upstream() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();

    let explanation = graph.observe().explain(dependent).unwrap();
    assert_eq!(explanation.node, dependent);
    assert!(matches!(
        explanation.upstream.as_slice(),
        [UpstreamCause::Changed { source: changed, aspect, cached_version: 1, current_version: 2, .. }]
        if *changed == source && *aspect == ASPECT_A
    ));
}

#[test]
fn explain_reports_clean_upstream_when_snapshot_matches() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    let explanation = graph.observe().explain(dependent).unwrap();
    assert!(matches!(
        explanation.upstream.as_slice(),
        [UpstreamCause::Clean { source: clean, aspect, cached_version: 1, current_version: 1, .. }]
        if *clean == source && *aspect == ASPECT_A
    ));
}

#[test]
fn explain_reports_skipped_by_comparator_via_runtime_policy() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let middle = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(middle, source, ASPECT_A).unwrap();
    graph
        .append_dependency(dependent, middle, ASPECT_A)
        .unwrap();

    let mut runtime = build_runtime(graph);
    runtime.set_node_tier(dependent, Tier::Slow);
    runtime.set_tier_policy(
        TierPolicy::new(
            Tier::Slow,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::LazyPull,
        )
        .with_default_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 }),
    );

    let mut source_v10 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    let mut source_v12 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(12, 0));
    let mut middle_v100 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(100, 0));
    let mut middle_v102 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(102, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1_000, 0));

    evaluate(runtime.graph_mut(), source, &mut source_v10).unwrap();
    evaluate(runtime.graph_mut(), middle, &mut middle_v100).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut dependent_compute).unwrap();
    mark_dirty(runtime.graph_mut(), source, ASPECT_A).unwrap();
    evaluate(runtime.graph_mut(), source, &mut source_v12).unwrap();
    evaluate(runtime.graph_mut(), middle, &mut middle_v102).unwrap();

    let explanation = runtime.observe().explain(dependent).unwrap();
    assert!(explanation.upstream.iter().any(|cause| matches!(
        cause,
        UpstreamCause::SkippedByComparator {
            source: skipped,
            aspect,
            cached_version: 100,
            current_version: 102,
            ..
        } if *skipped == middle && *aspect == ASPECT_A
    )));
}
