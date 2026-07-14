use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeState,
};
use crate::runtime::{
    WorthUiRuntimeActivationStatus, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPriorValidPlan {
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot_digest: CapabilitySnapshotDigest,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPriorValidPlanObservation {
    artifact_digest: u64,
    active_plan_digest: u64,
    snapshot_digest: u64,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
}

impl WorthUiPriorValidPlan {
    pub(crate) fn capture(active: &WorthUiActiveRuntimeState) -> Self {
        Self {
            active_artifact: active.active_artifact().clone(),
            active_plan: active.active_plan(),
            snapshot_digest: active.snapshot_digest(),
            lifecycle: active.lifecycle(),
            status: active.status(),
            frame_epoch: active.frame_epoch(),
        }
    }

    pub(crate) fn observation(&self) -> WorthUiPriorValidPlanObservation {
        WorthUiPriorValidPlanObservation {
            artifact_digest: self.active_artifact.digest().raw(),
            active_plan_digest: self.active_plan.digest().as_u64(),
            snapshot_digest: self.snapshot_digest.as_u64(),
            lifecycle: self.lifecycle,
            status: self.status,
            frame_epoch: self.frame_epoch,
        }
    }
}

impl WorthUiPriorValidPlanObservation {
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
}
