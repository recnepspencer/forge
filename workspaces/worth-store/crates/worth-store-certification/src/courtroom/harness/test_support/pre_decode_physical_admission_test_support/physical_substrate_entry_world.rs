use super::super::{
    bounded_memory_closeout_test_support::{
        background_bundle, foundational_receipt, foundational_receipt_with_protected_view,
        harness_evidence, physical_substrate_readiness, pressure_bundles, synthetic_rejections,
    },
    record_view_evidence_test_support::{admit_payload_frame, resident_frame_table},
};
use super::{
    checksum_fixture::checksum_declaration,
    physical_substrate_witness_world::{current_validation, frame_witness},
};
use crate::{
    BoundedMemoryCloseoutReport, BoundedMemoryOperationKind, BoundedMemoryResidencySuite,
    BoundedOperationEnvelopeReport, BufferPoolCertificationBundle, MemoryBoundaryDenialKind,
};
use worth_store_physical_format::{
    PhysicalHeaderDecodeWitness, PhysicalReferenceValidationWitness,
};
use worth_store_physical_integrity::{
    IntegrityEntryAdmission, IntegrityEntryRequest, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionSeed, ProtectedPhysicalByteView,
};

pub(crate) fn with_pre_decode_admission(
    payload: &[u8],
    run: impl FnOnce(
        PhysicalIntegrityAdmission<'_>,
        PhysicalReferenceValidationWitness,
        PhysicalHeaderDecodeWitness,
    ),
) {
    with_entry_seed(payload, |seed| {
        let admission = pre_decode_admission_from_seed(seed);
        run(admission, current_validation(), frame_witness(payload));
    });
}

pub(crate) fn with_pre_decode_seed(
    payload: &[u8],
    run: impl FnOnce(PhysicalIntegrityAdmission<'_>),
) {
    with_entry_seed(payload, |seed| {
        run(pre_decode_admission_from_seed(seed));
    });
}

pub(crate) fn with_entry_seed(
    payload: &[u8],
    run: impl FnOnce(PhysicalIntegrityAdmissionSeed<'_>),
) {
    let readiness = complete_closeout_report()
        .publish_physical_integrity_readiness(physical_substrate_readiness())
        .unwrap();
    let entry =
        IntegrityEntryAdmission::from_physical_integrity_payload(readiness.payload()).unwrap();
    let mut table = resident_frame_table();
    let frame = admit_payload_frame(&mut table, 7, 2, payload);
    let page = table.lease_page(frame.resident_frame_token()).unwrap();
    let pinned = page.pin().unwrap();
    let view = pinned.view().unwrap();
    let protected = ProtectedPhysicalByteView::from_pinned_frame(&view);
    let lease = entry.admit(IntegrityEntryRequest::new(protected)).unwrap();
    run(PhysicalIntegrityAdmission::from_entry(lease));
}

fn pre_decode_admission_from_seed(
    seed: PhysicalIntegrityAdmissionSeed<'_>,
) -> PhysicalIntegrityAdmission<'_> {
    let entry_witness = seed.entry_witness();
    seed.with_checksum_declaration(
        checksum_declaration().admit_for_physical_integrity_entry(entry_witness),
    )
    .unwrap()
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
        &MemoryBoundaryDenialKind::ALL,
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
