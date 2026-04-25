use serde::{Deserialize, Serialize};

use super::policy::{
    ResourceCancellationPolicyDeclaration, ResourceLifecyclePolicyDeclaration,
    ResourceObservationPolicyDeclaration, ResourceOutputContinuityPolicyDeclaration,
    ResourceRetentionPolicyDeclaration, ResourceRetryPolicyDeclaration,
    ResourceRevalidationPolicyDeclaration, ResourceStaleAfterPolicyDeclaration,
    ResourceSupersessionPolicyDeclaration, ResourceTimeoutPolicyDeclaration,
};
use super::request::ResourceNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePayloadContractId(u64);

impl ResourcePayloadContractId {
    pub const DEFAULT: Self = Self(0);

    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePayloadContract {
    id: ResourcePayloadContractId,
    max_payload_bytes: Option<u64>,
}

impl ResourcePayloadContract {
    pub fn new(id: ResourcePayloadContractId) -> Self {
        Self {
            id,
            max_payload_bytes: None,
        }
    }

    pub fn with_max_payload_bytes(mut self, max_payload_bytes: u64) -> Self {
        self.max_payload_bytes = Some(max_payload_bytes);
        self
    }

    pub fn id(&self) -> ResourcePayloadContractId {
        self.id
    }

    pub fn max_payload_bytes(&self) -> Option<u64> {
        self.max_payload_bytes
    }
}

impl Default for ResourcePayloadContract {
    fn default() -> Self {
        Self::new(ResourcePayloadContractId::DEFAULT)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceNodeDeclaration {
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
    payload_contract: ResourcePayloadContract,
}

impl ResourceNodeDeclaration {
    pub fn new(node: ResourceNodeId, payload_contract: ResourcePayloadContract) -> Self {
        Self {
            node,
            lifecycle_policy: ResourceLifecyclePolicyDeclaration::default(),
            retry_policy: ResourceRetryPolicyDeclaration::default(),
            timeout_policy: ResourceTimeoutPolicyDeclaration::default(),
            cancellation_policy: ResourceCancellationPolicyDeclaration::default(),
            stale_after_policy: ResourceStaleAfterPolicyDeclaration::default(),
            supersession_policy: ResourceSupersessionPolicyDeclaration::default(),
            revalidation_policy: ResourceRevalidationPolicyDeclaration::default(),
            observation_policy: ResourceObservationPolicyDeclaration::default(),
            output_continuity_policy: ResourceOutputContinuityPolicyDeclaration::default(),
            retention_policy: ResourceRetentionPolicyDeclaration::default(),
            payload_contract,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn lifecycle_policy(&self) -> ResourceLifecyclePolicyDeclaration {
        self.lifecycle_policy
    }

    pub fn retry_policy(&self) -> &ResourceRetryPolicyDeclaration {
        &self.retry_policy
    }

    pub fn timeout_policy(&self) -> &ResourceTimeoutPolicyDeclaration {
        &self.timeout_policy
    }

    pub fn cancellation_policy(&self) -> &ResourceCancellationPolicyDeclaration {
        &self.cancellation_policy
    }

    pub fn stale_after_policy(&self) -> &ResourceStaleAfterPolicyDeclaration {
        &self.stale_after_policy
    }

    pub fn supersession_policy(&self) -> &ResourceSupersessionPolicyDeclaration {
        &self.supersession_policy
    }

    pub fn revalidation_policy(&self) -> &ResourceRevalidationPolicyDeclaration {
        &self.revalidation_policy
    }

    pub fn observation_policy(&self) -> &ResourceObservationPolicyDeclaration {
        &self.observation_policy
    }

    pub fn output_continuity_policy(&self) -> &ResourceOutputContinuityPolicyDeclaration {
        &self.output_continuity_policy
    }

    pub fn retention_policy(&self) -> &ResourceRetentionPolicyDeclaration {
        &self.retention_policy
    }

    pub fn payload_contract(&self) -> &ResourcePayloadContract {
        &self.payload_contract
    }

    pub fn with_lifecycle_policy(mut self, policy: ResourceLifecyclePolicyDeclaration) -> Self {
        self.lifecycle_policy = policy;
        self
    }

    pub fn with_retry_policy(mut self, policy: ResourceRetryPolicyDeclaration) -> Self {
        self.retry_policy = policy;
        self
    }

    pub fn with_timeout_policy(mut self, policy: ResourceTimeoutPolicyDeclaration) -> Self {
        self.timeout_policy = policy;
        self
    }

    pub fn with_cancellation_policy(
        mut self,
        policy: ResourceCancellationPolicyDeclaration,
    ) -> Self {
        self.cancellation_policy = policy;
        self
    }

    pub fn with_stale_after_policy(mut self, policy: ResourceStaleAfterPolicyDeclaration) -> Self {
        self.stale_after_policy = policy;
        self
    }

    pub fn with_supersession_policy(
        mut self,
        policy: ResourceSupersessionPolicyDeclaration,
    ) -> Self {
        self.supersession_policy = policy;
        self
    }

    pub fn with_revalidation_policy(
        mut self,
        policy: ResourceRevalidationPolicyDeclaration,
    ) -> Self {
        self.revalidation_policy = policy;
        self
    }

    pub fn with_observation_policy(mut self, policy: ResourceObservationPolicyDeclaration) -> Self {
        self.observation_policy = policy;
        self
    }

    pub fn with_output_continuity_policy(
        mut self,
        policy: ResourceOutputContinuityPolicyDeclaration,
    ) -> Self {
        self.output_continuity_policy = policy;
        self
    }

    pub fn with_retention_policy(mut self, policy: ResourceRetentionPolicyDeclaration) -> Self {
        self.retention_policy = policy;
        self
    }
}
