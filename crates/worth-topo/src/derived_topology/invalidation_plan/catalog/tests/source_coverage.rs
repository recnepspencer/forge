use super::super::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalogCloseout,
    DerivedInvalidationFamilyCatalogErrorKind, DerivedTopologyProductFamilyIdentity,
};
use crate::derived_topology::invalidation_plan::inventory::{
    current_derived_invalidation_authority_inventory,
    DerivedInvalidationAuthorityInventoryCloseout, DerivedInvalidationAuthorityInventoryReport,
    DerivedInvalidationPhaseTwoSeed, DerivedInvalidationProductCategory,
};

#[test]
fn current_family_catalog_validates_against_inventory_source_coverage() {
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout =
        DerivedInvalidationAuthorityInventoryCloseout::close(inventory.clone()).unwrap();
    let catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .unwrap();
    let closeout = DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap();

    let coverage = closeout.validate_source_coverage(&inventory).unwrap();

    assert_eq!(
        coverage.covered_families(),
        DerivedTopologyProductFamilyIdentity::REQUIRED
    );
}

#[test]
fn source_coverage_rejects_inventory_seed_mismatch() {
    let closeout =
        DerivedInvalidationFamilyCatalogCloseout::close(super::current_catalog()).unwrap();
    let altered_inventory = inventory_without_loop_cycles();

    let error = closeout
        .validate_source_coverage(&altered_inventory)
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::InventorySeedMismatch
    );
}

#[test]
fn source_coverage_rejects_required_family_without_inventory_source() {
    let altered_inventory = inventory_without_loop_cycles();
    let phase_two_seed = DerivedInvalidationPhaseTwoSeed::from_inventory_report(&altered_inventory);
    let catalog = current_derived_invalidation_family_catalog(phase_two_seed).unwrap();
    let closeout = DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap();

    let error = closeout
        .validate_source_coverage(&altered_inventory)
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &DerivedInvalidationFamilyCatalogErrorKind::MissingInventorySourceForFamily {
            family: "loop_cycles"
        }
    );
}

fn inventory_without_loop_cycles() -> DerivedInvalidationAuthorityInventoryReport {
    let mut rows = current_derived_invalidation_authority_inventory()
        .rows()
        .to_vec();
    rows.retain(|row| row.product_category() != DerivedInvalidationProductCategory::LoopCycles);
    DerivedInvalidationAuthorityInventoryReport::new(rows)
}
