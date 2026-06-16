use crate::capability::{CapabilitySnapshot, CapabilitySnapshotDigest};
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeObservation,
};
use crate::runtime::{
    WorthUiRuntimeActivationStatus, WorthUiRuntimeAuthoringSnapshot,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeFrameEpoch, WorthUiRuntimeLifecycle,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WorthUiActiveRuntimeState {
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot: CapabilitySnapshot,
    snapshot_digest: CapabilitySnapshotDigest,
    authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
}

impl WorthUiActiveRuntimeState {
    pub(crate) fn new(
        active_artifact: WorthUiActiveArtifact,
        active_plan: WorthUiActiveExecutionPlan,
        snapshot: CapabilitySnapshot,
        snapshot_digest: CapabilitySnapshotDigest,
        authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    ) -> Self {
        Self {
            active_artifact,
            active_plan,
            snapshot,
            snapshot_digest,
            authoring_snapshot,
            lifecycle: WorthUiRuntimeLifecycle::Active,
            status: WorthUiRuntimeActivationStatus::Active,
            frame_epoch,
            diagnostic_policy,
        }
    }

    pub(crate) fn observation(&self) -> WorthUiActiveRuntimeObservation {
        WorthUiActiveRuntimeObservation::new(
            self.active_artifact.digest().raw(),
            self.active_plan.digest().as_u64(),
            self.snapshot_digest.as_u64(),
            self.lifecycle,
            self.status,
            self.frame_epoch,
        )
    }

    pub(crate) fn active_artifact(&self) -> &WorthUiActiveArtifact {
        &self.active_artifact
    }

    pub(crate) fn active_plan(&self) -> WorthUiActiveExecutionPlan {
        self.active_plan
    }

    pub(crate) fn snapshot_digest(&self) -> CapabilitySnapshotDigest {
        self.snapshot_digest
    }

    pub(crate) fn capability_snapshot(&self) -> &CapabilitySnapshot {
        &self.snapshot
    }

    pub(crate) fn authoring_snapshot(&self) -> Option<&WorthUiRuntimeAuthoringSnapshot> {
        self.authoring_snapshot.as_ref()
    }

    pub(crate) fn replace_capability_snapshot(
        &mut self,
        snapshot: CapabilitySnapshot,
        active_plan: WorthUiActiveExecutionPlan,
    ) {
        self.snapshot_digest = snapshot.digest();
        self.snapshot = snapshot;
        self.active_plan = active_plan;
    }

    pub(crate) fn replace_authoring_snapshot(
        &mut self,
        authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
    ) {
        self.authoring_snapshot = authoring_snapshot;
    }

    pub(crate) fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub(crate) fn lifecycle(&self) -> WorthUiRuntimeLifecycle {
        self.lifecycle
    }

    pub(crate) fn status(&self) -> WorthUiRuntimeActivationStatus {
        self.status
    }

    pub(crate) fn diagnostic_policy(&self) -> WorthUiRuntimeDiagnosticPolicy {
        self.diagnostic_policy
    }

    pub(crate) fn from_preserved_authority(
        active_artifact: WorthUiActiveArtifact,
        active_plan: WorthUiActiveExecutionPlan,
        snapshot: CapabilitySnapshot,
        snapshot_digest: CapabilitySnapshotDigest,
        authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
        lifecycle: WorthUiRuntimeLifecycle,
        status: WorthUiRuntimeActivationStatus,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    ) -> Self {
        Self {
            active_artifact,
            active_plan,
            snapshot,
            snapshot_digest,
            authoring_snapshot,
            lifecycle,
            status,
            frame_epoch,
            diagnostic_policy,
        }
    }

    #[cfg(test)]
    pub(crate) fn replace_artifact_for_swap_injection_for_test(
        &mut self,
        active_artifact: WorthUiActiveArtifact,
    ) {
        self.active_artifact = active_artifact;
    }

    #[cfg(test)]
    pub(crate) fn advance_frame_epoch_for_test(&mut self, frame_epoch: WorthUiRuntimeFrameEpoch) {
        self.frame_epoch = frame_epoch;
    }
}
