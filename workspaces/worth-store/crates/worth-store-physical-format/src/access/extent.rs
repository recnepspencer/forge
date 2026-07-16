use super::counters::PhysicalLayoutAccessCounterSnapshot;
use crate::{
    extent_record::ExtentRecordLocateReport, ExtentMembership, PhysicalExtentRecordAuthority,
    PhysicalReference, PhysicalReferenceAuthority, PhysicalStoreRuntime,
    PhysicalStoreRuntimeDenial, PhysicalStoreRuntimeDenialKind,
};

#[derive(Debug)]
pub struct ExtentAccess<'a> {
    facade: &'a mut PhysicalStoreRuntime,
}

impl<'a> ExtentAccess<'a> {
    pub(crate) fn new(facade: &'a mut PhysicalStoreRuntime) -> Self {
        Self { facade }
    }

    pub fn locate_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<ExtentRecordLocateReport<'_>, PhysicalStoreRuntimeDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_locate();
        locate_extent_record(
            self.facade.storage_ref(),
            self.facade.extent_records_ref(),
            PhysicalReferenceAuthority::for_canonical_physical_format(),
            reference,
        )
    }

    pub fn read_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<ExtentRecordLocateReport<'_>, PhysicalStoreRuntimeDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_read();
        locate_extent_record(
            self.facade.storage_ref(),
            self.facade.extent_records_ref(),
            PhysicalReferenceAuthority::for_canonical_physical_format(),
            reference,
        )
    }

    pub fn access_counters(
        report: ExtentRecordLocateReport<'_>,
    ) -> PhysicalLayoutAccessCounterSnapshot {
        extent_access_counters(report)
    }
}

pub fn extent_access_counters(
    report: ExtentRecordLocateReport<'_>,
) -> PhysicalLayoutAccessCounterSnapshot {
    let counters = report.counters();
    PhysicalLayoutAccessCounterSnapshot::point(
        counters.extent_read_count() as u64 * 4_096,
        counters.extent_read_count() as u16,
        counters.extent_locate_count() as u16 + 1,
    )
}

pub(crate) fn locate_extent_record<'a>(
    storage: &'a crate::physical_store_runtime::storage::PhysicalStoreRuntimeStorage,
    extent_records: &PhysicalExtentRecordAuthority,
    references: PhysicalReferenceAuthority,
    reference: PhysicalReference,
) -> Result<ExtentRecordLocateReport<'a>, PhysicalStoreRuntimeDenial> {
    let extent = storage.extent_for_reference(reference)?;
    let extent_cell = super::reference::extent_cell_from_reference(reference)?;
    let admission = references.admit_extent(extent_cell);
    let validation = references
        .validate_extent(admission, extent_cell)
        .map_err(|denial| {
            PhysicalStoreRuntimeDenial::new(
                PhysicalStoreRuntimeDenialKind::ReferenceValidationDenied,
            )
            .with_reference_denial(denial)
        })?;
    let membership = ExtentMembership::large_record(extent_cell, extent.bytes().len());
    extent_records
        .locate_extent_record(extent.bytes(), membership, validation)
        .map_err(|denial| {
            PhysicalStoreRuntimeDenial::new(PhysicalStoreRuntimeDenialKind::ExtentRecordDenied)
                .with_extent_denial(denial)
        })
}
