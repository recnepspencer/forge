use crate::facade_storage::PlatformPhysicalFacadeStorage;
use crate::{
    ExtentMembership, ExtentRecordAppendRequest, PhysicalExtentRecordAuthority, PhysicalPageKind,
    PhysicalPageRecordAuthority, PlatformPhysicalAppendReport, PlatformPhysicalAppendRequest,
    PlatformPhysicalFacadeCounterSnapshot, PlatformPhysicalFacadeDenial,
    PlatformPhysicalFacadeDenialKind, PlatformPhysicalRecordTarget, SlotAppendRequest,
    SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};

pub(crate) struct PlatformPhysicalAppendOutcome {
    report: PlatformPhysicalAppendReport,
    counters: PlatformPhysicalFacadeCounterSnapshot,
}

impl PlatformPhysicalAppendOutcome {
    const fn new(
        report: PlatformPhysicalAppendReport,
        counters: PlatformPhysicalFacadeCounterSnapshot,
    ) -> Self {
        Self { report, counters }
    }

    pub(crate) const fn report(&self) -> PlatformPhysicalAppendReport {
        self.report
    }

    pub(crate) const fn counters(&self) -> PlatformPhysicalFacadeCounterSnapshot {
        self.counters
    }
}

pub(crate) fn append_physical_record(
    storage: &mut PlatformPhysicalFacadeStorage,
    page_records: &PhysicalPageRecordAuthority,
    extent_records: &PhysicalExtentRecordAuthority,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    request: PlatformPhysicalAppendRequest<'_>,
) -> Result<PlatformPhysicalAppendOutcome, PlatformPhysicalFacadeDenial> {
    match request.target() {
        PlatformPhysicalRecordTarget::PageSlot(slot_cell) => append_page_slot_record(
            storage,
            page_records,
            counters,
            slot_cell,
            request.payload(),
        ),
        PlatformPhysicalRecordTarget::Extent(extent_cell) => append_extent_record(
            storage,
            extent_records,
            counters,
            extent_cell,
            request.payload(),
        ),
    }
}

fn append_page_slot_record(
    storage: &mut PlatformPhysicalFacadeStorage,
    page_records: &PhysicalPageRecordAuthority,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    slot_cell: SlotGenerationCell,
    payload: &[u8],
) -> Result<PlatformPhysicalAppendOutcome, PlatformPhysicalFacadeDenial> {
    let page_bytes = storage.page_bytes_for_append(slot_cell);
    let header = page_records
        .decode_record_page_header(
            page_cell_for_slot(slot_cell),
            page_bytes,
            PhysicalPageKind::DataPage,
        )
        .map_err(header_decode_denial)?;
    let page = page_records
        .admit_record_page_payload(page_bytes, header.witness())
        .map_err(header_decode_denial)?;
    let append = page_records
        .append_record(page, SlotAppendRequest::ordinary(slot_cell, payload))
        .map_err(page_record_denial)?;
    storage.replace_page_payload(slot_cell, append.page_payload());
    let updated_counters = counters.with_append().with_write();
    Ok(PlatformPhysicalAppendOutcome::new(
        PlatformPhysicalAppendReport::new(append.reference(), updated_counters),
        updated_counters,
    ))
}

fn append_extent_record(
    storage: &mut PlatformPhysicalFacadeStorage,
    extent_records: &PhysicalExtentRecordAuthority,
    counters: PlatformPhysicalFacadeCounterSnapshot,
    extent_cell: crate::ExtentGenerationCell,
    payload: &[u8],
) -> Result<PlatformPhysicalAppendOutcome, PlatformPhysicalFacadeDenial> {
    let frame_length = PHYSICAL_HEADER_LENGTH as usize + payload.len();
    let membership = ExtentMembership::large_record(extent_cell, frame_length);
    let append = extent_records
        .append_extent_record(
            membership,
            ExtentRecordAppendRequest::large_record(extent_cell, payload),
        )
        .map_err(extent_record_denial)?;
    storage.put_extent(extent_cell, append.extent_bytes());
    let updated_counters = counters.with_append().with_write();
    Ok(PlatformPhysicalAppendOutcome::new(
        PlatformPhysicalAppendReport::new(
            append.reference_admission().reference(),
            updated_counters,
        ),
        updated_counters,
    ))
}

fn page_cell_for_slot(slot_cell: SlotGenerationCell) -> crate::PageGenerationCell {
    crate::PhysicalGenerationAuthority::s1()
        .page_cell(slot_cell.segment_id(), slot_cell.page_id())
        .with_page_generation(slot_cell.generation())
}

fn header_decode_denial(denial: crate::PhysicalHeaderDecodeDenial) -> PlatformPhysicalFacadeDenial {
    PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::HeaderDecodeDenied)
        .with_header_denial(denial)
}

fn page_record_denial(denial: crate::PageRecordDenial) -> PlatformPhysicalFacadeDenial {
    PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::PageRecordDenied)
        .with_page_denial(denial)
}

fn extent_record_denial(denial: crate::ExtentRecordDenial) -> PlatformPhysicalFacadeDenial {
    PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::ExtentRecordDenied)
        .with_extent_denial(denial)
}
