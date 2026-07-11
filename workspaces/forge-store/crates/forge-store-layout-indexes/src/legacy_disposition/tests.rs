#[test]
fn legacy_disposition_home_exists() {
    let boundary = super::LegacySurfaceDispositionAndDedicatedWorkspaceBoundary::current();
    let inventory = boundary.inventory();

    assert_eq!(boundary.dedicated_workspace_crate(), "forge-store");
    assert!(!boundary.legacy_topology_is_precedent());
    assert_eq!(
        inventory.rows().len(),
        super::inventory::legacy_surface_rows().len()
    );
    assert!(boundary.forbids_legacy_authority("ForgeStore"));
    assert!(boundary.forbids_legacy_authority("ForgeStoreBuilder"));
    assert!(boundary.forbids_legacy_authority("ForgeStore::plan_aspect_layout_read"));
    assert!(boundary.forbids_legacy_authority("ForgeStore::admit_structural_block_reuse"));
    assert!(boundary.forbids_legacy_authority("ForgeStore::execute_aspect_layout_read"));
    assert!(boundary.forbids_legacy_authority("AspectLayoutReadPlanDecision"));
    assert!(boundary.forbids_legacy_authority("Milestone6ChunkModelExport"));
    assert!(boundary.forbids_legacy_authority("Milestone7IndependentReference"));
    assert!(boundary.forbids_legacy_authority("AspectLayoutReadRequest"));
    assert!(boundary.forbids_legacy_authority("Milestone6LayoutMaterializationReport"));
}

pub(crate) fn exercise_owner_outcome_cases() {
    let outcome = super::LegacySurfaceDispositionAndDedicatedWorkspaceBoundary::current()
        .inventory()
        .disposition_for("ForgeStore");
    assert_eq!(
        outcome.production_transition().outcome_case().name(),
        "Classified"
    );
}
