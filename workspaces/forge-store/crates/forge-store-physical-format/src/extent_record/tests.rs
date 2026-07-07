use crate::{
    AllocationClassKind, ExtentMembership, ExtentRecordAppendRequest, ExtentRecordDenialKind,
    PhysicalBinaryEncodingWitness, PhysicalExtentId, PhysicalExtentRecordAuthority,
    PhysicalFrameKind, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalPageId, PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalReferenceDenialKind,
    PhysicalReferenceKind, PhysicalSegmentId, RecordPlacementClass, PHYSICAL_HEADER_LENGTH,
};

#[test]
fn extent_backed_large_record_reopens_with_stable_reference() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let extent_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(5));
    let payload = b"large physical record payload";
    let membership = ExtentMembership::large_record(extent_cell, extent_frame_len(payload));

    assert_eq!(
        membership.allocation_class(),
        Some(AllocationClassKind::LargeRecordExtent)
    );
    assert_eq!(membership.owner(), Some(extent_cell.owner()));

    let append = records
        .append_extent_record(
            membership,
            ExtentRecordAppendRequest::large_record(extent_cell, payload),
        )
        .unwrap();

    assert_eq!(
        append.reference_admission().reference().kind(),
        PhysicalReferenceKind::ExtentBacked
    );
    assert_eq!(
        append.placement().reference(),
        append.reference_admission().reference()
    );
    assert_eq!(
        append.placement().placement_class(),
        RecordPlacementClass::ExtentBackedReference
    );
    assert_eq!(append.counters().extent_membership_check_count(), 1);
    assert_eq!(append.counters().extent_length_check_count(), 1);
    assert_eq!(append.counters().extent_write_count(), 1);
    assert_eq!(append.counters().extent_read_count(), 0);
    assert_eq!(append.counters().extent_locate_count(), 0);
    assert_eq!(append.counters().extent_payload_view_count(), 0);

    let validation = references
        .validate_extent(append.reference_admission(), extent_cell)
        .unwrap();
    let located = records
        .locate_extent_record(append.extent_bytes(), membership, validation)
        .unwrap();

    assert_eq!(
        located.reference(),
        append.reference_admission().reference()
    );
    assert_eq!(
        located.record_view().frame_kind(),
        PhysicalFrameKind::ExtentRecordFrame
    );
    assert_eq!(located.record_view().payload().as_bytes(), payload);
    assert_eq!(
        located.record_view().placement().placement_class(),
        RecordPlacementClass::ExtentBackedReference
    );
    assert_eq!(located.counters().extent_read_count(), 1);
    assert_eq!(located.counters().extent_locate_count(), 1);
    assert_eq!(located.counters().extent_membership_check_count(), 1);
    assert_eq!(located.counters().extent_length_check_count(), 1);
    assert_eq!(located.counters().extent_header_decode_count(), 1);
    assert_eq!(located.counters().extent_payload_view_count(), 1);
}

#[test]
fn stale_extent_generation_denies_before_extent_record_decode() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let old_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(5));
    let current_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(6));

    let denial = references
        .validate_extent(references.admit_extent(old_cell), current_cell)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalReferenceDenialKind::StaleExtentGeneration
    );
    assert_eq!(denial.counters().extent_validation_count(), 1);
    assert_eq!(denial.counters().stale_generation_rejection_count(), 1);
}

#[test]
fn extent_length_mismatch_denies_before_payload_view() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let extent_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(5));
    let payload = b"large physical record payload";
    let write_membership = ExtentMembership::large_record(extent_cell, extent_frame_len(payload));
    let read_membership =
        ExtentMembership::large_record(extent_cell, extent_frame_len(payload) + 1);
    let append = records
        .append_extent_record(
            write_membership,
            ExtentRecordAppendRequest::large_record(extent_cell, payload),
        )
        .unwrap();
    let validation = references
        .validate_extent(append.reference_admission(), extent_cell)
        .unwrap();

    let denial = records
        .locate_extent_record(append.extent_bytes(), read_membership, validation)
        .unwrap_err();

    assert_eq!(denial.kind(), ExtentRecordDenialKind::ExtentLengthMismatch);
    assert_eq!(
        denial.expected_length(),
        Some(extent_frame_len(payload) + 1)
    );
    assert_eq!(denial.actual_length(), Some(extent_frame_len(payload)));
    assert_eq!(denial.counters().extent_header_decode_count(), 0);
    assert_eq!(denial.counters().extent_payload_view_count(), 0);
}

#[test]
fn append_missing_extent_membership_counts_only_membership_check() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let extent_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(5));

    let denial = records
        .append_extent_record(
            ExtentMembership::missing(),
            ExtentRecordAppendRequest::large_record(extent_cell, b"large"),
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ExtentRecordDenialKind::MissingExtentMembership
    );
    assert_eq!(denial.counters().extent_membership_check_count(), 1);
    assert_eq!(denial.counters().extent_length_check_count(), 0);
    assert_eq!(denial.counters().extent_write_count(), 0);
}

#[test]
fn append_membership_mismatch_counts_no_length_check_or_write() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let membership_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(5));
    let request_cell = generations
        .extent_cell(segment(7), extent(21))
        .with_extent_generation(generation(5));
    let membership = ExtentMembership::large_record(membership_cell, extent_frame_len(b"large"));

    let denial = records
        .append_extent_record(
            membership,
            ExtentRecordAppendRequest::large_record(request_cell, b"large"),
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ExtentRecordDenialKind::ExtentReferenceMismatch
    );
    assert_eq!(denial.counters().extent_membership_check_count(), 1);
    assert_eq!(denial.counters().extent_length_check_count(), 0);
    assert_eq!(denial.counters().extent_write_count(), 0);
}

#[test]
fn missing_extent_membership_denies_before_header_decode() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let extent_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(5));
    let payload = b"large physical record payload";
    let membership = ExtentMembership::large_record(extent_cell, extent_frame_len(payload));
    let append = records
        .append_extent_record(
            membership,
            ExtentRecordAppendRequest::large_record(extent_cell, payload),
        )
        .unwrap();
    let validation = references
        .validate_extent(append.reference_admission(), extent_cell)
        .unwrap();

    let denial = records
        .locate_extent_record(
            append.extent_bytes(),
            ExtentMembership::missing(),
            validation,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ExtentRecordDenialKind::MissingExtentMembership
    );
    assert_eq!(denial.counters().extent_membership_check_count(), 1);
    assert_eq!(denial.counters().extent_header_decode_count(), 0);
    assert_eq!(denial.counters().extent_payload_view_count(), 0);
}

#[test]
fn moved_slot_misuse_denies_before_extent_membership_check() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let slot_cell = generations
        .slot_cell(segment(7), page(3), slot(1))
        .with_slot_generation(generation(5));
    let validation = references
        .validate_page_slot(references.admit_page_slot(slot_cell), slot_cell)
        .unwrap();

    let denial = records
        .locate_extent_record(&[], ExtentMembership::missing(), validation)
        .unwrap_err();

    assert_eq!(denial.kind(), ExtentRecordDenialKind::MovedSlotMisuse);
    assert_eq!(denial.counters().moved_slot_misuse_rejection_count(), 1);
    assert_eq!(denial.counters().extent_membership_check_count(), 0);
    assert_eq!(denial.counters().extent_payload_view_count(), 0);
}

fn extent_record_authority() -> PhysicalExtentRecordAuthority {
    PhysicalExtentRecordAuthority::s1(PhysicalHeaderAuthority::s1(
        PhysicalBinaryEncodingWitness::s1_canonical().unwrap(),
    ))
}

const fn extent_frame_len(payload: &[u8]) -> usize {
    PHYSICAL_HEADER_LENGTH as usize + payload.len()
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn extent(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
