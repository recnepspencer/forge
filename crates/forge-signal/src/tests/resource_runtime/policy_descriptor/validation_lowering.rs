use super::*;

#[test]
fn resource_named_retry_and_timeout_policies_deny_before_descriptor_lowering() {
    let mut graph = SignalGraph::new();
    let retry_node = graph.node().build();
    let timeout_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let retry_error = runtime
        .declare_resource_node(resource_declaration(retry_node).with_retry_policy(
            ResourceRetryPolicyDeclaration::Named {
                name: ResourcePolicyName::new("signal.resource.retry.fixed-delay"),
            },
        ))
        .expect_err("named retry policy should deny in the first ship runtime");
    assert!(
        retry_error
            .to_string()
            .contains("not executable in the first ship runtime"),
        "unexpected retry error: {retry_error}"
    );

    let timeout_error = runtime
        .declare_resource_node(resource_declaration(timeout_node).with_timeout_policy(
            ResourceTimeoutPolicyDeclaration::Named {
                name: ResourcePolicyName::new("signal.resource.timeout.fixed-timeout"),
            },
        ))
        .expect_err("named timeout policy should deny in the first ship runtime");
    assert!(
        timeout_error
            .to_string()
            .contains("not executable in the first ship runtime"),
        "unexpected timeout error: {timeout_error}"
    );
}

#[test]
fn resource_policy_unknown_named_policy_denies_before_descriptor_allocation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration =
        resource_declaration(node).with_retry_policy(ResourceRetryPolicyDeclaration::Named {
            name: ResourcePolicyName::new("example.resource.retry.unregistered"),
        });

    let err = runtime
        .declare_resource_node(declaration)
        .expect_err("unknown named retry policy should deny declaration");

    assert!(err
        .to_string()
        .contains("example.resource.retry.unregistered"));
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_resolution_denial_count,
        1
    );
}

#[test]
fn resource_policy_validation_binds_lowered_bundle_to_registry_digest() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    let registry = FrozenResourcePolicyRegistry::built_in();

    let validated = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect("built-in declaration should validate");
    let frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&validated, &registry)
            .expect("validated declaration should freeze against the same registry digest");
    let lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&frozen);

    assert_eq!(
        validated.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert_eq!(
        frozen.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert_eq!(
        lowered.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert_eq!(
        lowered.retry().descriptor().semantic_name().as_str(),
        "signal.resource.retry.fixed-delay"
    );
    assert_eq!(
        lowered.timeout().parameter_digest().as_str(),
        "timeout:fixed-timeout:3"
    );
}

#[test]
fn resource_policy_freeze_digest_changes_when_parameters_change() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let registry = FrozenResourcePolicyRegistry::built_in();
    let first_declaration = timeout_resource_declaration(first, 3);
    let second_declaration = timeout_resource_declaration(second, 9);

    let first_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&first_declaration, &registry)
            .expect("first declaration should validate");
    let second_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&second_declaration, &registry)
            .expect("second declaration should validate");
    let first_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&first_validated, &registry)
            .expect("first declaration should freeze");
    let second_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&second_validated, &registry)
            .expect("second declaration should freeze");
    let first_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&first_frozen);
    let second_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&second_frozen);

    assert_eq!(
        first_frozen
            .timeout()
            .descriptor()
            .descriptor_digest()
            .as_str(),
        second_frozen
            .timeout()
            .descriptor()
            .descriptor_digest()
            .as_str()
    );
    assert_ne!(
        first_frozen.timeout().parameter_digest().as_str(),
        second_frozen.timeout().parameter_digest().as_str()
    );
    assert_ne!(
        first_frozen.timeout().frozen_digest().as_str(),
        second_frozen.timeout().frozen_digest().as_str()
    );
    assert_ne!(
        first_lowered.bundle_digest().as_str(),
        second_lowered.bundle_digest().as_str()
    );

    let scoped_declaration = total_request_lifetime_timeout_resource_declaration(first, 3);
    let scoped_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&scoped_declaration, &registry)
            .expect("scoped declaration should validate");
    let scoped_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&scoped_validated, &registry)
            .expect("scoped declaration should freeze");
    let scoped_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&scoped_frozen);

    assert_ne!(
        first_frozen.timeout().parameter_digest().as_str(),
        scoped_frozen.timeout().parameter_digest().as_str()
    );
    assert_ne!(
        first_frozen.timeout().descriptor().semantic_name().as_str(),
        scoped_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );
    assert_ne!(
        first_lowered.bundle_digest().as_str(),
        scoped_lowered.bundle_digest().as_str()
    );

    let heartbeat_declaration = heartbeat_extension_timeout_resource_declaration(first, 3, 5);
    let heartbeat_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&heartbeat_declaration, &registry)
            .expect("heartbeat declaration should validate");
    let heartbeat_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &heartbeat_validated,
        &registry,
    )
    .expect("heartbeat declaration should freeze");
    let heartbeat_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&heartbeat_frozen);
    assert_ne!(
        first_frozen.timeout().parameter_digest().as_str(),
        heartbeat_frozen.timeout().parameter_digest().as_str()
    );
    assert_ne!(
        heartbeat_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        first_frozen.timeout().descriptor().semantic_name().as_str()
    );
    assert_ne!(
        heartbeat_lowered.bundle_digest().as_str(),
        first_lowered.bundle_digest().as_str()
    );

    let terminal_declaration = terminal_timeout_resource_declaration(first, 3);
    let terminal_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&terminal_declaration, &registry)
            .expect("terminal timeout declaration should validate");
    let terminal_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &terminal_validated,
        &registry,
    )
    .expect("terminal timeout declaration should freeze");
    assert_ne!(
        first_frozen.timeout().descriptor().semantic_name().as_str(),
        terminal_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );

    let revalidation_declaration = revalidation_eligible_timeout_resource_declaration(first, 3);
    let revalidation_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&revalidation_declaration, &registry)
            .expect("revalidation eligible timeout declaration should validate");
    let revalidation_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &revalidation_validated,
        &registry,
    )
    .expect("revalidation eligible timeout declaration should freeze");
    assert_ne!(
        terminal_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        revalidation_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );

    let transaction_deadline_declaration =
        transaction_inherited_deadline_resource_declaration(first);
    let transaction_deadline_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &transaction_deadline_declaration,
        &registry,
    )
    .expect("transaction inherited deadline declaration should validate");
    let transaction_deadline_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            &transaction_deadline_validated,
            &registry,
        )
        .expect("transaction inherited deadline declaration should freeze");
    assert_ne!(
        transaction_deadline_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        first_frozen.timeout().descriptor().semantic_name().as_str()
    );

    let runtime_deadline_declaration = runtime_inherited_deadline_resource_declaration(first);
    let runtime_deadline_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &runtime_deadline_declaration,
        &registry,
    )
    .expect("runtime inherited deadline declaration should validate");
    let runtime_deadline_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &runtime_deadline_validated,
        &registry,
    )
    .expect("runtime inherited deadline declaration should freeze");
    assert_ne!(
        transaction_deadline_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        runtime_deadline_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );
}

#[test]
fn resource_policy_malformed_named_policy_denies_before_descriptor_allocation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration =
        resource_declaration(node).with_retry_policy(ResourceRetryPolicyDeclaration::Named {
            name: ResourcePolicyName::new("   "),
        });

    let err = runtime
        .declare_resource_node(declaration)
        .expect_err("malformed named policy should deny declaration");

    assert!(err
        .to_string()
        .contains("malformed resource policy descriptor"));
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_resolution_denial_count,
        1
    );
}

#[test]
fn resource_policy_missing_builtin_descriptor_denies_during_validation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = resource_declaration(node);
    let registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .filter(|registration: &ResourcePolicyRegistration| {
            !matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (ResourcePolicyKind::Retry, "signal.resource.retry.disabled")
            )
        })
        .collect();
    let registry = FrozenResourcePolicyRegistry::new(registrations)
        .expect("custom registry should freeze without one built-in");

    let err = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect_err("missing built-in descriptor should deny validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::MissingDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("signal.resource.retry.disabled"),
        }
    );
}

#[test]
fn resource_policy_incompatible_named_descriptor_denies_during_validation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration =
        resource_declaration(node).with_retry_policy(ResourceRetryPolicyDeclaration::Named {
            name: ResourcePolicyName::new("example.resource.retry.incompatible"),
        });
    let mut registrations = built_in_policy_registrations();
    registrations.push(ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(400),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.incompatible"),
        ResourcePolicyVersion::new(2, 0),
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::IncompatibleVersion,
    ));
    let registry =
        FrozenResourcePolicyRegistry::new(registrations).expect("custom registry should freeze");

    let err = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect_err("incompatible named descriptor should deny validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::IncompatibleDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("example.resource.retry.incompatible"),
            version: ResourcePolicyVersion::new(2, 0),
            compatibility_posture: ResourcePolicyCompatibilityPosture::IncompatibleVersion,
        }
    );
}

#[test]
fn resource_policy_incompatible_builtin_descriptor_denies_during_validation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = resource_declaration(node);
    let registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (ResourcePolicyKind::Retry, "signal.resource.retry.disabled")
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 1),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::IncompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    let registry =
        FrozenResourcePolicyRegistry::new(registrations).expect("custom registry should freeze");

    let err = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect_err("incompatible built-in descriptor should deny validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::IncompatibleDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("signal.resource.retry.disabled"),
            version: ResourcePolicyVersion::new(2, 1),
            compatibility_posture: ResourcePolicyCompatibilityPosture::IncompatibleVersion,
        }
    );
}

#[test]
fn resource_policy_freeze_denies_registry_digest_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    let validated_registry = FrozenResourcePolicyRegistry::built_in();
    let drifted_registrations = built_in_policy_registrations()
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
                    registration.compatibility_posture(),
                )
            } else {
                registration
            }
        })
        .collect();
    let drifted_registry = FrozenResourcePolicyRegistry::new(drifted_registrations)
        .expect("alternate registry should still freeze");

    let validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &validated_registry)
            .expect("declaration should validate against the original registry");
    let err = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &validated,
        &drifted_registry,
    )
    .expect_err("freeze must deny when the registry digest drifts after validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::RegistryDigestDrift {
            expected: validated_registry.registry_digest().clone(),
            actual: drifted_registry.registry_digest().clone(),
        }
    );
}
