use crate::facade::*;
use crate::tests::support::*;

#[derive(Default)]
struct TestConditionResolver {
    debounce_ready: bool,
    temporal_ready: bool,
    custom_result: bool,
}

impl TemporalConditionResolver for TestConditionResolver {
    fn resolve_temporal(
        &mut self,
        condition: &TemporalCondition,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Ok(match condition {
            TemporalCondition::Debounce(_) => self.debounce_ready,
            _ => self.temporal_ready,
        })
    }

    fn debounce_ready(
        &mut self,
        _quiet_period: TemporalDuration,
        _ctx: &ConditionEvaluationContext,
    ) -> Result<bool, SignalError> {
        Ok(self.debounce_ready)
    }
}

impl ConditionResolver for TestConditionResolver {
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
        condition: EvaluationCondition::Temporal(TemporalCondition::debounce(2_000).unwrap()),
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

#[test]
fn temporal_conditions_default_to_monotonic_execution_clock() {
    let after = TemporalCondition::after(25).unwrap();
    let debounce = TemporalCondition::debounce(50).unwrap();
    let interval = TemporalCondition::interval(IntervalCondition::try_new(100).unwrap());

    assert_eq!(after.clock_domain(), ClockDomain::MonotonicExecution);
    assert_eq!(debounce.clock_domain(), ClockDomain::MonotonicExecution);
    assert_eq!(interval.clock_domain(), ClockDomain::MonotonicExecution);
    match after {
        TemporalCondition::After(condition) => assert_eq!(condition.delay().get(), 25),
        other => panic!("expected After condition, got {other:?}"),
    }
    match debounce {
        TemporalCondition::Debounce(condition) => assert_eq!(condition.quiet_period().get(), 50),
        other => panic!("expected Debounce condition, got {other:?}"),
    }
    match interval {
        TemporalCondition::Interval(condition) => assert_eq!(condition.period().get(), 100),
        other => panic!("expected Interval condition, got {other:?}"),
    }
}

#[test]
fn temporal_condition_clock_domain_rejects_metadata_only_domains() {
    let err = AfterCondition::try_new(25)
        .unwrap()
        .with_clock_domain(ClockDomain::WallClock)
        .unwrap_err();
    assert!(format!("{err}").contains("metadata-only"));

    let err = IntervalCondition::try_new(100)
        .unwrap()
        .with_clock_domain(ClockDomain::Presentation)
        .unwrap_err();
    assert!(format!("{err}").contains("metadata-only"));
}

#[test]
fn zero_width_temporal_declarations_are_rejected() {
    let err = TemporalCondition::after(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = TemporalCondition::debounce(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = TemporalCondition::throttle(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = TemporalCondition::stale_after(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = IntervalCondition::try_new(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));
}

#[test]
fn at_or_after_condition_uses_clock_tick_semantics() {
    let condition = TemporalCondition::at_or_after(ClockTick::new(42));

    match condition {
        TemporalCondition::AtOrAfter(condition) => {
            assert_eq!(condition.tick(), ClockTick::new(42));
            assert_eq!(condition.clock_domain(), ClockDomain::MonotonicExecution);
        }
        other => panic!("expected AtOrAfter condition, got {other:?}"),
    }
}
