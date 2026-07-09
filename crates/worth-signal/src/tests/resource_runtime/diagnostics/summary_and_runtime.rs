use super::*;

#[test]
fn resource_diagnostics_summary_preserves_truth_and_exposes_replay_debt() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(9_999),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown completion should retain denial provenance");

    let runtime_summary_before = runtime.resource_runtime_summary();
    let replay_count_before = runtime
        .telemetry()
        .resource
        .resource_replay_reconstruction_count;
    let allocation_telemetry_before = runtime.telemetry().resource;
    let diagnostics = runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();

    assert_eq!(
        diagnostics.schema_version(),
        RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION
    );
    assert_eq!(diagnostics.runtime_summary(), runtime_summary_before);
    assert_eq!(runtime.resource_runtime_summary(), runtime_summary_before);
    assert!(diagnostics.latest_branch_restore_report().is_none());
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .performance()
            .cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(
        diagnostics.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(
        diagnostics.performance().cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(
        diagnostics
            .expansion_budget()
            .max_replay_reconstruction_width(),
        u32::MAX
    );
    assert_eq!(
        diagnostics.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::BudgetedExpansion
    );
    assert!(
        !diagnostics.policy_decision_digest().as_str().is_empty(),
        "diagnostics summary should retain the effective diagnostics policy digest"
    );
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .denied_completion_width(),
        1
    );
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .performance()
            .diagnostics_allocation_count(),
        diagnostics
            .replay_reconstruction()
            .performance()
            .input_width()
    );
    assert_eq!(
        diagnostics.performance().diagnostics_allocation_count(),
        diagnostics
            .replay_reconstruction()
            .performance()
            .input_width()
    );
    assert_eq!(
        diagnostics.performance().facade_report_allocation_count(),
        1
    );
    assert_eq!(diagnostics.performance().operational_allocation_count(), 0);
    assert_eq!(
        diagnostics
            .performance()
            .retained_history_allocation_count(),
        0
    );
    assert!(!diagnostics.provenance_digest().is_empty());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        replay_count_before + 1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count
            - allocation_telemetry_before.resource_diagnostics_allocation_count,
        diagnostics
            .replay_reconstruction()
            .performance()
            .diagnostics_allocation_count() as u64
            + diagnostics.performance().diagnostics_allocation_count() as u64
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        diagnostics
            .replay_reconstruction()
            .performance()
            .facade_report_allocation_count() as u64
            + diagnostics.performance().facade_report_allocation_count() as u64
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
}

#[test]
fn resource_runtime_summary_read_report_is_zero_cold_reconstruction() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let replay_count_before = runtime
        .telemetry()
        .resource
        .resource_replay_reconstruction_count;
    let diagnostics_expansion_before = runtime
        .telemetry()
        .resource
        .resource_diagnostics_expansion_count;
    let allocation_telemetry_before = runtime.telemetry().resource;
    let report = runtime.resource_runtime_summary_read_report();

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::SummaryRead
    );
    assert_eq!(
        report.performance().cost_posture(),
        ResourceCostPosture::Verified
    );
    assert_eq!(report.performance().operational_allocation_count(), 0);
    assert_eq!(report.performance().retained_history_allocation_count(), 0);
    assert_eq!(report.performance().diagnostics_allocation_count(), 0);
    assert_eq!(report.performance().facade_report_allocation_count(), 1);
    assert_eq!(report.performance().broad_scan_denial_count(), 0);
    assert_eq!(report.summary(), runtime.resource_runtime_summary());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_summary_read_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        replay_count_before
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        diagnostics_expansion_before
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
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count,
        allocation_telemetry_before.resource_diagnostics_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        1
    );
}
