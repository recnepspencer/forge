use crate::topology_operators::TopologyTouchedOperatingWorld;
use crate::validator_invariant_catalog::test_support::{
    catalog_closeout_from_test_family_rows, WorthTopologyLegalityTestFamilyRow,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogSourceFirewallReport, WorthTopologyLegalitySelectionCloseout,
};

use super::fixtures::{
    loop_touch_applicability, routing_closure_for_loop_touch, unrelated_geometry_applicability,
};

#[test]
fn unrelated_catalog_families_do_not_inflate_touched_selection_breadth() {
    let mut rows = vec![WorthTopologyLegalityTestFamilyRow::invariant(
        "matching_loop",
        loop_touch_applicability(),
    )];
    rows.extend((0..24).map(|index| {
        WorthTopologyLegalityTestFamilyRow::invariant(
            format!("unrelated_geometry_{index}"),
            unrelated_geometry_applicability(),
        )
    }));
    let closeout = catalog_closeout_from_test_family_rows(rows)
        .expect("breadth catalog should lower through real Query projection");
    let routing_closure = routing_closure_for_loop_touch(TopologyTouchedOperatingWorld::mainline());

    let selection =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &closeout,
            &routing_closure,
        )
        .expect("small closure should select from pressure catalog");

    assert_eq!(
        selection
            .selected_plan()
            .counters()
            .selected_obligation_count(),
        1
    );
    assert!(
        selection
            .selected_plan()
            .counters()
            .selected_obligation_count()
            < closeout.catalog().records().len()
    );
    assert_eq!(
        selection
            .selected_plan()
            .counters()
            .query_registration_full_scan_count(),
        0
    );
}

#[test]
fn selection_authority_firewall_scans_wider_regions_for_old_validation_residue() {
    let report = WorthTopologyLegalityCatalogSourceFirewallReport::for_selection_authority()
        .expect("selection authority firewall should scan production regions");

    assert!(report.scanned_file_count() > 10);
    assert!(report.forbidden_token_count() > 0);
    assert!(
        report.violations().is_empty(),
        "selection authority firewall found old validation residue: {:?}",
        report.violations()
    );
}
