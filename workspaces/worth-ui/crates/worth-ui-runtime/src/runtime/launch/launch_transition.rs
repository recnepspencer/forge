use std::rc::Rc;

use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::allocation_planning::WorthUiRetainedAllocationPlanningEvidenceRegistry;

use super::build_active_state::build_active_runtime_state;
use super::derive_plan::derive_launch_execution_plan;
use super::host::WorthUiRuntimeHost;
use super::launch_request::{WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};
use super::preservation::WorthUiLastValidRuntimeState;
use super::seal_artifact::seal_launch_artifact;

impl WorthUiRuntimeHost {
    pub(crate) fn launch(
        launch: WorthUiRuntimeLaunch,
        snapshot_digest: CapabilitySnapshotDigest,
        retained_allocation_planning_evidence: Rc<
            WorthUiRetainedAllocationPlanningEvidenceRegistry,
        >,
    ) -> Result<Self, WorthUiRuntimeLaunchDenial> {
        let WorthUiRuntimeLaunch {
            artifact,
            frame_epoch,
            diagnostic_policy,
        } = launch;
        let (active_artifact, artifact_digest) = seal_launch_artifact(artifact);
        let active_plan = derive_launch_execution_plan(artifact_digest, snapshot_digest);
        let active = build_active_runtime_state(
            active_artifact,
            active_plan,
            snapshot_digest,
            frame_epoch,
            diagnostic_policy,
        );
        let last_valid = WorthUiLastValidRuntimeState::record_from_active(&active);

        Ok(Self {
            active,
            last_valid,
            retained_allocation_planning_evidence,
        })
    }
}
