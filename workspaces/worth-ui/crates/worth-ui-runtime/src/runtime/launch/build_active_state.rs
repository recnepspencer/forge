use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeState,
};
use crate::runtime::WorthUiRuntimeDiagnosticPolicy;
use crate::runtime::WorthUiRuntimeFrameEpoch;

pub(crate) fn build_active_runtime_state(
    generation_identity: crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot_digest: CapabilitySnapshotDigest,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
) -> WorthUiActiveRuntimeState {
    WorthUiActiveRuntimeState::new(
        generation_identity,
        active_artifact,
        active_plan,
        snapshot_digest,
        frame_epoch,
        diagnostic_policy,
    )
}
