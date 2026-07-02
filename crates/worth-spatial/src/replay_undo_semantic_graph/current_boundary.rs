use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, SpatialReplayFamilyIdentity,
    SpatialReplayFamilyIdentityAuthority,
};
use crate::workload_platform::evidence_ledger::SelectedLookupSliceLedger;
use crate::workload_platform::evidence_ledger::{
    SelectedLookupSliceLedgerAssembly, SpatialGeometryEvidenceTouchAuthority,
    WorkloadEvidenceStage, WorkloadEvidenceStageIndexProduct,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_input_admission::EvidenceLookupStageReceiptAdmission;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::evidence_lookup_reuse_decision::EvidenceLookupIndexReuseResolution;
use crate::workload_platform::evidence_lookup_stage_cutover::{
    current_path::{admit_current_family_stage_cutover_path, EvidenceLookupCurrentPathError},
    current_retained_replay_receipt_for_stage,
};
use crate::workload_platform::evidence_lookup_workload_cutover::{
    EvidenceLookupConsumedWorkloadHandoff, EvidenceLookupWorkloadCutoverError,
};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::reuse_evidence_lookup_index_product;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[derive(Clone, Debug)]
pub struct CurrentReplayUndoSpatialBoundary {
    replay_family_identity: SpatialReplayFamilyIdentity,
    authority: SpatialGeometryEvidenceTouchAuthority,
    execution_receipt: EvidenceLookupExecutionReceipt,
    workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
    selected_plan: EvidenceLookupSelectedPlan,
    selected_lookup_slice: SelectedLookupSliceLedger,
    index_product: EvidenceLookupIndexProduct,
    reuse_resolution: EvidenceLookupIndexReuseResolution,
    retained_replay_receipt: Option<RetainedReplayWorkloadReceipt>,
    stage_index_product: WorkloadEvidenceStageIndexProduct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentReplayUndoSpatialBoundaryError {
    detail: String,
}

pub fn current_boolean_event_ledger_spatial_boundary(
) -> Result<CurrentReplayUndoSpatialBoundary, CurrentReplayUndoSpatialBoundaryError> {
    current_spatial_boundary(
        SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        "spatial-touch.boolean.event-ledger-evidence.v1",
        WorkloadEvidenceStage::BooleanEventLedger,
        Some(current_retained_replay_receipt_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
        )),
    )
}

pub fn current_boolean_split_spatial_boundary(
) -> Result<CurrentReplayUndoSpatialBoundary, CurrentReplayUndoSpatialBoundaryError> {
    current_spatial_boundary(
        SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        "spatial-touch.boolean.event-ledger-evidence.v1",
        WorkloadEvidenceStage::BooleanSplit,
        Some(current_retained_replay_receipt_for_stage(
            WorkloadEvidenceStage::BooleanSplit,
        )),
    )
}

pub fn current_projection_receipt_spatial_boundary(
) -> Result<CurrentReplayUndoSpatialBoundary, CurrentReplayUndoSpatialBoundaryError> {
    current_spatial_boundary(
        SpatialReplayFamilyIdentityAuthority::projection_receipt(),
        "spatial-touch.boolean.projection-consumption-evidence.v1",
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        None,
    )
}

impl CurrentReplayUndoSpatialBoundary {
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

    pub fn selected_plan(&self) -> &EvidenceLookupSelectedPlan {
        &self.selected_plan
    }

    pub fn selected_lookup_slice(&self) -> &SelectedLookupSliceLedger {
        &self.selected_lookup_slice
    }

    pub fn index_product(&self) -> &EvidenceLookupIndexProduct {
        &self.index_product
    }

    pub fn reuse_resolution(&self) -> &EvidenceLookupIndexReuseResolution {
        &self.reuse_resolution
    }

    pub fn retained_replay_receipt(&self) -> Option<&RetainedReplayWorkloadReceipt> {
        self.retained_replay_receipt.as_ref()
    }

    pub fn stage_index_product(&self) -> &WorkloadEvidenceStageIndexProduct {
        &self.stage_index_product
    }
}

impl CurrentReplayUndoSpatialBoundaryError {
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn current_spatial_boundary(
    replay_family_identity_authority: SpatialReplayFamilyIdentityAuthority,
    family_identity: &str,
    stage: WorkloadEvidenceStage,
    retained_replay_receipt: Option<RetainedReplayWorkloadReceipt>,
) -> Result<CurrentReplayUndoSpatialBoundary, CurrentReplayUndoSpatialBoundaryError> {
    let catalog = current_evidence_lookup_family_catalog().map_err(|error| {
        CurrentReplayUndoSpatialBoundaryError {
            detail: format!(
                "current evidence lookup family catalog failed: {:?}",
                error.kind()
            ),
        }
    })?;
    let family = catalog.family_by_identity(family_identity).ok_or_else(|| {
        CurrentReplayUndoSpatialBoundaryError {
            detail: format!("missing covered current family `{family_identity}`"),
        }
    })?;
    let path = admit_current_family_stage_cutover_path(&catalog, family, stage)
        .map_err(CurrentReplayUndoSpatialBoundaryError::from_current_path)?;
    let proof = path
        .prove_for_family(family.identity().as_str())
        .map_err(|error| CurrentReplayUndoSpatialBoundaryError {
            detail: format!(
                "covered current family proof failed for `{}`: {}",
                family.identity().as_str(),
                error.detail()
            ),
        })?;
    let workload_handoff = EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof)
        .map_err(CurrentReplayUndoSpatialBoundaryError::from_workload_cutover)?;
    let reuse_resolution = reuse_evidence_lookup_index_product(
        path.selected_plan(),
        path.selected_lookup_slice(),
        path.index_product(),
    )
    .map_err(|error| CurrentReplayUndoSpatialBoundaryError {
        detail: format!(
            "current spatial boundary failed to admit typed reuse resolution: {:?}",
            error.kind()
        ),
    })?;
    let authority = path.spatial_touch_authority().clone();
    let stage_index_product = SelectedLookupSliceLedgerAssembly::from_touch_authority(
        &authority,
        &EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            &authority,
            EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        ),
    )
    .assemble()
    .map_err(|error| CurrentReplayUndoSpatialBoundaryError {
        detail: format!("current stage index assembly failed: {error:?}"),
    })?
    .stage_index()
    .clone();
    Ok(CurrentReplayUndoSpatialBoundary {
        replay_family_identity: admit_spatial_replay_family_identity(
            replay_family_identity_authority,
        ),
        authority,
        execution_receipt: path.execution_receipt().clone(),
        workload_handoff,
        selected_plan: path.selected_plan().clone(),
        selected_lookup_slice: path.selected_lookup_slice().clone(),
        index_product: path.index_product().clone(),
        reuse_resolution,
        retained_replay_receipt,
        stage_index_product,
    })
}

impl CurrentReplayUndoSpatialBoundaryError {
    fn from_current_path(error: EvidenceLookupCurrentPathError) -> Self {
        Self {
            detail: error.detail().to_string(),
        }
    }

    fn from_workload_cutover(error: EvidenceLookupWorkloadCutoverError) -> Self {
        Self {
            detail: format!(
                "lookup-consumed workload handoff lowering failed: {:?}",
                error.kind()
            ),
        }
    }
}
