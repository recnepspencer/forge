use super::*;

#[test]
fn resource_retry_freeze_digest_tracks_max_attempts_and_deterministic_jitter() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let third = graph.node().build();
    let fourth = graph.node().build();
    let fifth = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(retry_guarded_timeout_resource_declaration(
            first, 3, 7, 3, 5,
        ))
        .expect("guarded retry declaration should lower");
    runtime
        .declare_resource_node(retry_guarded_timeout_resource_declaration(
            second, 3, 7, 4, 5,
        ))
        .expect("max-attempt drift declaration should lower");
    runtime
        .declare_resource_node(retry_guarded_timeout_resource_declaration(
            third, 3, 7, 3, 6,
        ))
        .expect("jitter drift declaration should lower");
    runtime
        .declare_resource_node(
            retry_guarded_timeout_resource_declaration(fourth, 3, 7, 3, 5)
                .with_retry_budget(ResourceRetryBudgetScope::Runtime, 2),
        )
        .expect("runtime budget declaration should lower");
    runtime
        .declare_resource_node(
            retry_guarded_timeout_resource_declaration(fifth, 3, 7, 3, 5)
                .with_retry_budget(ResourceRetryBudgetScope::ResourceNode, 2),
        )
        .expect("node budget declaration should lower");

    let first_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(first))
        .expect("first descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let second_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(second))
        .expect("second descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let third_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(third))
        .expect("third descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let fourth_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(fourth))
        .expect("fourth descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let fifth_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(fifth))
        .expect("fifth descriptor should exist")
        .resolved_policy_bundle()
        .clone();

    assert_ne!(
        first_bundle.retry().parameter_digest().as_str(),
        second_bundle.retry().parameter_digest().as_str()
    );
    assert_ne!(
        first_bundle.retry().parameter_digest().as_str(),
        third_bundle.retry().parameter_digest().as_str()
    );
    assert_ne!(
        first_bundle.retry().frozen_digest().as_str(),
        second_bundle.retry().frozen_digest().as_str()
    );
    assert_ne!(
        first_bundle.retry().frozen_digest().as_str(),
        third_bundle.retry().frozen_digest().as_str()
    );
    assert_ne!(
        fourth_bundle.retry().parameter_digest().as_str(),
        fifth_bundle.retry().parameter_digest().as_str()
    );
    assert_ne!(
        fourth_bundle.retry().frozen_digest().as_str(),
        fifth_bundle.retry().frozen_digest().as_str()
    );
}

#[test]
fn resource_retry_deterministic_jitter_is_stable_for_same_lineage_and_preserved_across_restore() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_guarded_timeout_resource_declaration(node, 3, 7, 4, 5);

    let mut first_runtime = TestRuntime::build(graph.clone());
    first_runtime
        .declare_resource_node(declaration.clone())
        .expect("first jitter declaration should lower");
    let first_schedule = schedule_timed_out_retry(&mut first_runtime, node);
    let first_scheduled = first_schedule
        .scheduled_retry()
        .expect("first jitter retry should schedule");

    let mut second_runtime = TestRuntime::build(graph);
    second_runtime
        .declare_resource_node(declaration)
        .expect("second jitter declaration should lower");
    let second_schedule = schedule_timed_out_retry(&mut second_runtime, node);
    let second_scheduled = second_schedule
        .scheduled_retry()
        .expect("second jitter retry should schedule");

    assert_eq!(
        first_scheduled.scheduled_delay().get(),
        second_scheduled.scheduled_delay().get()
    );
    assert_eq!(
        first_scheduled.policy_decision_digest().as_str(),
        second_scheduled.policy_decision_digest().as_str()
    );
    assert_eq!(first_scheduled.previous(), second_scheduled.previous());
    assert_eq!(
        first_runtime
            .telemetry()
            .resource
            .resource_retry_jitter_decision_count,
        1
    );
    assert_eq!(
        second_runtime
            .telemetry()
            .resource
            .resource_retry_jitter_decision_count,
        1
    );

    let snapshot = first_runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    first_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot mutation should succeed");
    first_runtime
        .restore_snapshot(&snapshot)
        .expect("restore should preserve pending retry schedule");
    first_runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3u64.saturating_add(first_scheduled.scheduled_delay().get())),
        ))
        .expect("clock should reach restored jitter backoff");

    let restored_schedule = first_runtime
        .promote_temporal_wake_ready(first_scheduled.backoff_wake_id())
        .expect("restored jitter wake should still become ready");
    assert_eq!(restored_schedule.id(), first_scheduled.backoff_wake_id());
    assert_eq!(
        first_runtime
            .telemetry()
            .resource
            .resource_retry_policy_decision_count,
        1
    );
}
