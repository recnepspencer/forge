use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeState,
};
use crate::runtime::WorthUiRuntimeDiagnosticPolicy;
use crate::runtime::WorthUiRuntimeFrameEpoch;

pub(crate) fn build_active_runtime_state(
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot_digest: CapabilitySnapshotDigest,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
) -> WorthUiActiveRuntimeState {
    WorthUiActiveRuntimeState::new(
        active_artifact,
        active_plan,
        snapshot_digest,
        frame_epoch,
        diagnostic_policy,
    )
}