#![forbid(unsafe_code)]

use forge_store_physical_format::{
    PhysicalGeneration, PhysicalPageId, PhysicalReference, PhysicalSegmentId,
};

pub fn test_physical_reference(slot_index: u16) -> PhysicalReference {
    PhysicalReference::new(
        PhysicalSegmentId(1),
        PhysicalPageId(1),
        slot_index,
        PhysicalGeneration(1),
    )
}
