mod lowering_error;
mod replay_lowering;
mod undo_lowering;

use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphTouchedSubject;

use crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority;

pub use lowering_error::ReplayUndoSemanticGraphLoweringError;
pub use replay_lowering::{
    lower_spatial_replay_equivalence_basis,
    lower_spatial_replay_equivalence_basis_from_admitted_input,
    lower_spatial_replay_equivalence_basis_from_scope_product,
    lower_spatial_replay_equivalence_basis_from_selected_plan, lower_spatial_replay_scope_identity,
    lower_spatial_replay_scope_identity_from_admitted_input,
    lower_spatial_replay_scope_identity_from_scope_product,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_replay_scope_product_from_selected_plan,
};
pub use undo_lowering::{
    lower_spatial_undo_equivalence_basis, lower_spatial_undo_equivalence_basis_from_admitted_input,
    lower_spatial_undo_equivalence_basis_from_scope_product, lower_spatial_undo_scope_identity,
    lower_spatial_undo_scope_identity_from_admitted_input,
    lower_spatial_undo_scope_identity_from_scope_product,
    lower_spatial_undo_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_selected_plan,
};

pub(crate) fn lower_spatial_touched_subjects(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
) -> Vec<ReplayUndoSemanticGraphTouchedSubject> {
    spatial_touch_authority
        .authority_rows()
        .iter()
        .map(
            |row| ReplayUndoSemanticGraphTouchedSubject::SpatialAuthorityStage {
                evidence_stage: row.stage().human_name().to_string(),
                evidence_identity: row.evidence_identity().to_string(),
            },
        )
        .collect()
}

#[cfg(test)]
mod tests;
