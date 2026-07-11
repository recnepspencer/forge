use crate::{
    courtroom::harness::test_support::bounded_memory_closeout_test_support::{
        background_bundle, foundational_receipt, foundational_receipt_with_protected_view,
        harness_evidence, pressure_bundles, s2_readiness, synthetic_rejections,
    },
    courtroom::harness::test_support::record_view_evidence_test_support::{
        admit_payload_frame, resident_frame_table,
    },
    BoundedMemoryCloseoutReport, BoundedMemoryOperationKind, BoundedMemoryResidencySuite,
    BoundedOperationEnvelopeReport, BufferPoolCertificationBundle, S2BoundaryDenialKind,
};
use forge_store_physical_integrity::{
    IntegrityEntryAdmission, IntegrityEntryBasis, IntegrityEntryDenialKind, IntegrityEntryRequest,
    ProtectedPhysicalByteView, ScrubEnvelopeLimits, VerifierResidentLimits,
};
use forge_store_readiness::S2DeniedBoundaryKind;

#[test]
fn equivalent_s2_closeouts_lower_to_same_s3_entry_basis_and_scrub_limits() {
    let first = admit_entry_from_independent_closeout(b"phase1-first");
    let second = admit_entry_from_independent_closeout(b"phase1-second");

    assert_equivalent_entry_authority(first, second);
}

#[test]
fn s3_entry_admits_only_live_protected_s2_views() {
    let admitted = admit_entry_from_independent_closeout(b"phase1-live-view");

    assert_eq!(admitted.protected_bytes, b"phase1-live-view");
    assert!(admitted.basis.protected_view_count() > 0);
}

#[test]
fn s3_entry_denies_empty_live_protected_s2_view_before_witness_minting() {
    let readiness = complete_closeout_report()
        .publish_s3_physical_integrity_readiness(s2_readiness())
        .unwrap();
    let admission = IntegrityEntryAdmission::from_s3_payload(readiness.payload()).unwrap();
    let mut table = resident_frame_table();
    let frame = admit_payload_frame(&mut table, 31, 5, b"");
    let page = table.lease_page(frame.resident_frame_token()).unwrap();
    let pinned = page.pin().unwrap();
    let view = pinned.view().unwrap();
    let protected = ProtectedPhysicalByteView::from_pinned_frame(&view);

    let denial = admission
        .admit(IntegrityEntryRequest::new(protected))
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        IntegrityEntryDenialKind::MissingProtectedPhysicalByteView
    );
}

#[derive(Debug, PartialEq, Eq)]
struct EntryAdmissionObservation {
    basis: IntegrityEntryBasis,
    verifier_limits: VerifierResidentLimits,
    scrub_limits: ScrubEnvelopeLimits,
    protected_bytes: Vec<u8>,
}

fn admit_entry_from_independent_closeout(payload: &[u8]) -> EntryAdmissionObservation {
    let readiness = complete_closeout_report()
        .publish_s3_physical_integrity_readiness(s2_readiness())
        .unwrap();
    let admission = IntegrityEntryAdmission::from_s3_payload(readiness.payload()).unwrap();
    let mut table = resident_frame_table();
    let frame = admit_payload_frame(&mut table, 31, 5, payload);
    let page = table.lease_page(frame.resident_frame_token()).unwrap();
    let pinned = page.pin().unwrap();
    let view = pinned.view().unwrap();
    let protected = ProtectedPhysicalByteView::from_pinned_frame(&view);

    let lease = admission
        .admit(IntegrityEntryRequest::new(protected))
        .unwrap();
    assert!(!lease.entry_witness().proves_recovery_behavior());
    assert!(!lease.entry_witness().proves_blob_lifecycle());
    assert!(!lease.entry_witness().proves_repair_behavior());
    assert!(!lease.entry_witness().proves_authenticity());
    assert!(!lease.entry_witness().proves_certification_closeout());
    EntryAdmissionObservation {
        basis: lease.entry_witness().entry_basis(),
        verifier_limits: lease.entry_witness().verifier_resident_limits(),
        scrub_limits: lease.scrub_envelope_limits(),
        protected_bytes: lease.protected_bytes().as_bytes().to_vec(),
    }
}

fn assert_equivalent_entry_authority(
    first: EntryAdmissionObservation,
    second: EntryAdmissionObservation,
) {
    assert_eq!(
        first.basis.protected_view_count(),
        second.basis.protected_view_count()
    );
    assert_eq!(first.verifier_limits, second.verifier_limits);
    assert_eq!(first.scrub_limits, second.scrub_limits);
    assert_eq!(first.basis.counter_recap(), second.basis.counter_recap());
    assert_eq!(
        first.basis.denial_behavior(),
        second.basis.denial_behavior()
    );
    assert_eq!(
        first.basis.denial_behavior().named_denial_count(),
        S2DeniedBoundaryKind::ALL.len() as u32
    );
    assert_eq!(
        first.basis.physical_authority_recap(),
        second.basis.physical_authority_recap()
    );
    assert_eq!(
        first.basis.buffer_pool_authority_recap(),
        second.basis.buffer_pool_authority_recap()
    );
    assert!(first
        .basis
        .buffer_pool_authority_recap()
        .lease_pinning_proven());
    assert!(first
        .basis
        .buffer_pool_authority_recap()
        .resident_frame_authority_proven());
    assert!(first
        .basis
        .buffer_pool_authority_recap()
        .allocation_envelope_proven());
    assert!(first
        .basis
        .buffer_pool_authority_recap()
        .view_admission_authority_proven());
    assert_eq!(first.basis, second.basis);
}

fn complete_closeout_report() -> BoundedMemoryCloseoutReport {
    let (foundational, protected_view) = foundational_receipt_with_protected_view();
    BoundedMemoryCloseoutReport::close(
        BufferPoolCertificationBundle::admit(
            suite(),
            pressure_bundles(),
            background_bundle(),
            foundational,
            protected_view,
            synthetic_rejections(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn suite() -> BoundedMemoryResidencySuite {
    BoundedMemoryResidencySuite::admit(
        operation_reports(),
        &S2BoundaryDenialKind::ALL,
        harness_evidence(),
    )
    .unwrap()
}

fn operation_reports() -> Vec<BoundedOperationEnvelopeReport> {
    let background = background_bundle();
    crate::courtroom::harness::test_support::bounded_memory_closeout_test_support::operation_reports(
        &foundational_receipt(),
        &background,
    )
    .into_iter()
    .filter(|report| {
        matches!(
            report.operation(),
            BoundedMemoryOperationKind::AdmittedRead
                | BoundedMemoryOperationKind::AdmittedWrite
                | BoundedMemoryOperationKind::RecoveryPlanning
                | BoundedMemoryOperationKind::CompactionPlanning
                | BoundedMemoryOperationKind::LargeRecordStreaming
        )
    })
    .collect()
}
