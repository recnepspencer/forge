use super::super::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupReplacementPhase;

#[test]
fn current_family_catalog_closes_without_execution_claims() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let counters = closeout.counters();

    assert_eq!(counters.family_count(), 3);
    assert_eq!(counters.family_count(), closeout.declarations().len());
    assert_eq!(
        counters.diagnostic_witness_family_count(),
        counters.family_count()
    );
    assert_eq!(counters.query_required_family_count(), 2);
    assert_eq!(counters.topology_required_family_count(), 1);
    assert_eq!(counters.sparse_index_family_count(), 2);
    assert_eq!(counters.bounded_dense_index_family_count(), 1);
    assert!(counters.source_inventory_migrate_row_count() > 0);
    assert!(!closeout.claims_lookup_execution_authority());
    assert!(!closeout.claims_family_selection());
    assert!(!closeout.claims_index_construction());
    assert!(!closeout.claims_query_support_authority());
    assert!(!closeout.catalog_digest().is_empty());
}

#[test]
fn family_catalog_is_searchable_by_stable_identity() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");

    assert!(closeout
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .is_some());
    assert!(closeout
        .family_by_identity("spatial-touch.boolean.missing.v1")
        .is_none());
}

#[test]
fn source_inventory_pressure_is_structured_and_shared_by_family_declarations() {
    let closeout = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let expected_pressure = closeout.declarations()[0].source_inventory_pressure();

    assert_eq!(
        expected_pressure.replacement_phase(),
        EvidenceLookupReplacementPhase::PhaseTwoFamilyCatalog
    );
    assert_eq!(
        expected_pressure.migrate_row_count(),
        closeout.counters().source_inventory_migrate_row_count()
    );
    assert!(!expected_pressure.source_inventory_digest().is_empty());
    for declaration in closeout.declarations() {
        assert_eq!(declaration.source_inventory_pressure(), expected_pressure);
    }
}
