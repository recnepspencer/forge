use forge_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalChunkChecksumAuthority,
    PhysicalChunkPayloadIntegrityWitness, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalHeaderAuthority, PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority,
    PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId,
    SlotAppendRequest, StorePhysicalChunkWriteReceipt, PHYSICAL_HEADER_LENGTH,
};

pub(crate) fn physical_payload_for_bytes(bytes: &[u8]) -> PhysicalChunkPayloadIntegrityWitness {
    PhysicalChunkChecksumAuthority::canonical_blob_checksum()
        .admit_store_payload(record_receipt(bytes))
        .expect("payload should admit")
}

pub(crate) fn record_receipt(bytes: &[u8]) -> StorePhysicalChunkWriteReceipt {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
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
            SlotAppendRequest::ordinary(slot_cell, bytes),
        )
        .expect("physical record append should execute");
    let reopened_page = page_bytes(generation(5), append.page_payload());
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .expect("physical reference should validate");
    let located = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .expect("physical record locate should execute");
    StorePhysicalChunkWriteReceipt::from_page_record_view(located.record_view())
        .expect("physical record view should admit chunk write receipt")
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    page_cell: forge_store_physical_format::PageGenerationCell,
    bytes: &'a [u8],
) -> forge_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .expect("record page header should decode");
    records
        .admit_record_page_payload(bytes, header.witness())
        .expect("record page payload should admit")
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::s1(PhysicalHeaderAuthority::s1(
        PhysicalBinaryEncodingWitness::s1_canonical().expect("canonical physical binary format"),
    ))
}

fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
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

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("segment id")
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("page id")
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("record slot")
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).expect("generation")
}
