use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalChunkChecksumAuthority,
    PhysicalChunkPayloadIntegrityWitness, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalHeaderAuthority, PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId, SlotAppendRequest,
    StorePhysicalChunkWriteReceipt,
};

pub(crate) fn physical_payload_for_bytes(bytes: &[u8]) -> PhysicalChunkPayloadIntegrityWitness {
    PhysicalChunkChecksumAuthority::canonical_blob_checksum()
        .admit_store_payload(record_receipt(bytes))
        .expect("payload should admit")
}

pub(crate) fn record_receipt(bytes: &[u8]) -> StorePhysicalChunkWriteReceipt {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let empty_page = page_bytes(page_cell, &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, bytes),
        )
        .expect("physical record append should execute");
    let reopened_page = page_bytes(page_cell, append.page_payload());
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
    page_cell: worth_store_physical_format::PageGenerationCell,
    bytes: &'a [u8],
) -> worth_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .expect("record page header should decode");
    records
        .admit_record_page_payload(bytes, header.witness())
        .expect("record page payload should admit")
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical()
                .expect("canonical physical binary format"),
        ),
    )
}

fn page_bytes(cell: worth_store_physical_format::PageGenerationCell, payload: &[u8]) -> Vec<u8> {
    let headers = PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical()
            .expect("canonical physical binary format"),
    );
    let mut bytes = Vec::with_capacity(
        usize::from(worth_store_physical_format::PHYSICAL_HEADER_LENGTH) + payload.len(),
    );
    bytes.extend_from_slice(&headers.encode_page_header(
        cell,
        PhysicalPageKind::DataPage,
        u32::try_from(payload.len()).expect("test payload length should fit the physical format"),
    ));
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
