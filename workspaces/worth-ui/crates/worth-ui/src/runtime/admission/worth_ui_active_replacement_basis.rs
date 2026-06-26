use crate::runtime::admission::WorthUiRuntimeReplacementPosture;
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiRuntimeActivationStatus, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeLifecycle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiActiveReplacementBasis {
    artifact_digest: u64,
    active_plan_digest: u64,
    snapshot_digest: u64,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    replacement_posture: WorthUiRuntimeReplacementPosture,
}

impl WorthUiActiveReplacementBasis {
    pub(crate) fn from_observation(observation: WorthUiActiveRuntimeObservation) -> Self {
        let replacement_posture = WorthUiRuntimeReplacementPosture::from_runtime_truth(
            observation.lifecycle(),
            observation.status(),
        );
        Self {
            artifact_digest: observation.artifact_digest(),
            active_plan_digest: observation.active_plan_digest(),
            snapshot_digest: observation.snapshot_digest(),
            lifecycle: observation.lifecycle(),
            status: observation.status(),
            frame_epoch: observation.frame_epoch(),
            replacement_posture,
        }
    }

    pub fn artifact_digest(self) -> u64 {
        self.artifact_digest
    }

    pub fn active_plan_digest(self) -> u64 {
        self.active_plan_digest
    }

    pub fn snapshot_digest(self) -> u64 {
        self.snapshot_digest
    }

    pub fn lifecycle(self) -> WorthUiRuntimeLifecycle {
        self.lifecycle
    }

    pub fn status(self) -> WorthUiRuntimeActivationStatus {
        self.status
    }

    pub fn frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn replacement_posture(self) -> WorthUiRuntimeReplacementPosture {
        self.replacement_posture
    }

    #[cfg(test)]
    pub(crate) fn with_replacement_posture_for_test(
        mut self,
        replacement_posture: WorthUiRuntimeReplacementPosture,
    ) -> Self {
        self.replacement_posture = replacement_posture;
        self
    }
}
