use super::*;

#[test]
fn resource_policy_registry_rejects_duplicate_descriptor_ids() {
    let first = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(99),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.first"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let second = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(99),
        ResourcePolicyKind::Timeout,
        ResourcePolicyName::new("example.resource.timeout.second"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(4),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );

    let err = FrozenResourcePolicyRegistry::new(vec![first, second])
        .expect_err("duplicate policy descriptor ids must deny registry construction");

    assert_eq!(
        err,
        ResourcePolicyRegistryError::DuplicateId(ResourcePolicyDescriptorId::new(99))
    );
}

#[test]
fn resource_policy_registry_digest_is_canonical_across_registration_order() {
    let retry = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(100),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.fixed"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let timeout = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(101),
        ResourcePolicyKind::Timeout,
        ResourcePolicyName::new("example.resource.timeout.fixed"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(4),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let cancellation = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(102),
        ResourcePolicyKind::Cancellation,
        ResourcePolicyName::new("example.resource.cancellation.runtime"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(3),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );

    let forward = FrozenResourcePolicyRegistry::new(vec![
        retry.clone(),
        timeout.clone(),
        cancellation.clone(),
    ])
    .expect("first registry should freeze");
    let reversed = FrozenResourcePolicyRegistry::new(vec![cancellation, timeout, retry])
        .expect("equivalent registry should freeze");

    assert_eq!(forward.descriptor_count(), 3);
    assert_eq!(forward.freeze_report().descriptor_count(), 3);
    assert_eq!(forward.freeze_report().id_index_width(), 3);
    assert_eq!(forward.freeze_report().kind_name_index_width(), 3);
    assert_eq!(
        forward.registry_digest().as_str(),
        reversed.registry_digest().as_str()
    );
    assert_eq!(
        forward.freeze_report().registry_digest().as_str(),
        reversed.freeze_report().registry_digest().as_str()
    );
}

#[test]
fn resource_policy_registry_rejects_duplicate_kind_and_semantic_name() {
    let first = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(110),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.same-name"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let second = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(111),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.same-name"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(6),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );

    let err = FrozenResourcePolicyRegistry::new(vec![first, second])
        .expect_err("duplicate policy semantic names must deny registry construction");

    assert_eq!(
        err,
        ResourcePolicyRegistryError::DuplicateName {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("example.resource.retry.same-name")
        }
    );
}

#[test]
fn resource_policy_registry_rejects_malformed_semantic_name() {
    let err = FrozenResourcePolicyRegistry::new(vec![ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(120),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new(" "),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    )])
    .expect_err("blank semantic names must deny registry construction");

    assert_eq!(
        err,
        ResourcePolicyRegistryError::MalformedDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new(" "),
            reason: "resource policy name must not be empty",
        }
    );
}
