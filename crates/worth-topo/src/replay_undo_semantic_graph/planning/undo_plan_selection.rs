use super::{TopologyUndoPlanError, TopologyUndoSelectedPlan};
use crate::replay_undo_semantic_graph::TopologyUndoSemanticGraphAdmittedInput;
use crate::undo_family_catalog::{
    current_topology_undo_family_catalog, TopologyUndoFamilyScopeProductPosture,
};

pub fn select_topology_undo_plan<'a>(
    admitted_input: &'a TopologyUndoSemanticGraphAdmittedInput<'a>,
) -> Result<TopologyUndoSelectedPlan<'a>, TopologyUndoPlanError> {
    let catalog = current_topology_undo_family_catalog();
    let declaration = catalog
        .require_family(admitted_input.family_identity())
        .expect("admitted undo input must come from a declared undo family");
    let scope_product_posture = declaration.scope_product_posture();
    if scope_product_posture
        != TopologyUndoFamilyScopeProductPosture::RequiresTopologyUndoScopeProduct
    {
        return Err(TopologyUndoPlanError::UnsupportedScopeProductPosture {
            family_identity: declaration.identity(),
            scope_product_posture,
        });
    }
    Ok(TopologyUndoSelectedPlan::new(
        declaration.identity(),
        admitted_input,
        scope_product_posture,
    ))
}
