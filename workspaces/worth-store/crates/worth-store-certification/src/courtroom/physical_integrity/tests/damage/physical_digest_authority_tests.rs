use worth_store_aspect_native::StoreCanonicalBasisFamily;
use worth_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceProfile,
    StoreExecutedIntegrityEvidence, StoreIntegrityBoundaryClaim,
};

use crate::courtroom::harness::test_support::physical_container_integrity_test_support::{
    inspect_page_report, inspect_page_report_for_cell, page_payload_with_record,
    page_payload_with_record_for_cell, PageReportFixtureCell,
};

#[test]
fn physical_authority_claim_carries_typed_store_digest_evidence() {
    let payload = page_payload_with_record(b"phase-6-typed-authority-digest");
    let report = inspect_page_report(&payload);
    let claim = physical_authority_claim_for_report(&report);

    assert_eq!(
        claim.basis().family(),
        StoreCanonicalBasisFamily::PhysicalIntegrityEvidence
    );
    assert_eq!(
        claim.basis().source_kind(),
        worth_store_aspect_native::StoreCanonicalBasisSourceKind::StorePhysicalIntegrityEvidence
    );
    assert_eq!(
        claim.basis().equivalence_basis_identity().family(),
        StoreCanonicalBasisFamily::PhysicalIntegrityEvidence
    );
}

#[test]
fn physical_authority_digest_changes_with_native_physical_witness() {
    let first_cell = PageReportFixtureCell::new(1, 2, 3, 7);
    let second_cell = PageReportFixtureCell::new(1, 4, 3, 9);
    let first_payload = page_payload_with_record_for_cell(b"same-sized-record", first_cell);
    let second_payload = page_payload_with_record_for_cell(b"same-sized-record", second_cell);
    let first_claim = physical_authority_claim_for_report(&inspect_page_report_for_cell(
        &first_payload,
        first_cell,
    ));
    let second_claim = physical_authority_claim_for_report(&inspect_page_report_for_cell(
        &second_payload,
        second_cell,
    ));

    assert_eq!(first_claim.basis().family(), second_claim.basis().family());
    assert_eq!(
        first_claim.basis().source_kind(),
        second_claim.basis().source_kind()
    );
    assert_eq!(
        first_claim.basis().equivalence_basis_identity(),
        second_claim.basis().equivalence_basis_identity()
    );
    assert_ne!(
        first_claim.basis().canonical_digest().value().bytes(),
        second_claim.basis().canonical_digest().value().bytes()
    );
}

fn physical_authority_claim_for_report(
    report: &worth_store_physical_integrity::PageIntegrityReport,
) -> worth_store_physical_integrity::StorePhysicalAuthorityBoundaryClaim {
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    match evidence.store_claim() {
        StoreIntegrityBoundaryClaim::PhysicalAuthority(claim) => claim.clone(),
        other => panic!("expected physical authority claim, got {other:?}"),
    }
}
