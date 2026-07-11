use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalRootReference, PhysicalSegmentId,
    RootPublicationValidationWitness,
};
use forge_store_physical_isolation::GenerationCountedPhysicalReference;

pub(super) fn generation_counted_page_reference(
    generation: u64,
) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let cell = generations
        .slot_cell(
            PhysicalSegmentId::from_raw(17).unwrap(),
            PhysicalPageId::from_raw(23).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_page_slot(cell))
}

pub(super) fn root_publication_validation(
    root: u64,
    generation: u64,
) -> RootPublicationValidationWitness {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let cell = generations
        .root_publication_cell(PhysicalRootReference::from_raw(root).unwrap())
        .with_root_publication_generation(PhysicalGeneration::from_raw(generation).unwrap());
    references
        .validate_root_publication(references.admit_root_publication(cell), cell)
        .unwrap()
}
