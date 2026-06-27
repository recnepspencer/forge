use worth_spatial::facade::evidence_lookup_inventory::{
    current_evidence_lookup_inventory, EvidenceLookupInventoryRowScope,
};

#[test]
fn spatial_public_api_exports_read_only_evidence_lookup_inventory_closeout() {
    let closeout = current_evidence_lookup_inventory().expect("public closeout reads");

    assert!(!closeout.rows().is_empty());
    assert!(!closeout.claims_lookup_execution_authority());
    assert!(!closeout.claims_later_milestone_completion());
    assert_eq!(
        closeout.counters().classified_row_count(),
        closeout.rows().len()
    );
    assert!(closeout
        .rows()
        .iter()
        .any(|row| row.row_scope() == EvidenceLookupInventoryRowScope::ConcreteSource));
    assert!(closeout
        .rows()
        .iter()
        .any(|row| row.row_scope() == EvidenceLookupInventoryRowScope::FamilySummary));
    assert!(closeout
        .catalog_validation_report()
        .unexpected_missing_source_rows()
        .is_empty());
}
