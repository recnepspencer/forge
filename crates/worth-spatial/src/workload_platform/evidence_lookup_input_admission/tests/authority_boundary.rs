use super::fixtures::AdmissionSubject;

#[test]
fn lookup_input_requires_sealed_spatial_touch_authority() {
    let subject = AdmissionSubject::event_ledger();

    let admitted = subject.admit();

    assert_eq!(admitted.family_selection().family_count(), 1);
    assert_eq!(admitted.counters().catalog_candidate_family_count(), 3);
    assert_eq!(admitted.counters().raw_row_scan_count(), 0);
    assert_eq!(admitted.counters().lookup_product_construction_count(), 0);
    assert_eq!(
        admitted.stage_receipt_digest(),
        subject.authority().evidence_identity()
    );
    assert!(!admitted.claims_lookup_product_construction());
    assert!(!admitted.claims_lookup_execution());
}
