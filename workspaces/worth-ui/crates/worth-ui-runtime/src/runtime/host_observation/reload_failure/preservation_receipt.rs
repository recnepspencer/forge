use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiLastValidObservation, WorthUiRuntimeActivationStatus,
    WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiReloadPreservationReceipt {
    active_artifact_digest: u64,
    active_plan_digest: u64,
    active_snapshot_digest: u64,
    active_lifecycle: WorthUiRuntimeLifecycle,
    active_status: WorthUiRuntimeActivationStatus,
    active_frame_epoch: WorthUiRuntimeFrameEpoch,
    last_valid_artifact_digest: u64,
    last_valid_plan_digest: u64,
    last_valid_frame_epoch: WorthUiRuntimeFrameEpoch,
}

impl WorthUiReloadPreservationReceipt {
    pub(crate) fn from_active_and_last_valid(
        active: WorthUiActiveRuntimeObservation,
        last_valid: WorthUiLastValidObservation,
    ) -> Self {
        Self {
            active_artifact_digest: active.artifact_digest(),
            active_plan_digest: active.active_plan_digest(),
            active_snapshot_digest: active.snapshot_digest(),
            active_lifecycle: active.lifecycle(),
            active_status: active.status(),
            active_frame_epoch: active.frame_epoch(),
            last_valid_artifact_digest: last_valid.artifact_digest(),
            last_valid_plan_digest: last_valid.active_plan_digest(),
            last_valid_frame_epoch: last_valid.recorded_frame_epoch(),
        }
    }

    pub fn active_artifact_digest(self) -> u64 {
        self.active_artifact_digest
    }

    pub fn active_plan_digest(self) -> u64 {
        self.active_plan_digest
    }

    pub fn active_snapshot_digest(self) -> u64 {
        self.active_snapshot_digest
    }

    pub fn active_lifecycle(self) -> WorthUiRuntimeLifecycle {
        self.active_lifecycle
    }

    pub fn active_status(self) -> WorthUiRuntimeActivationStatus {
        self.active_status
    }

    pub fn active_frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.active_frame_epoch
    }

    pub fn last_valid_artifact_digest(self) -> u64 {
        self.last_valid_artifact_digest
    }

    pub fn last_valid_plan_digest(self) -> u64 {
        self.last_valid_plan_digest
    }

    pub fn last_valid_frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.last_valid_frame_epoch
    }
}
