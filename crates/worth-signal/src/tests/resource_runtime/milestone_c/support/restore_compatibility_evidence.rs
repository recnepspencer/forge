use super::super::*;

pub(super) struct ResourceMilestoneCRestoreCompatibilityEvidence {
    pub(super) compatible_restore: ResourcePolicyRestoreCompatibilityProof,
    pub(super) incompatible_restore: DeniedResourcePolicyRestoreCompatibility,
    pub(super) missing_restore: DeniedResourcePolicyRestoreCompatibility,
}

pub(super) fn resource_milestone_c_restore_compatibility_evidence(
) -> ResourceMilestoneCRestoreCompatibilityEvidence {
    ResourceMilestoneCRestoreCompatibilityEvidence {
        compatible_restore: compatible_resource_policy_restore(),
        incompatible_restore: incompatible_resource_policy_restore(),
        missing_restore: missing_resource_policy_restore(),
    }
}

fn compatible_resource_policy_restore() -> ResourcePolicyRestoreCompatibilityProof {
    let mut retention_restore_graph = SignalGraph::new();
    let retention_restore_node = retention_restore_graph.node().build();
    let retention_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let mut retention_restore_runtime = TestRuntime::builder(retention_restore_graph)
        .with_kernel_defaults()
        .resource_policy_registry(retention_registry)
        .build();
    retention_restore_runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(
            retention_restore_node,
        ))
        .expect("historical retention declaration should lower");
    retention_restore_runtime
        .admit_resource_policy_restore_compatibility(&terminal_summaries_only_resource_declaration(
            retention_restore_node,
        ))
        .expect("compatible retention drift should classify")
        .expect("compatible retention drift should admit")
}

fn incompatible_resource_policy_restore() -> DeniedResourcePolicyRestoreCompatibility {
    let mut incompatible_restore_graph = SignalGraph::new();
    let incompatible_restore_node = incompatible_restore_graph.node().build();
    let historical_incompatible_timeout =
        timeout_resource_declaration(incompatible_restore_node, 3);
    let historical_incompatible_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_incompatible_timeout,
        &FrozenResourcePolicyRegistry::built_in(),
    )
    .expect("historical timeout declaration should validate");
    let historical_incompatible_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            &historical_incompatible_validated,
            &FrozenResourcePolicyRegistry::built_in(),
        )
        .expect("historical timeout declaration should freeze");
    let historical_incompatible_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_incompatible_frozen);
    let incompatible_registrations = built_in_policy_registrations()
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
    let incompatible_registry = FrozenResourcePolicyRegistry::new(incompatible_registrations)
        .expect("incompatible registry should freeze");
    let current_incompatible_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(incompatible_restore_node),
        &incompatible_registry,
    )
    .expect("current declaration should validate against the incompatible registry");
    let incompatible_report =
        ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
            ResourceDescriptorId::new(127),
            ResourceNodeId::from_node(incompatible_restore_node),
            &historical_incompatible_lowered,
            &current_incompatible_validated,
            &incompatible_registry,
        )
        .expect("incompatible-version compatibility classification should succeed");
    let current_incompatible_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            &current_incompatible_validated,
            &incompatible_registry,
        )
        .expect("current declaration should freeze against the incompatible registry");
    let incompatible_replay_plan = ResourceReplayDecisionPlan::lower(
        current_incompatible_validated.declaration().replay_policy(),
        current_incompatible_frozen.replay(),
    )
    .expect("default replay policy should lower for incompatible-version denial");

    DeniedResourcePolicyRestoreCompatibility::from_compatibility(
        incompatible_report,
        &incompatible_replay_plan,
    )
}

fn missing_resource_policy_restore() -> DeniedResourcePolicyRestoreCompatibility {
    let mut missing_restore_graph = SignalGraph::new();
    let missing_restore_node = missing_restore_graph.node().build();
    let missing_registry = FrozenResourcePolicyRegistry::new(
        built_in_policy_registrations()
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
            .collect(),
    )
    .expect("missing registry should still freeze");
    let historical_missing_timeout = timeout_resource_declaration(missing_restore_node, 3);
    let historical_missing_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_missing_timeout,
        &FrozenResourcePolicyRegistry::built_in(),
    )
    .expect("historical timeout declaration should validate against the built-in registry");
    let historical_missing_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_missing_validated,
        &FrozenResourcePolicyRegistry::built_in(),
    )
    .expect("historical timeout declaration should freeze against the built-in registry");
    let historical_missing_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_missing_frozen);
    let current_missing_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(missing_restore_node),
        &missing_registry,
    )
    .expect("current declaration should validate against the reduced registry");
    let missing_report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(177),
        ResourceNodeId::from_node(missing_restore_node),
        &historical_missing_lowered,
        &current_missing_validated,
        &missing_registry,
    )
    .expect("missing-descriptor compatibility classification should succeed");
    let current_missing_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_missing_validated,
        &missing_registry,
    )
    .expect("current declaration should freeze against the reduced registry");
    let missing_replay_plan = ResourceReplayDecisionPlan::lower(
        current_missing_validated.declaration().replay_policy(),
        current_missing_frozen.replay(),
    )
    .expect("default replay policy should lower for missing-descriptor denial");

    DeniedResourcePolicyRestoreCompatibility::from_compatibility(
        missing_report,
        &missing_replay_plan,
    )
}
