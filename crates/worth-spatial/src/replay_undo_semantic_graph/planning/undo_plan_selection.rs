use super::{SpatialUndoPlanError, SpatialUndoSelectedPlan};
use crate::replay_undo_semantic_graph::SpatialUndoSemanticGraphAdmittedInput;
use crate::undo_family_catalog::{
    current_spatial_undo_family_catalog, SpatialUndoFamilyScopeProductPosture,
};

pub fn select_spatial_undo_plan<'a>(
    admitted_input: &'a SpatialUndoSemanticGraphAdmittedInput<'a>,
) -> Result<SpatialUndoSelectedPlan<'a>, SpatialUndoPlanError> {
    let catalog = current_spatial_undo_family_catalog();
    let declaration = catalog
        .require_family(admitted_input.family_identity())
        .expect("admitted undo input must come from a declared undo family");
    let scope_product_posture = declaration.scope_product_posture();
    if scope_product_posture
        != SpatialUndoFamilyScopeProductPosture::RequiresSpatialUndoScopeProduct
    {
        return Err(SpatialUndoPlanError::UnsupportedScopeProductPosture {
            family_identity: declaration.identity(),
            scope_product_posture,
        });
    }
    Ok(SpatialUndoSelectedPlan::new(
        declaration.identity(),
        admitted_input,
        declaration.workload_dependency_posture(),
        scope_product_posture,
    ))
}
