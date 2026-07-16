use worth_store_physical_format::{
    ExtentGenerationCell, PageGenerationCell, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalPageId, PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId,
    RootPublicationCell,
};
use worth_store_physical_isolation::{
    CurrentGenerationPhysicalReference, GenerationCountedPhysicalReference,
};

pub(super) fn current_root_reference(
    cell: RootPublicationCell,
) -> CurrentGenerationPhysicalReference {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    GenerationCountedPhysicalReference::from_admitted_reference(
        references.admit_root_publication(cell),
    )
    .require_current_generation(cell.generation())
    .expect("current root")
}

pub(super) fn current_page_reference(
    cell: PageGenerationCell,
) -> CurrentGenerationPhysicalReference {
    GenerationCountedPhysicalReference::from_page_cell(cell)
        .require_current_generation(cell.generation())
        .expect("current page")
}

pub(super) fn current_extent_reference(
    cell: ExtentGenerationCell,
) -> CurrentGenerationPhysicalReference {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_extent(cell))
        .require_current_generation(cell.generation())
        .expect("current extent")
}

pub(super) fn current_slot_reference(
    segment: PhysicalSegmentId,
    page: PhysicalPageId,
    slot: PhysicalRecordSlot,
    generation: PhysicalGeneration,
) -> CurrentGenerationPhysicalReference {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment, page, slot)
        .with_slot_generation(generation);
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_page_slot(cell))
        .require_current_generation(generation)
        .expect("current slot")
}

pub(super) fn current_segment_reference(
    segment: PhysicalSegmentId,
    generation: PhysicalGeneration,
) -> CurrentGenerationPhysicalReference {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .segment_cell(segment)
        .with_segment_generation(generation);
    GenerationCountedPhysicalReference::from_segment_cell(cell)
        .require_current_generation(generation)
        .expect("current segment")
}

pub(super) fn root_identity(cell: RootPublicationCell) -> String {
    format!(
        "root:{}:{}",
        cell.root_reference().get(),
        cell.generation().get()
    )
}

pub(super) fn page_identity(cell: PageGenerationCell) -> String {
    format!(
        "page:{}:{}:{}",
        cell.segment_id().get(),
        cell.page_id().get(),
        cell.generation().get()
    )
}

pub(super) fn extent_identity(cell: ExtentGenerationCell) -> String {
    format!(
        "extent:{}:{}:{}",
        cell.segment_id().get(),
        cell.extent_id().get(),
        cell.generation().get()
    )
}

pub(super) fn segment(raw: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(raw).expect("segment")
}

pub(super) fn page(raw: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(raw).expect("page")
}

pub(super) fn extent(raw: u64) -> worth_store_physical_format::PhysicalExtentId {
    worth_store_physical_format::PhysicalExtentId::from_raw(raw).expect("extent")
}

pub(super) fn record_slot(raw: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(raw).expect("slot")
}
