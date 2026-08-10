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
