use super::*;

#[test]
fn resource_policy_restore_compatibility_retention_narrowing_names_exact_retention_posture() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.compact-cancelled",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("historical declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(&compact_cancelled_resource_declaration(node))
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
        Some(ResourceRetentionDecisionClass::CompactCancelled)
    );
    assert!(retention.retained_history_unavailable());
}

#[test]
fn resource_policy_restore_compatibility_diagnostics_richness_change_distinguishes_retained_only_from_deny_cold(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();

    let retained_only_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut retained_only_runtime = TestRuntime::builder(graph.clone())
        .with_kernel_defaults()
        .resource_policy_registry(retained_only_registry)
        .build();
    retained_only_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical diagnostics declaration should lower");
    let retained_only_proof = retained_only_runtime
        .admit_resource_policy_restore_compatibility(
            &retained_only_diagnostics_resource_declaration(node),
        )
        .expect("declared node should classify")
        .expect("retained-only diagnostics change should admit restore proof");

    let deny_cold_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.deny-cold-expansion",
    );
    let mut deny_cold_runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(deny_cold_registry)
        .build();
    deny_cold_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical diagnostics declaration should lower");
    let deny_cold_proof = deny_cold_runtime
        .admit_resource_policy_restore_compatibility(&deny_cold_diagnostics_resource_declaration(
            node,
        ))
        .expect("declared node should classify")
        .expect("deny-cold diagnostics change should admit restore proof");

    let retained_only_family = retained_only_proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("retained-only diagnostics family should exist");
    let deny_cold_family = deny_cold_proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("deny-cold diagnostics family should exist");

    assert_eq!(
        retained_only_family.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::RetainedOnly)
    );
    assert_eq!(
        deny_cold_family.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::DenyColdExpansion)
    );
    assert_ne!(
        retained_only_proof.compatibility_digest().as_str(),
        deny_cold_proof.compatibility_digest().as_str()
    );
}
