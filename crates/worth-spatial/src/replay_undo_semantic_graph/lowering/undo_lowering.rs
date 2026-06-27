use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphEquivalenceBasis, UndoScopeIdentity,
};

use super::ReplayUndoSemanticGraphLoweringError;
use crate::replay_undo_semantic_graph::{
    admit_spatial_undo_semantic_graph_input, select_spatial_undo_plan, SpatialUndoPlanError,
    SpatialUndoScopeProduct, SpatialUndoSemanticGraphAdmissionRequest,
    SpatialUndoSemanticGraphAdmittedInput,
};

pub fn lower_spatial_undo_scope_identity(
    request: SpatialUndoSemanticGraphAdmissionRequest<'_>,
) -> Result<UndoScopeIdentity, ReplayUndoSemanticGraphLoweringError> {
    let admitted_input = admit_spatial_undo_semantic_graph_input(request)
        .map_err(ReplayUndoSemanticGraphLoweringError::from)?;
    lower_spatial_undo_scope_identity_from_admitted_input(&admitted_input)
}

pub fn lower_spatial_undo_scope_identity_from_admitted_input(
    admitted_input: &SpatialUndoSemanticGraphAdmittedInput<'_>,
) -> Result<UndoScopeIdentity, ReplayUndoSemanticGraphLoweringError> {
    let undo_plan = select_spatial_undo_plan(admitted_input)
        .map_err(ReplayUndoSemanticGraphLoweringError::from)?;
    let scope_product = lower_spatial_undo_scope_product_from_selected_plan(&undo_plan)?;
    Ok(lower_spatial_undo_scope_identity_from_scope_product(
        &scope_product,
    ))
}

pub fn lower_spatial_undo_equivalence_basis(
    request: SpatialUndoSemanticGraphAdmissionRequest<'_>,
) -> Result<ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLoweringError> {
    let admitted_input = admit_spatial_undo_semantic_graph_input(request)
        .map_err(ReplayUndoSemanticGraphLoweringError::from)?;
    Ok(lower_spatial_undo_equivalence_basis_from_admitted_input(
        &admitted_input,
    ))
}

pub fn lower_spatial_undo_equivalence_basis_from_admitted_input(
    admitted_input: &SpatialUndoSemanticGraphAdmittedInput<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let undo_plan =
        select_spatial_undo_plan(admitted_input).expect("admitted spatial undo input should lower");
    super::super::scope_product::lower_spatial_undo_equivalence_basis_from_selected_plan(&undo_plan)
}

pub fn lower_spatial_undo_scope_product_from_admitted_input<'a>(
    admitted_input: &'a SpatialUndoSemanticGraphAdmittedInput<'a>,
) -> Result<SpatialUndoScopeProduct<'a>, SpatialUndoPlanError> {
    let undo_plan = select_spatial_undo_plan(admitted_input)?;
    lower_spatial_undo_scope_product_from_selected_plan(&undo_plan)
}

pub fn lower_spatial_undo_scope_product_from_selected_plan<'a>(
    undo_plan: &crate::replay_undo_semantic_graph::SpatialUndoSelectedPlan<'a>,
) -> Result<SpatialUndoScopeProduct<'a>, SpatialUndoPlanError> {
    super::super::scope_product::lower_spatial_undo_scope_product_from_selected_plan(undo_plan)
}

pub fn lower_spatial_undo_scope_identity_from_scope_product(
    scope_product: &SpatialUndoScopeProduct<'_>,
) -> UndoScopeIdentity {
    super::super::scope_product::lower_spatial_undo_scope_identity_from_scope_product(scope_product)
}

pub fn lower_spatial_undo_equivalence_basis_from_scope_product(
    scope_product: &SpatialUndoScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    super::super::scope_product::lower_spatial_undo_equivalence_basis_from_scope_product(
        scope_product,
    )
}
