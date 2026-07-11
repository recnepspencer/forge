use forge_store_buffer_pool::{
    AllocationAdmission, AllocationRequest, AllocationScope, RecordViewMaterializationProfile,
};

use crate::{
    courtroom::harness::test_support::record_view_evidence_test_support::{
        admit_payload_frame, allocation_admission, framed_record, resident_frame_table,
    },
    RecordViewEvidenceReport, RecordViewEvidenceRow,
};

#[test]
fn real_zero_copy_and_bounded_copy_views_certify_record_view_rows() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, 7, 2, b"certify-view");
    let framed = framed_record(7, 2, b"certify-view");
    let mut allocation = allocation_admission(32);

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let zero_copy = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();
    let zero_report = RecordViewEvidenceReport::from_zero_copy_view(
        RecordViewEvidenceRow::ZeroCopyLeaseScopedPhysicalBytes,
        &zero_copy,
    )
    .unwrap();
    let request = AllocationRequest::copied_payload(AllocationScope::Foreground, 12).unwrap();
    let grant = allocation.admit(request).unwrap();
    let receipt = allocation.record_allocation(grant).unwrap();
    let bounded = zero_copy.bounded_copy(receipt).unwrap();
    let bounded_report = RecordViewEvidenceReport::from_bounded_copy_view(
        RecordViewEvidenceRow::BoundedCopyRequiresAllocationAndExactCounters,
        &bounded,
    )
    .unwrap();

    assert_eq!(
        zero_report.row(),
        RecordViewEvidenceRow::ZeroCopyLeaseScopedPhysicalBytes
    );
    assert_eq!(
        bounded_report.row(),
        RecordViewEvidenceRow::BoundedCopyRequiresAllocationAndExactCounters
    );
    assert_eq!(bounded_report.counters().copied_bytes(), 12);
}

#[test]
fn real_view_denial_certifies_denial_before_construction() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, 7, 2, b"certify-view");
    let wrong_reference = framed_record(7, 3, b"certify-view");

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let denial = pinned
        .zero_copy_record_view(
            wrong_reference,
            RecordViewMaterializationProfile::PhysicalBytesOnly,
        )
        .unwrap_err();
    let report = RecordViewEvidenceReport::from_view_denial(
        RecordViewEvidenceRow::InvalidInputsDenyBeforeConstruction,
        denial,
    )
    .unwrap();

    assert_eq!(
        report.row(),
        RecordViewEvidenceRow::InvalidInputsDenyBeforeConstruction
    );
}

#[test]
fn allocation_support_uses_real_admission_envelopes() {
    let mut allocation: AllocationAdmission = allocation_admission(16);
    let request =
        AllocationRequest::materialized_record_set(AllocationScope::Foreground, 4).unwrap();
    let grant = allocation.admit(request).unwrap();
    let receipt = allocation.record_allocation(grant).unwrap();

    assert_eq!(receipt.bytes(), 4);
}
