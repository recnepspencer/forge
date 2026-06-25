mod composition;
mod declare_once;
mod identity_sealing;
mod milestone_nine_closeout;
mod no_execution_boundary;
mod operator_certification_cutover;
mod query_vocabulary_parity;
mod rejection;
mod relational_invariant_catalog;
mod selected_graph_obligation_enforcement;
mod selected_validator_enforcement;
mod selection_from_touched_closure;

use crate::validation_authority_inventory::{
    WorthValidationAuthorityInventory, WorthValidationAuthorityInventoryInput,
    WorthValidationAuthorityMilestoneEightSeedSummary,
};
use crate::validator_invariant_catalog::WorthTopologyLegalityCatalogCloseout;

fn production_phase_two_closeout() -> WorthTopologyLegalityCatalogCloseout {
    let seed =
        WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout();
    let inventory = WorthValidationAuthorityInventory::from_current_sources_with_input(
        WorthValidationAuthorityInventoryInput::from_milestone_eight_seed_summary(seed.clone()),
    )
    .expect("Phase 1 inventory should build");
    WorthTopologyLegalityCatalogCloseout::from_phase_one_inventory_and_milestone_eight_summary(
        &inventory, &seed,
    )
    .expect("Phase 2 catalog should close from Phase 1 and Milestone 8 summary")
}
