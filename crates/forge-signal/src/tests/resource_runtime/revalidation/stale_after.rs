use super::*;

#[test]
fn resource_stale_after_completion_schedules_ready_revalidation_wake_and_revalidates() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(node, 3))
        .expect("stale-after declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("staged completion should commit");

    let stale_after_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("fulfilled node should retain a stale-after wake");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach stale-after due tick");
    let ready_wake = runtime
        .promote_temporal_wake_ready(stale_after_wake)
        .expect("stale-after wake should promote when due");

    let report = runtime
        .admit_stale_after_resource_revalidation(ResourceNodeId::from_node(node), ready_wake)
        .expect("stale-after ready wake should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("stale-after ready wake should revalidate");

    assert_eq!(revalidation.expected_active(), None);
    assert_eq!(revalidation.forced_active_handle(), None);
    assert_eq!(
        revalidation
            .stale_after_ready_wake()
            .expect("admitted stale-after revalidation should retain ready wake")
            .id(),
        stale_after_wake
    );
    assert_eq!(
        revalidation.admitted_request().handle().generation(),
        ResourceGeneration::new(2)
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 0);
    assert_eq!(
        runtime.active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node)),
        None
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_after_revalidation_count,
        1
    );
}

#[test]
fn resource_stale_after_revalidation_denies_when_policy_does_not_allow_it() {
    let mut graph = SignalGraph::new();
    let resource_node = graph.node().build();
    let timeout_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(resource_node).with_stale_after_policy(
            ResourceStaleAfterPolicyDeclaration::RuntimeStaleAfter {
                stale_after: TemporalDuration::temporal_duration(3).unwrap(),
            },
        ))
        .expect("policy-disabled stale-after declaration should still lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(timeout_node, 1))
        .expect("timeout declaration should lower");
    let timeout_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timeout_node,
        )))
        .expect("timeout request should admit")
        .admitted_request()
        .handle();
    let timeout_wake = runtime
        .in_flight_resource_request(timeout_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should reach timeout due tick");
    let stray_ready = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote");

    let report = runtime
        .admit_stale_after_resource_revalidation(
            ResourceNodeId::from_node(resource_node),
            stray_ready,
        )
        .expect("policy-disabled stale-after should still return a report");
    let denied = report
        .denied_revalidation()
        .expect("policy-disabled stale-after revalidation must deny");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::StaleAfterRevalidationPolicyDisabled
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_stale_after_policy_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_policy_decision_count,
        1
    );
}

#[test]
fn resource_stale_after_revalidation_denies_before_fulfillment_even_with_ready_wake() {
    let mut graph = SignalGraph::new();
    let pending_node = graph.node().build();
    let timeout_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(
            pending_node,
            3,
        ))
        .expect("stale-after declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(timeout_node, 1))
        .expect("timeout declaration should lower");
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            pending_node,
        )))
        .expect("pending request should admit");
    let timeout_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timeout_node,
        )))
        .expect("timeout request should admit")
        .admitted_request()
        .handle();
    let timeout_wake = runtime
        .in_flight_resource_request(timeout_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should reach timeout due tick");
    let stray_ready = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote");

    let report = runtime
        .admit_stale_after_resource_revalidation(
            ResourceNodeId::from_node(pending_node),
            stray_ready,
        )
        .expect("fulfilled-only denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("pending node must not admit stale-after revalidation");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::StaleAfterRequiresFulfilledLifecycle
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_stale_after_fulfilled_only_denial_count,
        1
    );
}

#[test]
fn resource_new_request_retires_stale_after_wake_before_it_can_fire() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(node, 3))
        .expect("stale-after declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should commit");
    let stale_after_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("fulfilled node should retain a stale-after wake");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("fresh request should supersede stale-after wake");

    assert_eq!(
        runtime.active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node)),
        None
    );
    assert!(runtime
        .promote_temporal_wake_ready(stale_after_wake)
        .is_err());
}

#[test]
fn resource_stale_after_revalidation_survives_snapshot_restore_with_same_ready_wake_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(node, 3))
        .expect("stale-after declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should commit");
    let stale_after_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("fulfilled node should retain stale-after wake before restore");
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot mutation should change active state");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate stale-after state");

    let restored_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("restore should preserve stale-after wake evidence");
    assert_eq!(restored_wake, stale_after_wake);
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach restored stale-after due tick");
    let ready_wake = runtime
        .promote_temporal_wake_ready(restored_wake)
        .expect("restored stale-after wake should promote");

    let report = runtime
        .admit_stale_after_resource_revalidation(ResourceNodeId::from_node(node), ready_wake)
        .expect("restored stale-after ready wake should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("restored stale-after wake should still revalidate");

    assert_eq!(
        revalidation.admitted_request().handle().generation(),
        ResourceGeneration::new(2)
    );
    assert_eq!(
        runtime.active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node)),
        None
    );
}

#[test]
fn resource_retry_and_revalidation_decision_artifacts_remain_distinct() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            forced_revalidation_resource_declaration(node).with_retry_policy(
                ResourceRetryPolicyDeclaration::FixedDelay {
                    delay: TemporalDuration::temporal_duration(3).unwrap(),
                },
            ),
        )
        .expect("retry and revalidation declaration should lower");

    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should be visible");

    assert_ne!(
        descriptor.retry_decision_plan().decision_digest().as_str(),
        descriptor
            .revalidation_decision_plan()
            .decision_digest()
            .as_str()
    );
    assert_eq!(
        descriptor.revalidation_decision_plan().class(),
        ResourceRevalidationDecisionClass::ExplicitOrActiveHandleForced
    );
}
