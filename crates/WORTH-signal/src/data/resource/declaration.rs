use serde::{Deserialize, Serialize};

use crate::data::temporal::TemporalDuration;

use super::policy::{
    ResourceCancellationPolicyDeclaration, ResourceDiagnosticsPolicyDeclaration,
    ResourceLifecyclePolicyDeclaration, ResourceObservationPolicyDeclaration,
    ResourceOutputContinuityPolicyDeclaration, ResourceReplayPolicyDeclaration,
    ResourceRetentionPolicyDeclaration, ResourceRetryBudgetScope, ResourceRetryPolicyDeclaration,
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
    diagnostics_policy: ResourceDiagnosticsPolicyDeclaration,
    replay_policy: ResourceReplayPolicyDeclaration,
    cancellation_grace_period: Option<TemporalDuration>,
    declared_dependent_cancellation_nodes: Vec<ResourceNodeId>,
    retry_max_attempts: Option<u32>,
    retry_deterministic_jitter: Option<TemporalDuration>,
    retry_budget_scope: Option<ResourceRetryBudgetScope>,
    retry_budget_limit: Option<u32>,
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
            diagnostics_policy: ResourceDiagnosticsPolicyDeclaration::default(),
            replay_policy: ResourceReplayPolicyDeclaration::default(),
            cancellation_grace_period: None,
            declared_dependent_cancellation_nodes: Vec::new(),
            retry_max_attempts: None,
            retry_deterministic_jitter: None,
            retry_budget_scope: None,
            retry_budget_limit: None,
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

    pub fn diagnostics_policy(&self) -> &ResourceDiagnosticsPolicyDeclaration {
        &self.diagnostics_policy
    }

    pub fn replay_policy(&self) -> &ResourceReplayPolicyDeclaration {
        &self.replay_policy
    }

    pub fn payload_contract(&self) -> &ResourcePayloadContract {
        &self.payload_contract
    }

    pub fn cancellation_grace_period(&self) -> Option<TemporalDuration> {
        self.cancellation_grace_period
    }

    pub fn declared_dependent_cancellation_nodes(&self) -> &[ResourceNodeId] {
        &self.declared_dependent_cancellation_nodes
    }

    pub fn retry_max_attempts(&self) -> Option<u32> {
        self.retry_max_attempts
    }

    pub fn retry_deterministic_jitter(&self) -> Option<TemporalDuration> {
        self.retry_deterministic_jitter
    }

    pub fn retry_budget_scope(&self) -> Option<ResourceRetryBudgetScope> {
        self.retry_budget_scope
    }

    pub fn retry_budget_limit(&self) -> Option<u32> {
        self.retry_budget_limit
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

    pub fn with_diagnostics_policy(mut self, policy: ResourceDiagnosticsPolicyDeclaration) -> Self {
        self.diagnostics_policy = policy;
        self
    }

    pub fn with_replay_policy(mut self, policy: ResourceReplayPolicyDeclaration) -> Self {
        self.replay_policy = policy;
        self
    }

    pub fn with_cancellation_grace_period(mut self, grace_period: TemporalDuration) -> Self {
        self.cancellation_grace_period = Some(grace_period);
        self
    }

    pub fn with_declared_dependent_cancellation_nodes(
        mut self,
        nodes: impl IntoIterator<Item = ResourceNodeId>,
    ) -> Self {
        self.declared_dependent_cancellation_nodes = nodes.into_iter().collect();
        self
    }

    pub fn with_retry_max_attempts(mut self, max_attempts: u32) -> Self {
        self.retry_max_attempts = Some(max_attempts);
        self
    }

    pub fn with_retry_deterministic_jitter(mut self, max_jitter: TemporalDuration) -> Self {
        self.retry_deterministic_jitter = Some(max_jitter);
        self
    }

    pub fn with_retry_budget(
        mut self,
        scope: ResourceRetryBudgetScope,
        retry_budget_limit: u32,
    ) -> Self {
        self.retry_budget_scope = Some(scope);
        self.retry_budget_limit = Some(retry_budget_limit);
        self
    }
}
