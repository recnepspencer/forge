use crate::orchestration_inventory::ForgeQueryOrchestrationSurfaceFamily;
use crate::public_doc_coverage::ForgeQueryPublicDocCoverageInventory;

#[test]
fn helper_rows_stay_visible_as_family_helper_coverage() {
    let coverage = ForgeQueryPublicDocCoverageInventory::current();
    let helper = coverage
        .row_for_public_name("prepare_preview_for_active_face_selection")
        .expect("helper row should exist");

    assert_eq!(
        helper.orchestration_family(),
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
    );
    assert_eq!(helper.readme_discovery_label(), "Family Helpers");
    assert_eq!(
        helper.doc_reference().path(),
        "crates/forge-query/docs/domain-capabilities/family-helpers.md"
    );
}

#[test]
fn grouped_rows_stay_visible_as_grouped_authoring_discovery() {
    let coverage = ForgeQueryPublicDocCoverageInventory::current();
    let grouped = coverage
        .row_for_public_name("orchestrate_local_neighborhood_for_active_face_selection")
        .expect("grouped helper row should exist");

    assert_eq!(
        grouped.orchestration_family(),
        ForgeQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration
    );
    assert_eq!(grouped.readme_discovery_label(), "Grouped Authoring");
    assert!(grouped.has_journey_coverage());
}
