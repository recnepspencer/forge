use crate::facade::*;
use crate::tests::support::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Cache,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    Slow,
}

fn build_runtime(graph: SignalGraph) -> SignalRuntime<Domain, Impact, Ev, (), Tier> {
    SignalRuntime::builder(graph)
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build()
}

#[test]
fn explain_reports_changed_upstream() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();

    let explanation = graph.explain(dependent).unwrap();
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
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    let explanation = graph.explain(dependent).unwrap();
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
    graph.add_dependency(middle, source, ASPECT_A).unwrap();
    graph.add_dependency(dependent, middle, ASPECT_A).unwrap();

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

    let explanation = runtime.explain(dependent).unwrap();
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

#[test]
fn explain_reports_condition_deferred_for_on_demand_nodes() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().on_demand().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate_on_demand(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    let explanation = graph.explain(dependent).unwrap();
    assert!(explanation
        .upstream
        .iter()
        .any(|cause| matches!(cause, UpstreamCause::ConditionDeferred { source: deferred, aspect, condition: EvaluationCondition::OnDemand, decision: ConditionDecision::Deferred, .. } if *deferred == source && *aspect == ASPECT_A)));
}

#[test]
fn explain_reports_missing_snapshot_and_dependency_removed() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut source_compute).unwrap();

    graph.add_dependency(dependent, source, ASPECT_A).unwrap();
    let missing_snapshot = graph.explain(dependent).unwrap();
    assert!(missing_snapshot
        .upstream
        .iter()
        .any(|cause| matches!(cause, UpstreamCause::MissingSnapshot { source: missing, aspect, current_version: Some(1), .. } if *missing == source && *aspect == ASPECT_A)));

    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    graph
        .remove_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let removed = graph.explain(dependent).unwrap();
    assert!(removed
        .upstream
        .iter()
        .any(|cause| matches!(cause, UpstreamCause::DependencyRemoved { source: removed_source, aspect, cached_version: 1, .. } if *removed_source == source && *aspect == ASPECT_A)));
}

#[test]
fn explanation_surfaces_causality_and_trace_summary() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph
        .set_causality(
            node,
            Some(CausalityMetadata {
                kind: "bridge".to_string(),
                fields: [("commit".to_string(), "42".to_string())]
                    .into_iter()
                    .collect(),
            }),
        )
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut compute).unwrap();

    let explanation = graph.explain(node).unwrap();
    assert_eq!(explanation.causality.as_ref().unwrap().kind, "bridge");
    assert!(explanation.trace_summary.is_some());
    assert!(format!("{explanation}").contains("Causality: bridge"));
}

#[test]
fn dependency_inspection_apis_are_deterministic() {
    let mut graph = SignalGraph::new();
    let root = graph.node().build();
    let middle = graph.node().build();
    let target = graph.node().build();
    graph.add_dependency(middle, root, ASPECT_A).unwrap();
    graph.add_dependency(target, middle, ASPECT_B).unwrap();

    assert_eq!(graph.dependencies_of(target).unwrap().len(), 1);
    assert_eq!(graph.subscribers_of(root).unwrap(), &[middle]);
    assert!(graph.depends_on(target, middle, ASPECT_B).unwrap());
    assert_eq!(
        graph.dependency_chain_to(root, target).unwrap(),
        Some(vec![root, middle, target])
    );
}

#[test]
fn dot_export_contains_state_color_and_edge_labels() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().on_demand().build();
    graph
        .add_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();

    let dot = graph.to_dot();
    assert!(dot.contains(&format!("\"{}\"", source)));
    assert!(dot.contains("fillcolor=green"));
    assert!(dot.contains("aspect:0"));
    assert!(dot.contains("scope:"));
}

#[test]
fn metrics_snapshots_reflect_runtime_activity() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();
    let mut runtime = build_runtime(graph);

    let outcome = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            transaction.emit_event(Ev::Tick);
            transaction.flush_events(CheckpointBarrier::PerOperation)?;
            Ok(())
        })
        .unwrap();

    assert_eq!(outcome, TransactionOutcome::Committed);
    assert!(runtime.metrics().transaction_begin_count >= 1);
    assert!(runtime.metrics().transaction_commit_count >= 1);
    assert!(runtime.metrics().event_flushes >= 1);
    assert!(runtime.graph().metrics().invalidation_nodes_visited >= 1);
}

#[test]
fn explanation_is_deterministic_with_multiple_upstreams_and_mixed_states() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let source_c = graph.node().build();
    let dependent = graph.node().on_demand().build();
    graph.add_dependency(dependent, source_b, ASPECT_B).unwrap();
    graph.add_dependency(dependent, source_a, ASPECT_A).unwrap();
    graph.add_dependency(dependent, source_c, ASPECT_A).unwrap();

    let mut source_a_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_a_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut source_b_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut source_c_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(3, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source_a, &mut source_a_v1).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v1).unwrap();
    evaluate(&mut graph, source_c, &mut source_c_v1).unwrap();
    evaluate_on_demand(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source_a, ASPECT_A).unwrap();
    evaluate(&mut graph, source_a, &mut source_a_v2).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    let explanation = graph.explain(dependent).unwrap();
    let rendered = format!("{explanation}");
    assert!(matches!(
        explanation.upstream.first(),
        Some(UpstreamCause::ConditionDeferred { source, .. }) if *source == source_a
    ));
    assert!(explanation.upstream.iter().any(|cause| matches!(
        cause,
        UpstreamCause::Clean { source, aspect, cached_version: 1, current_version: 1, .. }
        if *source == source_b && *aspect == ASPECT_B
    )));
    assert!(rendered.contains("condition OnDemand/Deferred"));
}

#[test]
fn rollback_preserves_committed_explanation_and_increments_rollback_metric() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();
    let mut runtime = build_runtime(graph);

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut dependent_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(runtime.graph_mut(), source, &mut source_v1).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut dependent_v1).unwrap();
    let before = runtime.explain(dependent).unwrap();
    let rollback_before = runtime.metrics().transaction_rollback_count;

    let err = runtime.transaction(&mut (), |tx| {
        tx.mark_dirty(source, ASPECT_A)?;
        tx.evaluate_with_plan(
            dependent,
            &|_id, view| Ok(view.finish(version_ab(99, 0))),
            EvaluationRequestMode::Default,
        )?;
        Err(SignalError::invalid_input("rollback for test"))
    });
    assert!(err.is_err());

    let after = runtime.explain(dependent).unwrap();
    assert_eq!(before.trace_summary, after.trace_summary);
    assert_eq!(before.upstream, after.upstream);
    assert_eq!(
        runtime.metrics().transaction_rollback_count,
        rollback_before + 1
    );
}
