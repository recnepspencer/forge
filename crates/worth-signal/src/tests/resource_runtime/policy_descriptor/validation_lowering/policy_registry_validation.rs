use super::*;

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
