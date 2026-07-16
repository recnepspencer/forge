use crate::{
    PhysicalBinaryEncodingWitness, PhysicalGeneration, PhysicalHeaderAuthority, PhysicalPageId,
    PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalRecordSlot, PhysicalSegmentId,
    SlotAppendRequest, PHYSICAL_HEADER_LENGTH,
};

pub(crate) fn one_record_page_payload(
    records: &PhysicalPageRecordAuthority,
    page_cell: crate::PageGenerationCell,
    slot_cell: crate::SlotGenerationCell,
) -> Vec<u8> {
    let empty_page = page_bytes(page_cell, &[]);
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
    PhysicalPageRecordAuthority::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
        ),
    )
}

pub(crate) fn page_bytes(page_cell: crate::PageGenerationCell, payload: &[u8]) -> Vec<u8> {
    page_bytes_for_kind(PhysicalPageKind::DataPage, page_cell, payload)
}

pub(crate) fn page_bytes_for_kind(
    kind: PhysicalPageKind,
    page_cell: crate::PageGenerationCell,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.extend_from_slice(&crate::header::encode_page_header(
        crate::PhysicalByteOrder::LittleEndian,
        kind,
        page_cell,
        payload.len() as u32,
    ));
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
