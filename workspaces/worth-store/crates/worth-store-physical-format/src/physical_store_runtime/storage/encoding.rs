use crate::{
    ExtentGenerationCell, PhysicalGenerationAuthority, PhysicalPageKind, PhysicalPublicationState,
    PhysicalReference, RootPublicationCell, SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};

pub(super) fn encode_empty_page(generation: crate::PhysicalGeneration) -> Vec<u8> {
    encode_page(generation, &[])
}

pub(super) fn encode_page(generation: crate::PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(
        &crate::PhysicalFormatVersion::initial_format_version()
            .value()
            .to_le_bytes(),
    );
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
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
