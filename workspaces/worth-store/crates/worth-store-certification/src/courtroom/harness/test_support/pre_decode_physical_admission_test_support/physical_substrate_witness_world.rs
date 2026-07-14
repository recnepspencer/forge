use worth_store_physical_format::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalPageKind, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceValidationWitness, PhysicalSegmentId,
    SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
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
            &record_frame_bytes(7, payload),
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

pub(crate) fn page_witness(payload: &[u8]) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_page_header(
            current_page_cell(),
            &page_bytes(5, payload),
            PhysicalPageKind::DataPage,
        )
        .unwrap()
        .witness()
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

fn record_frame_bytes(generation: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::RecordFrame.tag());
    write_header_tail(&mut bytes, generation, payload);
    bytes
}

fn page_bytes(generation: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    write_header_tail(&mut bytes, generation, payload);
    bytes
}

fn write_header_tail(bytes: &mut Vec<u8>, generation: u64, payload: &[u8]) {
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}
