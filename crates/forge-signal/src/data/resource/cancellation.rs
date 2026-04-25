use serde::{Deserialize, Serialize};

use super::lifecycle::ResourceLifecycleTransition;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelledResourceRequest {
    handle: ResourceRequestHandle,
    cancellation_ordinal: ResourceCancellationOrdinal,
    reason: ResourceCancellationReason,
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
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            handle,
            cancellation_ordinal,
            reason,
            lifecycle_transition,
        }
    }

    pub fn handle(self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn cancellation_ordinal(self) -> ResourceCancellationOrdinal {
        self.cancellation_ordinal
    }

    pub fn reason(self) -> ResourceCancellationReason {
        self.reason
    }

    pub fn lifecycle_transition(self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}
