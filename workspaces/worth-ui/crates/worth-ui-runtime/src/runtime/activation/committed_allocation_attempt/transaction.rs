use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::active::{
    WorthUiActiveArtifact, WorthUiActiveExecutionPlan, WorthUiActiveRuntimeState,
};
use crate::runtime::WorthUiRuntimeFrameEpoch;

pub(super) struct PreparedActiveSuccessor {
    active_artifact: WorthUiActiveArtifact,
    active_plan: WorthUiActiveExecutionPlan,
    snapshot_digest: CapabilitySnapshotDigest,
    ledger_transition: crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
    committed: crate::runtime::UiCommittedAllocationReplan,
}

impl PreparedActiveSuccessor {
    pub(super) fn prepare(
        ready: super::UiCommittedAllocationValidation,
        candidate_plan_digest: crate::runtime::WorthUiExecutionPlanDigest,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Result<Self, ()> {
        if candidate_plan_digest.raw() != ready.candidate_execution_plan_digest() {
            return Err(());
        }
        let candidate_bundle = ready
            .pending_activation()
            .staged_replacement()
            .admitted_candidate()
            .artifact_bundle();
        let active_artifact = WorthUiActiveArtifact::new(
            candidate_bundle.artifact_authority(),
            candidate_bundle.artifact_digest(),
        );
        let committed = ready.committed().clone();
        Ok(Self {
            active_artifact,
            active_plan: WorthUiActiveExecutionPlan::from_swap_authority(candidate_plan_digest),
            snapshot_digest,
            ledger_transition: ready.into_ledger_transition(),
            committed,
        })
    }
}

pub(super) fn prepare_active_successor(
    active: &WorthUiActiveRuntimeState,
    payload: PreparedActiveSuccessor,
    runtime_frame_epoch: WorthUiRuntimeFrameEpoch,
) -> (
    WorthUiActiveRuntimeState,
    crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
    crate::runtime::UiCommittedAllocationReplan,
) {
    let next_active = WorthUiActiveRuntimeState::replacement_successor(
        active,
        payload.active_artifact,
        payload.active_plan,
        payload.snapshot_digest,
        runtime_frame_epoch,
    );
    (next_active, payload.ledger_transition, payload.committed)
}
