use crate::runtime::{
    WorthUiRuntimeActivationStatus, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
};

/// Read-only projection over active runtime truth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiActiveRuntimeObservation {
    artifact_digest: u64,
    active_plan_digest: u64,
    snapshot_digest: u64,
    authoring_snapshot_digest: Option<u64>,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
}

impl WorthUiActiveRuntimeObservation {
    pub(crate) fn new(
        artifact_digest: u64,
        active_plan_digest: u64,
        snapshot_digest: u64,
        authoring_snapshot_digest: Option<u64>,
        lifecycle: WorthUiRuntimeLifecycle,
        status: WorthUiRuntimeActivationStatus,
        frame_epoch: WorthUiRuntimeFrameEpoch,
    ) -> Self {
        Self {
            artifact_digest,
            active_plan_digest,
            snapshot_digest,
            authoring_snapshot_digest,
            lifecycle,
            status,
            frame_epoch,
        }
    }

    pub fn artifact_digest(&self) -> u64 {
        self.artifact_digest
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn snapshot_digest(&self) -> u64 {
        self.snapshot_digest
    }

    pub fn authoring_snapshot_digest(&self) -> Option<u64> {
        self.authoring_snapshot_digest
    }

    pub fn lifecycle(&self) -> WorthUiRuntimeLifecycle {
        self.lifecycle
    }

    pub fn status(&self) -> WorthUiRuntimeActivationStatus {
        self.status
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }
}
