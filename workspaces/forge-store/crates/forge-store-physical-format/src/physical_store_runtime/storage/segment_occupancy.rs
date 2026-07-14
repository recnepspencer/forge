use crate::{ExtentGenerationCell, PhysicalSegmentId, SlotGenerationCell};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct StoredSegmentOccupancy {
    page_slots: u32,
    extents: u32,
}

impl StoredSegmentOccupancy {
    pub(crate) const fn page_slots(self) -> u32 {
        self.page_slots
    }

    pub(crate) const fn extents(self) -> u32 {
        self.extents
    }

    pub(crate) fn record_page_slot(&mut self) {
        self.page_slots += 1;
    }

    pub(crate) fn record_extent(&mut self) {
        self.extents += 1;
    }
}

pub(crate) fn build_segment_occupancy(
    page_slots: &[SlotGenerationCell],
    extent_cells: &[ExtentGenerationCell],
) -> BTreeMap<PhysicalSegmentId, StoredSegmentOccupancy> {
    let mut occupancy: BTreeMap<PhysicalSegmentId, StoredSegmentOccupancy> = BTreeMap::new();
    for slot_cell in page_slots {
        occupancy
            .entry(slot_cell.segment_id())
            .or_default()
            .record_page_slot();
    }
    for extent_cell in extent_cells {
        occupancy
            .entry(extent_cell.segment_id())
            .or_default()
            .record_extent();
    }
    occupancy
}
