use crate::{
    InMemoryPhysicalFormatModelDenial, InMemoryPhysicalFormatModelDenialKind,
    PhysicalGenerationAuthority, PhysicalReference,
};

pub(crate) fn slot_cell_from_reference(
    reference: PhysicalReference,
) -> Result<crate::SlotGenerationCell, InMemoryPhysicalFormatModelDenial> {
    let segment_id = reference.segment_id().ok_or_else(missing_record)?;
    let page_id = reference.page_id().ok_or_else(missing_record)?;
    let slot = reference.slot().ok_or_else(missing_record)?;
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment_id, page_id, slot)
        .with_slot_generation(reference.generation()))
}

pub(crate) fn extent_cell_from_reference(
    reference: PhysicalReference,
) -> Result<crate::ExtentGenerationCell, InMemoryPhysicalFormatModelDenial> {
    let segment_id = reference.segment_id().ok_or_else(missing_record)?;
    let extent_id = reference.extent_id().ok_or_else(missing_record)?;
    Ok(PhysicalGenerationAuthority::for_canonical_physical_format()
        .extent_cell(segment_id, extent_id)
        .with_extent_generation(reference.generation()))
}

fn missing_record() -> InMemoryPhysicalFormatModelDenial {
    InMemoryPhysicalFormatModelDenial::new(
        InMemoryPhysicalFormatModelDenialKind::MissingPhysicalRecord,
    )
}
