use crate::closeout_fixture;
use worth_store_physical_format::{
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId,
};
use worth_store_physical_isolation::{
    admit_physical_isolation_entry, admit_physical_read_stability_authority,
    CurrentGenerationPhysicalReference, CurrentPhysicalRoot, GenerationCountedPhysicalReference,
    PhysicalIsolationEntryRequest, PhysicalOrderingContract,
};

pub(crate) fn generation_counted_page_reference(
    generation: u64,
) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let segment = PhysicalSegmentId::from_raw(17).unwrap();
    let page = PhysicalPageId::from_raw(23).unwrap();
    let slot = PhysicalRecordSlot::from_raw(1).unwrap();
    let cell = generations
        .slot_cell(segment, page, slot)
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_page_slot(cell))
}

pub(crate) fn current_generation_page_reference(
    generation: u64,
) -> CurrentGenerationPhysicalReference {
    generation_counted_page_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}

pub(crate) fn current_generation_extent_reference(
    generation: u64,
) -> CurrentGenerationPhysicalReference {
    generation_counted_extent_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}

pub(crate) fn current_generation_segment_reference(
    generation: u64,
) -> CurrentGenerationPhysicalReference {
    generation_counted_segment_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}

pub(crate) fn current_root_from_authority(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
) -> CurrentPhysicalRoot {
    CurrentPhysicalRoot::from_s5_entry(
        authority.root_epoch_basis().current_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .unwrap()
}

pub(crate) fn physical_authority_from_complete_closeout(
) -> worth_store_physical_isolation::PhysicalReadStabilityAuthority {
    physical_authority_from_readiness(
        closeout_fixture::certify_complete_closeout().publish_s5_readiness(),
    )
}

#[allow(dead_code)]
pub(crate) fn physical_authority_from_operation_digest_closeout(
    operation_digest: &str,
) -> worth_store_physical_isolation::PhysicalReadStabilityAuthority {
    physical_authority_from_readiness(
        closeout_fixture::certify_closeout_with_operation_digest(operation_digest)
            .publish_s5_readiness(),
    )
}

fn generation_counted_extent_reference(generation: u64) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let segment = PhysicalSegmentId::from_raw(29).unwrap();
    let extent = PhysicalExtentId::from_raw(31).unwrap();
    let cell = generations
        .extent_cell(segment, extent)
        .with_extent_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_extent(cell))
}

fn generation_counted_segment_reference(generation: u64) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::s1();
    let segment = PhysicalSegmentId::from_raw(37).unwrap();
    let cell = generations
        .segment_cell(segment)
        .with_segment_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_segment_cell(cell)
}

fn physical_authority_from_readiness(
    readiness: worth_store_recovery_physics::S5PhysicalIsolationRecoveryReadiness,
) -> worth_store_physical_isolation::PhysicalReadStabilityAuthority {
    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_s4_recovery_readiness(&readiness),
    )
    .unwrap();
    admit_physical_read_stability_authority(&entry).unwrap()
}
