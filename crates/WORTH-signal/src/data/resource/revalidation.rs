use serde::{Deserialize, Serialize};

use super::lifecycle::ResourceLifecycleTransition;
use super::lifecycle::{ResourceLifecycleClass, ResourceLifecycleOrdinal};
use super::policy_registry::ResourcePolicyDigest;
use super::proof::AdmittedResourceRequest;
use super::request::{ResourceNodeId, ResourceRequestHandle, ResourceRequestId};
use super::supersession::ResourceSupersessionRecord;
use crate::data::node::NodeState;
use crate::data::temporal::{ReadyTemporalWake, TemporalDuration, TemporalWakeId};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceRevalidationCoalescing {
    winner: ResourceRequestHandle,
    coalesced_request: AdmittedResourceRequest,
    freshness_decision: ResourceRevalidationFreshnessDecision,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl ResourceRevalidationCoalescing {
    pub(crate) fn new(
        winner: ResourceRequestHandle,
        coalesced_request: AdmittedResourceRequest,
        freshness_decision: ResourceRevalidationFreshnessDecision,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            winner,
            coalesced_request,
            freshness_decision,
            lifecycle_transition,
        }
    }

    pub fn winner(&self) -> ResourceRequestHandle {
        self.winner
    }

    pub fn coalesced_request(&self) -> AdmittedResourceRequest {
        self.coalesced_request
    }

    pub fn freshness_decision(&self) -> &ResourceRevalidationFreshnessDecision {
        &self.freshness_decision
    }

    pub fn lifecycle_transition(&self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActiveResourceRevalidationProof {
    node: ResourceNodeId,
    handle: ResourceRequestHandle,
    decision_digest: ResourcePolicyDigest,
}

impl ActiveResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        handle: ResourceRequestHandle,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            handle,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn handle(&self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DependencyChangeResourceRevalidationProof {
    node: ResourceNodeId,
    node_state: NodeState,
    decision_digest: ResourcePolicyDigest,
}

impl DependencyChangeResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        node_state: NodeState,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            node_state,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn node_state(&self) -> NodeState {
        self.node_state
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObserverDemandResourceRevalidationProof {
    node: ResourceNodeId,
    observer_id: u64,
    handle_id: u64,
    observation_digest: String,
    decision_digest: ResourcePolicyDigest,
}

impl ObserverDemandResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        observer_id: u64,
        handle_id: u64,
        observation_digest: String,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            observer_id,
            handle_id,
            observation_digest,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn observer_id(&self) -> u64 {
        self.observer_id
    }

    pub fn handle_id(&self) -> u64 {
        self.handle_id
    }

    pub fn observation_digest(&self) -> &str {
        &self.observation_digest
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalStateResourceRevalidationProof {
    node: ResourceNodeId,
    lifecycle: ResourceLifecycleClass,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    decision_digest: ResourcePolicyDigest,
}

impl TerminalStateResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        lifecycle: ResourceLifecycleClass,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            lifecycle,
            lifecycle_ordinal,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn lifecycle(&self) -> ResourceLifecycleClass {
        self.lifecycle
    }

    pub fn lifecycle_ordinal(&self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FulfilledLifecycleResourceRevalidationProof {
    node: ResourceNodeId,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    decision_digest: ResourcePolicyDigest,
}

impl FulfilledLifecycleResourceRevalidationProof {
    pub(crate) fn new(
        node: ResourceNodeId,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            node,
            lifecycle_ordinal,
            decision_digest,
        }
    }

    pub fn node(&self) -> ResourceNodeId {
        self.node
    }

    pub fn lifecycle_ordinal(&self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedResourceRevalidation {
    admitted_request: AdmittedResourceRequest,
    freshness_decision: ResourceRevalidationFreshnessDecision,
    expected_active: Option<ResourceRequestHandle>,
    forced_active_handle: Option<ResourceRequestHandle>,
    dependency_change_proof: Option<DependencyChangeResourceRevalidationProof>,
    observer_demand_proof: Option<ObserverDemandResourceRevalidationProof>,
    terminal_state_proof: Option<TerminalStateResourceRevalidationProof>,
    fulfilled_lifecycle_proof: Option<FulfilledLifecycleResourceRevalidationProof>,
    stale_after_ready_wake: Option<ReadyTemporalWake>,
    coalescing: Option<ResourceRevalidationCoalescing>,
    supersession_record: Option<ResourceSupersessionRecord>,
    decision_digest: ResourcePolicyDigest,
}

impl AdmittedResourceRevalidation {
    pub(crate) fn new(
        admitted_request: AdmittedResourceRequest,
        freshness_decision: ResourceRevalidationFreshnessDecision,
        expected_active: Option<ResourceRequestHandle>,
        forced_active_handle: Option<ResourceRequestHandle>,
        dependency_change_proof: Option<DependencyChangeResourceRevalidationProof>,
        observer_demand_proof: Option<ObserverDemandResourceRevalidationProof>,
        terminal_state_proof: Option<TerminalStateResourceRevalidationProof>,
        fulfilled_lifecycle_proof: Option<FulfilledLifecycleResourceRevalidationProof>,
        stale_after_ready_wake: Option<ReadyTemporalWake>,
        coalescing: Option<ResourceRevalidationCoalescing>,
        supersession_record: Option<ResourceSupersessionRecord>,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            admitted_request,
            freshness_decision,
            expected_active,
            forced_active_handle,
            dependency_change_proof,
            observer_demand_proof,
            terminal_state_proof,
            fulfilled_lifecycle_proof,
            stale_after_ready_wake,
            coalescing,
            supersession_record,
            decision_digest,
        }
    }

    pub fn admitted_request(&self) -> AdmittedResourceRequest {
        self.admitted_request
    }

    pub fn freshness_decision(&self) -> &ResourceRevalidationFreshnessDecision {
        &self.freshness_decision
    }

    pub fn expected_active(&self) -> Option<ResourceRequestHandle> {
        self.expected_active
    }

    pub fn forced_active_handle(&self) -> Option<ResourceRequestHandle> {
        self.forced_active_handle
    }

    pub fn dependency_change_proof(&self) -> Option<&DependencyChangeResourceRevalidationProof> {
        self.dependency_change_proof.as_ref()
    }

    pub fn observer_demand_proof(&self) -> Option<&ObserverDemandResourceRevalidationProof> {
        self.observer_demand_proof.as_ref()
    }

    pub fn terminal_state_proof(&self) -> Option<&TerminalStateResourceRevalidationProof> {
        self.terminal_state_proof.as_ref()
    }

    pub fn fulfilled_lifecycle_proof(
        &self,
    ) -> Option<&FulfilledLifecycleResourceRevalidationProof> {
        self.fulfilled_lifecycle_proof.as_ref()
    }

    pub fn stale_after_ready_wake(&self) -> Option<&ReadyTemporalWake> {
        self.stale_after_ready_wake.as_ref()
    }

    pub fn coalescing(&self) -> Option<&ResourceRevalidationCoalescing> {
        self.coalescing.as_ref()
    }

    pub fn supersession_record(&self) -> Option<ResourceSupersessionRecord> {
        self.supersession_record.clone()
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceRevalidation {
    node: ResourceNodeId,
    request_id: Option<ResourceRequestId>,
    class: ResourceRevalidationDenialClass,
}

impl DeniedResourceRevalidation {
    pub(crate) fn new(
        node: ResourceNodeId,
        request_id: Option<ResourceRequestId>,
        class: ResourceRevalidationDenialClass,
    ) -> Self {
        Self {
            node,
            request_id,
            class,
        }
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn request_id(self) -> Option<ResourceRequestId> {
        self.request_id
    }

    pub fn class(self) -> ResourceRevalidationDenialClass {
        self.class
    }
}
