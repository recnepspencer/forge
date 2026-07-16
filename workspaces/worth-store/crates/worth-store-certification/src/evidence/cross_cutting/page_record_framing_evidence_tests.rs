use crate::{
    PhysicalPageRecordFramingEvidenceDenial, PhysicalPageRecordFramingEvidenceReport,
    PhysicalPageRecordFramingEvidenceRow, PhysicalSubstrateLane,
};
use worth_store_physical_format::{
    PageGenerationCell, PageRecordCounterSnapshot, PhysicalBinaryEncodingWitness,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId,
    PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalSegmentId, RecordPagePayload, SlotAppendRequest, SlotDirectoryEntryState,
};

#[test]
fn every_page_record_framing_row_maps_to_physical_substrate() {
    for row in PhysicalPageRecordFramingEvidenceRow::physical_format_required() {
        assert_eq!(
            row.physical_substrate_lane().family().as_str(),
            "physical_substrate"
        );
    }
}

#[test]
fn slot_lookup_counters_feed_foundational_counter_backed_receipt() {
    let counters = successful_locate_counters();
    let evidence = PhysicalPageRecordFramingEvidenceReport::from_counters(
        PhysicalPageRecordFramingEvidenceRow::SlotLookupCountersExact,
        counters,
    )
    .unwrap();

    assert_eq!(evidence.counters().slot_lookup_count(), 1);
    assert_eq!(evidence.lane(), PhysicalSubstrateLane::ScaleLocality);
    assert!(evidence
        .performance_receipt()
        .counter_rows()
        .iter()
        .any(|row| row.name().as_str() == "physical.slot_lookup" && row.observed_count() == 1));
}

#[test]
fn framing_evidence_rejects_counterless_slot_lookup_claims() {
    let denial = PhysicalPageRecordFramingEvidenceReport::from_counters(
        PhysicalPageRecordFramingEvidenceRow::SlotLookupCountersExact,
        PageRecordCounterSnapshot::default(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalPageRecordFramingEvidenceDenial::MissingSlotLookupCounter
    );
}

#[test]
fn framing_evidence_rejects_page_local_scan_drift() {
    let counters = successful_locate_counters().merge(PageRecordCounterSnapshot::for_append(1));
    let denial = PhysicalPageRecordFramingEvidenceReport::from_counters(
        PhysicalPageRecordFramingEvidenceRow::SlotLookupCountersExact,
        counters,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalPageRecordFramingEvidenceDenial::CounterExpectationMismatch {
            expected: successful_locate_counters(),
            actual: counters,
        }
    );
}

#[test]
fn moved_slot_denial_certifies_from_real_page_record_denial() {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let empty_page = crate::physical_fixture_encoding::data_page_bytes(page_cell, &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, b"moved"),
        )
        .unwrap();
    let mut page_payload = append.page_payload().to_vec();
    page_payload[4] = SlotDirectoryEntryState::Moved.code();
    let reopened_page = crate::physical_fixture_encoding::data_page_bytes(page_cell, &page_payload);
    let validation = references
        .validate_page_slot(references.admit_page_slot(slot_cell), slot_cell)
        .unwrap();
    let denial = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .unwrap_err();

    let evidence = PhysicalPageRecordFramingEvidenceReport::from_page_record_denial(
        PhysicalPageRecordFramingEvidenceRow::MovedSlotBoundedOrDenied,
        denial,
    )
    .unwrap();

    assert_eq!(
        evidence.counters(),
        PageRecordCounterSnapshot::for_locate_attempt().with_slot_lookup()
    );
    assert_eq!(evidence.lane(), PhysicalSubstrateLane::HostileFormat);
}

fn successful_locate_counters() -> PageRecordCounterSnapshot {
    PageRecordCounterSnapshot::for_locate_attempt()
        .with_slot_lookup()
        .with_frame_decode()
        .with_record_payload_view()
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    page_cell: PageGenerationCell,
    bytes: &'a [u8],
) -> RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .unwrap();
    records
        .admit_record_page_payload(bytes, header.witness())
        .unwrap()
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
        ),
    )
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
