use crate::{
    PhysicalExtentRecordFramingEvidenceDenial, PhysicalExtentRecordFramingEvidenceReport,
    PhysicalExtentRecordFramingEvidenceRow, PhysicalSubstrateLane,
};
use forge_foundational::FoundationalPerformanceCounterRow;
use forge_store_physical_format::{
    ExtentMembership, ExtentRecordAppendRequest, ExtentRecordCounterSnapshot,
    PhysicalBinaryEncodingWitness, PhysicalExtentId, PhysicalExtentRecordAuthority,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

#[test]
fn every_extent_record_framing_row_maps_to_physical_substrate() {
    for row in PhysicalExtentRecordFramingEvidenceRow::physical_format_required() {
        assert_eq!(
            row.physical_substrate_lane().family().as_str(),
            "physical_substrate"
        );
    }
}

#[test]
fn extent_locate_report_feeds_foundational_counter_receipt() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
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
    let report = records
        .locate_extent_record(append.extent_bytes(), membership, validation)
        .unwrap();

    let evidence = PhysicalExtentRecordFramingEvidenceReport::from_locate_report(
        PhysicalExtentRecordFramingEvidenceRow::ExtentLocalCountersExact,
        report,
    )
    .unwrap();

    assert_eq!(evidence.lane(), PhysicalSubstrateLane::ScaleLocality);
    assert_eq!(evidence.counters(), successful_locate_counters());
    assert_receipt_rows(&evidence, successful_locate_counters());
}

#[test]
fn length_mismatch_denial_certifies_from_real_extent_denial() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
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

    let evidence = PhysicalExtentRecordFramingEvidenceReport::from_extent_denial(
        PhysicalExtentRecordFramingEvidenceRow::ExtentLengthMismatchDenied,
        denial,
    )
    .unwrap();

    assert_eq!(
        evidence.counters(),
        ExtentRecordCounterSnapshot::for_locate_attempt()
            .with_membership_check()
            .with_length_check()
    );
}

#[test]
fn missing_membership_denial_certifies_from_real_extent_denial() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
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

    let evidence = PhysicalExtentRecordFramingEvidenceReport::from_extent_denial(
        PhysicalExtentRecordFramingEvidenceRow::MissingExtentMembershipDenied,
        denial,
    )
    .unwrap();

    assert_eq!(
        evidence.counters(),
        ExtentRecordCounterSnapshot::for_locate_attempt().with_membership_check()
    );
}

#[test]
fn moved_slot_misuse_denial_certifies_from_real_extent_denial() {
    let records = extent_record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let slot_cell = generations
        .slot_cell(segment(7), page(3), slot(1))
        .with_slot_generation(generation(5));
    let validation = references
        .validate_page_slot(references.admit_page_slot(slot_cell), slot_cell)
        .unwrap();
    let denial = records
        .locate_extent_record(&[], ExtentMembership::missing(), validation)
        .unwrap_err();

    let evidence = PhysicalExtentRecordFramingEvidenceReport::from_extent_denial(
        PhysicalExtentRecordFramingEvidenceRow::MovedSlotMisuseDenied,
        denial,
    )
    .unwrap();

    assert_eq!(
        evidence.counters(),
        ExtentRecordCounterSnapshot::for_locate_attempt().with_moved_slot_misuse_rejection()
    );
    assert_receipt_rows(
        &evidence,
        ExtentRecordCounterSnapshot::for_locate_attempt().with_moved_slot_misuse_rejection(),
    );
}

#[test]
fn extent_counter_drift_is_rejected_before_receipt() {
    let counters = successful_locate_counters().with_moved_slot_misuse_rejection();
    let denial = PhysicalExtentRecordFramingEvidenceReport::from_counters(
        PhysicalExtentRecordFramingEvidenceRow::ExtentLocalCountersExact,
        counters,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalExtentRecordFramingEvidenceDenial::CounterExpectationMismatch {
            expected: successful_locate_counters(),
            actual: counters,
        }
    );
}

fn successful_locate_counters() -> ExtentRecordCounterSnapshot {
    ExtentRecordCounterSnapshot::for_locate_attempt()
        .with_membership_check()
        .with_length_check()
        .with_header_decode()
        .with_payload_view()
}

fn assert_receipt_rows(
    evidence: &PhysicalExtentRecordFramingEvidenceReport,
    counters: ExtentRecordCounterSnapshot,
) {
    let rows = evidence.performance_receipt().counter_rows();
    assert_eq!(rows.len(), 8);
    assert_counter_row(rows, "physical.extent_read", counters.extent_read_count());
    assert_counter_row(rows, "physical.extent_write", counters.extent_write_count());
    assert_counter_row(
        rows,
        "physical.extent_header_decode",
        counters.extent_header_decode_count(),
    );
    assert_counter_row(
        rows,
        "physical.extent_membership_check",
        counters.extent_membership_check_count(),
    );
    assert_counter_row(
        rows,
        "physical.extent_length_check",
        counters.extent_length_check_count(),
    );
    assert_counter_row(
        rows,
        "physical.extent_locate",
        counters.extent_locate_count(),
    );
    assert_counter_row(
        rows,
        "physical.extent_payload_view",
        counters.extent_payload_view_count(),
    );
    assert_counter_row(
        rows,
        "physical.moved_slot_misuse_rejection",
        counters.moved_slot_misuse_rejection_count(),
    );
}

fn assert_counter_row(rows: &[FoundationalPerformanceCounterRow], name: &str, count: u32) {
    assert!(rows
        .iter()
        .any(|row| row.name().as_str() == name && row.observed_count() == count as u64));
}

fn extent_record_authority() -> PhysicalExtentRecordAuthority {
    PhysicalExtentRecordAuthority::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
        ),
    )
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
