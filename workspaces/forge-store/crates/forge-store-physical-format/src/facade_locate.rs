use crate::facade_storage::PlatformPhysicalFacadeStorage;
use crate::{
    ExtentMembership, PhysicalExtentRecordAuthority, PhysicalPageKind, PhysicalPageRecordAuthority,
    PhysicalReference, PhysicalReferenceAuthority, PlatformPhysicalFacadeCounterSnapshot,
    PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind, PlatformPhysicalFramedRecord,
    PlatformPhysicalLocateReport,
};

pub(crate) fn locate_page_slot<'a>(
    storage: &'a PlatformPhysicalFacadeStorage,
    page_records: &PhysicalPageRecordAuthority,
    references: PhysicalReferenceAuthority,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    reference: PhysicalReference,
) -> Result<PlatformPhysicalLocateReport<'a>, PlatformPhysicalFacadeDenial> {
    let page = storage.page_for_reference(reference)?;
    let slot_cell = slot_cell_from_reference(reference)?;
    let admission = references.admit_page_slot(slot_cell);
    let validation = references
        .validate_page_slot(admission, slot_cell)
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::ReferenceValidationDenied,
            )
            .with_reference_denial(denial)
        })?;
    let header = page_records
        .decode_record_page_header(page.cell(), page.bytes(), PhysicalPageKind::DataPage)
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::HeaderDecodeDenied)
                .with_header_denial(denial)
        })?;
    let page_payload = page_records
        .admit_record_page_payload(page.bytes(), header.witness())
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::HeaderDecodeDenied)
                .with_header_denial(denial)
        })?;
    let located = page_records
        .locate_record(page_payload, validation)
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::PageRecordDenied)
                .with_page_denial(denial)
        })?;
    Ok(PlatformPhysicalLocateReport::new(
        reference,
        PlatformPhysicalFramedRecord::PageSlot(located.record_view()),
        counters,
    ))
}

pub(crate) fn locate_extent<'a>(
    storage: &'a PlatformPhysicalFacadeStorage,
    extent_records: &PhysicalExtentRecordAuthority,
    references: PhysicalReferenceAuthority,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    reference: PhysicalReference,
) -> Result<PlatformPhysicalLocateReport<'a>, PlatformPhysicalFacadeDenial> {
    let extent = storage.extent_for_reference(reference)?;
    let extent_cell = extent_cell_from_reference(reference)?;
    let admission = references.admit_extent(extent_cell);
    let validation = references
        .validate_extent(admission, extent_cell)
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(
                PlatformPhysicalFacadeDenialKind::ReferenceValidationDenied,
            )
            .with_reference_denial(denial)
        })?;
    let membership = ExtentMembership::large_record(extent_cell, extent.bytes().len());
    let located = extent_records
        .locate_extent_record(extent.bytes(), membership, validation)
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::ExtentRecordDenied)
                .with_extent_denial(denial)
        })?;
    Ok(PlatformPhysicalLocateReport::new(
        reference,
        PlatformPhysicalFramedRecord::Extent(located.record_view()),
        counters,
    ))
}

fn slot_cell_from_reference(
    reference: PhysicalReference,
) -> Result<crate::SlotGenerationCell, PlatformPhysicalFacadeDenial> {
    let segment_id = reference.segment_id().ok_or_else(missing_record)?;
    let page_id = reference.page_id().ok_or_else(missing_record)?;
    let slot = reference.slot().ok_or_else(missing_record)?;
    Ok(crate::PhysicalGenerationAuthority::s1()
        .slot_cell(segment_id, page_id, slot)
        .with_slot_generation(reference.generation()))
}

fn extent_cell_from_reference(
    reference: PhysicalReference,
) -> Result<crate::ExtentGenerationCell, PlatformPhysicalFacadeDenial> {
    let segment_id = reference.segment_id().ok_or_else(missing_record)?;
    let extent_id = reference.extent_id().ok_or_else(missing_record)?;
    Ok(crate::PhysicalGenerationAuthority::s1()
        .extent_cell(segment_id, extent_id)
        .with_extent_generation(reference.generation()))
}

fn missing_record() -> PlatformPhysicalFacadeDenial {
    PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord)
}
