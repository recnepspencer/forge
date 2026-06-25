use crate::topology_operators::{
    TopologyTouchedOperatingWorld, TopologyTouchedOperatingWorldIdentityDigest,
};
use crate::validator_invariant_catalog::WorthTopologyLegalitySelectionCloseout;

use super::super::production_phase_two_closeout;
use super::fixtures::{
    routing_closure_for_loop_touch, routing_closure_for_rewire_operator, selected_family_names,
};

#[test]
fn phase_three_selects_obligations_through_query_from_touched_closure() {
    let closeout = production_phase_two_closeout();
    let routing_closure =
        routing_closure_for_rewire_operator(TopologyTouchedOperatingWorld::mainline());

    let selection =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &closeout,
            &routing_closure,
        )
        .expect("Phase 3 selection should preserve catalog projection authority");

    assert_eq!(
        selection.selected_plan().catalog_digest(),
        closeout.catalog().catalog_digest()
    );
    assert_eq!(
        selection.selected_plan().routing_closure_digest(),
        routing_closure.closure_digest()
    );
    assert!(
        selected_family_names(&selection, &closeout)
            .iter()
            .any(|name| name == "loop_wiring"),
        "rewire operator closure should select loop wiring through Query obligation authority"
    );
    assert_eq!(
        selection
            .selected_plan()
            .counters()
            .whole_view_residue_count(),
        0
    );
    assert!(!selection.claims_enforcement_receipts());
}

#[test]
fn operator_declaration_path_selects_legality_obligations_without_test_basis_shortcut() {
    let closeout = production_phase_two_closeout();
    let routing_closure =
        routing_closure_for_rewire_operator(TopologyTouchedOperatingWorld::mainline());
    let selection =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &closeout,
            &routing_closure,
        )
        .expect("operator declaration path should select from Query obligation index");

    assert!(routing_closure.semantic_family_key().contains("rewire"));
    assert!(selection
        .selected_plan()
        .selected_obligation_rows()
        .iter()
        .all(|row| !row.worth_family_identity_digest().is_empty()));
    assert_eq!(
        selection
            .selected_plan()
            .counters()
            .query_registration_full_scan_count(),
        0
    );
}

#[test]
fn operating_world_identity_remains_part_of_routing_closure_authority() {
    let branch = routing_closure_for_loop_touch(TopologyTouchedOperatingWorld::branch(
        TopologyTouchedOperatingWorldIdentityDigest::for_test("branch-a"),
    ));
    let preview = routing_closure_for_loop_touch(TopologyTouchedOperatingWorld::preview(
        TopologyTouchedOperatingWorldIdentityDigest::for_test("preview-a"),
    ));

    assert_ne!(branch.closure_digest(), preview.closure_digest());
    assert_eq!(branch.operating_world_posture(), "branch");
    assert_eq!(preview.operating_world_posture(), "preview");
    assert_eq!(branch.operating_world_identity_digest(), Some("branch-a"));
    assert_eq!(
        branch
            .query_operating_world_descriptor()
            .descriptor_digest(),
        forge_query::facade::ForgeQueryGraphObligationOperatingWorldDescriptor::branch()
            .descriptor_digest()
    );
}

#[test]
fn selected_plan_counters_are_bounded_by_touched_basis_counters() {
    let closeout = production_phase_two_closeout();
    let routing_closure = routing_closure_for_loop_touch(TopologyTouchedOperatingWorld::mainline());
    let selection =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &closeout,
            &routing_closure,
        )
        .expect("Phase 3 counters should build from selected Query obligation proof");

    assert_eq!(
        selection.selected_plan().counters().touched_entity_count(),
        2
    );
    assert_eq!(
        selection
            .selected_plan()
            .counters()
            .touched_relation_count(),
        1
    );
    assert_eq!(
        selection.selected_plan().counters().touched_aspect_count(),
        2
    );
    assert_eq!(
        selection.selected_plan().counters().touched_scope_count(),
        2
    );
    assert_eq!(
        selection
            .selected_plan()
            .phase_four_seed()
            .enforcement_receipt_count(),
        0
    );
    assert_eq!(
        selection.selected_plan().counters().budget_denial_count(),
        0
    );
}
