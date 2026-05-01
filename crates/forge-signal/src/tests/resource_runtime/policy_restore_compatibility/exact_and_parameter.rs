use super::*;

#[test]
fn resource_policy_compatibility_accepts_exact_descriptor_match() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");

    let report = runtime
        .classify_resource_policy_compatibility(&declaration)
        .expect("identical declaration should classify as exactly compatible");

    assert!(report.is_compatible());
    assert_eq!(report.compared_width(), 10);
    assert_eq!(report.incompatible_width(), 0);
    assert_eq!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::ExactDescriptorMatch
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .historical_version(),
        ResourcePolicyVersion::INITIAL
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_compatibility_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_comparison_count,
        10
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
fn resource_policy_restore_compatibility_admits_exact_descriptor_match() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(&declaration)
        .expect("declared node should classify")
        .expect("identical declaration should admit restore compatibility proof");

    assert!(proof.compatibility().is_compatible());
    assert_eq!(proof.compatibility().compared_width(), 10);
    assert_eq!(
        proof.compatibility_digest().as_str(),
        proof.compatibility().compatibility_digest().as_str()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_compatibility_count,
        1
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
            .resource_replay_incompatible_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_policy_decision_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
}

#[test]
fn resource_policy_compatibility_denies_parameter_digest_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");

    let report = runtime
        .classify_resource_policy_compatibility(&timeout_resource_declaration(node, 9))
        .expect("compatibility classification should still produce a report");

    assert!(!report.is_compatible());
    assert_eq!(report.incompatible_width(), 1);
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::ParameterDigestDrift
    );
    assert_eq!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatibility_decision_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_parameter_drift_before_current_policy_code_executes(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(&timeout_resource_declaration(node, 9))
        .expect("declared node should classify")
        .expect_err("parameter drift must deny restore compatibility");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ParameterDigestDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Timeout)
    );
    assert_eq!(denial.incompatible_width(), 1);
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::ParameterDigestDrift
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        1
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
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_incompatible_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_missing_policy_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_policy_decision_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
}
