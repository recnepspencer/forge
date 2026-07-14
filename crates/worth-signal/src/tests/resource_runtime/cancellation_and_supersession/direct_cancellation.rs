use super::*;

#[test]
fn resource_cancellation_marks_request_cancelled_and_removes_active_frontier() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted.handle();

    let report = runtime
        .cancel_resource_request(handle, ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire timeout side effects cleanly");

    let cancelled = report
        .cancelled_request()
        .expect("active pending request should cancel");
    let lifecycle = report
        .lifecycle()
        .expect("admitted cancellation should report lifecycle truth");
    let transition = report
        .transition()
        .expect("admitted cancellation should report transition truth");
    assert!(report.denied_cancellation().is_none());

    assert_eq!(cancelled.handle(), handle);
    assert_eq!(
        cancelled.reason(),
        ResourceCancellationReason::HostRequested
    );
    assert!(
        cancelled
            .policy_decision_digest()
            .as_str()
            .starts_with("resource-policy-cancellation-plan:"),
        "cancellation artifact should retain lowered cancellation decision proof"
    );
    assert!(
        cancelled.host_advisory().is_some(),
        "default best-effort cancellation policy should emit host advisory evidence"
    );
    assert_eq!(
        cancelled.cancellation_ordinal(),
        ResourceCancellationOrdinal::new(1)
    );
    assert_eq!(lifecycle.node(), ResourceNodeId::from_node(node));
    assert_eq!(lifecycle.lifecycle(), ResourceLifecycleClass::Cancelled);
    assert_eq!(
        transition.kind(),
        ResourceLifecycleTransitionKind::RequestCancelled
    );
    assert_eq!(transition.from(), ResourceLifecycleClass::Pending);
    assert_eq!(transition.to(), ResourceLifecycleClass::Cancelled);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::Cancellation
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 0);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("cancelled request remains retained for late completion denial");
    assert_eq!(in_flight.lifecycle(), ResourceLifecycleClass::Cancelled);
    assert_eq!(in_flight.status(), ResourceInFlightStatus::Cancelled);
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(runtime.telemetry().resource.resource_cancellation_count, 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_policy_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_runtime_hard_cancellation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_host_cancellation_advisory_count,
        1
    );
}

#[test]
fn resource_cancellation_denies_stale_handle_without_mutating_active_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first")
        .admitted_request()
        .handle();

    let report = runtime
        .cancel_resource_request(first, ResourceCancellationReason::HostRequested)
        .expect("denied cancellation should not trip temporal cleanup");

    let denied = report
        .denied_cancellation()
        .expect("stale superseded handle should be denied");
    assert_eq!(
        denied.class(),
        ResourceCancellationDenialClass::NonActiveRequest
    );
    assert!(report.cancelled_request().is_none());
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_non_active_cancellation_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second)
            .expect("current request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
}

#[test]
fn resource_runtime_denial_only_cancellation_omits_host_advisory_evidence() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(runtime_denial_only_cancellation_resource_declaration(node))
        .expect("runtime denial only declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let report = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("runtime denial only cancellation should admit");
    let cancelled = report
        .cancelled_request()
        .expect("active request should cancel");

    assert!(cancelled.host_advisory().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_policy_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_runtime_hard_cancellation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_host_cancellation_advisory_count,
        0
    );
}

#[test]
fn resource_cancellation_reports_declared_grace_window() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(graceful_cancellation_resource_declaration(node, 25))
        .expect("graceful cancellation declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let report = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("graceful cancellation should admit");
    let cancelled = report
        .cancelled_request()
        .expect("graceful cancellation should retain the cancelled request");

    assert_eq!(
        cancelled
            .grace_window()
            .expect("declared grace window should be retained")
            .duration(),
        TemporalDuration::temporal_duration(25).unwrap()
    );
    assert!(report.dependent_propagation().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_grace_period_count,
        1
    );
}
