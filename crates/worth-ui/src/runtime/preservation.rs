use crate::runtime::{
    active::{WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeState},
    WorthUiRuntimeFrameEpoch,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiLastValidRuntimeState {
    artifact: WorthUiActiveArtifact,
    plan: WorthUiActiveExecutionPlan,
    receipt: WorthUiLastValidPreservationReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiLastValidPreservationReceipt {
    recorded_frame_epoch: WorthUiRuntimeFrameEpoch,
    artifact_digest: u64,
    active_plan_digest: u64,
}

/// Public observation that proves a last-valid state exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiLastValidObservation {
    recorded_frame_epoch: WorthUiRuntimeFrameEpoch,
    artifact_digest: u64,
    active_plan_digest: u64,
}

impl WorthUiLastValidRuntimeState {
    pub(crate) fn record_from_active(active: &WorthUiActiveRuntimeState) -> Self {
        let artifact = active.active_artifact().clone();
        let plan = active.active_plan();
        let receipt =
            WorthUiLastValidPreservationReceipt::record(active.frame_epoch(), &artifact, plan);
        Self {
            artifact,
            plan,
            receipt,
        }
    }

    pub(crate) fn observation(&self) -> WorthUiLastValidObservation {
        WorthUiLastValidObservation {
            recorded_frame_epoch: self.receipt.recorded_frame_epoch,
            artifact_digest: self.receipt.artifact_digest,
            active_plan_digest: self.receipt.active_plan_digest,
        }
    }
}

impl WorthUiLastValidPreservationReceipt {
    fn record(
        recorded_frame_epoch: WorthUiRuntimeFrameEpoch,
        artifact: &WorthUiActiveArtifact,
        plan: WorthUiActiveExecutionPlan,
    ) -> Self {
        Self {
            recorded_frame_epoch,
            artifact_digest: artifact.digest().raw(),
            active_plan_digest: plan.digest().as_u64(),
        }
    }
}

impl WorthUiLastValidObservation {
    pub fn recorded_frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.recorded_frame_epoch
    }

    pub fn artifact_digest(&self) -> u64 {
        self.artifact_digest
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn was_recorded_before_candidates(&self) -> bool {
        self.recorded_frame_epoch == WorthUiRuntimeFrameEpoch::initial()
    }
}
