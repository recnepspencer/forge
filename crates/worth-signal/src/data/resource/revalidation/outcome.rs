use serde::{Deserialize, Serialize};

use super::contract::{ResourceRevalidationDenialClass, ResourceRevalidationFreshnessDecision};
use super::proof::{
    ActiveResourceRevalidationProof, DependencyChangeResourceRevalidationProof,
    FulfilledLifecycleResourceRevalidationProof, ObserverDemandResourceRevalidationProof,
    TerminalStateResourceRevalidationProof,
};
use crate::data::resource::{
    AdmittedResourceRequest, ResourceLifecycleTransition, ResourceNodeId, ResourcePolicyDigest,
    ResourceRequestHandle, ResourceRequestId, ResourceSupersessionRecord,
};
use crate::data::temporal::ReadyTemporalWake;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ResourceRevalidationEvidence {
    ExplicitIntent {
        expected_active: Option<ResourceRequestHandle>,
    },
    ForcedActive(ActiveResourceRevalidationProof),
    DependencyChange(DependencyChangeResourceRevalidationProof),
    ObserverDemand(ObserverDemandResourceRevalidationProof),
    TerminalState(TerminalStateResourceRevalidationProof),
    FulfilledLifecycle(FulfilledLifecycleResourceRevalidationProof),
    StaleAfter(ReadyTemporalWake),
}

impl ResourceRevalidationEvidence {
    pub fn expected_active(&self) -> Option<ResourceRequestHandle> {
        match self {
            Self::ExplicitIntent { expected_active } => *expected_active,
            Self::ForcedActive(proof) => Some(proof.handle()),
            Self::DependencyChange(_)
            | Self::ObserverDemand(_)
            | Self::TerminalState(_)
            | Self::FulfilledLifecycle(_)
            | Self::StaleAfter(_) => None,
        }
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
pub struct AdmittedResourceRevalidation {
    admitted_request: AdmittedResourceRequest,
    freshness_decision: ResourceRevalidationFreshnessDecision,
    evidence: ResourceRevalidationEvidence,
    coalescing: Option<ResourceRevalidationCoalescing>,
    supersession_record: Option<ResourceSupersessionRecord>,
    decision_digest: ResourcePolicyDigest,
}

impl AdmittedResourceRevalidation {
    pub(crate) fn new(
        admitted_request: AdmittedResourceRequest,
        freshness_decision: ResourceRevalidationFreshnessDecision,
        evidence: ResourceRevalidationEvidence,
        coalescing: Option<ResourceRevalidationCoalescing>,
        supersession_record: Option<ResourceSupersessionRecord>,
        decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            admitted_request,
            freshness_decision,
            evidence,
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
        self.evidence.expected_active()
    }

    pub fn evidence(&self) -> &ResourceRevalidationEvidence {
        &self.evidence
    }

    pub fn forced_active_handle(&self) -> Option<ResourceRequestHandle> {
        match &self.evidence {
            ResourceRevalidationEvidence::ForcedActive(proof) => Some(proof.handle()),
            _ => None,
        }
    }

    pub fn dependency_change_proof(&self) -> Option<&DependencyChangeResourceRevalidationProof> {
        match &self.evidence {
            ResourceRevalidationEvidence::DependencyChange(proof) => Some(proof),
            _ => None,
        }
    }

    pub fn observer_demand_proof(&self) -> Option<&ObserverDemandResourceRevalidationProof> {
        match &self.evidence {
            ResourceRevalidationEvidence::ObserverDemand(proof) => Some(proof),
            _ => None,
        }
    }

    pub fn terminal_state_proof(&self) -> Option<&TerminalStateResourceRevalidationProof> {
        match &self.evidence {
            ResourceRevalidationEvidence::TerminalState(proof) => Some(proof),
            _ => None,
        }
    }

    pub fn fulfilled_lifecycle_proof(
        &self,
    ) -> Option<&FulfilledLifecycleResourceRevalidationProof> {
        match &self.evidence {
            ResourceRevalidationEvidence::FulfilledLifecycle(proof) => Some(proof),
            _ => None,
        }
    }

    pub fn stale_after_ready_wake(&self) -> Option<&ReadyTemporalWake> {
        match &self.evidence {
            ResourceRevalidationEvidence::StaleAfter(wake) => Some(wake),
            _ => None,
        }
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
