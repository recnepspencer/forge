use crate::validation_authority_inventory::{
    WorthValidationAuthorityInventory, WorthValidationAuthorityMilestoneEightSeedSummary,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogCloseout, WorthTopologyLegalityCatalogError,
};

#[test]
fn milestone_eight_summary_must_not_claim_validator_selection() {
    let inventory = WorthValidationAuthorityInventory::from_current_sources()
        .expect("Phase 1 inventory should build");
    let seed = WorthValidationAuthorityMilestoneEightSeedSummary::from_parts(
        "dishonest-seed",
        true,
        true,
        true,
    );

    let error =
        WorthTopologyLegalityCatalogCloseout::from_phase_one_inventory_and_milestone_eight_summary(
            &inventory, &seed,
        )
        .expect_err("Milestone 8 seed cannot claim validator selection");

    assert!(matches!(
        error,
        WorthTopologyLegalityCatalogError::MilestoneEightSeedClaimsValidatorSelection(_)
    ));
}
