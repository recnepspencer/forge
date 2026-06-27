use super::super::{
    current_evidence_lookup_inventory, EvidenceLookupDisposition, EvidenceLookupInventoryRowScope,
};

#[test]
fn evidence_lookup_inventory_closes_with_real_catalog_surfaces() {
    let closeout = current_evidence_lookup_inventory().expect("phase one inventory closes");

    closeout
        .explain()
        .assert_no_keep_dispositions()
        .expect("no keep disposition survives closeout");
    closeout
        .explain()
        .assert_no_unclassified_surfaces()
        .expect("current source scanner has no unclassified lookup-shaped surface");
    closeout
        .explain()
        .assert_query_rows_are_non_lookup_authority()
        .expect("query rows remain adjacent proof only");
    assert!(!closeout.claims_lookup_execution_authority());
    assert!(!closeout.claims_later_milestone_completion());
    assert_eq!(
        closeout.counters().classified_row_count(),
        closeout.rows().len()
    );
    assert!(
        closeout
            .catalog_validation_report()
            .unexpected_missing_source_rows()
            .is_empty(),
        "catalog rows must point at real source roots"
    );
    assert!(
        closeout
            .catalog_validation_report()
            .unexpected_non_discovered_rows()
            .is_empty(),
        "discovery-required catalog rows must remain lookup-shaped in current source"
    );
}

#[test]
fn current_closeout_counters_are_derived_from_real_rows() {
    let closeout = current_evidence_lookup_inventory().expect("phase one inventory closes");
    let counters = closeout.counters();

    let migrate_rows = closeout
        .rows()
        .iter()
        .filter(|row| row.disposition() == EvidenceLookupDisposition::Migrate)
        .count();
    let family_rows = closeout
        .rows()
        .iter()
        .filter(|row| row.row_scope() == EvidenceLookupInventoryRowScope::FamilySummary)
        .count();
    let concrete_rows = closeout
        .rows()
        .iter()
        .filter(|row| row.row_scope() == EvidenceLookupInventoryRowScope::ConcreteSource)
        .count();

    assert_eq!(counters.migrate_row_count(), migrate_rows);
    assert_eq!(counters.family_summary_row_count(), family_rows);
    assert_eq!(counters.concrete_source_row_count(), concrete_rows);
    assert_eq!(family_rows + concrete_rows, counters.classified_row_count());
}
