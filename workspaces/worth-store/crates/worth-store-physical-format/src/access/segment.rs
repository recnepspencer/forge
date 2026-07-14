use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::PhysicalLayoutAccessFamily;
use crate::{
    PhysicalSegmentId, PhysicalStoreRuntime, PhysicalStoreRuntimeDenial,
    PhysicalStoreRuntimeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSegmentLayoutReport {
    segment_id: PhysicalSegmentId,
    page_slots: u32,
    extents: u32,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

#[derive(Debug)]
pub struct SegmentAccess<'a> {
    facade: &'a mut PhysicalStoreRuntime,
}

impl<'a> SegmentAccess<'a> {
    pub(crate) fn new(facade: &'a mut PhysicalStoreRuntime) -> Self {
        Self { facade }
    }

    pub fn read_segment(
        &mut self,
        segment_id: PhysicalSegmentId,
    ) -> Result<PhysicalSegmentLayoutReport, PhysicalStoreRuntimeDenial> {
        let occupancy = self
            .facade
            .storage_ref()
            .segment_occupancy(segment_id)
            .ok_or_else(|| {
                PhysicalStoreRuntimeDenial::new(
                    PhysicalStoreRuntimeDenialKind::MissingPhysicalRecord,
                )
            })?;
        let _ = self.facade.mark_read();
        Ok(PhysicalSegmentLayoutReport {
            segment_id,
            page_slots: occupancy.page_slots(),
            extents: occupancy.extents(),
            counters: PhysicalLayoutAccessCounterSnapshot::point(0, 0, 1),
        })
    }
}

impl PhysicalSegmentLayoutReport {
    pub const fn family(self) -> PhysicalLayoutAccessFamily {
        PhysicalLayoutAccessFamily::Segment
    }

    pub const fn segment_id(self) -> PhysicalSegmentId {
        self.segment_id
    }

    pub const fn page_slots(self) -> u32 {
        self.page_slots
    }

    pub const fn extents(self) -> u32 {
        self.extents
    }

    pub const fn counters(self) -> PhysicalLayoutAccessCounterSnapshot {
        self.counters
    }
}
