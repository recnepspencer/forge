use serde::{Deserialize, Serialize};

use crate::data::resource::policy_registry::ResourcePolicyDigest;
use crate::data::temporal::ReadyTemporalWake;
use crate::data::temporal::ScheduledTemporalWake;
use crate::data::temporal::TemporalDuration;
use crate::data::temporal::TemporalWakeId;

use super::lifecycle::ResourceLifecycleTransition;
use super::policy::ResourceTimeoutOutcomeClass;
use super::request::{ResourceRequestHandle, ResourceRequestId, ResourceTimeoutOrdinal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceTimeoutDenialClass {
    UnknownOrStaleRequest,
    NonActiveRequest,
    MissingTimeoutWake,
    WakeMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceTimeoutHeartbeatExtensionDenialClass {
    UnknownOrStaleRequest,
    NonActiveRequest,
    MissingTimeoutWake,
    PolicyDoesNotAllowHeartbeatExtension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceTimeoutDeadlineAuthority {
    Descriptor,
    TransactionIntent,
    RuntimeConfig,
}

impl ResourceTimeoutDeadlineAuthority {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Descriptor => "descriptor",
            Self::TransactionIntent => "transaction-intent",
            Self::RuntimeConfig => "runtime-config",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimedOutResourceRequest {
    handle: ResourceRequestHandle,
    timeout_ordinal: ResourceTimeoutOrdinal,
    ready_wake: ReadyTemporalWake,
    timeout_duration: TemporalDuration,
    outcome_class: ResourceTimeoutOutcomeClass,
    deadline_authority: ResourceTimeoutDeadlineAuthority,
    policy_decision_digest: ResourcePolicyDigest,
    lifecycle_transition: ResourceLifecycleTransition,
}

impl TimedOutResourceRequest {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        timeout_ordinal: ResourceTimeoutOrdinal,
        ready_wake: ReadyTemporalWake,
        timeout_duration: TemporalDuration,
        outcome_class: ResourceTimeoutOutcomeClass,
        deadline_authority: ResourceTimeoutDeadlineAuthority,
        policy_decision_digest: ResourcePolicyDigest,
        lifecycle_transition: ResourceLifecycleTransition,
    ) -> Self {
        Self {
            handle,
            timeout_ordinal,
            ready_wake,
            timeout_duration,
            outcome_class,
            deadline_authority,
            policy_decision_digest,
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

    pub fn timeout_duration(&self) -> TemporalDuration {
        self.timeout_duration
    }

    pub fn outcome_class(&self) -> ResourceTimeoutOutcomeClass {
        self.outcome_class
    }

    pub fn deadline_authority(&self) -> ResourceTimeoutDeadlineAuthority {
        self.deadline_authority
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedResourceTimeoutHeartbeat {
    handle: ResourceRequestHandle,
    previous_timeout_wake_id: TemporalWakeId,
    extended_timeout_wake: ScheduledTemporalWake,
    extension_duration: TemporalDuration,
    policy_decision_digest: ResourcePolicyDigest,
}

impl ExtendedResourceTimeoutHeartbeat {
    pub(crate) fn new(
        handle: ResourceRequestHandle,
        previous_timeout_wake_id: TemporalWakeId,
        extended_timeout_wake: ScheduledTemporalWake,
        extension_duration: TemporalDuration,
        policy_decision_digest: ResourcePolicyDigest,
    ) -> Self {
        Self {
            handle,
            previous_timeout_wake_id,
            extended_timeout_wake,
            extension_duration,
            policy_decision_digest,
        }
    }

    pub fn handle(&self) -> ResourceRequestHandle {
        self.handle
    }

    pub fn previous_timeout_wake_id(&self) -> TemporalWakeId {
        self.previous_timeout_wake_id
    }

    pub fn extended_timeout_wake(&self) -> &ScheduledTemporalWake {
        &self.extended_timeout_wake
    }

    pub fn extension_duration(&self) -> TemporalDuration {
        self.extension_duration
    }

    pub fn policy_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.policy_decision_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedResourceTimeoutHeartbeatExtension {
    request_id: ResourceRequestId,
    class: ResourceTimeoutHeartbeatExtensionDenialClass,
}

impl DeniedResourceTimeoutHeartbeatExtension {
    pub(crate) fn new(
        request_id: ResourceRequestId,
        class: ResourceTimeoutHeartbeatExtensionDenialClass,
    ) -> Self {
        Self { request_id, class }
    }

    pub fn request_id(self) -> ResourceRequestId {
        self.request_id
    }

    pub fn class(self) -> ResourceTimeoutHeartbeatExtensionDenialClass {
        self.class
    }
}
