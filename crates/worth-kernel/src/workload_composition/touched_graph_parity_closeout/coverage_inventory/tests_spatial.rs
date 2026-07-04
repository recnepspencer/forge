use super::current::current_cross_family_coverage_inventory;
use super::row::CrossFamilyCoverageFamilyKind as FamilyKind;

#[test]
fn spatial_inventory_includes_retained_public_closeout_row() {
    let inventory =
        current_cross_family_coverage_inventory().expect("cross-family coverage inventory");
    let retained_row = inventory
        .rows()
        .iter()
        .find(|row| {
            row.family_kind() == FamilyKind::RetainedSpatial
                && row.current_surface() == "current_evidence_lookup_public_closeout"
        })
        .expect("retained spatial evidence row should exist");

    assert_eq!(
        retained_row.source_path(),
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs"
    );
    assert_eq!(
        retained_row.query_surface_kind().as_str(),
        "consumer_residue"
    );
    assert_eq!(
        retained_row.ordinary_path_live_caller_surface(),
        "current_evidence_lookup_public_closeout"
    );
    assert_eq!(
        retained_row.ordinary_path_live_caller_path(),
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs"
    );
}
