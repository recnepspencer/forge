use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourceCancellationPolicyDeclaration, ResourceDiagnosticsPolicyDeclaration,
    ResourceLifecyclePolicyDeclaration, ResourceNodeDeclaration, ResourceNodeId,
    ResourceObservationPolicyDeclaration, ResourceOutputContinuityPolicyDeclaration,
    ResourceReplayPolicyDeclaration, ResourceRetentionPolicyDeclaration, ResourceRetryBudgetScope,
    ResourceRetryPolicyDeclaration, ResourceRevalidationPolicyDeclaration,
    ResourceStaleAfterPolicyDeclaration, ResourceSupersessionPolicyDeclaration,
    ResourceTimeoutPolicyDeclaration, ValidatedResourcePolicyDeclaration,
};
use crate::data::temporal::TemporalDuration;

use super::payload::AsyncNodePayloadContract;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncNodeCapabilityDeclaration {
    inner: ResourceNodeDeclaration,
}

impl AsyncNodeCapabilityDeclaration {
    pub fn new(node: NodeId, payload_contract: AsyncNodePayloadContract) -> Self {
        Self {
            inner: ResourceNodeDeclaration::new(
                ResourceNodeId::from_node(node),
                payload_contract.as_resource().clone(),
            ),
        }
    }

    pub fn node(&self) -> NodeId {
        self.inner.node().node()
    }

    pub fn lifecycle_policy(&self) -> ResourceLifecyclePolicyDeclaration {
        self.inner.lifecycle_policy()
    }

    pub fn retry_policy(&self) -> &ResourceRetryPolicyDeclaration {
        self.inner.retry_policy()
    }

    pub fn timeout_policy(&self) -> &ResourceTimeoutPolicyDeclaration {
        self.inner.timeout_policy()
    }

    pub fn cancellation_policy(&self) -> &ResourceCancellationPolicyDeclaration {
        self.inner.cancellation_policy()
    }

    pub fn stale_after_policy(&self) -> &ResourceStaleAfterPolicyDeclaration {
        self.inner.stale_after_policy()
    }

    pub fn supersession_policy(&self) -> &ResourceSupersessionPolicyDeclaration {
        self.inner.supersession_policy()
    }

    pub fn revalidation_policy(&self) -> &ResourceRevalidationPolicyDeclaration {
        self.inner.revalidation_policy()
    }

    pub fn observation_policy(&self) -> &ResourceObservationPolicyDeclaration {
        self.inner.observation_policy()
    }

    pub fn output_continuity_policy(&self) -> &ResourceOutputContinuityPolicyDeclaration {
        self.inner.output_continuity_policy()
    }

    pub fn retention_policy(&self) -> &ResourceRetentionPolicyDeclaration {
        self.inner.retention_policy()
    }

    pub fn diagnostics_policy(&self) -> &ResourceDiagnosticsPolicyDeclaration {
        self.inner.diagnostics_policy()
    }

    pub fn replay_policy(&self) -> &ResourceReplayPolicyDeclaration {
        self.inner.replay_policy()
    }

    pub fn payload_contract(&self) -> AsyncNodePayloadContract {
        AsyncNodePayloadContract::from_resource(self.inner.payload_contract().clone())
    }

    pub fn cancellation_grace_period(&self) -> Option<TemporalDuration> {
        self.inner.cancellation_grace_period()
    }

    pub fn declared_dependent_cancellation_nodes(&self) -> Vec<NodeId> {
        self.inner
            .declared_dependent_cancellation_nodes()
            .iter()
            .map(|node| node.node())
            .collect()
    }

    pub fn retry_max_attempts(&self) -> Option<u32> {
        self.inner.retry_max_attempts()
    }

    pub fn retry_deterministic_jitter(&self) -> Option<TemporalDuration> {
        self.inner.retry_deterministic_jitter()
    }

    pub fn retry_budget_scope(&self) -> Option<ResourceRetryBudgetScope> {
        self.inner.retry_budget_scope()
    }

    pub fn retry_budget_limit(&self) -> Option<u32> {
        self.inner.retry_budget_limit()
    }

    pub fn with_lifecycle_policy(mut self, policy: ResourceLifecyclePolicyDeclaration) -> Self {
        self.inner = self.inner.with_lifecycle_policy(policy);
        self
    }

    pub fn with_retry_policy(mut self, policy: ResourceRetryPolicyDeclaration) -> Self {
        self.inner = self.inner.with_retry_policy(policy);
        self
    }

    pub fn with_timeout_policy(mut self, policy: ResourceTimeoutPolicyDeclaration) -> Self {
        self.inner = self.inner.with_timeout_policy(policy);
        self
    }

    pub fn with_cancellation_policy(
        mut self,
        policy: ResourceCancellationPolicyDeclaration,
    ) -> Self {
        self.inner = self.inner.with_cancellation_policy(policy);
        self
    }

    pub fn with_stale_after_policy(mut self, policy: ResourceStaleAfterPolicyDeclaration) -> Self {
        self.inner = self.inner.with_stale_after_policy(policy);
        self
    }

    pub fn with_supersession_policy(
        mut self,
        policy: ResourceSupersessionPolicyDeclaration,
    ) -> Self {
        self.inner = self.inner.with_supersession_policy(policy);
        self
    }

    pub fn with_revalidation_policy(
        mut self,
        policy: ResourceRevalidationPolicyDeclaration,
    ) -> Self {
        self.inner = self.inner.with_revalidation_policy(policy);
        self
    }

    pub fn with_observation_policy(mut self, policy: ResourceObservationPolicyDeclaration) -> Self {
        self.inner = self.inner.with_observation_policy(policy);
        self
    }

    pub fn with_output_continuity_policy(
        mut self,
        policy: ResourceOutputContinuityPolicyDeclaration,
    ) -> Self {
        self.inner = self.inner.with_output_continuity_policy(policy);
        self
    }

    pub fn with_retention_policy(mut self, policy: ResourceRetentionPolicyDeclaration) -> Self {
        self.inner = self.inner.with_retention_policy(policy);
        self
    }

    pub fn with_diagnostics_policy(mut self, policy: ResourceDiagnosticsPolicyDeclaration) -> Self {
        self.inner = self.inner.with_diagnostics_policy(policy);
        self
    }

    pub fn with_replay_policy(mut self, policy: ResourceReplayPolicyDeclaration) -> Self {
        self.inner = self.inner.with_replay_policy(policy);
        self
    }

    pub fn with_cancellation_grace_period(mut self, grace_period: TemporalDuration) -> Self {
        self.inner = self.inner.with_cancellation_grace_period(grace_period);
        self
    }

    pub fn with_declared_dependent_cancellation_nodes(
        mut self,
        nodes: impl IntoIterator<Item = NodeId>,
    ) -> Self {
        self.inner = self.inner.with_declared_dependent_cancellation_nodes(
            nodes.into_iter().map(ResourceNodeId::from_node),
        );
        self
    }

    pub fn with_retry_max_attempts(mut self, max_attempts: u32) -> Self {
        self.inner = self.inner.with_retry_max_attempts(max_attempts);
        self
    }

    pub fn with_retry_deterministic_jitter(mut self, max_jitter: TemporalDuration) -> Self {
        self.inner = self.inner.with_retry_deterministic_jitter(max_jitter);
        self
    }

    pub fn with_retry_budget(
        mut self,
        scope: ResourceRetryBudgetScope,
        retry_budget_limit: u32,
    ) -> Self {
        self.inner = self.inner.with_retry_budget(scope, retry_budget_limit);
        self
    }

    pub fn into_legacy_resource_declaration(self) -> ResourceNodeDeclaration {
        self.inner
    }

    pub fn from_legacy_resource_declaration(inner: ResourceNodeDeclaration) -> Self {
        Self { inner }
    }

    pub(crate) fn as_resource_declaration(&self) -> &ResourceNodeDeclaration {
        &self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedAsyncNodeCapabilityDeclaration {
    declaration: AsyncNodeCapabilityDeclaration,
    validated: ValidatedResourcePolicyDeclaration,
}

impl ValidatedAsyncNodeCapabilityDeclaration {
    pub(crate) fn new(
        declaration: AsyncNodeCapabilityDeclaration,
        validated: ValidatedResourcePolicyDeclaration,
    ) -> Self {
        Self {
            declaration,
            validated,
        }
    }

    pub fn declaration(&self) -> &AsyncNodeCapabilityDeclaration {
        &self.declaration
    }

    pub fn node(&self) -> NodeId {
        self.declaration.node()
    }

    pub fn registry_digest(&self) -> &crate::data::resource::ResourcePolicyDigest {
        self.validated.registry_digest()
    }

    pub(crate) fn validated(&self) -> &ValidatedResourcePolicyDeclaration {
        &self.validated
    }
}
