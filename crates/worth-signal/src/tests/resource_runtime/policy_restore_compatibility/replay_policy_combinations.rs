use super::*;

#[test]
fn resource_policy_restore_compatibility_admits_parameter_expansion_and_names_defaulted_parameter()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.forensic-expansion-budget",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical diagnostics declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(
            &parameter_expansion_only_replay_resource_declaration(node).with_diagnostics_policy(
                ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                    max_replay_reconstruction_width: 5,
                    max_forensic_reconstruction_width: 5,
                },
            ),
        )
        .expect("declared node should classify")
        .expect("compatible parameter expansion should admit restore proof");

    let diagnostics = proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("diagnostics family report should exist");
    assert_eq!(
        diagnostics.class(),
        ResourcePolicyCompatibilityClass::CompatibleParameterExpansion
    );
    assert_eq!(
        diagnostics.historical_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::BudgetedExpansion)
    );
    assert_eq!(
        diagnostics.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::ForensicExpansionBudget)
    );
    assert_eq!(
        diagnostics.defaulted_parameter_names(),
        ["max_forensic_reconstruction_width"]
    );
    assert!(diagnostics.canonical_truth_preserved());
    assert!(!diagnostics.retained_history_unavailable());
    assert!(!diagnostics.diagnostics_details_unavailable());
    assert_eq!(proof.retained_history_unavailable_width(), 0);
    assert_eq!(proof.diagnostics_details_unavailable_width(), 0);
    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansion
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        1
    );
}

#[test]
fn resource_policy_restore_compatibility_parameter_and_retention_replay_policy_admits_mixed_drift()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for_entries([
        (
            ResourcePolicyKind::Retention,
            "signal.resource.retention.terminal-summaries-only",
        ),
        (
            ResourcePolicyKind::Diagnostics,
            "signal.resource.diagnostics.forensic-expansion-budget",
        ),
    ]);
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(
            retain_all_transitions_resource_declaration(node).with_diagnostics_policy(
                ResourceDiagnosticsPolicyDeclaration::BudgetedExpansion {
                    max_replay_reconstruction_width: 5,
                },
            ),
        )
        .expect("historical declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(
            &parameter_and_retention_replay_resource_declaration(node)
                .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly)
                .with_diagnostics_policy(
                    ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                        max_replay_reconstruction_width: 5,
                        max_forensic_reconstruction_width: 5,
                    },
                ),
        )
        .expect("declared node should classify")
        .expect("parameter-and-retention replay policy should admit both compatible drifts");

    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowing
    );
    assert_eq!(proof.retained_history_unavailable_width(), 1);
    assert_eq!(proof.diagnostics_details_unavailable_width(), 0);
    assert_eq!(
        proof
            .compatibility()
            .family(ResourcePolicyKind::Retention)
            .expect("retention family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing
    );
    let diagnostics = proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("diagnostics family report should exist");
    assert_eq!(
        diagnostics.class(),
        ResourcePolicyCompatibilityClass::CompatibleParameterExpansion
    );
    assert_eq!(
        diagnostics.defaulted_parameter_names(),
        ["max_forensic_reconstruction_width"]
    );
}

#[test]
fn resource_policy_restore_compatibility_parameter_and_retention_replay_policy_still_denies_diagnostics_richness_change(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(
            &parameter_and_retention_replay_resource_declaration(node)
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("declared node should classify")
        .expect_err("parameter-and-retention replay policy should deny diagnostics richness drift");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        denial.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowing
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Diagnostics)
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Diagnostics)
            .expect("diagnostics family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange
    );
}

#[test]
fn resource_policy_restore_compatibility_parameter_and_diagnostics_replay_policy_admits_parameter_expansion(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.forensic-expansion-budget",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(
            &parameter_and_diagnostics_replay_resource_declaration(node).with_diagnostics_policy(
                ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                    max_replay_reconstruction_width: 5,
                    max_forensic_reconstruction_width: 5,
                },
            ),
        )
        .expect("declared node should classify")
        .expect("parameter-and-diagnostics replay policy should admit parameter expansion");

    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndDiagnosticsRichnessChange
    );
    assert_eq!(
        proof
            .compatibility()
            .family(ResourcePolicyKind::Diagnostics)
            .expect("diagnostics family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleParameterExpansion
    );
}

#[test]
fn resource_policy_restore_compatibility_parameter_and_diagnostics_replay_policy_admits_diagnostics_richness_change(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(
            &parameter_and_diagnostics_replay_resource_declaration(node)
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("declared node should classify")
        .expect("parameter-and-diagnostics replay policy should admit diagnostics richness drift");

    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndDiagnosticsRichnessChange
    );
    assert_eq!(
        proof
            .compatibility()
            .family(ResourcePolicyKind::Diagnostics)
            .expect("diagnostics family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange
    );
    assert_eq!(proof.diagnostics_details_unavailable_width(), 1);
}

#[test]
fn resource_policy_restore_compatibility_parameter_and_diagnostics_replay_policy_still_denies_retention_narrowing(
) {
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

    let denial = runtime
        .admit_resource_policy_restore_compatibility(
            &parameter_and_diagnostics_replay_resource_declaration(node)
                .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly),
        )
        .expect("declared node should classify")
        .expect_err(
            "parameter-and-diagnostics replay policy should still deny retention narrowing",
        );

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        denial.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndDiagnosticsRichnessChange
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Retention)
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Retention)
            .expect("retention family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing
    );
}

#[test]
fn resource_policy_restore_compatibility_replay_policy_can_deny_parameter_expansion() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.forensic-expansion-budget",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(
            &forensic_diagnostics_resource_declaration(node, 5, 5).with_replay_policy(
                ResourceReplayPolicyDeclaration::CompatibleDiagnosticsRichnessChange,
            ),
        )
        .expect("declared node should classify")
        .expect_err("diagnostics-richness replay policy should deny parameter expansion");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Diagnostics)
    );
    assert_eq!(
        denial.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleDiagnosticsRichnessChange
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Diagnostics)
            .expect("diagnostics family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleParameterExpansion
    );
}
