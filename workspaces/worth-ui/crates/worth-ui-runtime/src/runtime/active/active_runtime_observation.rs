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
    cross_lane_bundle: super::WorthUiCrossLaneBundleReceipt,
    snapshot_digest: u64,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
}

impl WorthUiActiveRuntimeObservation {
    pub(crate) fn from_active_state(state: &super::WorthUiActiveRuntimeState) -> Self {
        let active_plan = state.active_plan_ref();
        Self {
            generation_identity: state.generation_identity().clone(),
            artifact_digest: state.active_artifact().digest().raw(),
            active_plan_digest: active_plan.digest().as_u64(),
            cross_lane_bundle: active_plan.cross_lane_receipt(),
            snapshot_digest: state.snapshot_digest().as_u64(),
            lifecycle: state.lifecycle(),
            status: state.status(),
            frame_epoch: state.frame_epoch(),
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

    pub fn cross_lane_bundle(&self) -> super::WorthUiCrossLaneBundleReceipt {
        self.cross_lane_bundle
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
