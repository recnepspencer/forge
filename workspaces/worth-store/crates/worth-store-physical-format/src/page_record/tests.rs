use super::test_support::{
    admitted_page, generation, one_record_page_payload, page, page_bytes, page_bytes_for_kind,
    record_authority, segment, slot,
};
use crate::{
    PageRecordDenialKind, PhysicalGenerationAuthority, PhysicalHeaderDecodeDenialKind,
    PhysicalPageKind, PhysicalReferenceAuthority, SlotAppendRequest, SlotDirectoryEntryState,
    PHYSICAL_HEADER_LENGTH,
};

#[test]
fn append_and_reopen_locate_by_slot_yields_stable_framed_record() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));

    let empty_page = page_bytes(generation(5), &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, b"stable bytes"),
        )
        .unwrap();
    let reopened_page = page_bytes(generation(5), append.page_payload());
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .unwrap();

    let located = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap();

    assert_eq!(located.reference(), append.reference());
    assert_eq!(located.record_view().payload().as_bytes(), b"stable bytes");
    assert_eq!(located.counters().record_locate_count(), 1);
    assert_eq!(located.counters().slot_lookup_count(), 1);
    assert_eq!(located.counters().page_local_scan_count(), 0);
    assert_eq!(located.counters().frame_decode_count(), 1);
    assert_eq!(located.counters().record_payload_view_count(), 1);
}

#[test]
fn manifest_page_payload_cannot_enter_record_page_authority() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let manifest_page = page_bytes_for_kind(PhysicalPageKind::ManifestPage, generation(5), &[]);
    let header = records
        .decode_record_page_header(page_cell, &manifest_page, PhysicalPageKind::ManifestPage)
        .unwrap();

    let denial = records
        .admit_record_page_payload(&manifest_page, header.witness())
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalHeaderDecodeDenialKind::UnexpectedPageKind
    );
}

#[test]
fn append_rejects_slot_cell_for_different_page_before_write() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let wrong_page_slot = generations
        .slot_cell(segment(7), page(12), slot(1))
        .with_slot_generation(generation(9));
    let empty_page = page_bytes(generation(5), &[]);

    let denial = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(wrong_page_slot, b"wrong page"),
        )
        .unwrap_err();

    assert_eq!(denial.kind(), PageRecordDenialKind::PageReferenceMismatch);
    assert_eq!(denial.counters().page_write_count(), 0);
    assert_eq!(denial.counters().page_local_scan_count(), 0);
}

#[test]
fn out_of_range_slot_denies_before_frame_payload_view() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_one = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let slot_two = generations
        .slot_cell(segment(7), page(11), slot(2))
        .with_slot_generation(generation(9));

    let empty_page = page_bytes(generation(5), &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_one, b"one"),
        )
        .unwrap();
    let reopened_page = page_bytes(generation(5), append.page_payload());
    let validation = references
        .validate_page_slot(references.admit_page_slot(slot_two), slot_two)
        .unwrap();

    let denial = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap_err();

    assert_eq!(denial.kind(), PageRecordDenialKind::SlotOutOfRange);
    assert_eq!(denial.counters().slot_lookup_count(), 1);
    assert_eq!(denial.counters().frame_decode_count(), 0);
    assert_eq!(denial.counters().record_payload_view_count(), 0);
}

#[test]
fn locate_rejects_reference_for_different_page_before_slot_lookup() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let wrong_page_slot = generations
        .slot_cell(segment(7), page(12), slot(1))
        .with_slot_generation(generation(9));
    let page_payload = one_record_page_payload(&records, page_cell, slot_cell);
    let reopened_page = page_bytes(generation(5), &page_payload);
    let validation = references
        .validate_page_slot(references.admit_page_slot(wrong_page_slot), wrong_page_slot)
        .unwrap();

    let denial = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap_err();

    assert_eq!(denial.kind(), PageRecordDenialKind::PageReferenceMismatch);
    assert_eq!(denial.counters().record_locate_count(), 1);
    assert_eq!(denial.counters().slot_lookup_count(), 0);
    assert_eq!(denial.counters().frame_decode_count(), 0);
    assert_eq!(denial.counters().record_payload_view_count(), 0);
}

#[test]
fn locate_rejects_reference_for_different_segment_before_slot_lookup() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let wrong_segment_slot = generations
        .slot_cell(segment(8), page(11), slot(1))
        .with_slot_generation(generation(9));
    let page_payload = one_record_page_payload(&records, page_cell, slot_cell);
    let reopened_page = page_bytes(generation(5), &page_payload);
    let validation = references
        .validate_page_slot(
            references.admit_page_slot(wrong_segment_slot),
            wrong_segment_slot,
        )
        .unwrap();

    let denial = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap_err();

    assert_eq!(denial.kind(), PageRecordDenialKind::PageReferenceMismatch);
    assert_eq!(denial.counters().slot_lookup_count(), 0);
}

#[test]
fn appending_second_slot_rebases_existing_frame_offsets() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_one = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let slot_two = generations
        .slot_cell(segment(7), page(11), slot(2))
        .with_slot_generation(generation(10));

    let empty_page = page_bytes(generation(5), &[]);
    let first = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_one, b"first"),
        )
        .unwrap();
    let first_page = page_bytes(generation(5), first.page_payload());
    let second = records
        .append_record(
            admitted_page(&records, page_cell, &first_page),
            SlotAppendRequest::ordinary(slot_two, b"second"),
        )
        .unwrap();
    let reopened_page = page_bytes(generation(5), second.page_payload());

    let located_first = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            references
                .validate_page_slot(first.reference_admission(), slot_one)
                .unwrap(),
        )
        .unwrap();
    let located_second = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            references
                .validate_page_slot(second.reference_admission(), slot_two)
                .unwrap(),
        )
        .unwrap();

    assert_eq!(located_first.record_view().payload().as_bytes(), b"first");
    assert_eq!(located_second.record_view().payload().as_bytes(), b"second");
    assert_eq!(located_first.counters().slot_lookup_count(), 1);
    assert_eq!(located_first.counters().page_local_scan_count(), 0);
}

#[test]
fn moved_slot_without_admitted_reference_denies_before_payload_view() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let mut page_payload = one_record_page_payload(&records, page_cell, slot_cell);
    page_payload[4] = SlotDirectoryEntryState::Moved.code();
    let reopened_page = page_bytes(generation(5), &page_payload);
    let validation = references
        .validate_page_slot(references.admit_page_slot(slot_cell), slot_cell)
        .unwrap();

    let denial = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PageRecordDenialKind::MovedSlotWithoutAdmittedReference
    );
    assert_eq!(denial.counters().slot_lookup_count(), 1);
    assert_eq!(denial.counters().frame_decode_count(), 0);
    assert_eq!(denial.counters().record_payload_view_count(), 0);
}

#[test]
fn malformed_slot_entry_preserves_locate_counters_before_payload_view() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let mut page_payload = one_record_page_payload(&records, page_cell, slot_cell);
    page_payload[4] = 0xff;
    let reopened_page = page_bytes(generation(5), &page_payload);
    let validation = references
        .validate_page_slot(references.admit_page_slot(slot_cell), slot_cell)
        .unwrap();

    let denial = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap_err();

    assert_eq!(denial.kind(), PageRecordDenialKind::ReservedSlot);
    assert_eq!(denial.counters().slot_lookup_count(), 1);
    assert_eq!(denial.counters().frame_decode_count(), 0);
    assert_eq!(denial.counters().record_payload_view_count(), 0);
}

#[test]
fn frame_length_mismatch_denies_before_record_payload_view() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let mut page_payload = one_record_page_payload(&records, page_cell, slot_cell);
    page_payload[12..16].copy_from_slice(&(PHYSICAL_HEADER_LENGTH as u32).to_le_bytes());
    let reopened_page = page_bytes(generation(5), &page_payload);
    let validation = references
        .validate_page_slot(references.admit_page_slot(slot_cell), slot_cell)
        .unwrap();

    let denial = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap_err();

    assert_eq!(denial.kind(), PageRecordDenialKind::HeaderDecodeDenied);
    assert_eq!(
        denial.header_denial().unwrap().kind(),
        PhysicalHeaderDecodeDenialKind::PayloadLengthMismatch
    );
    assert_eq!(denial.counters().frame_decode_count(), 1);
    assert_eq!(denial.counters().record_payload_view_count(), 0);
}
