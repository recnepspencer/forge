#[path = "../../../support/recovery/foundational_evidence_support/foundational_evidence_support.rs"]
mod evidence_support;

#[test]
fn foundational_proof_evidence_split_suite_smoke() {
    let source = evidence_support::verified_source();
    let bundle = evidence_support::bundle_from_source(&source);

    assert_eq!(
        bundle.receipt().recovered_physical_root(),
        source.recovered_state().recovered_physical_root()
    );
    assert_eq!(
        bundle.performance().exact_counter_assertions(),
        bundle.performance().rows().len()
    );
}
