use serde::{Deserialize, Serialize};

use super::request::{
    ResourceNodeId, ResourceRejectionOrdinal, ResourceRequestHandle, ResourceRequestId,
};
use super::{ResourceLifecycleTransition, ResourcePolicyDigest};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRejectionReason {
    HostFailure,
    SemanticFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRejectionDenialClass {
    UnknownOrStaleRequest,
    NonActiveRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedResourceRequest {
    handle: ResourceRequestHandle,
    node: ResourceNodeId,
    rejection_ordinal: ResourceRejectionOrdinal,
    reason: ResourceRejectionReason,
    rejection_digest: ResourcePolicyDigest,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl RejectedResourceRequest {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        node: ResourceNodeId,
        rejection_ordinal: ResourceRejectionOrdinal,
        reason: ResourceRejectionReason,
        rejection_digest: ResourcePolicyDigest,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            handle,
            node,
            rejection_ordinal,
            reason,
            rejection_digest,
            lifecycle_transition,
        }
    }

    pub fn handle(self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn rejection_ordinal(self) -> ResourceRejectionOrdinal {
        self.rejection_ordinal
    }

    pub fn reason(self) -> ResourceRejectionReason {
        self.reason
    }

    pub fn rejection_digest(&self) -> &ResourcePolicyDigest {
        &self.rejection_digest
    }

    pub fn lifecycle_transition(self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceRejection {
    request_id: ResourceRequestId,
    class: ResourceRejectionDenialClass,
}

impl DeniedResourceRejection {
    pub(crate) fn new(request_id: ResourceRequestId, class: ResourceRejectionDenialClass) -> Self {
        Self { request_id, class }
    }

    pub fn request_id(self) -> ResourceRequestId {
        self.request_id
    }

    pub fn class(self) -> ResourceRejectionDenialClass {
        self.class
    }
}
