use super::super::current_evidence_lookup_query_consumer_kit;

#[test]
fn query_residue_and_spatial_lookup_residue_are_separate() {
    let closeout = current_evidence_lookup_query_consumer_kit().expect("consumer kit closeout");

    assert_eq!(
        closeout.query_residue_rows().len(),
        closeout.counters().query_residue_row_count()
    );
    assert!(!closeout.claims_spatial_lookup_residue_authority());
    assert!(closeout
        .query_residue_rows()
        .iter()
        .all(|row| !row.report_identity().is_empty() && !row.source_inventory_digest().is_empty()));
}
