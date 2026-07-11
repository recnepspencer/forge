use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::PhysicalLayoutAccessFamily;
use super::page_family::locate_page_record;
use crate::{
    FramedRecordView, PhysicalReference, PhysicalReferenceAuthority, PlatformPhysicalFacade,
    PlatformPhysicalFacadeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameLayoutFamilyAdmission {
    family: PhysicalLayoutAccessFamily,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFrameLayoutReport<'a> {
    reference: PhysicalReference,
    frame_view: FramedRecordView<'a>,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

impl FrameLayoutFamilyHome {
    pub const fn physical() -> Self {
        Self
    }

    pub fn admit(&self) -> Result<FrameLayoutFamilyAdmission, PlatformPhysicalFacadeDenial> {
        Ok(FrameLayoutFamilyAdmission {
            family: PhysicalLayoutAccessFamily::Frame,
        })
    }
}

impl FrameLayoutFamilyAdmission {
    pub const fn family(self) -> PhysicalLayoutAccessFamily {
        self.family
    }
}

#[derive(Debug)]
pub struct AdmittedFrameLayoutFamily<'a> {
    facade: &'a mut PlatformPhysicalFacade,
    admission: FrameLayoutFamilyAdmission,
}

impl<'a> AdmittedFrameLayoutFamily<'a> {
    pub(crate) fn new(
        facade: &'a mut PlatformPhysicalFacade,
        admission: FrameLayoutFamilyAdmission,
    ) -> Self {
        Self { facade, admission }
    }

    pub const fn family(&self) -> PhysicalLayoutAccessFamily {
        self.admission.family()
    }

    pub fn read_frame(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<PhysicalFrameLayoutReport<'_>, PlatformPhysicalFacadeDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_read();
        let located = locate_page_record(
            self.facade.storage_ref(),
            self.facade.page_records_ref(),
            PhysicalReferenceAuthority::s1(),
            reference,
        )?;
        let report_counters = located.counters();
        Ok(PhysicalFrameLayoutReport {
            reference: located.reference(),
            frame_view: located.record_view(),
            counters: PhysicalLayoutAccessCounterSnapshot::point(
                report_counters.page_read_count() as u64 * 4_096,
                report_counters.page_read_count() as u16,
                report_counters.slot_lookup_count() as u16 + 1,
            ),
        })
    }
}

impl<'a> PhysicalFrameLayoutReport<'a> {
    pub const fn family(self) -> PhysicalLayoutAccessFamily {
        PhysicalLayoutAccessFamily::Frame
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn frame_view(self) -> FramedRecordView<'a> {
        self.frame_view
    }

    pub const fn counters(self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }
}
