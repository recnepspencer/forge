use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, SpatialReplayFamilyIdentity,
    SpatialReplayFamilyIdentityAuthority,
};
use crate::workload_platform::evidence_ledger::{
    SelectedLookupSliceLedgerAssembly, SpatialGeometryEvidenceTouchAuthority,
    WorkloadEvidenceStage, WorkloadEvidenceStageIndexProduct,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_input_admission::EvidenceLookupStageReceiptAdmission;
use crate::workload_platform::evidence_lookup_stage_cutover::{
    admit_current_family_stage_cutover_path, current_retained_replay_receipt_for_stage,
};
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[derive(Clone)]
pub struct ReplayUndoSpatialBoundaryFixture {
    replay_family_identity: SpatialReplayFamilyIdentity,
    authority: SpatialGeometryEvidenceTouchAuthority,
    execution_receipt: EvidenceLookupExecutionReceipt,
    workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
    retained_replay_receipt: Option<RetainedReplayWorkloadReceipt>,
    stage_index_product: WorkloadEvidenceStageIndexProduct,
}

impl ReplayUndoSpatialBoundaryFixture {
    pub fn replay_family_identity(&self) -> SpatialReplayFamilyIdentity {
        self.replay_family_identity
    }

    pub fn authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.authority
    }

    pub fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }

    pub fn workload_handoff(&self) -> &EvidenceLookupConsumedWorkloadHandoff {
        &self.workload_handoff
    }

    pub fn retained_replay_receipt(&self) -> Option<&RetainedReplayWorkloadReceipt> {
        self.retained_replay_receipt.as_ref()
    }

    pub fn stage_index_product(&self) -> &WorkloadEvidenceStageIndexProduct {
        &self.stage_index_product
    }
}

pub fn boolean_event_ledger_spatial_boundary_fixture() -> ReplayUndoSpatialBoundaryFixture {
    replay_undo_spatial_boundary_fixture(
        SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        "spatial-touch.boolean.event-ledger-evidence.v1",
        WorkloadEvidenceStage::BooleanEventLedger,
        Some(current_retained_replay_receipt_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
        )),
    )
}

pub fn projection_receipt_spatial_boundary_fixture() -> ReplayUndoSpatialBoundaryFixture {
    replay_undo_spatial_boundary_fixture(
        SpatialReplayFamilyIdentityAuthority::projection_receipt(),
        "spatial-touch.boolean.projection-consumption-evidence.v1",
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        None,
    )
}

fn replay_undo_spatial_boundary_fixture(
    replay_family_identity_authority: SpatialReplayFamilyIdentityAuthority,
    family_identity: &str,
    stage: WorkloadEvidenceStage,
    retained_replay_receipt: Option<RetainedReplayWorkloadReceipt>,
) -> ReplayUndoSpatialBoundaryFixture {
    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    let family = catalog
        .family_by_identity(family_identity)
        .expect("covered family declaration");
    let path = admit_current_family_stage_cutover_path(&catalog, family, stage)
        .expect("current cutover path");
    let proof = path
        .prove_for_family(family.identity().as_str())
        .expect("covered family proof");
    let authority = path.spatial_touch_authority().clone();
    let execution_receipt = path.execution_receipt().clone();
    let workload_handoff =
        EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof).expect("handoff");
    let stage_index_product = SelectedLookupSliceLedgerAssembly::from_touch_authority(
        &authority,
        &EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            &authority,
            EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        ),
    )
    .assemble()
    .expect("assembled lookup ledger closes")
    .stage_index()
    .clone();

    ReplayUndoSpatialBoundaryFixture {
        replay_family_identity: admit_spatial_replay_family_identity(
            replay_family_identity_authority,
        ),
        authority,
        execution_receipt,
        workload_handoff,
        retained_replay_receipt,
        stage_index_product,
    }
}
