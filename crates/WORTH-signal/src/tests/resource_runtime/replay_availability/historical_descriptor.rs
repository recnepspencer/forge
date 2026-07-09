use super::*;

#[test]
fn resource_policy_compatibility_denies_missing_historical_descriptor() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let historical_declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_declaration,
        &historical_registry,
    )
    .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .filter(|registration| {
            !matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            )
        })
        .collect();
    let current_registry = FrozenResourcePolicyRegistry::new(current_registrations)
        .expect("current registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect("current declaration should validate against the reduced registry");

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(77),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");

    assert!(!report.is_compatible());
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::MissingDescriptor
    );
    assert_ne!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_missing_historical_descriptor() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let historical_declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_declaration,
        &historical_registry,
    )
    .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .filter(|registration| {
            !matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            )
        })
        .collect();
    let current_registry = FrozenResourcePolicyRegistry::new(current_registrations)
        .expect("current registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect("current declaration should validate against the reduced registry");

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(77),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");
    let current_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_validated,
        &current_registry,
    )
    .expect("current declaration should freeze");
    let replay_plan = ResourceReplayDecisionPlan::lower(
        current_validated.declaration().replay_policy(),
        current_frozen.replay(),
    )
    .expect("default replay policy should lower");
    let denial = DeniedResourcePolicyRestoreCompatibility::from_compatibility(report, &replay_plan);

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::MissingDescriptor
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Timeout)
    );
    assert_eq!(denial.incompatible_width(), 1);
    assert_eq!(
        denial
            .compatibility()
            .families()
            .iter()
            .filter(|family| !family.class().is_compatible())
            .count(),
        1
    );
}

#[test]
fn resource_policy_compatibility_denies_incompatible_version_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &historical_registry)
            .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::IncompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    let current_registry =
        FrozenResourcePolicyRegistry::new(current_registrations).expect("registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect(
        "current declaration should validate while the historical descriptor remains incompatible",
    );

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(78),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");

    assert!(!report.is_compatible());
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::VersionIncompatible
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .current_compatibility_posture()
            .expect("current posture should exist"),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch
    );
    assert_ne!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .historical_version(),
        ResourcePolicyVersion::INITIAL
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_incompatible_version_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &historical_registry)
            .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::IncompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    let current_registry =
        FrozenResourcePolicyRegistry::new(current_registrations).expect("registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect(
        "current declaration should validate while the historical descriptor remains incompatible",
    );

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(78),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");
    let current_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_validated,
        &current_registry,
    )
    .expect("current declaration should freeze");
    let replay_plan = ResourceReplayDecisionPlan::lower(
        current_validated.declaration().replay_policy(),
        current_frozen.replay(),
    )
    .expect("default replay policy should lower");
    let denial = DeniedResourcePolicyRestoreCompatibility::from_compatibility(report, &replay_plan);

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::VersionIncompatible
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Timeout)
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::VersionIncompatible
    );
    assert_eq!(
        denial
            .compatibility()
            .families()
            .iter()
            .filter(|family| !family.class().is_compatible())
            .count(),
        1
    );
}
