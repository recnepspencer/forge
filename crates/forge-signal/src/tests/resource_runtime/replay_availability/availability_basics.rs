use super::*;

#[test]
fn resource_replay_availability_retained_when_restore_is_compatible_and_history_is_present() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let report = runtime
        .resource_replay_availability(&resource_declaration(node))
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Retained);
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_retained_count,
        1
    );
}
#[test]
fn resource_replay_availability_omits_cold_reconstruction_when_history_is_unavailable_and_no_budget_is_requested(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = terminal_summaries_only_resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(3).unwrap(),
        },
    );
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    let compaction = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(compaction.retained_history_unavailable_count(), 1);

    let report = runtime
        .resource_replay_availability(&declaration)
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Omitted);
    assert!(report.restore_compatibility().is_some());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_omitted_count,
        1
    );
}

#[test]
fn resource_replay_availability_reports_unavailable_when_cold_reconstruction_is_policy_denied() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = terminal_summaries_only_resource_declaration(node)
        .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(3).unwrap(),
        });
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    runtime.compact_resource_lifecycle_history(1);

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Unavailable);
    assert!(report.restore_compatibility().is_some());
    assert!(report.diagnostics_summary().is_none());
    assert_eq!(
        report
            .diagnostics_denial()
            .expect("diagnostics denial should be present")
            .class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyRetainedOnly
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_unavailable_count,
        1
    );
}

#[test]
fn resource_replay_availability_reconstructs_when_history_is_unavailable_and_budget_admits_cold_work(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = terminal_summaries_only_resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(3).unwrap(),
        },
    );
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    runtime.compact_resource_lifecycle_history(1);

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(
        report.class(),
        ResourceReplayAvailabilityClass::Reconstructed
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.diagnostics_summary().is_some());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_reconstructed_count,
        1
    );
}

#[test]
fn resource_replay_availability_denied_by_restore_compatibility_does_not_attempt_cold_reconstruction(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &timeout_resource_declaration(node, 9),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert!(report.restore_compatibility().is_none());
    assert!(report.restore_compatibility_denial().is_some());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_denied_count,
        1
    );
}
