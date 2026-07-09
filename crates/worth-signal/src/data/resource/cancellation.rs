use serde::{Deserialize, Serialize};

use crate::data::temporal::TemporalDuration;

use super::lifecycle::ResourceLifecycleTransition;
use super::policy_registry::ResourcePolicyDigest;
use super::request::{ResourceCancellationOrdinal, ResourceRequestHandle, ResourceRequestId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCancellationReason {
    HostRequested,
    RuntimePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCancellationDenialClass {
    UnknownOrStaleRequest,
    NonActiveRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceHostCancellationAdvisory {
    policy_decision_digest: ResourcePolicyDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCancellationGraceWindow {
    duration: TemporalDuration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDependentCancellationPropagation {
    parent: ResourceRequestHandle,
    cancelled_dependents: Vec<CancelledResourceRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelledResourceRequest {
    handle: ResourceRequestHandle,
    cancellation_ordinal: ResourceCancellationOrdinal,
    reason: ResourceCancellationReason,
    policy_decision_digest: ResourcePolicyDigest,
    host_advisory: Option<ResourceHostCancellationAdvisory>,
    grace_window: Option<ResourceCancellationGraceWindow>,
    lifecycle_transition: ResourceLifecycleTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceCancellation {
    request_id: ResourceRequestId,
    class: ResourceCancellationDenialClass,
}

impl DeniedResourceCancellation {
    pub(crate) fn new(
        request_id: ResourceRequestId,
        class: ResourceCancellationDenialClass,
    ) -> Self {
        Self { request_id, class }
    }

    pub fn request_id(self) -> ResourceRequestId {
        self.request_id
    }

    pub fn class(self) -> ResourceCancellationDenialClass {
        self.class
    }
}

impl CancelledResourceRequest {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        cancellation_ordinal: ResourceCancellationOrdinal,
        reason: ResourceCancellationReason,
        policy_decision_digest: ResourcePolicyDigest,
        host_advisory: Option<ResourceHostCancellationAdvisory>,
        grace_window: Option<ResourceCancellationGraceWindow>,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            handle,
            cancellation_ordinal,
            reason,
            policy_decision_digest,
            host_advisory,
            grace_window,
            lifecycle_transition,
        }
    }

    pub fn handle(&self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn cancellation_ordinal(&self) -> ResourceCancellationOrdinal {
        self.cancellation_ordinal
    }

    pub fn reason(&self) -> ResourceCancellationReason {
        self.reason
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }

    pub fn host_advisory(&self) -> Option<&ResourceHostCancellationAdvisory> {
        self.host_advisory.as_ref()
    }

    pub fn grace_window(&self) -> Option<&ResourceCancellationGraceWindow> {
        self.grace_window.as_ref()
    }

    pub fn lifecycle_transition(&self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

impl ResourceHostCancellationAdvisory {
    pub(crate) fn requested(policy_decision_digest: ResourcePolicyDigest) -> Self {
        Self {
            policy_decision_digest,
        }
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }
}

impl ResourceCancellationGraceWindow {
    pub(crate) fn new(duration: TemporalDuration) -> Self {
        Self { duration }
    }

    pub fn duration(&self) -> TemporalDuration {
        self.duration
    }
}

impl ResourceDependentCancellationPropagation {
    pub(crate) fn new(
        parent: ResourceRequestHandle,
        cancelled_dependents: Vec<CancelledResourceRequest>,
    ) -> Self {
        Self {
            parent,
            cancelled_dependents,
        }
    }

    pub fn parent(&self) -> ResourceRequestHandle {
        self.parent
    }

    pub fn cancelled_dependents(&self) -> &[CancelledResourceRequest] {
        &self.cancelled_dependents
    }

    pub fn cancelled_dependent_width(&self) -> u32 {
        self.cancelled_dependents.len() as u32
    }
}
