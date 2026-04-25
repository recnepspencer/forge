use serde::{Deserialize, Serialize};

use crate::data::temporal::{ReadyTemporalWake, TemporalWakeId};

use super::proof::AdmittedResourceRequest;
use super::request::{
    ResourceAttemptId, ResourceRequestHandle, ResourceRequestId, ResourceRetryOrdinal,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRetryReason {
    TimedOut,
    HostRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRetryDenialClass {
    UnknownOrStaleRequest,
    NonRetryableRequest,
    RetryPolicyDisabled,
    RetryAlreadyScheduled,
    MissingRetryBackoffWake,
    WakeMismatch,
    SupersededByNewerRequest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledResourceRetry {
    previous: ResourceRequestHandle,
    retry_ordinal: ResourceRetryOrdinal,
    reason: ResourceRetryReason,
    next_attempt: ResourceAttemptId,
    backoff_wake_id: TemporalWakeId,
}

impl ScheduledResourceRetry {
    pub(crate) fn new(
        previous: ResourceRequestHandle,
        retry_ordinal: ResourceRetryOrdinal,
        reason: ResourceRetryReason,
        next_attempt: ResourceAttemptId,
        backoff_wake_id: TemporalWakeId,
    ) -> Self {
        Self {
            previous,
            retry_ordinal,
            reason,
            next_attempt,
            backoff_wake_id,
        }
    }

    pub fn previous(self) -> ResourceRequestHandle {
        self.previous
    }

    pub fn retry_ordinal(self) -> ResourceRetryOrdinal {
        self.retry_ordinal
    }

    pub fn reason(self) -> ResourceRetryReason {
        self.reason
    }

    pub fn next_attempt(self) -> ResourceAttemptId {
        self.next_attempt
    }

    pub fn backoff_wake_id(self) -> TemporalWakeId {
        self.backoff_wake_id
    }

    pub(crate) fn with_previous(self, previous: ResourceRequestHandle) -> Self {
        Self { previous, ..self }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmittedResourceRetry {
    scheduled: ScheduledResourceRetry,
    admitted_request: AdmittedResourceRequest,
    ready_wake: ReadyTemporalWake,
}

impl AdmittedResourceRetry {
    pub(crate) fn new(
        scheduled: ScheduledResourceRetry,
        admitted_request: AdmittedResourceRequest,
        ready_wake: ReadyTemporalWake,
    ) -> Self {
        Self {
            scheduled,
            admitted_request,
            ready_wake,
        }
    }

    pub fn scheduled(&self) -> ScheduledResourceRetry {
        self.scheduled
    }

    pub fn admitted_request(&self) -> AdmittedResourceRequest {
        self.admitted_request
    }

    pub fn ready_wake(&self) -> &ReadyTemporalWake {
        &self.ready_wake
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceRetry {
    request_id: ResourceRequestId,
    class: ResourceRetryDenialClass,
}

impl DeniedResourceRetry {
    pub(crate) fn new(request_id: ResourceRequestId, class: ResourceRetryDenialClass) -> Self {
        Self { request_id, class }
    }

    pub fn request_id(self) -> ResourceRequestId {
        self.request_id
    }

    pub fn class(self) -> ResourceRetryDenialClass {
        self.class
    }
}
