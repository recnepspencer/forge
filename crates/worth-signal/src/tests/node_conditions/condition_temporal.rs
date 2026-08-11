use super::condition_host_resolver::TestConditionResolver;
use crate::facade::{
    DefaultComparatorResolver, EvaluationCondition, EvaluationRequestMode, IntervalAnchor,
    IntervalCondition, MissedTickPolicy, NodeId, NodeState, SignalGraph, TemporalCondition,
};
use crate::tests::support::{evaluate, evaluate_with_resolvers, version_ab};

#[test]
fn after_without_resolver_errors_deterministically() {
    let mut graph = SignalGraph::new();
    let node = graph.node().after(50).unwrap().build();
    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));

    let err = evaluate(&mut graph, node, &mut compute).unwrap_err();
    assert!(format!("{err}").contains("After(50ms) requires a temporal condition resolver"));
}

#[test]
fn after_with_resolver_obeys_host_decision() {
    let mut graph = SignalGraph::new();
    let node = graph.node().after(50).unwrap().build();
    let mut resolver = TestConditionResolver {
        temporal_ready: true,
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

#[test]
fn interval_without_resolver_errors_deterministically() {
    let mut graph = SignalGraph::new();
    let node = graph.node().interval(250).unwrap().build();
    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));

    let err = evaluate(&mut graph, node, &mut compute).unwrap_err();
    assert!(format!("{err}").contains("Interval(250ms) requires a temporal condition resolver"));
}

#[test]
fn interval_condition_builder_preserves_anchor_and_missed_tick_policy() {
    let mut graph = SignalGraph::new();
    let interval = IntervalCondition::try_new(250)
        .unwrap()
        .with_anchor(IntervalAnchor::FirstEvaluation)
        .with_missed_tick_policy(MissedTickPolicy::CatchUpAll);
    let node = graph.node().interval_with(interval.clone()).build();

    let entry = graph.get_entry(node).unwrap();
    let condition = &entry.get_eval_config().condition;
    assert_eq!(
        condition,
        &EvaluationCondition::Temporal(TemporalCondition::Interval(interval))
    );
}

#[test]
fn throttle_and_stale_after_helpers_store_temporal_conditions() {
    let mut graph = SignalGraph::new();
    let throttle = graph.node().throttle(75).unwrap().build();
    let stale_after = graph.node().stale_after(500).unwrap().build();

    assert_eq!(
        graph
            .get_entry(throttle)
            .unwrap()
            .get_eval_config()
            .condition,
        EvaluationCondition::Temporal(TemporalCondition::throttle(75).unwrap())
    );
    assert_eq!(
        graph
            .get_entry(stale_after)
            .unwrap()
            .get_eval_config()
            .condition,
        EvaluationCondition::Temporal(TemporalCondition::stale_after(500).unwrap())
    );
}
