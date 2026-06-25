use forge_query::facade::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationOperatingWorldDescriptor,
};

use super::production_phase_two_closeout;
use crate::topology_operators::{
    TopologyGraphLifecyclePosture, TopologyTouchedAspect, TopologyTouchedScope,
};
use crate::validator_invariant_catalog::test_support::{
    catalog_closeout_from_test_family_rows, WorthTopologyLegalityTestFamilyRow,
};
use crate::validator_invariant_catalog::WorthTopologyTouchedApplicability;

#[test]
fn one_family_declaration_selects_for_multiple_matching_touched_closures() {
    let closeout = production_phase_two_closeout();
    let catalog = closeout.catalog();
    let loop_touch = WorthTopologyTouchedApplicability::from_parts(
        [
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedAspect::TopologyStructure,
        ],
        [TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    );
    let shell_touch = WorthTopologyTouchedApplicability::from_parts(
        [
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedAspect::TopologyStructure,
        ],
        [
            TopologyTouchedScope::Shell,
            TopologyTouchedScope::LocalNeighborhood,
        ],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    );
    let descriptor_one = loop_touch
        .query_touch_descriptor()
        .expect("operator loop touch should lower to Query touch descriptor");
    let descriptor_two = shell_touch
        .query_touch_descriptor()
        .expect("operator shell touch should lower to Query touch descriptor");
    let index =
        ForgeQueryGraphObligationIndex::from_catalog(catalog.query_projection().query_catalog());
    let operating_world =
        ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();

    let selected_one = index.select_for_touch(&descriptor_one, &operating_world);
    let selected_two = index.select_for_touch(&descriptor_two, &operating_world);

    let selected_one_loop = selected_one
        .matched_registrations()
        .iter()
        .find(|registration| registration.rule_identity().name() == "loop_wiring")
        .expect("loop wiring should be selected for first matching touch");
    let selected_two_loop = selected_two
        .matched_registrations()
        .iter()
        .find(|registration| registration.rule_identity().name() == "loop_wiring")
        .expect("loop wiring should be selected for second matching touch");

    assert_eq!(
        selected_one_loop.registration_digest(),
        selected_two_loop.registration_digest()
    );
}

#[test]
fn one_catalog_overlay_family_changes_multiple_matching_operator_selections() {
    let matching_touch = WorthTopologyTouchedApplicability::from_parts(
        [TopologyTouchedAspect::TopologyStructure],
        [TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    );
    let closeout =
        catalog_closeout_from_test_family_rows([WorthTopologyLegalityTestFamilyRow::invariant(
            "overlay_loop_topology_authority",
            matching_touch,
        )])
        .expect("test overlay catalog should build through real Query projection");
    let index = ForgeQueryGraphObligationIndex::from_catalog(
        closeout.catalog().query_projection().query_catalog(),
    );
    let first_operator_touch = WorthTopologyTouchedApplicability::from_parts(
        [
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedAspect::TopologyStructure,
        ],
        [TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    )
    .query_touch_descriptor()
    .expect("first touched basis should lower to Query");
    let second_operator_touch = WorthTopologyTouchedApplicability::from_parts(
        [TopologyTouchedAspect::TopologyStructure],
        [
            TopologyTouchedScope::Loop,
            TopologyTouchedScope::Relation,
            TopologyTouchedScope::LocalNeighborhood,
        ],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    )
    .query_touch_descriptor()
    .expect("second touched basis should lower to Query");
    let operating_world =
        ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();

    let first_selection = index.select_for_touch(&first_operator_touch, &operating_world);
    let second_selection = index.select_for_touch(&second_operator_touch, &operating_world);
    let first_registration = first_selection
        .matched_registrations()
        .iter()
        .find(|registration| {
            registration.rule_identity().name() == "overlay_loop_topology_authority"
        })
        .expect("overlay family should select for first matching operator touch");
    let second_registration = second_selection
        .matched_registrations()
        .iter()
        .find(|registration| {
            registration.rule_identity().name() == "overlay_loop_topology_authority"
        })
        .expect("overlay family should select for second matching operator touch");

    assert_eq!(
        first_registration.registration_digest(),
        second_registration.registration_digest(),
        "one catalog overlay family must propagate through Query selection for every matching touched basis"
    );
}
