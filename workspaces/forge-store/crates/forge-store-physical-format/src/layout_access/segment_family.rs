use super::counters::PhysicalLayoutAccessCounterSnapshot;
use super::grammar::{AdmittedSegmentLayoutRule, PhysicalLayoutAccessFamily};
use crate::{
    PhysicalSegmentId, PlatformPhysicalFacade, PlatformPhysicalFacadeDenial,
    PlatformPhysicalFacadeDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentLayoutFamilyAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSegmentLayoutReport {
    segment_id: PhysicalSegmentId,
    page_slots: u32,
    extents: u32,
    counters: PhysicalLayoutAccessCounterSnapshot,
}

impl SegmentLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        &self,
        _rule: &AdmittedSegmentLayoutRule,
    ) -> Result<SegmentLayoutFamilyAdmission, PlatformPhysicalFacadeDenial> {
        Ok(SegmentLayoutFamilyAdmission)
    }
}

#[derive(Debug)]
pub struct AdmittedSegmentLayoutFamily<'a> {
    facade: &'a mut PlatformPhysicalFacade,
    _admission: SegmentLayoutFamilyAdmission,
}

impl<'a> AdmittedSegmentLayoutFamily<'a> {
    pub(crate) fn new(
        facade: &'a mut PlatformPhysicalFacade,
        admission: SegmentLayoutFamilyAdmission,
    ) -> Self {
        Self {
            facade,
            _admission: admission,
        }
    }

    pub fn read_segment(
        &mut self,
        segment_id: PhysicalSegmentId,
    ) -> Result<PhysicalSegmentLayoutReport, PlatformPhysicalFacadeDenial> {
        let occupancy = self
            .facade
            .storage_ref()
            .segment_occupancy(segment_id)
            .ok_or_else(|| {
                PlatformPhysicalFacadeDenial::new(
                    PlatformPhysicalFacadeDenialKind::MissingPhysicalRecord,
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
