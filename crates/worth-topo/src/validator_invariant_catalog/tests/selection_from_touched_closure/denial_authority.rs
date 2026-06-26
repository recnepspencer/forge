use crate::topology_operators::TopologyTouchedOperatingWorld;
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogError, WorthTopologyLegalitySelectionCloseout,
    WorthTopologyLegalitySelectionDenialKind, WorthTopologyValidatorRoutingClosure,
};

use super::super::production_phase_two_closeout;
use super::fixtures::touched_basis_proof;

#[test]
fn selected_plan_denies_obligations_without_milestone_eight_receipt_context() {
    let closeout = production_phase_two_closeout();
    let proof = touched_basis_proof(TopologyTouchedOperatingWorld::mainline());
    let seed_without_receipts =
        WorthValidationAuthorityMilestoneEightSeedSummary::imported_public_closeout(
            "missing-receipts",
            false,
            true,
        );
    let routing_closure =
        WorthTopologyValidatorRoutingClosure::from_declared_touch(&proof, &seed_without_receipts)
            .expect("missing receipts must remain visible to selected-plan denial rows");

    let selection =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &closeout,
            &routing_closure,
        )
        .expect("missing receipts deny matched obligations instead of aborting routing");

    assert!(selection
        .selected_plan()
        .selected_obligation_rows()
        .is_empty());
    assert!(selection
        .selected_plan()
        .denial_rows()
        .iter()
        .all(|denial| denial.kind()
            == WorthTopologyLegalitySelectionDenialKind::MissingAccessReceipt));
    assert!(
        selection
            .selected_plan()
            .counters()
            .missing_access_receipt_count()
            > 0
    );
}

#[test]
fn routing_closure_rejects_milestone_eight_seed_without_posture_context() {
    let proof = touched_basis_proof(TopologyTouchedOperatingWorld::mainline());
    let seed_without_posture =
        WorthValidationAuthorityMilestoneEightSeedSummary::imported_public_closeout(
            "missing-posture",
            true,
            false,
        );

    let error =
        WorthTopologyValidatorRoutingClosure::from_declared_touch(&proof, &seed_without_posture)
            .expect_err("routing closure must fail closed without support posture context");

    assert!(matches!(
        error,
        WorthTopologyLegalityCatalogError::MissingMilestoneEightReceiptContext
    ));
}
