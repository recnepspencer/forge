use super::*;

#[test]
fn resource_revalidation_admits_new_generation_when_no_request_is_active() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("live declared resource should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("no-active revalidation should admit");
    let admitted = revalidation.admitted_request();

    assert_eq!(revalidation.expected_active(), None);
    assert_eq!(admitted.handle().generation(), ResourceGeneration::new(1));
    assert_eq!(admitted.attempt(), ResourceAttemptId::ZERO);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::RevalidationAdmission
    );
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("revalidation request should be retained")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_admission_count,
        1
    );
}

#[test]
fn resource_revalidation_coalesces_duplicate_explicit_refresh_while_pending() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");

    let first = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("first explicit revalidation should admit")
        .admitted_revalidation()
        .expect("first explicit revalidation should be admitted")
        .admitted_request();
    let first_timeout_wake = runtime
        .in_flight_resource_request(first.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("first revalidation should carry its timeout wake");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("duplicate explicit revalidation should coalesce");
    let revalidation = report
        .admitted_revalidation()
        .expect("duplicate explicit revalidation should still be admitted");
    let coalescing = revalidation
        .coalescing()
        .expect("coalesced revalidation should retain explicit winner/loser evidence");
    let loser = coalescing.coalesced_request();

    assert_eq!(revalidation.admitted_request(), first);
    assert_eq!(
        revalidation.freshness_decision().class(),
        ResourceRevalidationFreshnessClass::ExplicitIntent
    );
    assert_eq!(coalescing.winner(), first.handle());
    assert_ne!(loser.handle(), first.handle());
    assert_eq!(report.performance().coalescing_width(), 1);
    assert_eq!(report.performance().temporal_wake_footprint(), 1);
    let replacement_timeout_wake = runtime
        .in_flight_resource_request(first.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("coalesced winner should carry the replacement timeout wake");
    assert_ne!(replacement_timeout_wake, first_timeout_wake);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime
            .in_flight_resource_request(first.handle())
            .expect("winner should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(loser.handle())
            .expect("coalesced loser should be retained")
            .status(),
        ResourceInFlightStatus::Superseded
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_coalesced_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_temporal_wake_footprint,
        2
    );
}

#[test]
fn resource_revalidation_requires_expected_handle_when_active_request_exists() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("expected-handle denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("ambient active request should require explicit expected handle");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle
    );
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_active_requires_expected_denial_count,
        1
    );
}

#[test]
fn denied_resource_revalidation_preserves_the_active_timeout_wake() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let active = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("timed request should admit")
        .admitted_request()
        .handle();
    let timeout_wake = runtime
        .in_flight_resource_request(active)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("active timed request should own one timeout wake");

    let denied = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("revalidation denial should remain report-shaped");

    assert_eq!(
        denied
            .denied_revalidation()
            .expect("active request requires exact expected-handle authority")
            .class(),
        ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(active)
            .and_then(|in_flight| in_flight.timeout_wake_id()),
        Some(timeout_wake)
    );
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("authoritative clock should advance to the original deadline");
    runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("denial must leave the original timeout wake promotable");
}

#[test]
fn resource_revalidation_supersedes_only_the_expected_active_handle() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let first_wake = runtime
        .in_flight_resource_request(first)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::with_expected_active(
            ResourceNodeId::from_node(node),
            first,
        ))
        .expect("expected active request should revalidate");
    let revalidation = report
        .admitted_revalidation()
        .expect("expected active revalidation should admit");
    let admitted = revalidation.admitted_request();
    let supersession = revalidation
        .supersession_record()
        .expect("revalidation should retain explicit supersession lineage");

    assert_eq!(revalidation.expected_active(), Some(first));
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), admitted.handle());
    assert_eq!(admitted.handle().generation(), ResourceGeneration::new(2));
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert!(runtime.promote_temporal_wake_ready(first_wake).is_err());
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("prior request should be retained")
            .status(),
        ResourceInFlightStatus::Superseded
    );
}

#[test]
fn resource_revalidation_denies_stale_expected_handle_after_newer_generation() {
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
        .revalidate_resource_node(ResourceRevalidationIntent::with_expected_active(
            ResourceNodeId::from_node(node),
            first,
        ))
        .expect("stale expected denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("stale expected handle must not overwrite newer active request");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second)
            .expect("newer active request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_expected_mismatch_denial_count,
        1
    );
}
