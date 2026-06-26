use topology::derived_invalidation_authority_inventory::{
    current_derived_invalidation_authority_inventory,
    DerivedInvalidationAuthorityInventoryCloseout, DerivedInvalidationAuthorityInventoryError,
    DerivedInvalidationAuthorityInventoryErrorKind, DerivedInvalidationAuthorityInventoryReport,
    DerivedInvalidationAuthorityInventoryRow, DerivedInvalidationOrdinaryProofAdmission,
};

fn _derived_invalidation_authority_inventory_contract(
) -> Result<(), DerivedInvalidationAuthorityInventoryError> {
    let report: DerivedInvalidationAuthorityInventoryReport =
        current_derived_invalidation_authority_inventory();
    let closeout = DerivedInvalidationAuthorityInventoryCloseout::close(report)?;
    let row: &DerivedInvalidationAuthorityInventoryRow = &closeout.inventory().rows()[0];
    let _ = row.source_path();
    let _ = row.surface();
    let _ = row.owner();
    let _ = row.blocker();
    let _ = row.removal_trigger();
    let _ = closeout.source_scan().scanned_source_count();
    let _ = closeout.source_scan().observed_pattern_count();
    let _ = closeout.inventory().counters().migrate_count();
    let _ = closeout.inventory().counters().ordinary_path_count();

    let residue = closeout
        .inventory()
        .rows()
        .iter()
        .find(|row| row.certification_or_bootstrap_only())
        .expect("public contract expects a certification residue proof row");
    let denial = DerivedInvalidationOrdinaryProofAdmission::admit_inventory_row(residue)
        .expect_err("certification residue cannot satisfy ordinary invalidation");
    assert!(matches!(
        denial.kind(),
        DerivedInvalidationAuthorityInventoryErrorKind::CertificationResidueCannotSatisfyOrdinaryInvalidation { .. }
    ));
    Ok(())
}
