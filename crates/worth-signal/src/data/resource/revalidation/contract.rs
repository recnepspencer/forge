use serde::{Deserialize, Serialize};

use super::proof::{
    DependencyChangeResourceRevalidationProof, FulfilledLifecycleResourceRevalidationProof,
    ObserverDemandResourceRevalidationProof, TerminalStateResourceRevalidationProof,
};
use crate::data::resource::{ResourceNodeId, ResourcePolicyDigest, ResourceRequestHandle};
use crate::data::temporal::{TemporalDuration, TemporalWakeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRevalidationIntent {
    node: ResourceNodeId,
    expected_active: Option<ResourceRequestHandle>,
    transaction_deadline: Option<TemporalDuration>,
}

impl ResourceRevalidationIntent {
    pub fn new(node: ResourceNodeId) -> Self {
        Self {
            node,
            expected_active: None,
            transaction_deadline: None,
        }
    }

    pub fn with_expected_active(
        node: ResourceNodeId,
        expected_active: ResourceRequestHandle,
    ) -> Self {
        Self {
            node,
            expected_active: Some(expected_active),
            transaction_deadline: None,
        }
    }

    pub fn with_transaction_deadline(node: ResourceNodeId, deadline: TemporalDuration) -> Self {
        Self {
            node,
            expected_active: None,
            transaction_deadline: Some(deadline),
        }
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn expected_active(self) -> Option<ResourceRequestHandle> {
        self.expected_active
    }

    pub fn transaction_deadline(self) -> Option<TemporalDuration> {
        self.transaction_deadline
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRevalidationDenialClass {
    UndeclaredResourceNode,
    ActiveRequestRequiresExpectedHandle,
    ExpectedActiveRequestMismatch,
    ForcedRevalidationPolicyDisabled,
    ActiveHandleProofMismatch,
    DependencyChangeRevalidationPolicyDisabled,
    DependencyChangeProofMismatch,
    ObserverDemandRevalidationPolicyDisabled,
    ObserverDemandProofMismatch,
    TerminalStateRevalidationPolicyDisabled,
    TerminalStateProofMismatch,
    FulfilledLifecycleRevalidationPolicyDisabled,
    FulfilledLifecycleProofMismatch,
    StaleAfterRevalidationPolicyDisabled,
    StaleAfterWakeMismatch,
    StaleAfterRequiresFulfilledLifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRevalidationFreshnessClass {
    ExplicitIntent,
    ForcedActiveHandle,
    DependencyChange,
    ObserverDemand,
    TerminalState,
    FulfilledLifecycle,
    StaleAfter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceRevalidationFreshnessDecision {
    class: ResourceRevalidationFreshnessClass,
    freshness_digest: String,
    policy_decision_digest: ResourcePolicyDigest,
}

impl ResourceRevalidationFreshnessDecision {
    pub(crate) fn explicit_intent(policy_decision_digest: ResourcePolicyDigest) -> Self {
        Self::new(
            ResourceRevalidationFreshnessClass::ExplicitIntent,
            format!(
                "resource-revalidation-freshness:explicit:{}",
                policy_decision_digest.as_str()
            ),
            policy_decision_digest,
        )
    }

    pub(crate) fn forced_active_handle(
        handle: ResourceRequestHandle,
        policy_decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self::new(
            ResourceRevalidationFreshnessClass::ForcedActiveHandle,
            format!(
                "resource-revalidation-freshness:forced-active:{}:{}:{}",
                handle.request_id().get(),
                handle.generation().get(),
                policy_decision_digest.as_str()
            ),
            policy_decision_digest,
        )
    }

    pub(crate) fn dependency_change(proof: &DependencyChangeResourceRevalidationProof) -> Self {
        Self::new(
            ResourceRevalidationFreshnessClass::DependencyChange,
            format!(
                "resource-revalidation-freshness:dependency-change:{}:{:?}:{}",
                proof.node().node().index(),
                proof.node_state(),
                proof.decision_digest().as_str()
            ),
            proof.decision_digest().clone(),
        )
    }

    pub(crate) fn observer_demand(proof: &ObserverDemandResourceRevalidationProof) -> Self {
        Self::new(
            ResourceRevalidationFreshnessClass::ObserverDemand,
            format!(
                "resource-revalidation-freshness:observer-demand:{}:{}:{}:{}",
                proof.node().node().index(),
                proof.observer_id(),
                proof.handle_id(),
                proof.observation_digest()
            ),
            proof.decision_digest().clone(),
        )
    }

    pub(crate) fn terminal_state(proof: &TerminalStateResourceRevalidationProof) -> Self {
        Self::new(
            ResourceRevalidationFreshnessClass::TerminalState,
            format!(
                "resource-revalidation-freshness:terminal-state:{}:{:?}:{}:{}",
                proof.node().node().index(),
                proof.lifecycle(),
                proof.lifecycle_ordinal().get(),
                proof.decision_digest().as_str()
            ),
            proof.decision_digest().clone(),
        )
    }

    pub(crate) fn fulfilled_lifecycle(proof: &FulfilledLifecycleResourceRevalidationProof) -> Self {
        Self::new(
            ResourceRevalidationFreshnessClass::FulfilledLifecycle,
            format!(
                "resource-revalidation-freshness:fulfilled-lifecycle:{}:{}:{}",
                proof.node().node().index(),
                proof.lifecycle_ordinal().get(),
                proof.decision_digest().as_str()
            ),
            proof.decision_digest().clone(),
        )
    }

    pub(crate) fn stale_after(
        node: ResourceNodeId,
        wake_id: TemporalWakeId,
        policy_decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self::new(
            ResourceRevalidationFreshnessClass::StaleAfter,
            format!(
                "resource-revalidation-freshness:stale-after:{}:{}:{}",
                node.node().index(),
                wake_id.get(),
                policy_decision_digest.as_str()
            ),
            policy_decision_digest,
        )
    }

    pub(crate) fn new(
        class: ResourceRevalidationFreshnessClass,
        freshness_digest: String,
        policy_decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            class,
            freshness_digest,
            policy_decision_digest,
        }
    }

    pub fn class(&self) -> ResourceRevalidationFreshnessClass {
        self.class
    }

    pub fn freshness_digest(&self) -> &str {
        &self.freshness_digest
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }
}
