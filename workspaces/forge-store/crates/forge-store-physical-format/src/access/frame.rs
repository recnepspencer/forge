use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::PhysicalLayoutAccessFamily;
use super::page::locate_page_record;
use crate::{
    FramedRecordView, PhysicalReference, PhysicalReferenceAuthority, PhysicalStoreRuntime,
    PhysicalStoreRuntimeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFrameLayoutReport<'a> {
    reference: PhysicalReference,
    frame_view: FramedRecordView<'a>,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

#[derive(Debug)]
pub struct FrameAccess<'a> {
    facade: &'a mut PhysicalStoreRuntime,
}

impl<'a> FrameAccess<'a> {
    pub(crate) fn new(facade: &'a mut PhysicalStoreRuntime) -> Self {
        Self { facade }
    }
    pub fn read_frame(
        &mut self,
        reference: PhysicalReference,
    ) -> Result<PhysicalFrameLayoutReport<'_>, PhysicalStoreRuntimeDenial> {
        self.facade.ensure_admitted_reference(reference)?;
        self.facade.mark_read();
        let located = locate_page_record(
            self.facade.storage_ref(),
            self.facade.page_records_ref(),
            PhysicalReferenceAuthority::for_canonical_physical_format(),
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
