use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentity, ReplayScopeIdentityInput,
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLocalityScope,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity, admit_spatial_evidence_lookup_prior_proof_identity,
};

use super::ReplayUndoSemanticGraphLoweringError;
use crate::replay_undo_semantic_graph::scope_product::{
    lower_spatial_replay_equivalence_basis_from_scope_product as lower_basis_from_scope_product,
    lower_spatial_replay_equivalence_basis_from_selected_plan as lower_basis_from_selected_plan,
    lower_spatial_replay_scope_identity_from_scope_product as lower_scope_identity_from_scope_product,
};
use crate::replay_undo_semantic_graph::{
    select_spatial_replay_plan, SpatialReplayScopeProduct, SpatialReplaySelectedPlan,
    SpatialReplaySemanticGraphAdmittedInput,
};
use crate::workload_platform::evidence_ledger::{
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStageIndexProduct,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;

pub fn lower_spatial_replay_scope_identity(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    evidence_lookup_receipt: &EvidenceLookupExecutionReceipt,
    stage_index_product: &WorkloadEvidenceStageIndexProduct,
) -> Result<ReplayScopeIdentity, ReplayUndoSemanticGraphLoweringError> {
    Ok(admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        lower_spatial_replay_equivalence_basis(
            spatial_touch_authority,
            evidence_lookup_receipt,
            stage_index_product,
        )?,
    )))
}

pub fn lower_spatial_replay_scope_identity_from_admitted_input(
    admitted_input: &SpatialReplaySemanticGraphAdmittedInput<'_>,
) -> Result<ReplayScopeIdentity, ReplayUndoSemanticGraphLoweringError> {
    let scope_product = lower_spatial_replay_scope_product_from_admitted_input(admitted_input)?;
    Ok(lower_spatial_replay_scope_identity_from_scope_product(
        &scope_product,
    ))
}

pub fn lower_spatial_replay_equivalence_basis(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    evidence_lookup_receipt: &EvidenceLookupExecutionReceipt,
    stage_index_product: &WorkloadEvidenceStageIndexProduct,
) -> Result<ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLoweringError> {
    require_matching_stage_index_identity(spatial_touch_authority, stage_index_product)?;
    Ok(ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::SpatialTouchAuthority,
        crate::replay_undo_semantic_graph::lower_spatial_touched_subjects(spatial_touch_authority),
        admit_spatial_evidence_lookup_prior_proof_identity(
            evidence_lookup_receipt.execution_receipt_digest(),
        ),
        Some(admit_replay_undo_stage_index_identity(
            stage_index_product.index_identity(),
        )),
    ))
}

pub fn lower_spatial_replay_equivalence_basis_from_admitted_input(
    admitted_input: &SpatialReplaySemanticGraphAdmittedInput<'_>,
) -> Result<ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLoweringError> {
    let scope_product = lower_spatial_replay_scope_product_from_admitted_input(admitted_input)?;
    Ok(lower_spatial_replay_equivalence_basis_from_scope_product(
        &scope_product,
    ))
}

pub fn lower_spatial_replay_scope_product_from_admitted_input<'a>(
    admitted_input: &'a SpatialReplaySemanticGraphAdmittedInput<'a>,
) -> Result<SpatialReplayScopeProduct<'a>, ReplayUndoSemanticGraphLoweringError> {
    let replay_plan = select_spatial_replay_plan(admitted_input)?;
    Ok(lower_spatial_replay_scope_product_from_selected_plan(
        &replay_plan,
    ))
}

pub fn lower_spatial_replay_scope_product_from_selected_plan<'a>(
    replay_plan: &SpatialReplaySelectedPlan<'a>,
) -> SpatialReplayScopeProduct<'a> {
    crate::replay_undo_semantic_graph::scope_product::lower_spatial_replay_scope_product_from_selected_plan(
        replay_plan,
    )
}

pub fn lower_spatial_replay_equivalence_basis_from_selected_plan(
    replay_plan: &SpatialReplaySelectedPlan<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    lower_basis_from_selected_plan(replay_plan)
}

pub fn lower_spatial_replay_scope_identity_from_scope_product(
    scope_product: &SpatialReplayScopeProduct<'_>,
) -> ReplayScopeIdentity {
    lower_scope_identity_from_scope_product(scope_product)
}

pub fn lower_spatial_replay_equivalence_basis_from_scope_product(
    scope_product: &SpatialReplayScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    lower_basis_from_scope_product(scope_product)
}

fn require_matching_stage_index_identity(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    stage_index_product: &WorkloadEvidenceStageIndexProduct,
) -> Result<(), ReplayUndoSemanticGraphLoweringError> {
    if spatial_touch_authority.stage_index_identity() == stage_index_product.index_identity() {
        return Ok(());
    }
    Err(
        ReplayUndoSemanticGraphLoweringError::StageIndexIdentityMismatch {
            authority_stage_index_identity: spatial_touch_authority
                .stage_index_identity()
                .to_string(),
            product_stage_index_identity: stage_index_product.index_identity().to_string(),
        },
    )
}
