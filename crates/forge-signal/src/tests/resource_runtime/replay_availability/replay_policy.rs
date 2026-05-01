use super::*;

#[test]
fn resource_replay_availability_replay_policy_gate_denial_stays_zero_cold() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("historical declaration should lower");

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &identical_only_replay_resource_declaration(node)
                .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    let denial = report
        .restore_compatibility_denial()
        .expect("restore compatibility denial should be present");
    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
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
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_denied_count,
        1
    );
}

#[test]
fn resource_replay_availability_digest_includes_replay_decision_provenance() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();

    let diagnostics_only_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut diagnostics_only_runtime = TestRuntime::builder(graph.clone())
        .with_kernel_defaults()
        .resource_policy_registry(diagnostics_only_registry)
        .build();
    diagnostics_only_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");
    let diagnostics_only_report = diagnostics_only_runtime
        .resource_replay_availability(
            &diagnostics_only_replay_resource_declaration(node)
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("diagnostics-only replay availability should classify");

    let combined_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut combined_runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(combined_registry)
        .build();
    combined_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");
    let combined_report = combined_runtime
        .resource_replay_availability(
            &resource_declaration(node)
                .with_replay_policy(
                    ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange,
                )
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("combined replay availability should classify");

    assert_eq!(
        diagnostics_only_report.class(),
        ResourceReplayAvailabilityClass::Retained
    );
    assert_eq!(
        combined_report.class(),
        ResourceReplayAvailabilityClass::Retained
    );
    assert_ne!(
        diagnostics_only_report.availability_digest(),
        combined_report.availability_digest(),
        "availability digest must reflect replay-decision provenance"
    );
}

#[test]
fn resource_replay_availability_distinguishes_pair_replay_adapter_provenance() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();

    let parameter_and_retention_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.forensic-expansion-budget",
    );
    let mut parameter_and_retention_runtime = TestRuntime::builder(graph.clone())
        .with_kernel_defaults()
        .resource_policy_registry(parameter_and_retention_registry)
        .build();
    parameter_and_retention_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");
    let parameter_and_retention_report = parameter_and_retention_runtime
        .resource_replay_availability(
            &parameter_and_retention_replay_resource_declaration(node).with_diagnostics_policy(
                ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                    max_replay_reconstruction_width: 5,
                    max_forensic_reconstruction_width: 5,
                },
            ),
        )
        .expect("parameter-and-retention replay availability should classify");

    let parameter_and_diagnostics_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.forensic-expansion-budget",
    );
    let mut parameter_and_diagnostics_runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(parameter_and_diagnostics_registry)
        .build();
    parameter_and_diagnostics_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");
    let parameter_and_diagnostics_report = parameter_and_diagnostics_runtime
        .resource_replay_availability(
            &parameter_and_diagnostics_replay_resource_declaration(node).with_diagnostics_policy(
                ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                    max_replay_reconstruction_width: 5,
                    max_forensic_reconstruction_width: 5,
                },
            ),
        )
        .expect("parameter-and-diagnostics replay availability should classify");

    assert_eq!(
        parameter_and_retention_report.class(),
        ResourceReplayAvailabilityClass::Retained
    );
    assert_eq!(
        parameter_and_diagnostics_report.class(),
        ResourceReplayAvailabilityClass::Retained
    );
    assert_ne!(
        parameter_and_retention_report.availability_digest(),
        parameter_and_diagnostics_report.availability_digest(),
        "pair replay adapters must remain digest-distinct even when they admit the same compatible drift"
    );
}

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

#[test]
fn resource_replay_availability_budget_history_denial_is_distinct_from_restore_compatibility_denial(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut strict_runtime = TestRuntime::build(graph.clone());
    strict_runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let payload_digest = strict_runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();
    for request_id in [ResourceRequestId::new(930), ResourceRequestId::new(931)] {
        strict_runtime
            .admit_resource_completion(RawCompletionEnvelope::new(
                request_id,
                ResourceGeneration::new(1),
                ResourceBranchEpoch::new(strict_runtime.graph().current_branch().id, 0),
                ResourceAttemptId::ZERO,
                payload_digest.clone(),
                32,
            ))
            .denied_completion()
            .expect("unknown request should retain denied completion evidence");
    }
    strict_runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );
    let budget_history_denied = strict_runtime
        .resource_replay_availability(&deny_on_unknown_or_missing_replay_resource_declaration(
            node,
        ))
        .expect("strict replay availability should classify");

    let mut restore_runtime = TestRuntime::build(graph);
    restore_runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("timeout declaration should lower");
    let restore_denied = restore_runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &timeout_resource_declaration(node, 9),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("restore-denied replay availability should classify");

    assert_eq!(
        budget_history_denied.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert_eq!(
        restore_denied.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::RestoreCompatibilityDenied)
    );
    assert_ne!(
        budget_history_denied.availability_digest(),
        restore_denied.availability_digest(),
        "budget-history denial and restore-compatibility denial must remain distinct"
    );
}
