use super::admission_test_support::{
    assert_indexed_single_receipt_lookup, segment_pair_request_subject,
};
use super::*;

#[test]
fn ledger_locality_uses_stage_index_lookup_product() {
    let subject = segment_pair_request_subject();
    let lookup = subject
        .complete
        .require_boolean_receipt_lookup(&subject.receipt)
        .expect("receipt-backed row should produce lookup product");

    assert_eq!(lookup.lookup_counters().required_stage_count(), 1);
    assert_eq!(lookup.lookup_counters().indexed_lookup_count(), 1);
    assert_eq!(lookup.lookup_counters().raw_row_scan_count(), 0);
    assert_eq!(lookup.lookup_counters().rejected_raw_row_scan_count(), 0);
    assert_eq!(
        lookup
            .lookup_counters()
            .rejected_string_prefix_stage_link_count(),
        0
    );

    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&subject.receipt)
        .with_complete_ledger(&subject.complete)
        .admit()
        .expect("receipt plus indexed complete ledger should admit");
    assert_indexed_single_receipt_lookup(&authority);
}
