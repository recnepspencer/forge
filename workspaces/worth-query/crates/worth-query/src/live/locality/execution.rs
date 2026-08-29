use super::super::delivery::{DeliveryLocalityOutcome, RegionScopedReplayBundle};
use super::super::patches::LivePatchEnvelope;
use super::super::telemetry::{LivePolicyCounters, RegionScopedLiveCounters};
use super::super::{LiveExecutionError, LiveReplayBundle};
use super::matching::{LocalityMatchClass, LocalityWideningDecision};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedExecutionReport {
    pub(in crate::live) query_digest: String,
    pub(in crate::live) locality_digest: String,
    pub(in crate::live) locality_outcome: DeliveryLocalityOutcome,
    pub(in crate::live) locality_match_class: LocalityMatchClass,
    pub(in crate::live) widening_decision: Option<LocalityWideningDecision>,
    pub(in crate::live) result_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) replay_digest: String,
}

impl RegionScopedExecutionReport {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn locality_outcome(&self) -> &DeliveryLocalityOutcome {
        &self.locality_outcome
    }

    pub fn locality_match_class(&self) -> &LocalityMatchClass {
        &self.locality_match_class
    }

    pub fn widening_decision(&self) -> Option<&LocalityWideningDecision> {
        self.widening_decision.as_ref()
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionScopedLiveExecutionEnvelope {
    pub(in crate::live) report: RegionScopedExecutionReport,
    pub(in crate::live) patch_envelope: LivePatchEnvelope,
    pub(in crate::live) replay_bundle: RegionScopedReplayBundle,
    pub(in crate::live) counters: RegionScopedLiveCounters,
}

impl RegionScopedLiveExecutionEnvelope {
    pub fn report(&self) -> &RegionScopedExecutionReport {
        &self.report
    }

    pub fn patch_envelope(&self) -> &LivePatchEnvelope {
        &self.patch_envelope
    }

    pub fn replay_bundle(&self) -> &LiveReplayBundle {
        self.replay_bundle.live_replay_bundle()
    }

    pub fn region_scoped_replay_bundle(&self) -> &RegionScopedReplayBundle {
        &self.replay_bundle
    }

    pub fn counters(&self) -> &LivePolicyCounters {
        self.counters.snapshot()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegionScopedLiveError {
    UnsupportedLocalityFamily,
    UnsupportedLocalityPredicate,
    LocalityBreadthBudgetExceeded {
        limit: usize,
        actual: usize,
    },
    WideningDenied {
        expected: String,
        received: Vec<String>,
    },
    StreamWindowWidthBudgetExceeded {
        limit: usize,
        actual: usize,
    },
    StreamMemberWidthBudgetExceeded {
        limit: usize,
        actual: usize,
    },
    BridgeSliceIncompatibility,
    UnsupportedStreamConsumerShape,
    LiveExecution(LiveExecutionError),
}

impl From<LiveExecutionError> for RegionScopedLiveError {
    fn from(value: LiveExecutionError) -> Self {
        Self::LiveExecution(value)
    }
}
