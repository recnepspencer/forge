use super::counters::PhysicalLayoutAccessCounterSnapshot;
use crate::{
    extent_record::ExtentRecordLocateReport, ExtentMembership, PhysicalExtentRecordAuthority,
    PhysicalReference, PhysicalReferenceAuthority, PlatformPhysicalFacade,
    PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtentLayoutFamilyAdmission;

impl ExtentLayoutFamilyHome {
    pub const fn physical() -> Self {
        Self
    }

    pub fn admit(&self) -> Result<ExtentLayoutFamilyAdmission, PlatformPhysicalFacadeDenial> {
        Ok(ExtentLayoutFamilyAdmission)
    }
}

#[derive(Debug)]
pub struct AdmittedExtentLayoutFamily<'a> {
    facade: &'a mut PlatformPhysicalFacade,
    _admission: ExtentLayoutFamilyAdmission,
}

impl<'a> AdmittedExtentLayoutFamily<'a> {
    pub(crate) fn new(
        facade: &'a mut PlatformPhysicalFacade,
        admission: ExtentLayoutFamilyAdmission,
    ) -> Self {
        Self {
            facade,
            _admission: admission,
        }
    }

    pub fn locate_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<ExtentRecordLocateReport<'_>, PlatformPhysicalFacadeDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_locate();
        locate_extent_record(
            self.facade.storage_ref(),
            self.facade.extent_records_ref(),
            PhysicalReferenceAuthority::s1(),
            reference,
        )
    }

    pub fn read_record(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<ExtentRecordLocateReport<'_>, PlatformPhysicalFacadeDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_read();
        locate_extent_record(
            self.facade.storage_ref(),
            self.facade.extent_records_ref(),
            PhysicalReferenceAuthority::s1(),
            reference,
        )
    }

    pub fn access_counters(
        report: ExtentRecordLocateReport<'_>,
    ) -> PhysicalLayoutAccessCounterSnapshot {
        extent_layout_access_counters(report)
    }
}

pub fn extent_layout_access_counters(
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
    storage: &'a crate::facade::storage::PlatformPhysicalFacadeStorage,
    extent_records: &PhysicalExtentRecordAuthority,
    references: PhysicalReferenceAuthority,
    reference: PhysicalReference,
) -> Result<ExtentRecordLocateReport<'a>, PlatformPhysicalFacadeDenial> {
    let extent = storage.extent_for_reference(reference)?;
    let extent_cell = super::record_family::extent_cell_from_reference(reference)?;
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
    extent_records
        .locate_extent_record(extent.bytes(), membership, validation)
        .map_err(|denial| {
            PlatformPhysicalFacadeDenial::new(PlatformPhysicalFacadeDenialKind::ExtentRecordDenied)
                .with_extent_denial(denial)
        })
        .map(|located| located)
}
