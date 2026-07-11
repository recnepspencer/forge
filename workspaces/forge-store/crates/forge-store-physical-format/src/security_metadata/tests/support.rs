use crate::{
    AllocationClassKind, PhysicalBinaryEncodingWitness, PhysicalDecodedHeader, PhysicalExtentId,
    PhysicalFrameHeader, PhysicalFrameKind, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalHeaderAuthority, PhysicalPageHeader, PhysicalPageId, PhysicalPageKind,
    PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalRootManifest,
    PhysicalRootReference, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

pub(super) fn decoded_page_header(generation_value: u64) -> PhysicalPageHeader {
    let cell = PhysicalGenerationAuthority::s1()
        .page_cell(segment(1), page(2))
        .with_page_generation(generation(generation_value));
    let report = header_authority()
        .decode_page_header(
            cell,
            &header_bytes(PhysicalPageKind::DataPage.tag(), generation_value, b"page"),
            PhysicalPageKind::DataPage,
        )
        .expect("page header should decode");
    match report.witness().header() {
        PhysicalDecodedHeader::Page(header) => header,
        PhysicalDecodedHeader::Frame(_) => panic!("expected decoded page header"),
    }
}

pub(super) fn decoded_frame_header(generation_value: u64) -> PhysicalFrameHeader {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let cell = generations
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(generation_value));
    let admitted = references.admit_page_slot(cell);
    let reference = references
        .validate_page_slot(admitted, cell)
        .expect("page slot should validate");
    let report = header_authority()
        .decode_frame_header(
            reference,
            &header_bytes(
                PhysicalFrameKind::RecordFrame.tag(),
                generation_value,
                b"frame",
            ),
            PhysicalFrameKind::RecordFrame,
        )
        .expect("frame header should decode");
    match report.witness().header() {
        PhysicalDecodedHeader::Frame(header) => header,
        PhysicalDecodedHeader::Page(_) => panic!("expected decoded frame header"),
    }
}

pub(super) fn root_manifest_with_all_entry_kinds() -> PhysicalRootManifest {
    let generations = PhysicalGenerationAuthority::s1();
    let root_cell = generations
        .root_publication_cell(PhysicalRootReference::from_raw(9).expect("non-zero root"))
        .with_root_publication_generation(generation(1));
    let segment_cell = generations
        .segment_cell(segment(1))
        .with_segment_generation(generation(2));
    let slot_cell = generations
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(3));
    let extent_cell = generations
        .extent_cell(segment(1), extent(4))
        .with_extent_generation(generation(4));
    let free_space_cell = generations
        .free_space_slot_cell(
            segment(1),
            page(2),
            slot(5),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .expect("valid free-space cell")
        .with_free_space_generation(generation(5));

    crate::PhysicalManifestUniverseBuilder::s1(root_cell)
        .segment(segment_cell)
        .ordinary_page(slot_cell)
        .extent(extent_cell)
        .free_space_reuse(free_space_cell)
        .publish()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::s1(
        PhysicalBinaryEncodingWitness::s1_canonical().expect("canonical encoding witness"),
    )
}

fn header_bytes(kind_tag: u8, generation_value: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(kind_tag);
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation_value.to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("non-zero segment")
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("non-zero page")
}

fn extent(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).expect("non-zero extent")
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("non-zero slot")
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).expect("non-zero generation")
}
