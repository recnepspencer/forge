use crate::runtime::{WorthUiPlanLoweringCounters, WorthUiRuntimeFrameEpoch};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanLoweringDenialReason {
    MissingActivationReadiness,
    StalePendingActivation,
    ExecutionPlanLoweringInputMismatch,
    UnregisteredPlanNodeFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanLoweringDenial {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    pending_frame_epoch: WorthUiRuntimeFrameEpoch,
    active_frame_epoch: WorthUiRuntimeFrameEpoch,
    reason: WorthUiPlanLoweringDenialReason,
    counters: WorthUiPlanLoweringCounters,
}

impl WorthUiPlanLoweringDenial {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        pending_frame_epoch: WorthUiRuntimeFrameEpoch,
        active_frame_epoch: WorthUiRuntimeFrameEpoch,
        reason: WorthUiPlanLoweringDenialReason,
        counters: WorthUiPlanLoweringCounters,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            pending_frame_epoch,
            active_frame_epoch,
            reason,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn pending_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.pending_frame_epoch
    }

    pub fn active_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.active_frame_epoch
    }

    pub fn reason(&self) -> WorthUiPlanLoweringDenialReason {
        self.reason
    }

    pub fn counters(&self) -> WorthUiPlanLoweringCounters {
        self.counters
    }
}
