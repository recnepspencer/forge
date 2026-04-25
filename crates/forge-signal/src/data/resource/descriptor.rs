use serde::{Deserialize, Serialize};

use super::declaration::{ResourceNodeDeclaration, ResourcePayloadContractId};
use super::policy::{
    ResourceCancellationPolicyDeclaration, ResourceLifecyclePolicyDeclaration,
    ResourceObservationPolicyDeclaration, ResourceOutputContinuityPolicyDeclaration,
    ResourceRetentionPolicyDeclaration, ResourceRetryPolicyDeclaration,
    ResourceRevalidationPolicyDeclaration, ResourceStaleAfterPolicyDeclaration,
    ResourceSupersessionPolicyDeclaration, ResourceTimeoutPolicyDeclaration,
};
use super::policy_registry::ResourceResolvedPolicyBundle;
use super::request::ResourceNodeId;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceDescriptorId(u64);

impl ResourceDescriptorId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceDescriptorVersion(u64);

impl ResourceDescriptorVersion {
    pub const INITIAL: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePayloadContractDigest(String);

impl ResourcePayloadContractDigest {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub(crate) fn from_contract(
        id: ResourcePayloadContractId,
        max_payload_bytes: Option<u64>,
    ) -> Self {
        Self(format!(
            "payload-contract:{}:{}",
            id.get(),
            max_payload_bytes
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unbounded".to_owned())
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredResourceDescriptor {
    descriptor_id: ResourceDescriptorId,
    descriptor_version: ResourceDescriptorVersion,
    node: ResourceNodeId,
    lifecycle_policy: ResourceLifecyclePolicyDeclaration,
    retry_policy: ResourceRetryPolicyDeclaration,
    timeout_policy: ResourceTimeoutPolicyDeclaration,
    cancellation_policy: ResourceCancellationPolicyDeclaration,
    stale_after_policy: ResourceStaleAfterPolicyDeclaration,
    supersession_policy: ResourceSupersessionPolicyDeclaration,
    revalidation_policy: ResourceRevalidationPolicyDeclaration,
    observation_policy: ResourceObservationPolicyDeclaration,
    output_continuity_policy: ResourceOutputContinuityPolicyDeclaration,
    retention_policy: ResourceRetentionPolicyDeclaration,
    resolved_policy_bundle: ResourceResolvedPolicyBundle,
    payload_contract_digest: ResourcePayloadContractDigest,
    max_payload_bytes: Option<u64>,
}

impl LoweredResourceDescriptor {
    pub(crate) fn from_declaration(
        descriptor_id: ResourceDescriptorId,
        descriptor_version: ResourceDescriptorVersion,
        declaration: &ResourceNodeDeclaration,
        resolved_policy_bundle: ResourceResolvedPolicyBundle,
    ) -> Self {
        Self {
            descriptor_id,
            descriptor_version,
            node: declaration.node(),
            lifecycle_policy: declaration.lifecycle_policy(),
            retry_policy: declaration.retry_policy().clone(),
            timeout_policy: declaration.timeout_policy().clone(),
            cancellation_policy: declaration.cancellation_policy().clone(),
            stale_after_policy: declaration.stale_after_policy().clone(),
            supersession_policy: declaration.supersession_policy().clone(),
            revalidation_policy: declaration.revalidation_policy().clone(),
            observation_policy: declaration.observation_policy().clone(),
            output_continuity_policy: declaration.output_continuity_policy().clone(),
            retention_policy: declaration.retention_policy().clone(),
            resolved_policy_bundle,
            payload_contract_digest: ResourcePayloadContractDigest::from_contract(
                declaration.payload_contract().id(),
                declaration.payload_contract().max_payload_bytes(),
            ),
            max_payload_bytes: declaration.payload_contract().max_payload_bytes(),
        }
    }

    pub fn descriptor_id(&self) -> ResourceDescriptorId {
        self.descriptor_id
    }

    pub fn descriptor_version(&self) -> ResourceDescriptorVersion {
        self.descriptor_version
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }

    pub fn resolved_policy_bundle(&self) -> &ResourceResolvedPolicyBundle {
        &self.resolved_policy_bundle
    }

    pub fn max_payload_bytes(&self) -> Option<u64> {
        self.max_payload_bytes
    }

    pub fn timeout_policy(&self) -> &ResourceTimeoutPolicyDeclaration {
        &self.timeout_policy
    }

    pub fn retry_policy(&self) -> &ResourceRetryPolicyDeclaration {
        &self.retry_policy
    }
}
