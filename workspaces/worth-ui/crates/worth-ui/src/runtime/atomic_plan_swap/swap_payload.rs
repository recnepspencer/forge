use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::active::{WorthUiActiveArtifact, WorthUiActiveExecutionPlan};
use crate::runtime::{
    WorthUiExecutionPlanDigest, WorthUiPlanSwapDenialReason, WorthUiReadyActivation,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WorthUiReadyActivationSwapPayload {
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot_digest: CapabilitySnapshotDigest,
}

impl WorthUiReadyActivationSwapPayload {
    pub(crate) fn from_ready_activation(
        ready: WorthUiReadyActivation,
        candidate_plan_digest: WorthUiExecutionPlanDigest,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Result<Self, WorthUiPlanSwapDenialReason> {
        if candidate_plan_digest.raw() != ready.candidate_execution_plan_digest() {
            return Err(WorthUiPlanSwapDenialReason::CandidateExecutionPlanDigestMismatch);
        }
        let candidate_bundle = ready
            .pending_activation()
            .staged_replacement()
            .admitted_candidate()
            .artifact_bundle();
        let active_artifact = WorthUiActiveArtifact::new(
            candidate_bundle.artifact().clone(),
            candidate_bundle.artifact_digest(),
        );
        let active_plan = WorthUiActiveExecutionPlan::from_swap_authority(candidate_plan_digest);
        Ok(Self {
            active_artifact,
            active_plan,
            snapshot_digest,
        })
    }

    #[cfg(test)]
    pub(crate) fn active_artifact(&self) -> &WorthUiActiveArtifact {
        &self.active_artifact
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthUiActiveArtifact,
        WorthUiActiveExecutionPlan,
        CapabilitySnapshotDigest,
    ) {
        (self.active_artifact, self.active_plan, self.snapshot_digest)
    }
}
