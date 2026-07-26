use super::*;

#[test]
fn resource_retry_attempt_limit_denies_before_temporal_wake_allocation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            retry_timeout_resource_declaration(node, 3, 7).with_retry_max_attempts(1),
        )
        .expect("attempt-limited retry declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");

    let report = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("attempt-limit denial should remain report-shaped");
    let denied = report
        .denied_retry()
        .expect("attempt-limited retry should deny before wake allocation");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryAttemptLimitReached
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_attempt_limit_denial_count,
        1
    );
}

#[test]
fn resource_retry_runtime_budget_exhaustion_denies_before_temporal_wake_allocation() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            first,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("first runtime-budget retry declaration should lower");
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            second,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("second runtime-budget retry declaration should lower");

    let first_report = schedule_timed_out_retry(&mut runtime, first);
    let first_scheduled = first_report
        .scheduled_retry()
        .expect("first runtime-budget retry should schedule");
    assert_eq!(
        first_scheduled.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Runtime)
    );
    assert_eq!(first_scheduled.retry_budget_limit(), Some(1));
    assert_eq!(first_scheduled.retry_budget_usage(), Some(1));
    assert_eq!(
        first_report.performance().retry_budget_scope_touch_count(),
        1
    );

    let second_report = schedule_timed_out_retry(&mut runtime, second);
    let denied = second_report
        .denied_retry()
        .expect("second runtime-budget retry should deny");
    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryBudgetExhausted
    );
    assert_eq!(
        denied.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Runtime)
    );
    assert_eq!(denied.retry_budget_limit(), Some(1));
    assert_eq!(denied.retry_budget_usage(), Some(1));
    assert_eq!(second_report.performance().temporal_wake_footprint(), 0);
    assert_eq!(
        second_report.performance().retry_budget_scope_touch_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_budget_exhaustion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_budget_scope_touch_count,
        2
    );
}

#[test]
fn resource_retry_node_budget_scope_accumulates_across_requests_for_same_node() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            node,
            3,
            7,
            ResourceRetryBudgetScope::ResourceNode,
            1,
        ))
        .expect("node-budget retry declaration should lower");

    let first_report = schedule_timed_out_retry(&mut runtime, node);
    assert!(first_report.scheduled_retry().is_some());

    let second_report = schedule_timed_out_retry(&mut runtime, node);
    let denied = second_report
        .denied_retry()
        .expect("second node-budget retry should deny");
    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryBudgetExhausted
    );
    assert_eq!(
        denied.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::ResourceNode)
    );
    assert_eq!(denied.retry_budget_limit(), Some(1));
    assert_eq!(denied.retry_budget_usage(), Some(1));
    assert_eq!(second_report.performance().temporal_wake_footprint(), 0);
}

#[test]
fn resource_retry_request_budget_scope_is_isolated_across_distinct_lineages() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            first,
            3,
            7,
            ResourceRetryBudgetScope::Request,
            1,
        ))
        .expect("first request-budget retry declaration should lower");
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            second,
            3,
            7,
            ResourceRetryBudgetScope::Request,
            1,
        ))
        .expect("second request-budget retry declaration should lower");

    let first_report = schedule_timed_out_retry(&mut runtime, first);
    let second_report = schedule_timed_out_retry(&mut runtime, second);

    assert_eq!(
        first_report
            .scheduled_retry()
            .expect("first request-budget retry should schedule")
            .retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Request)
    );
    assert_eq!(
        second_report
            .scheduled_retry()
            .expect("second request-budget retry should schedule")
            .retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Request)
    );
}

#[test]
fn resource_retry_request_budget_scope_accumulates_across_retry_lineage() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            node,
            3,
            7,
            ResourceRetryBudgetScope::Request,
            1,
        ))
        .expect("request-budget retry declaration should lower");

    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach first timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("first timeout admission should succeed");

    let first_schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry schedule should stay report-shaped");
    let first_scheduled = first_schedule
        .scheduled_retry()
        .expect("first request-budget retry should schedule");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(first_scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(first_scheduled.backoff_wake_id())
        .expect("retry backoff wake should become ready");
    let retry_report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("scheduled retry should admit");
    let retry = retry_report
        .admitted_retry()
        .expect("scheduled retry should produce admitted artifact");
    let retried_handle = retry.admitted_request().handle();
    let retry_timeout_wake = runtime
        .in_flight_resource_request(retried_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("retried request should receive timeout wake");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach second timeout");
    let retry_ready_timeout = runtime
        .promote_temporal_wake_ready(retry_timeout_wake)
        .expect("retried timeout wake should become ready");
    runtime
        .admit_resource_timeout(retried_handle, retry_ready_timeout)
        .expect("retried timeout admission should succeed");

    let second_schedule = runtime
        .schedule_resource_retry(retried_handle, ResourceRetryReason::TimedOut)
        .expect("second retry schedule should stay report-shaped");
    let denied = second_schedule
        .denied_retry()
        .expect("request-budget retry should deny once the lineage budget is spent");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryBudgetExhausted
    );
    assert_eq!(
        denied.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Request)
    );
    assert_eq!(denied.retry_budget_limit(), Some(1));
    assert_eq!(denied.retry_budget_usage(), Some(1));
    assert_eq!(second_schedule.performance().temporal_wake_footprint(), 0);
}
