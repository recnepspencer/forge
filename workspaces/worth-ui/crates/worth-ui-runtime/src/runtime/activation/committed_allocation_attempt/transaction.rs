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
        candidate_bundle: crate::runtime::active::WorthUiSealedExecutionPlanBundle,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Result<Self, ()> {
        if candidate_bundle.digest().raw() != ready.candidate_execution_plan_digest()
            || candidate_bundle.generation_identity()
                != ready
                    .pending_activation()
                    .candidate_application_authority()
                    .generation_identity()
        {
            return Err(());
        }
        let artifact_bundle = ready
            .pending_activation()
            .staged_replacement()
            .admitted_candidate()
            .artifact_bundle();
        let active_artifact = WorthUiActiveArtifact::new_with_dependency_report(
            artifact_bundle.artifact_authority(),
            artifact_bundle.artifact_digest(),
            artifact_bundle
                .dependency_metadata()
                .dependency_report_authority(),
        );
        let committed = ready.committed().clone();
        Ok(Self {
            active_artifact,
            active_plan: WorthUiActiveExecutionPlan::from_lowered_bundle(candidate_bundle),
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
        payload.active_artifact,
        payload.active_plan,
        payload.snapshot_digest,
        runtime_frame_epoch,
        active.diagnostic_policy(),
    );
    (next_active, payload.ledger_transition, payload.committed)
}
