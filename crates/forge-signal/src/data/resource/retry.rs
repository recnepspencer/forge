use serde::{Deserialize, Serialize};

use crate::data::resource::policy_registry::ResourcePolicyDigest;
use crate::data::temporal::TemporalDuration;
use crate::data::temporal::{ReadyTemporalWake, TemporalWakeId};

use super::policy::ResourceRetryBudgetScope;
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
    RetryAttemptLimitReached,
    RetryBudgetExhausted,
    RetryTimeoutWindowExhausted,
    RetryAlreadyScheduled,
    MissingRetryBackoffWake,
    WakeMismatch,
    SupersededByNewerRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledResourceRetry {
    previous: ResourceRequestHandle,
    retry_ordinal: ResourceRetryOrdinal,
    reason: ResourceRetryReason,
    next_attempt: ResourceAttemptId,
    backoff_wake_id: TemporalWakeId,
    scheduled_delay: TemporalDuration,
    policy_decision_digest: ResourcePolicyDigest,
    retry_budget_scope: Option<ResourceRetryBudgetScope>,
    retry_budget_limit: Option<u32>,
    retry_budget_usage: Option<u32>,
}

impl ScheduledResourceRetry {
    pub(crate) fn new(
        previous: ResourceRequestHandle,
        retry_ordinal: ResourceRetryOrdinal,
        reason: ResourceRetryReason,
        next_attempt: ResourceAttemptId,
        backoff_wake_id: TemporalWakeId,
        scheduled_delay: TemporalDuration,
        policy_decision_digest: ResourcePolicyDigest,
        retry_budget_scope: Option<ResourceRetryBudgetScope>,
        retry_budget_limit: Option<u32>,
        retry_budget_usage: Option<u32>,
    ) -> Self {
        Self {
            previous,
            retry_ordinal,
            reason,
            next_attempt,
            backoff_wake_id,
            scheduled_delay,
            policy_decision_digest,
            retry_budget_scope,
            retry_budget_limit,
            retry_budget_usage,
        }
    }

    pub fn previous(&self) -> ResourceRequestHandle {
        self.previous
    }

    pub fn retry_ordinal(&self) -> ResourceRetryOrdinal {
        self.retry_ordinal
    }

    pub fn reason(&self) -> ResourceRetryReason {
        self.reason
    }

    pub fn next_attempt(&self) -> ResourceAttemptId {
        self.next_attempt
    }

    pub fn backoff_wake_id(&self) -> TemporalWakeId {
        self.backoff_wake_id
    }

    pub fn scheduled_delay(&self) -> TemporalDuration {
        self.scheduled_delay
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }

    pub fn retry_budget_scope(&self) -> Option<ResourceRetryBudgetScope> {
        self.retry_budget_scope
    }

    pub fn retry_budget_limit(&self) -> Option<u32> {
        self.retry_budget_limit
    }

    pub fn retry_budget_usage(&self) -> Option<u32> {
        self.retry_budget_usage
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
        self.scheduled.clone()
    }

    pub fn admitted_request(&self) -> AdmittedResourceRequest {
        self.admitted_request
    }

    pub fn ready_wake(&self) -> &ReadyTemporalWake {
        &self.ready_wake
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceRetry {
    request_id: ResourceRequestId,
    class: ResourceRetryDenialClass,
    policy_decision_digest: ResourcePolicyDigest,
    retry_budget_scope: Option<ResourceRetryBudgetScope>,
    retry_budget_limit: Option<u32>,
    retry_budget_usage: Option<u32>,
}

impl DeniedResourceRetry {
    pub(crate) fn new(
        request_id: ResourceRequestId,
        class: ResourceRetryDenialClass,
        policy_decision_digest: ResourcePolicyDigest,
        retry_budget_scope: Option<ResourceRetryBudgetScope>,
        retry_budget_limit: Option<u32>,
        retry_budget_usage: Option<u32>,
    ) -> Self {
        Self {
            request_id,
            class,
            policy_decision_digest,
            retry_budget_scope,
            retry_budget_limit,
            retry_budget_usage,
        }
    }

    pub fn request_id(&self) -> ResourceRequestId {
        self.request_id
    }

    pub fn class(&self) -> ResourceRetryDenialClass {
        self.class
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }

    pub fn retry_budget_scope(&self) -> Option<ResourceRetryBudgetScope> {
        self.retry_budget_scope
    }

    pub fn retry_budget_limit(&self) -> Option<u32> {
        self.retry_budget_limit
    }

    pub fn retry_budget_usage(&self) -> Option<u32> {
        self.retry_budget_usage
    }
}
