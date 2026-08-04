use super::super::refresh::{CoalescingDecision, RefreshAdmissionClass};
use super::super::{LiveExecutionEnvelope, LivePolicyCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveExpectedRejectionError {
    UnexpectedRefreshAdmission {
        admission_class: RefreshAdmissionClass,
        admission_status: crate::live_performance::RefreshAdmissionStatus,
    },
    UnexpectedCoalescingAdmission {
        decision: CoalescingDecision,
    },
    UnexpectedProgressAdvance {
        ordinal: u64,
        replay_digest: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveCertificationLane {
    lane_name: String,
    execution: LiveExecutionEnvelope,
}

impl LiveCertificationLane {
    pub fn lane_name(&self) -> &str {
        &self.lane_name
    }

    pub fn execution(&self) -> &LiveExecutionEnvelope {
        &self.execution
    }

    pub fn new(lane_name: impl Into<String>, execution: LiveExecutionEnvelope) -> Self {
        Self {
            lane_name: lane_name.into(),
            execution,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveCertificationRejectionLane {
    lane_name: String,
    failure_class: String,
    failure_digest: String,
    counters: LivePolicyCounters,
}

impl LiveCertificationRejectionLane {
    pub fn lane_name(&self) -> &str {
        &self.lane_name
    }

    pub fn failure_class(&self) -> &str {
        &self.failure_class
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn counters(&self) -> &LivePolicyCounters {
        &self.counters
    }

    pub fn new(
        lane_name: impl Into<String>,
        failure_class: impl Into<String>,
        failure_digest: impl Into<String>,
        counters: LivePolicyCounters,
    ) -> Self {
        Self {
            lane_name: lane_name.into(),
            failure_class: failure_class.into(),
            failure_digest: failure_digest.into(),
            counters,
        }
    }
}
