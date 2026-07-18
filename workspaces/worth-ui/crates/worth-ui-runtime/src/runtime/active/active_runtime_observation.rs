use crate::runtime::{
    WorthUiRuntimeActivationStatus, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
};

/// Read-only projection over active runtime truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiActiveRuntimeObservation {
    generation_identity:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    artifact_digest: u64,
    active_plan_digest: u64,
    snapshot_digest: u64,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
}

impl WorthUiActiveRuntimeObservation {
    pub(crate) fn new(
        generation_identity: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
        artifact_digest: u64,
        active_plan_digest: u64,
        snapshot_digest: u64,
        lifecycle: WorthUiRuntimeLifecycle,
        status: WorthUiRuntimeActivationStatus,
        frame_epoch: WorthUiRuntimeFrameEpoch,
    ) -> Self {
        Self {
            generation_identity,
            artifact_digest,
            active_plan_digest,
            snapshot_digest,
            lifecycle,
            status,
            frame_epoch,
        }
    }

    pub fn artifact_digest(&self) -> u64 {
        self.artifact_digest
    }

    pub fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation_identity
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn snapshot_digest(&self) -> u64 {
        self.snapshot_digest
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
