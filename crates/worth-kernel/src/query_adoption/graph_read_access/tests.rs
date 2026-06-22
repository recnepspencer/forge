use super::current_worth_kernel_construction_graph_read_access_adoption;

#[test]
fn construction_graph_read_access_adoption_has_zero_unclassified_bypass_residue() {
    let report = current_worth_kernel_construction_graph_read_access_adoption()
        .expect("construction graph-read adoption should certify");

    assert!(report.source_inventory_count() > 0);
    assert!(report.evaluated_source_count() > 0);
    assert!(report
        .covered_roots()
        .iter()
        .any(|root| root.ends_with("src/construction")));
    assert!(report
        .audited_source_labels()
        .iter()
        .any(|label| label.contains("phase_chain")));
    assert!(report
        .audited_source_labels()
        .iter()
        .any(|label| label.contains("query_access_planning")));
    assert!(!report.source_inventory_identity().is_empty());
    assert_eq!(report.unclassified_finding_count(), 0);
    assert!(!report.adoption_manifest_digest().is_empty());
}
