use topology::derived_invalidation_authority_inventory::{
    current_derived_invalidation_authority_inventory as current_catalog_seed_inventory,
    DerivedInvalidationAuthorityInventoryCloseout as CatalogSeedInventoryCloseout,
};
use topology::derived_invalidation_family_catalog::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalogCloseout,
    DerivedTopologyProductFamilyIdentity, DerivedTopologySpatialEvidencePosture,
};

fn _derived_invalidation_family_catalog_contract(
) -> Result<(), Box<dyn std::error::Error>> {
    let inventory = current_catalog_seed_inventory();
    let inventory_closeout = CatalogSeedInventoryCloseout::close(inventory)?;
    let catalog = current_derived_invalidation_family_catalog(
        inventory_closeout.phase_two_seed().clone(),
    )?;
    let closeout = DerivedInvalidationFamilyCatalogCloseout::close(catalog)?;
    let family = closeout
        .catalog()
        .family(DerivedTopologyProductFamilyIdentity::LoopCycles)
        .expect("public contract expects loop-cycle family declaration");

    let _ = family.identity();
    let _ = family.consumed_graph_facts().relation_kinds();
    let _ = family.consumed_graph_facts().aspects();
    let _ = family.query_receipt_posture();
    let _ = family.legality_receipt_posture();
    let _ = family.update_posture();
    let _ = family.family_digest();
    assert_eq!(
        family.spatial_evidence_posture(),
        DerivedTopologySpatialEvidencePosture::NoSpatialEvidenceConsumed
    );
    assert_eq!(closeout.catalog().counters().family_count(), 7);
    assert_eq!(
        closeout
            .catalog()
            .counters()
            .spatial_receipt_required_family_count(),
        0
    );
    assert_eq!(
        closeout.phase_three_seed().catalog_digest(),
        closeout.catalog().catalog_digest()
    );
    Ok(())
}
