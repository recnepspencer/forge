use super::condition_host_resolver::TestConditionResolver;
use crate::facade::{
    mark_dirty, Aspect, DefaultComparatorResolver, DependencyEdge, EvaluationRequestMode, NodeId,
    NodeState, SignalGraph,
};
use crate::tests::support::{evaluate, evaluate_on_demand, evaluate_with_resolvers, version_ab};

#[test]
fn custom_condition_without_resolver_errors_deterministically() {
    let mut graph = SignalGraph::new();
    let node = graph.node().custom_condition("test").build();
    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));

    let err = evaluate(&mut graph, node, &mut compute).unwrap_err();
    assert!(format!("{err}").contains("Custom condition 'test' requires a condition resolver"));
}

#[test]
fn custom_condition_with_resolver_obeys_host_decision() {
    let mut graph = SignalGraph::new();
    let node = graph.node().custom_condition("test").build();
    let mut resolver = TestConditionResolver {
        custom_result: true,
        ..TestConditionResolver::default()
    };
    let mut comparator = DefaultComparatorResolver;
    let mut compute_calls = 0_u64;
    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        compute_calls += 1;
        Ok(version_ab(0, 1))
    };

    evaluate_with_resolvers(
        &mut graph,
        node,
        &mut compute,
        &mut comparator,
        &mut resolver,
        EvaluationRequestMode::Default,
    )
    .unwrap();

    assert_eq!(compute_calls, 1);
    assert_eq!(
        graph
            .observe()
            .metrics()
            .temporal
            .temporal_eligibility_lowering_count,
        0
    );
}

#[test]
fn pending_dependency_precedes_custom_condition_resolution() {
    let mut graph = SignalGraph::new();
    let source = graph
        .node()
        .on_demand()
        .produces_aspects(Aspect::new(0))
        .build();
    let consumer = graph.node().custom_condition("pending-custom").build();
    graph
        .set_dependencies(consumer, [DependencyEdge::new(source, Aspect::new(0))])
        .unwrap();
    let mut resolver = TestConditionResolver {
        custom_result: true,
        ..TestConditionResolver::default()
    };
    let mut comparator = DefaultComparatorResolver;
    let mut baseline = |_id, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate_on_demand(&mut graph, source, &mut baseline).unwrap();
    evaluate_with_resolvers(
        &mut graph,
        consumer,
        &mut baseline,
        &mut comparator,
        &mut resolver,
        EvaluationRequestMode::Default,
    )
    .unwrap();
    resolver.custom_calls = 0;
    mark_dirty(&mut graph, source, Aspect::new(0)).unwrap();

    let mut consumer_calls = 0;
    evaluate_with_resolvers(
        &mut graph,
        consumer,
        &mut |_id, _graph: &SignalGraph| {
            consumer_calls += 1;
            Ok(version_ab(2, 0))
        },
        &mut comparator,
        &mut resolver,
        EvaluationRequestMode::Default,
    )
    .unwrap();

    assert_eq!(resolver.custom_calls, 0);
    assert_eq!(consumer_calls, 0);
    assert_eq!(
        graph
            .pending_dependency_revalidation(consumer)
            .unwrap()
            .unwrap()
            .unresolved_producers(),
        &[source]
    );
}

#[test]
fn debounce_not_ready_defers_recompute() {
    let mut graph = SignalGraph::new();
    let node = graph.node().debounce(50).unwrap().build();
    let mut resolver = TestConditionResolver::default();
    let mut comparator = DefaultComparatorResolver;
    let mut compute_calls = 0_u64;
    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        compute_calls += 1;
        Ok(version_ab(0, 1))
    };

    evaluate_with_resolvers(
        &mut graph,
        node,
        &mut compute,
        &mut comparator,
        &mut resolver,
        EvaluationRequestMode::Default,
    )
    .unwrap();

    assert_eq!(compute_calls, 0);
    assert_eq!(graph.get_state(node).unwrap(), NodeState::MaybeStale);
    assert_eq!(graph.telemetry().evaluation.debounce_deferred_count, 1);
    assert_eq!(
        graph
            .observe()
            .metrics()
            .temporal
            .temporal_eligibility_lowering_count,
        1
    );
}

#[test]
fn debounce_ready_allows_recompute() {
    let mut graph = SignalGraph::new();
    let node = graph.node().debounce(50).unwrap().build();
    let mut resolver = TestConditionResolver {
        debounce_ready: true,
        ..TestConditionResolver::default()
    };
    let mut comparator = DefaultComparatorResolver;
    let mut compute_calls = 0_u64;
    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        compute_calls += 1;
        Ok(version_ab(0, 1))
    };

    evaluate_with_resolvers(
        &mut graph,
        node,
        &mut compute,
        &mut comparator,
        &mut resolver,
        EvaluationRequestMode::Default,
    )
    .unwrap();

    assert_eq!(compute_calls, 1);
    assert_eq!(graph.get_state(node).unwrap(), NodeState::Clean);
    assert_eq!(
        graph
            .observe()
            .metrics()
            .temporal
            .temporal_eligibility_lowering_count,
        1
    );
}
