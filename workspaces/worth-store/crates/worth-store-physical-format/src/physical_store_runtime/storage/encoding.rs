use crate::header::encode_page_header;
use crate::{
    ExtentGenerationCell, PageGenerationCell, PhysicalGenerationAuthority, PhysicalPageKind,
    PhysicalReference, RootPublicationCell, SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};

pub(super) fn encode_empty_page(owner: PageGenerationCell) -> Vec<u8> {
    encode_page(owner, &[])
}

pub(super) fn encode_page(owner: PageGenerationCell, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.extend_from_slice(&encode_page_header(
        crate::PhysicalByteOrder::LittleEndian,
        PhysicalPageKind::DataPage,
        owner,
        payload.len() as u32,
    ));
    bytes.extend_from_slice(payload);
    bytes
}

pub(super) fn reference_to_slot_cell(reference: &PhysicalReference) -> Option<SlotGenerationCell> {
    Some(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .slot_cell(
                reference.segment_id()?,
                reference.page_id()?,
                reference.slot()?,
            )
            .with_slot_generation(reference.generation()),
    )
}

pub(super) fn reference_to_extent_cell(
    reference: &PhysicalReference,
) -> Option<ExtentGenerationCell> {
    Some(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .extent_cell(reference.segment_id()?, reference.extent_id()?)
            .with_extent_generation(reference.generation()),
    )
}

pub(super) fn reference_to_root_publication_cell(
    reference: &PhysicalReference,
) -> Option<RootPublicationCell> {
    Some(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .root_publication_cell(reference.root_reference()?)
            .with_root_publication_generation(reference.generation()),
    )
}
