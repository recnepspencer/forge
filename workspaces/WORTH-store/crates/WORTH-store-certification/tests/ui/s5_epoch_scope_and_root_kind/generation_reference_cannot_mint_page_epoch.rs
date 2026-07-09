use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId,
};
use worth_store_physical_isolation::GenerationCountedPhysicalReference;

fn main() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let segment = PhysicalSegmentId::from_raw(1).unwrap();
    let page = PhysicalPageId::from_raw(2).unwrap();
    let slot = PhysicalRecordSlot::from_raw(3).unwrap();
    let cell = generations
        .slot_cell(segment, page, slot)
        .with_slot_generation(PhysicalGeneration::from_raw(4).unwrap());
    let counted = GenerationCountedPhysicalReference::from_admitted_reference(
        references.admit_page_slot(cell),
    );

    let _ = counted.page_epoch();
}
