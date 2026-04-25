use serde::{Deserialize, Serialize};

use crate::data::temporal::ReadyTemporalWake;

use super::lifecycle::ResourceLifecycleTransition;
use super::request::{ResourceRequestHandle, ResourceRequestId, ResourceTimeoutOrdinal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceTimeoutDenialClass {
    UnknownOrStaleRequest,
    NonActiveRequest,
    MissingTimeoutWake,
    WakeMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimedOutResourceRequest {
    handle: ResourceRequestHandle,
    timeout_ordinal: ResourceTimeoutOrdinal,
    ready_wake: ReadyTemporalWake,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl TimedOutResourceRequest {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        timeout_ordinal: ResourceTimeoutOrdinal,
        ready_wake: ReadyTemporalWake,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            handle,
            timeout_ordinal,
            ready_wake,
            lifecycle_transition,
        }
    }

    pub fn handle(&self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn timeout_ordinal(&self) -> ResourceTimeoutOrdinal {
        self.timeout_ordinal
    }

    pub fn ready_wake(&self) -> &ReadyTemporalWake {
        &self.ready_wake
    }

    pub fn lifecycle_transition(&self) -> ResourceLifecycleTransition {
        self.lifecycle_transition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceTimeout {
    request_id: ResourceRequestId,
    class: ResourceTimeoutDenialClass,
}

impl DeniedResourceTimeout {
    pub(crate) fn new(request_id: ResourceRequestId, class: ResourceTimeoutDenialClass) -> Self {
        Self { request_id, class }
    }

    pub fn request_id(self) -> ResourceRequestId {
        self.request_id
    }

    pub fn class(self) -> ResourceTimeoutDenialClass {
        self.class
    }
}
