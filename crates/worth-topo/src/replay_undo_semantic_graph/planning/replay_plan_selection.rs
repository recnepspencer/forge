use super::replay_plan::TopologyReplaySelectedPlan;
use super::replay_plan_error::TopologyReplayPlanError;
use crate::replay_family_catalog::{
    current_topology_replay_family_catalog, TopologyReplayFamilyScopeProductPosture,
};
use crate::replay_undo_semantic_graph::TopologyReplaySemanticGraphAdmittedInput;

pub fn select_topology_replay_plan<'a>(
    admitted_input: &'a TopologyReplaySemanticGraphAdmittedInput<'a>,
) -> Result<TopologyReplaySelectedPlan<'a>, TopologyReplayPlanError> {
    let catalog = current_topology_replay_family_catalog();
    let declaration = catalog
        .require_family(admitted_input.family_identity())
        .expect("admitted replay input must come from a declared replay family");
    let scope_product_posture = declaration.scope_product_posture();
    if scope_product_posture
        != TopologyReplayFamilyScopeProductPosture::RequiresTopologyReplayScopeProduct
    {
        return Err(TopologyReplayPlanError::UnsupportedScopeProductPosture {
            family_identity: declaration.identity(),
            scope_product_posture,
        });
    }
    Ok(TopologyReplaySelectedPlan::new(
        declaration.identity(),
        admitted_input,
        scope_product_posture,
    ))
}
