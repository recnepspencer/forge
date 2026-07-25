use worth_store_physical_format::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalPageKind, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, SlotGenerationCell,
};

pub(crate) fn current_validation() -> PhysicalReferenceValidationWitness {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = slot_cell(7);
    references
        .validate_page_slot(references.admit_page_slot(cell), cell)
        .unwrap()
}

pub(crate) fn current_page_cell() -> PageGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment(1), PhysicalPageId::from_raw(2).unwrap())
        .with_page_generation(PhysicalGeneration::from_raw(5).unwrap())
}

pub(crate) fn stale_validation() -> PhysicalReferenceValidationWitness {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let admitted = references.admit_page_slot(slot_cell(8));
    references
        .validate_page_slot(admitted, slot_cell(8))
        .unwrap()
}

pub(crate) fn frame_witness(payload: &[u8]) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            current_validation(),
            &current_frame_bytes(payload),
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

pub(crate) fn page_witness(payload: &[u8]) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_page_header(
            current_page_cell(),
            &current_page_bytes(payload),
            PhysicalPageKind::DataPage,
        )
        .unwrap()
        .witness()
}

pub(crate) fn current_frame_bytes(payload: &[u8]) -> Vec<u8> {
    crate::physical_fixture_encoding::record_frame_bytes(slot_cell(7), payload)
}

pub(crate) fn current_frame_bytes_with_declared_payload(
    actual_payload: &[u8],
    declared_payload: &[u8],
) -> Vec<u8> {
    let mut bytes = current_frame_bytes(declared_payload);
    bytes.truncate(bytes.len() - declared_payload.len());
    bytes.extend_from_slice(actual_payload);
    bytes
}

pub(crate) fn current_page_bytes(payload: &[u8]) -> Vec<u8> {
    crate::physical_fixture_encoding::data_page_bytes(current_page_cell(), payload)
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
}

fn slot_cell(generation: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            segment(1),
            PhysicalPageId::from_raw(2).unwrap(),
            PhysicalRecordSlot::from_raw(3).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap())
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}
