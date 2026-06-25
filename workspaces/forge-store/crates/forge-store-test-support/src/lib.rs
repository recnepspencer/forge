#![forbid(unsafe_code)]

use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
};

pub fn test_physical_reference(slot_index: u16) -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).expect("test segment id is non-zero"),
            PhysicalPageId::from_raw(1).expect("test page id is non-zero"),
            PhysicalRecordSlot::from_raw(slot_index).expect("test slot index is non-zero"),
        )
        .with_slot_generation(
            PhysicalGeneration::from_raw(1).expect("test generation is non-zero"),
        );

    PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference()
}
