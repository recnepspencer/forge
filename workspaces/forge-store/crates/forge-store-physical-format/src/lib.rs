#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalSegmentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalPageId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalExtentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalGeneration(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReference {
    pub segment_id: PhysicalSegmentId,
    pub page_id: PhysicalPageId,
    pub slot_index: u16,
    pub generation: PhysicalGeneration,
}

impl PhysicalReference {
    pub const fn new(
        segment_id: PhysicalSegmentId,
        page_id: PhysicalPageId,
        slot_index: u16,
        generation: PhysicalGeneration,
    ) -> Self {
        Self {
            segment_id,
            page_id,
            slot_index,
            generation,
        }
    }
}
