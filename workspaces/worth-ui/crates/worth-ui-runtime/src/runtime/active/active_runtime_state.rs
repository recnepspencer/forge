use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeObservation,
};
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameEpochAssignment;
use crate::runtime::{
    WorthUiRuntimeActivationStatus, WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeLifecycle,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WorthUiActiveRuntimeState {
    generation_identity:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot_digest: CapabilitySnapshotDigest,
    lifecycle: WorthUiRuntimeLifecycle,
    status: WorthUiRuntimeActivationStatus,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
}

impl WorthUiActiveRuntimeState {
    pub(crate) fn new(
        active_artifact: WorthUiActiveArtifact,
        active_plan: WorthUiActiveExecutionPlan,
        snapshot_digest: CapabilitySnapshotDigest,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    ) -> Self {
        let generation_identity = active_plan.generation_identity().clone();
        Self {
            generation_identity,
            active_artifact,
            active_plan,
            snapshot_digest,
            lifecycle: WorthUiRuntimeLifecycle::Active,
            status: WorthUiRuntimeActivationStatus::Active,
            frame_epoch,
            diagnostic_policy,
        }
    }

    pub(crate) fn observation(&self) -> WorthUiActiveRuntimeObservation {
        WorthUiActiveRuntimeObservation::from_active_state(self)
    }

    pub(crate) fn active_artifact(&self) -> &WorthUiActiveArtifact {
        &self.active_artifact
    }

    pub(crate) fn generation_identity(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation_identity
    }

    pub(crate) fn active_plan(&self) -> WorthUiActiveExecutionPlan {
        self.active_plan.clone()
    }

    pub(crate) fn active_plan_ref(&self) -> &WorthUiActiveExecutionPlan {
        &self.active_plan
    }

    pub(crate) fn handle_arena_identity(&self) -> crate::runtime::WorthUiHandleArenaIdentity {
        self.active_plan.handle_arena_identity()
    }

    pub(crate) fn active_plan_shares_lowering_identity_with(
        &self,
        identity: &crate::runtime::planning::WorthUiExecutionPlanLoweringIdentity,
    ) -> bool {
        self.active_plan.shares_lowering_identity_with(identity)
    }

    pub(crate) fn predecessor_region_proof(
        &self,
        authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    ) -> Result<
        crate::runtime::planning::plan_topology::WorthUiPredecessorRegionProof,
        crate::runtime::planning::plan_topology::WorthUiPredecessorRegionProofDenial,
    > {
        self.active_plan
            .predecessor_region_proof(self.active_artifact.digest().raw(), authority)
    }

    pub(crate) fn snapshot_digest(&self) -> CapabilitySnapshotDigest {
        self.snapshot_digest
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

    pub(crate) fn replacement_successor(
        active_artifact: WorthUiActiveArtifact,
        active_plan: WorthUiActiveExecutionPlan,
        snapshot_digest: CapabilitySnapshotDigest,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
    ) -> Self {
        let generation_identity = active_plan.generation_identity().clone();
        Self {
            generation_identity,
            active_artifact,
            active_plan,
            snapshot_digest,
            lifecycle: WorthUiRuntimeLifecycle::Active,
            status: WorthUiRuntimeActivationStatus::Active,
            frame_epoch,
            diagnostic_policy,
        }
    }

    pub(crate) fn diagnostic_policy(&self) -> WorthUiRuntimeDiagnosticPolicy {
        self.diagnostic_policy
    }

    pub(crate) fn apply_allocation_frame_epoch_assignment(
        &mut self,
        assignment: UiAllocationFrameEpochAssignment,
    ) {
        self.frame_epoch = assignment.epoch();
    }

    #[cfg(test)]
    pub(crate) fn advance_frame_epoch_for_test(&mut self) {
        self.frame_epoch = self.frame_epoch.next();
    }
}
