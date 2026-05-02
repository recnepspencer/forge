use serde::{Deserialize, Serialize};

use super::lifecycle::ResourceLifecycleTransition;
use super::policy_registry::ResourcePolicyDigest;
use super::proof::AdmittedResourceRequest;
use super::request::{
    ResourceRequestHandle, ResourceRequestIntentDigest, ResourceSupersessionOrdinal,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceOldHostWorkCancellationAdvisory {
    policy_decision_digest: ResourcePolicyDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceOverlappingGenerationAdmission {
    previous: ResourceRequestHandle,
    replacing: ResourceRequestHandle,
    policy_decision_digest: ResourcePolicyDigest,
    old_host_work_cancellation_advisory: Option<ResourceOldHostWorkCancellationAdvisory>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSupersessionRecord {
    supersession_ordinal: ResourceSupersessionOrdinal,
    previous: ResourceRequestHandle,
    replacing: ResourceRequestHandle,
    policy_decision_digest: ResourcePolicyDigest,
    overlap_admission: Option<ResourceOverlappingGenerationAdmission>,
    lifecycle_transition: ResourceLifecycleTransition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceIntentEquivalenceCoalescing {
    supersession_ordinal: ResourceSupersessionOrdinal,
    winner: ResourceRequestHandle,
    coalesced_request: AdmittedResourceRequest,
    intent_digest: ResourceRequestIntentDigest,
    policy_decision_digest: ResourcePolicyDigest,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl ResourceSupersessionRecord {
    pub(crate) fn new(
        supersession_ordinal: ResourceSupersessionOrdinal,
        previous: ResourceRequestHandle,
        replacing: ResourceRequestHandle,
        policy_decision_digest: ResourcePolicyDigest,
        overlap_admission: Option<ResourceOverlappingGenerationAdmission>,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            supersession_ordinal,
            previous,
            replacing,
            policy_decision_digest,
            overlap_admission,
            lifecycle_transition,
        }
    }

    pub fn supersession_ordinal(&self) -> ResourceSupersessionOrdinal {
        self.supersession_ordinal
    }

    pub fn previous(&self) -> ResourceRequestHandle {
        self.previous
    }

    pub fn replacing(&self) -> ResourceRequestHandle {
        self.replacing
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }

    pub fn overlap_admission(&self) -> Option<&ResourceOverlappingGenerationAdmission> {
        self.overlap_admission.as_ref()
    }

    pub fn lifecycle_transition(&self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

impl ResourceOverlappingGenerationAdmission {
    pub(crate) fn new(
        previous: ResourceRequestHandle,
        replacing: ResourceRequestHandle,
        policy_decision_digest: ResourcePolicyDigest,
        old_host_work_cancellation_advisory: Option<ResourceOldHostWorkCancellationAdvisory>,
    ) -> Self {
        Self {
            previous,
            replacing,
            policy_decision_digest,
            old_host_work_cancellation_advisory,
        }
    }

    pub fn previous(&self) -> ResourceRequestHandle {
        self.previous
    }

    pub fn replacing(&self) -> ResourceRequestHandle {
        self.replacing
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }

    pub fn old_host_work_cancellation_advisory(
        &self,
    ) -> Option<&ResourceOldHostWorkCancellationAdvisory> {
        self.old_host_work_cancellation_advisory.as_ref()
    }
}

impl ResourceIntentEquivalenceCoalescing {
    pub(crate) fn new(
        supersession_ordinal: ResourceSupersessionOrdinal,
        winner: ResourceRequestHandle,
        coalesced_request: AdmittedResourceRequest,
        intent_digest: ResourceRequestIntentDigest,
        policy_decision_digest: ResourcePolicyDigest,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            supersession_ordinal,
            winner,
            coalesced_request,
            intent_digest,
            policy_decision_digest,
            lifecycle_transition,
        }
    }

    pub fn supersession_ordinal(&self) -> ResourceSupersessionOrdinal {
        self.supersession_ordinal
    }

    pub fn winner(&self) -> ResourceRequestHandle {
        self.winner
    }

    pub fn coalesced_request(&self) -> AdmittedResourceRequest {
        self.coalesced_request
    }

    pub fn intent_digest(&self) -> &ResourceRequestIntentDigest {
        &self.intent_digest
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }

    pub fn lifecycle_transition(&self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

impl ResourceOldHostWorkCancellationAdvisory {
    pub(crate) fn requested(policy_decision_digest: ResourcePolicyDigest) -> Self {
        Self {
            policy_decision_digest,
        }
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }
}
