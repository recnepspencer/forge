use super::*;

#[test]
fn resource_policy_restore_compatibility_admits_retention_narrowing_with_unavailable_rich_history()
{
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
        .expect("historical resource declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(&terminal_summaries_only_resource_declaration(
            node,
        ))
        .expect("declared node should classify")
        .expect("compatible retention narrowing should admit restore proof");

    let retention = proof
        .compatibility()
        .family(ResourcePolicyKind::Retention)
        .expect("retention family report should exist");
    assert_eq!(
        retention.class(),
        ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing
    );
    assert_eq!(
        retention.historical_retention_class(),
        Some(ResourceRetentionDecisionClass::RetainAllTransitions)
    );
    assert_eq!(
        retention.current_retention_class(),
        Some(ResourceRetentionDecisionClass::TerminalSummariesOnly)
    );
    assert!(retention.canonical_truth_preserved());
    assert!(retention.retained_history_unavailable());
    assert!(!retention.diagnostics_details_unavailable());
    assert_eq!(proof.retained_history_unavailable_width(), 1);
    assert_eq!(proof.diagnostics_details_unavailable_width(), 0);
    assert_eq!(proof.canonical_truth_preserved_width(), 10);
    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatibility_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_admits_diagnostics_richness_change_with_explicit_availability_posture(
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
        .expect("historical diagnostics declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(
            &retained_only_diagnostics_resource_declaration(node),
        )
        .expect("declared node should classify")
        .expect("compatible diagnostics change should admit restore proof");

    let diagnostics = proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("diagnostics family report should exist");
    assert_eq!(
        diagnostics.class(),
        ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange
    );
    assert_eq!(
        diagnostics.historical_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::BudgetedExpansion)
    );
    assert_eq!(
        diagnostics.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::RetainedOnly)
    );
    assert!(diagnostics.canonical_truth_preserved());
    assert!(!diagnostics.retained_history_unavailable());
    assert!(diagnostics.diagnostics_details_unavailable());
    assert_eq!(proof.retained_history_unavailable_width(), 0);
    assert_eq!(proof.diagnostics_details_unavailable_width(), 1);
    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_replay_policy_can_deny_retention_narrowing() {
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
            &identical_only_replay_resource_declaration(node)
                .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly),
        )
        .expect("declared node should classify")
        .expect_err("identical-only replay policy should deny retention narrowing");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Retention)
    );
    assert_eq!(
        denial.replay_decision_class(),
        ResourceReplayDecisionClass::IdenticalOnly
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
fn resource_policy_restore_compatibility_replay_policy_can_deny_diagnostics_richness_change() {
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
            &retention_only_replay_resource_declaration(node)
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("declared node should classify")
        .expect_err("retention-only replay policy should deny diagnostics richness change");

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
        ResourceReplayDecisionClass::CompatibleRetentionNarrowing
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
fn resource_policy_restore_compatibility_proof_constructor_rejects_replay_gated_compatible_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let historical_declaration = retain_all_transitions_resource_declaration(node);
    let current_declaration = identical_only_replay_resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly);
    let historical_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_declaration,
        &compatible_registry,
    )
    .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &compatible_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &current_declaration,
        &compatible_registry,
    )
    .expect("current declaration should validate");
    let current_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_validated,
        &compatible_registry,
    )
    .expect("current declaration should freeze");
    let replay_plan = ResourceReplayDecisionPlan::lower(
        current_validated.declaration().replay_policy(),
        current_frozen.replay(),
    )
    .expect("replay plan should lower");
    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(91),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &compatible_registry,
    )
    .expect("compatibility classification should succeed");

    assert!(report.is_compatible());
    assert!(
        ResourcePolicyRestoreCompatibilityProof::from_compatibility(report, &replay_plan).is_err(),
        "proof constructor must reject replay-gated compatible drift"
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_retention_widening_even_under_compatible_posture() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.retain-all-transitions",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(terminal_summaries_only_resource_declaration(node))
        .expect("historical declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(&retain_all_transitions_resource_declaration(
            node,
        ))
        .expect("declared node should classify")
        .expect_err("retention widening should still deny restore compatibility");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::DecisionSemanticsDrift
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
        ResourcePolicyCompatibilityClass::DecisionSemanticsDrift
    );
}
