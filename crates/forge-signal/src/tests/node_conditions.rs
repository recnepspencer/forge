use crate::facade::*;
use crate::tests::support::*;

#[derive(Default)]
struct TestConditionResolver {
    debounce_ready: bool,
    custom_result: bool,
}

impl ConditionResolver for TestConditionResolver {
    fn debounce_ready(
        &mut self,
        _quiet_period_ms: u64,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Ok(self.debounce_ready)
    }

    fn resolve_custom(
        &mut self,
        _key: &str,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Ok(self.custom_result)
    }
}

#[test]
fn node_entry_stores_evaluation_condition_config() {
    let mut graph = SignalGraph::new();
    let node = graph.create_node();

    let cfg = NodeEvaluationConfig {
        condition: EvaluationCondition::Debounce(2_000),
        ..NodeEvaluationConfig::default()
    };
    graph
        .get_entry_mut(node)
        .unwrap()
        .set_eval_config(cfg.clone());

    let stored = graph.get_entry(node).unwrap().get_eval_config().clone();
    assert_eq!(stored, cfg);
}

#[test]
fn create_node_with_config_sets_condition() {
    let mut graph = SignalGraph::new();
    let node = graph.node().on_demand().build();
    assert!(matches!(
        graph.get_entry(node).unwrap().get_eval_config().condition,
        EvaluationCondition::OnDemand
    ));
}

#[test]
fn ondemand_blocks_default_evaluate() {
    let mut graph = SignalGraph::new();
    let node = graph.node().on_demand().build();
    let mut compute_calls = 0_u64;
    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        compute_calls += 1;
        Ok(version_ab(0, 1))
    };

    evaluate(&mut graph, node, &mut compute).unwrap();

    assert_eq!(compute_calls, 0);
    assert_eq!(graph.get_state(node).unwrap(), NodeState::MaybeStale);
    assert_eq!(graph.telemetry().evaluation.ondemand_deferred_count, 1);
}

#[test]
fn ondemand_forced_request_recomputes() {
    let mut graph = SignalGraph::new();
    let node = graph.node().on_demand().build();
    let mut compute_calls = 0_u64;
    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        compute_calls += 1;
        Ok(version_ab(0, 1))
    };

    evaluate_on_demand(&mut graph, node, &mut compute).unwrap();

    assert_eq!(compute_calls, 1);
    assert_eq!(graph.get_state(node).unwrap(), NodeState::Clean);
}

#[test]
fn aspect_filter_skips_unmatched_dirty_aspect() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.node().aspect_filter(mask_a()).build();
    graph
        .append_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 10));
    let mut dependent_calls = 0_u64;
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        dependent_calls += 1;
        Ok(version_ab(0, 20))
    };

    evaluate(&mut graph, source, &mut source_compute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    let mut source_recompute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 11));
    evaluate(&mut graph, source, &mut source_recompute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    assert_eq!(dependent_calls, 1);
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::MaybeStale);
}

#[test]
fn aspect_filter_recomputes_on_matched_aspect() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.node().aspect_filter(mask_b()).build();
    graph
        .append_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 10));
    let mut dependent_calls = 0_u64;
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        dependent_calls += 1;
        Ok(version_ab(0, 20 + dependent_calls))
    };

    evaluate(&mut graph, source, &mut source_compute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    let mut source_recompute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 11));
    evaluate(&mut graph, source, &mut source_recompute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    assert_eq!(dependent_calls, 2);
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn invalidation_skips_direct_subscriber_when_contract_reads_do_not_care() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.node().reads_aspects(mask_a()).build();
    graph
        .append_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn invalidation_skips_direct_subscriber_when_contract_partition_scope_does_not_care() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph
        .node()
        .reads_aspects(mask_a())
        .with_partition_scope(PartitionSubscription::partition_and_detail(
            "wing", "rib-12",
        ))
        .build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion {
            partition: PartitionToken::new("tail"),
            detail: Some("rib-2".to_owned()),
        }],
    )
    .unwrap();

    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn invalidation_respects_mixed_aspect_and_partition_contracts() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph
        .node()
        .reads_aspects(mask_a())
        .with_contract(NodeContract::reads(mask_a()).with_partition_scopes([
            PartitionSubscription::partition_and_detail("wing", "rib-12"),
            PartitionSubscription::partition_and_detail("tail", "rib-2"),
        ]))
        .build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion {
            partition: PartitionToken::new("wing"),
            detail: Some("rib-12".to_owned()),
        }],
    )
    .unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Dirty);

    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion {
            partition: PartitionToken::new("wing"),
            detail: Some("rib-99".to_owned()),
        }],
    )
    .unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn delta_threshold_skips_small_delta() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.node().delta_threshold(2.0).build();
    graph
        .append_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut source_v10 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 10));
    let mut source_v12 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 12));
    let mut dependent_calls = 0_u64;
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        dependent_calls += 1;
        Ok(version_ab(0, 100))
    };

    evaluate(&mut graph, source, &mut source_v10).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_B).unwrap();
    evaluate(&mut graph, source, &mut source_v12).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    assert_eq!(dependent_calls, 1);
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn delta_threshold_recomputes_large_delta() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.node().delta_threshold(2.0).build();
    graph
        .append_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut source_v10 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 10));
    let mut source_v13 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 13));
    let mut dependent_calls = 0_u64;
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        dependent_calls += 1;
        Ok(version_ab(0, 100 + dependent_calls))
    };

    evaluate(&mut graph, source, &mut source_v10).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_B).unwrap();
    evaluate(&mut graph, source, &mut source_v13).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    assert_eq!(dependent_calls, 2);
}

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
}

#[test]
fn debounce_not_ready_defers_recompute() {
    let mut graph = SignalGraph::new();
    let node = graph.node().debounce(50).build();
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
}

#[test]
fn debounce_ready_allows_recompute() {
    let mut graph = SignalGraph::new();
    let node = graph.node().debounce(50).build();
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
}
