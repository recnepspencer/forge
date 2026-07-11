use crate::{
    PhysicalBinaryEncodingWitness, PhysicalGeneration, PhysicalHeaderAuthority, PhysicalPageId,
    PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalSegmentId, SlotAppendRequest, PHYSICAL_HEADER_LENGTH,
};

pub(crate) fn one_record_page_payload(
    records: &PhysicalPageRecordAuthority,
    page_cell: crate::PageGenerationCell,
    slot_cell: crate::SlotGenerationCell,
) -> Vec<u8> {
    let empty_page = page_bytes(generation(5), &[]);
    records
        .append_record(
            admitted_page(records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, b"one"),
        )
        .unwrap()
        .page_payload()
        .to_vec()
}

pub(crate) fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    page_cell: crate::PageGenerationCell,
    bytes: &'a [u8],
) -> crate::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .unwrap();
    records
        .admit_record_page_payload(bytes, header.witness())
        .unwrap()
}

pub(crate) fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::for_canonical_physical_format(PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    ))
}

pub(crate) fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    page_bytes_for_kind(PhysicalPageKind::DataPage, generation, payload)
}

pub(crate) fn page_bytes_for_kind(
    kind: PhysicalPageKind,
    generation: PhysicalGeneration,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(kind.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

pub(crate) fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

pub(crate) fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

pub(crate) fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

pub(crate) fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
