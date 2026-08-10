use super::*;

#[test]
fn resource_replay_availability_strict_replay_policy_denies_pruned_budget_history_before_cold_work()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let payload_digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    for request_id in [ResourceRequestId::new(910), ResourceRequestId::new(911)] {
        runtime
            .admit_resource_completion(RawCompletionEnvelope::new(
                request_id,
                ResourceGeneration::new(1),
                ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
                ResourceAttemptId::ZERO,
                payload_digest.clone(),
                32,
            ))
            .denied_completion()
            .expect("unknown request should retain denied completion evidence");
    }

    runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &deny_on_unknown_or_missing_replay_resource_declaration(node),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert_eq!(
        report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(report.denied_completion_unavailable_count(), 1);
    assert_eq!(report.retry_lineage_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_budget_history_unavailable_count,
        1
    );
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
}

#[test]
fn resource_replay_availability_strict_replay_policy_denies_pruned_lifecycle_history_before_cold_work(
) {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(deny_on_unknown_or_missing_replay_resource_declaration(
            first_node,
        ))
        .expect("first declaration should lower");
    runtime
        .declare_resource_node(deny_on_unknown_or_missing_replay_resource_declaration(
            second_node,
        ))
        .expect("second declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_node,
        )))
        .expect("first request should admit")
        .admitted_request();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_node,
        )))
        .expect("second request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(first.handle(), ResourceCancellationReason::HostRequested)
        .expect("first cancellation should admit");
    runtime
        .cancel_resource_request(second.handle(), ResourceCancellationReason::HostRequested)
        .expect("second cancellation should admit");

    runtime.compact_resource_lifecycle_history_with_budget(
        2,
        ResourceRetentionCompactionBudget::unbounded().with_retained_lifecycle_history_limit(1),
    );

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &deny_on_unknown_or_missing_replay_resource_declaration(first_node),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("strict replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert_eq!(
        report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert!(
        report.retained_history_unavailable_count() > 0,
        "strict replay denial should be driven by typed unavailable lifecycle history"
    );
    assert_eq!(report.denied_completion_unavailable_count(), 0);
    assert_eq!(report.retry_lineage_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_budget_history_unavailable_count,
        1
    );
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
}

#[test]
fn resource_replay_availability_strict_replay_policy_denies_pruned_retry_lineage_before_cold_work()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_timeout_resource_declaration(node, 3, 7)
        .with_replay_policy(ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing);
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");

    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let first_timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("initial timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach first timeout");
    let first_ready_timeout = runtime
        .promote_temporal_wake_ready(first_timeout_wake)
        .expect("initial timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), first_ready_timeout)
        .expect("initial timeout should admit");

    let first_schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry schedule should return report");
    let first_scheduled = first_schedule
        .scheduled_retry()
        .expect("first retry should schedule");
    let first_retry_ordinal = first_scheduled.retry_ordinal();
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
        .expect("clock should reach first retry backoff");
    let first_ready_retry = runtime
        .promote_temporal_wake_ready(first_scheduled.backoff_wake_id())
        .expect("first retry wake should become ready");
    let first_retry_report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), first_ready_retry)
        .expect("first scheduled retry should admit");
    let first_retry_request = first_retry_report
        .admitted_retry()
        .expect("first retry should produce admitted retry artifact")
        .admitted_request();

    let second_timeout_wake = runtime
        .in_flight_resource_request(first_retry_request.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("retried request should attach timeout wake");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach second timeout");
    let second_ready_timeout = runtime
        .promote_temporal_wake_ready(second_timeout_wake)
        .expect("second timeout wake should become ready");
    runtime
        .admit_resource_timeout(first_retry_request.handle(), second_ready_timeout)
        .expect("second timeout should admit");

    let second_schedule = runtime
        .schedule_resource_retry(first_retry_request.handle(), ResourceRetryReason::TimedOut)
        .expect("second retry schedule should return report");
    let second_scheduled = second_schedule
        .scheduled_retry()
        .expect("second retry should schedule");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(second_scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach second retry backoff");
    let second_ready_retry = runtime
        .promote_temporal_wake_ready(second_scheduled.backoff_wake_id())
        .expect("second retry wake should become ready");
    runtime
        .admit_scheduled_resource_retry(first_retry_request.handle(), second_ready_retry)
        .expect("second scheduled retry should admit");

    runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_retry_lineage_limit(1),
    );

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("strict replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert_eq!(
        report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(report.denied_completion_unavailable_count(), 0);
    assert_eq!(report.retry_lineage_unavailable_count(), 1);
    assert!(
        runtime
            .retained_retry_lineage_availability(first_retry_ordinal)
            .is_some(),
        "strict denial should come from typed unavailable retry-lineage evidence"
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_budget_history_unavailable_count,
        1
    );
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
}

#[test]
fn resource_replay_availability_default_lane_omits_pruned_budget_history_where_strict_lane_denies()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let payload_digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    for request_id in [ResourceRequestId::new(920), ResourceRequestId::new(921)] {
        runtime
            .admit_resource_completion(RawCompletionEnvelope::new(
                request_id,
                ResourceGeneration::new(1),
                ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
                ResourceAttemptId::ZERO,
                payload_digest.clone(),
                32,
            ))
            .denied_completion()
            .expect("unknown request should retain denied completion evidence");
    }

    runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );

    let default_report = runtime
        .resource_replay_availability(&resource_declaration(node))
        .expect("default replay availability should classify");
    let strict_report = runtime
        .resource_replay_availability(&deny_on_unknown_or_missing_replay_resource_declaration(
            node,
        ))
        .expect("strict replay availability should classify");

    assert_eq!(
        default_report.class(),
        ResourceReplayAvailabilityClass::Omitted
    );
    assert_eq!(default_report.denial_class(), None);
    assert_eq!(
        strict_report.class(),
        ResourceReplayAvailabilityClass::Denied
    );
    assert_eq!(
        strict_report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert_ne!(
        default_report.availability_digest(),
        strict_report.availability_digest(),
        "strict budget-history denial must not collapse into default omitted availability"
    );
}
