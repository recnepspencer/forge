use super::super::policy::ResourcePolicyName;
use super::super::summary::ResourceCostContractId;
use super::descriptor::ResourcePolicyDescriptor;
use super::identity::{
    ResourcePolicyDescriptorId, ResourcePolicyDigest, ResourcePolicyKind,
    ResourcePolicySelectionBasis, ResourcePolicyVersion,
};
use super::reference::FrozenResourcePolicyDescriptor;

pub(super) fn descriptor_digest(
    id: ResourcePolicyDescriptorId,
    kind: ResourcePolicyKind,
    semantic_name: &ResourcePolicyName,
    version: ResourcePolicyVersion,
    cost_contract: ResourceCostContractId,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "resource-policy-descriptor:{}:{}:{}:{}.{}:{}",
        id.get(),
        kind.as_str(),
        semantic_name.as_str(),
        version.major(),
        version.minor(),
        cost_contract.get()
    ))
}

pub(super) fn registry_digest(descriptors: &[ResourcePolicyDescriptor]) -> ResourcePolicyDigest {
    let mut rows = descriptors
        .iter()
        .map(|descriptor| {
            format!(
                "{}:{}:{}:{}",
                descriptor.id().get(),
                descriptor.kind().as_str(),
                descriptor.semantic_name().as_str(),
                descriptor.descriptor_digest().as_str()
            )
        })
        .collect::<Vec<_>>();
    rows.sort();
    ResourcePolicyDigest::new(format!("resource-policy-registry:{}", rows.join("|")))
}

pub(super) fn bundle_digest(policies: &[&FrozenResourcePolicyDescriptor]) -> ResourcePolicyDigest {
    let joined = policies
        .iter()
        .map(|policy| policy.frozen_digest().as_str())
        .collect::<Vec<_>>()
        .join("|");
    ResourcePolicyDigest::new(format!("resource-policy-bundle:{joined}"))
}

pub(super) fn frozen_policy_descriptor_digest(
    descriptor: &ResourcePolicyDescriptor,
    selection_basis: ResourcePolicySelectionBasis,
    parameter_digest: &ResourcePolicyDigest,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "frozen-resource-policy:{}:{}:{}",
        descriptor.descriptor_digest().as_str(),
        selection_basis.as_str(),
        parameter_digest.as_str()
    ))
}
