use super::fixtures::AdmissionSubject;

#[test]
fn admission_product_never_claims_lookup_or_query_authority() {
    let subject = AdmissionSubject::event_ledger();
    let admitted = subject.admit();

    assert!(!admitted
        .product_separation()
        .claims_lookup_product_construction());
    assert!(!admitted.product_separation().claims_lookup_execution());
    assert!(!admitted
        .product_separation()
        .claims_query_descriptor_authority());
    assert!(!admitted
        .product_separation()
        .claims_topology_product_authority());
}
