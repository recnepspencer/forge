use crate::{
    BufferPoolBudget, DirtyPageBudget, ResidentFrameAdmission, ResidentFrameLoadRequest,
    ResidentFrameTable, ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use worth_store_contracts::PhysicalSubstrateReadinessSnapshot;
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

pub(crate) fn admit_payload_frame(
    table: &mut ResidentFrameTable,
    generation_value: u64,
    page_value: u64,
    payload: &[u8],
) -> ResidentFrameAdmission {
    let frame = frame_bytes(generation_value, page_value, payload);
    let request = load_request_from_frame(generation_value, page_value, &frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

pub(crate) fn resident_frame_table(
    resident_bytes: u64,
    frame_count: u32,
    dirty_pages: u32,
) -> ResidentFrameTable {
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(resident_bytes).unwrap(),
        crate::PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(dirty_pages).unwrap(),
    );
    let admitted =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
            .unwrap()
            .with_budget(budget)
            .admit()
            .unwrap();
    ResidentFrameTable::open(
        admitted,
        ResidentFrameTableCapacity::frames(frame_count).unwrap(),
    )
}

pub(crate) fn load_request(
    generation_value: u64,
    page_value: u64,
    payload: &[u8],
) -> ResidentFrameLoadRequest {
    let frame = frame_bytes(generation_value, page_value, payload);
    load_request_from_frame(generation_value, page_value, &frame)
}

fn load_request_from_frame(
    generation_value: u64,
    page_value: u64,
    frame_bytes: &[u8],
) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_physical_format_physical_frame(
        validated_slot_reference(generation_value, page_value),
        frame_header_witness(generation_value, page_value, frame_bytes),
    )
    .unwrap()
}

fn validated_slot_reference(
    generation_value: u64,
    page_value: u64,
) -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value));
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn frame_header_witness(
    generation_value: u64,
    page_value: u64,
    bytes: &[u8],
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(generation_value, page_value),
            bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
}

fn frame_bytes(generation_value: u64, page_value: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value));
    bytes.extend_from_slice(&header_authority().encode_record_frame_header(
        cell,
        payload.len().try_into().expect("bounded fixture payload"),
    ));
    bytes.extend_from_slice(payload);
    bytes
}

fn algorithm_model_snapshot() -> PhysicalSubstrateReadinessSnapshot {
    PhysicalSubstrateReadinessSnapshot::from_exact_counts(true, 4, 2, 2, 3, 1, 9)
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
