use super::*;

#[test]
fn resource_diagnostics_summary_respects_cold_reconstruction_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let allocation_telemetry_before = runtime.telemetry().resource;

    let err = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        )
        .expect_err("retained-summary-only diagnostics should deny replay reconstruction");

    assert_eq!(
        err.class(),
        ResourceDiagnosticsExpansionDenialClass::ColdReconstructionDisabled
    );
    assert_eq!(
        err.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::BudgetedExpansion
    );
    assert!(
        !err.policy_decision_digest().as_str().is_empty(),
        "diagnostics denial should retain the effective diagnostics policy digest"
    );
    assert_eq!(err.replay_reconstruction_width(), 2);
    assert_eq!(
        err.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(err.performance().denied_count(), 1);
    assert_eq!(
        err.performance().cost_posture(),
        ResourceCostPosture::DeniedFallback
    );
    assert_eq!(err.performance().operational_allocation_count(), 0);
    assert_eq!(err.performance().retained_history_allocation_count(), 0);
    assert_eq!(err.performance().diagnostics_allocation_count(), 0);
    assert_eq!(err.performance().facade_report_allocation_count(), 1);
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
            .resource_replay_reconstruction_count,
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
            .resource_diagnostics_policy_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        2
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

    let allocation_telemetry_before_admission = runtime.telemetry().resource;
    let admitted = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(2),
        )
        .expect("budget that admits descriptor plus lifecycle reconstruction should pass");

    assert_eq!(
        admitted.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(
        admitted
            .expansion_budget()
            .max_replay_reconstruction_width(),
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        1
    );
    assert_eq!(admitted.performance().diagnostics_allocation_count(), 2);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count
            - allocation_telemetry_before_admission.resource_diagnostics_allocation_count,
        4
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before_admission.resource_facade_report_allocation_count,
        2
    );
}

#[test]
fn resource_diagnostics_summary_denies_when_replay_width_exceeds_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(1),
        )
        .expect_err("descriptor plus lifecycle width should exceed budget one");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::ReplayReconstructionBudgetExceeded
    );
    assert_eq!(denial.budget().max_replay_reconstruction_width(), 1);
    assert_eq!(denial.replay_reconstruction_width(), 2);
    assert_eq!(denial.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        0
    );
}

#[test]
fn resource_diagnostics_policy_retained_only_denies_cold_reconstruction_even_when_caller_budget_allows(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retained_only_diagnostics_resource_declaration(node))
        .expect("retained-only diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("retained-only policy should deny cold reconstruction");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyRetainedOnly
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::RetainedOnly
    );
    assert_eq!(denial.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_policy_decision_count,
        1
    );
}

#[test]
fn resource_diagnostics_policy_budgeted_expansion_denies_above_descriptor_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 1))
        .expect("budgeted diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("descriptor-backed diagnostics budget should cap cold reconstruction");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyReplayReconstructionBudgetExceeded
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::BudgetedExpansion
    );
    assert_eq!(denial.replay_reconstruction_width(), 2);
}

#[test]
fn resource_diagnostics_policy_forensic_expansion_budget_denies_above_descriptor_forensic_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(forensic_diagnostics_resource_declaration(node, 2, 1))
        .expect("forensic diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction_with_forensic_budget(
                u32::MAX,
                u32::MAX,
            ),
        )
        .expect_err("descriptor forensic budget should deny above-forensic reconstruction");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyForensicReconstructionBudgetExceeded
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::ForensicExpansionBudget
    );
    assert_eq!(denial.replay_reconstruction_width(), 2);
    assert_eq!(denial.forensic_reconstruction_width(), 2);
}

#[test]
fn resource_diagnostics_summary_denies_when_caller_forensic_budget_is_tighter_than_replay_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction_with_forensic_budget(
                8, 1,
            ),
        )
        .expect_err("caller forensic budget should deny even when replay budget allows");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::ForensicReconstructionBudgetExceeded
    );
    assert_eq!(denial.replay_reconstruction_width(), 2);
    assert_eq!(denial.forensic_reconstruction_width(), 2);
    assert_eq!(denial.budget().max_replay_reconstruction_width(), 8);
    assert_eq!(denial.budget().max_forensic_reconstruction_width(), 1);
}

#[test]
fn resource_diagnostics_policy_mixed_nodes_use_hard_denial_posture_over_budgeted_nodes() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(first, 5))
        .expect("budgeted diagnostics declaration should lower");
    runtime
        .declare_resource_node(deny_cold_diagnostics_resource_declaration(second))
        .expect("deny-cold diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(5),
        )
        .expect_err("deny-cold diagnostics policy should dominate mixed nodes");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyColdReconstructionDisabled
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::DenyColdExpansion
    );
}
