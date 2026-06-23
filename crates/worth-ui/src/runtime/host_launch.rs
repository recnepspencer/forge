use crate::capability::{CapabilitySnapshot, CapabilitySnapshotDigest};
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeState,
};
use crate::runtime::{
    WorthUiCandidateRuntimeAuthoringSnapshot, WorthUiRuntimeAuthoringSnapshot,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeFrameEpoch,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDigest, WorthUiArtifactDigestor,
    WorthUiArtifactEquivalenceBasis,
};

/// Launch request for creating an active runtime host from canonical artifact truth.
#[derive(Debug)]
pub struct WorthUiRuntimeLaunch {
    pub(crate) artifact: WorthUiArtifact,
    pub(crate) authoring_snapshot: Option<WorthUiCandidateRuntimeAuthoringSnapshot>,
    pub(crate) frame_epoch: WorthUiRuntimeFrameEpoch,
    pub(crate) diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLaunchDenial {
    StalePendingActivation {
        pending_epoch: WorthUiRuntimeFrameEpoch,
        active_epoch: WorthUiRuntimeFrameEpoch,
    },
}

impl WorthUiRuntimeLaunch {
    pub(crate) fn from_facade_authoring(
        artifact: WorthUiArtifact,
        authoring_snapshot: WorthUiCandidateRuntimeAuthoringSnapshot,
        diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    ) -> Self {
        Self {
            artifact,
            authoring_snapshot: Some(authoring_snapshot),
            frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
            diagnostic_policy,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_canonical_artifact(artifact: WorthUiArtifact) -> Self {
        Self {
            artifact,
            authoring_snapshot: None,
            frame_epoch: WorthUiRuntimeFrameEpoch::initial(),
            diagnostic_policy: WorthUiRuntimeDiagnosticPolicy::minimal(),
        }
    }

    pub fn with_diagnostics(mut self, diagnostic_policy: WorthUiRuntimeDiagnosticPolicy) -> Self {
        self.diagnostic_policy = diagnostic_policy;
        self
    }
}

pub(crate) fn seal_launch_artifact(
    artifact: WorthUiArtifact,
) -> (WorthUiActiveArtifact, WorthUiArtifactDigest) {
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    (
        WorthUiActiveArtifact::new(artifact, artifact_digest),
        artifact_digest,
    )
}

pub(crate) fn derive_launch_execution_plan(
    artifact_digest: WorthUiArtifactDigest,
    snapshot_digest: CapabilitySnapshotDigest,
) -> WorthUiActiveExecutionPlan {
    WorthUiActiveExecutionPlan::from_launch_authority(artifact_digest, snapshot_digest)
}

pub(crate) fn build_active_runtime_state(
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot: CapabilitySnapshot,
    snapshot_digest: CapabilitySnapshotDigest,
    authoring_snapshot: Option<WorthUiRuntimeAuthoringSnapshot>,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
) -> WorthUiActiveRuntimeState {
    WorthUiActiveRuntimeState::new(
        active_artifact,
        active_plan,
        snapshot,
        snapshot_digest,
        authoring_snapshot,
        frame_epoch,
        diagnostic_policy,
    )
}
