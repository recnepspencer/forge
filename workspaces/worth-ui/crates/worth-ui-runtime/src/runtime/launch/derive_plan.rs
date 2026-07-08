use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::active::WorthUiActiveExecutionPlan;
use crate::source::WorthUiArtifactDigest;

pub(crate) fn derive_launch_execution_plan(
    artifact_digest: WorthUiArtifactDigest,
    snapshot_digest: CapabilitySnapshotDigest,
) -> WorthUiActiveExecutionPlan {
    WorthUiActiveExecutionPlan::from_launch_authority(artifact_digest, snapshot_digest)
}