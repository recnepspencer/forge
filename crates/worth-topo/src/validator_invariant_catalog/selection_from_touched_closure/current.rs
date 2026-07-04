use crate::replay_undo_semantic_graph::current_topology_invalidation_declared_touch_proof;
use crate::topology_operators::TopologyDeclaredTouchedGraphBasisProof;
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::selection_from_touched_closure::{
    WorthTopologyLegalitySelectionCloseout, WorthTopologyValidatorRoutingClosure,
};
use crate::validator_invariant_catalog::{
    current_worth_topology_legality_catalog_closeout, WorthTopologyLegalityCatalogError,
};

pub fn current_topology_validator_invariant_selection_closeout_for_declared_touch(
    proof: &TopologyDeclaredTouchedGraphBasisProof,
) -> Result<WorthTopologyLegalitySelectionCloseout, WorthTopologyLegalityCatalogError> {
    let milestone_eight_summary =
        WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout();
    let routing_closure =
        WorthTopologyValidatorRoutingClosure::from_declared_touch(proof, &milestone_eight_summary)?;
    let catalog_closeout = current_worth_topology_legality_catalog_closeout()?;
    WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
        &catalog_closeout,
        &routing_closure,
    )
}

pub fn current_topology_validator_invariant_selection_closeout(
) -> Result<WorthTopologyLegalitySelectionCloseout, WorthTopologyLegalityCatalogError> {
    let proof = current_topology_invalidation_declared_touch_proof().map_err(|error| {
        WorthTopologyLegalityCatalogError::SourceFirewall(format!(
            "current validator/invariant selection closeout requires current declared touch proof: {}",
            error.detail()
        ))
    })?;
    current_topology_validator_invariant_selection_closeout_for_declared_touch(&proof)
}
