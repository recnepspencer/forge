use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_undo_scope_identity, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphLocalityScope, UndoScopeIdentity, UndoScopeIdentityInput,
};

use super::undo_scope_product_counters::SpatialUndoScopeProductCounters;
use super::SpatialUndoScopeProduct;
use crate::replay_undo_semantic_graph::{SpatialUndoPlanError, SpatialUndoSelectedPlan};

pub fn lower_spatial_undo_scope_product_from_selected_plan<'a>(
    undo_plan: &SpatialUndoSelectedPlan<'a>,
) -> Result<SpatialUndoScopeProduct<'a>, SpatialUndoPlanError> {
    let admitted_input = undo_plan.admitted_input();
    let equivalence_basis = lower_spatial_undo_equivalence_basis_from_selected_plan(undo_plan);
    let scope_identity =
        admit_undo_scope_identity(UndoScopeIdentityInput::new(equivalence_basis.clone()));
    let handoff = admitted_input.lookup_consumed_workload_handoff();
    let counters = SpatialUndoScopeProductCounters::new(
        equivalence_basis.touched_subjects().len(),
        usize::from(handoff.is_some()),
        handoff.map_or(0, |value| value.counters().raw_row_scan_count()),
        handoff.map_or(0, |value| value.counters().broad_receipt_scan_count()),
        handoff.map_or(0, |value| value.counters().caller_owned_scan_count()),
    );
    Ok(SpatialUndoScopeProduct::new(
        undo_plan.family_identity(),
        undo_plan.workload_dependency_posture(),
        admitted_input.semantic_graph_identity().to_string(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.stage_index_identity().clone(),
        handoff,
        counters,
        equivalence_basis,
        scope_identity,
    ))
}

pub fn lower_spatial_undo_scope_identity_from_scope_product(
    scope_product: &SpatialUndoScopeProduct<'_>,
) -> UndoScopeIdentity {
    scope_product.scope_identity().clone()
}

pub fn lower_spatial_undo_equivalence_basis_from_scope_product(
    scope_product: &SpatialUndoScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    scope_product.equivalence_basis().clone()
}

pub fn lower_spatial_undo_equivalence_basis_from_selected_plan(
    undo_plan: &SpatialUndoSelectedPlan<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let admitted_input = undo_plan.admitted_input();
    ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::SpatialTouchAuthority,
        crate::replay_undo_semantic_graph::lower_spatial_touched_subjects(
            admitted_input.spatial_touch_authority(),
        ),
        admitted_input.prior_proof_identity().clone(),
        Some(admitted_input.stage_index_identity().clone()),
    )
}
