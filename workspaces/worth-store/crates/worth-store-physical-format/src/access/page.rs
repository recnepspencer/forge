use super::counters::PhysicalLayoutAccessCounterSnapshot;
use crate::{
    page_record::RecordLocateReport, InMemoryPhysicalFormatModel,
    InMemoryPhysicalFormatModelDenial, InMemoryPhysicalFormatModelDenialKind,
    PhysicalPageRecordAuthority, PhysicalReference, PhysicalReferenceAuthority,
};

#[derive(Debug)]
pub struct PageAccess<'a> {
    facade: &'a mut InMemoryPhysicalFormatModel,
}

impl<'a> PageAccess<'a> {
    pub(crate) fn new(facade: &'a mut InMemoryPhysicalFormatModel) -> Self {
        Self { facade }
    }

    pub fn locate_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<RecordLocateReport<'_>, InMemoryPhysicalFormatModelDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_locate();
        locate_page_record(
            self.facade.storage_ref(),
            self.facade.page_records_ref(),
            PhysicalReferenceAuthority::for_canonical_physical_format(),
            reference,
        )
    }

    pub fn read_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<RecordLocateReport<'_>, InMemoryPhysicalFormatModelDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_read();
        locate_page_record(
            self.facade.storage_ref(),
            self.facade.page_records_ref(),
            PhysicalReferenceAuthority::for_canonical_physical_format(),
            reference,
        )
    }

    pub fn access_counters(report: RecordLocateReport<'_>) -> PhysicalLayoutAccessCounterSnapshot {
        page_access_counters(report)
    }
}

pub fn page_access_counters(report: RecordLocateReport<'_>) -> PhysicalLayoutAccessCounterSnapshot {
    let counters = report.counters();
    PhysicalLayoutAccessCounterSnapshot::point(
        counters.page_read_count() as u64 * 4_096,
        counters.page_read_count() as u16,
        counters.slot_lookup_count() as u16 + 1,
    )
}

pub(crate) fn locate_page_record<'a>(
    storage: &'a crate::in_memory_physical_format_model::storage::InMemoryPhysicalFormatModelStorage,
    page_records: &PhysicalPageRecordAuthority,
    references: PhysicalReferenceAuthority,
    reference: PhysicalReference,
) -> Result<RecordLocateReport<'a>, InMemoryPhysicalFormatModelDenial> {
    let page = storage.page_for_reference(reference)?;
    let slot_cell = super::reference::slot_cell_from_reference(reference)?;
    let admission = references.admit_page_slot(slot_cell);
    let validation = references
        .validate_page_slot(admission, slot_cell)
        .map_err(|denial| {
            InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::ReferenceValidationDenied,
            )
            .with_reference_denial(denial)
        })?;
    let header = page_records
        .decode_record_page_header(page.cell(), page.bytes(), crate::PhysicalPageKind::DataPage)
        .map_err(|denial| {
            InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::HeaderDecodeDenied,
            )
            .with_header_denial(denial)
        })?;
    let page_payload = page_records
        .admit_record_page_payload(page.bytes(), header.witness())
        .map_err(|denial| {
            InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::HeaderDecodeDenied,
            )
            .with_header_denial(denial)
        })?;
    page_records
        .locate_record(page_payload, validation)
        .map_err(|denial| {
            InMemoryPhysicalFormatModelDenial::new(
                InMemoryPhysicalFormatModelDenialKind::PageRecordDenied,
            )
            .with_page_denial(denial)
        })
}
