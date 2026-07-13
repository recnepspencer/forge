use crate::harness::recovery::closeout as closeout_fixture;
use forge_store_physical_format::{
    PhysicalExtentId, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId,
};
use forge_store_physical_isolation::{
    admit_physical_isolation_entry, admit_physical_read_stability_authority,
    CurrentGenerationPhysicalReference, CurrentPhysicalRoot, GenerationCountedPhysicalReference,
    PhysicalIsolationEntryRequest, PhysicalOrderingContract,
};

pub fn generation_counted_page_reference(generation: u64) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(17).unwrap();
    let page = PhysicalPageId::from_raw(23).unwrap();
    let slot = PhysicalRecordSlot::from_raw(1).unwrap();
    let cell = generations
        .slot_cell(segment, page, slot)
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_page_slot(cell))
}

pub fn current_generation_page_reference(generation: u64) -> CurrentGenerationPhysicalReference {
    generation_counted_page_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}

pub fn current_generation_extent_reference(generation: u64) -> CurrentGenerationPhysicalReference {
    generation_counted_extent_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}

pub fn current_generation_segment_reference(generation: u64) -> CurrentGenerationPhysicalReference {
    generation_counted_segment_reference(generation)
        .require_current_generation(PhysicalGeneration::from_raw(generation).unwrap())
        .unwrap()
}

pub fn current_root_from_authority(
    authority: &forge_store_physical_isolation::PhysicalReadStabilityAuthority,
) -> CurrentPhysicalRoot {
    CurrentPhysicalRoot::from_physical_isolation_entry(
        authority.root_epoch_basis().current_root_basis(),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .unwrap()
}

pub fn physical_authority_from_complete_closeout(
) -> forge_store_physical_isolation::PhysicalReadStabilityAuthority {
    physical_authority_from_completion(closeout_fixture::recovery_completion())
}

pub fn physical_authority_from_complete_closeout_for_store(
    store_identity: &forge_store_physical_format::PhysicalStoreIdentity,
) -> forge_store_physical_isolation::PhysicalReadStabilityAuthority {
    let completion = closeout_fixture::recovery_completion();
    let entry = admit_physical_isolation_entry(PhysicalIsolationEntryRequest::for_store(
        &completion,
        store_identity,
    ))
    .unwrap();
    admit_physical_read_stability_authority(&entry).unwrap()
}

pub fn physical_authority_from_operation_digest_closeout(
    operation_digest: &str,
) -> forge_store_physical_isolation::PhysicalReadStabilityAuthority {
    physical_authority_from_completion(closeout_fixture::recovery_completion_with_operation_digest(
        operation_digest,
    ))
}

fn generation_counted_extent_reference(generation: u64) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(29).unwrap();
    let extent = PhysicalExtentId::from_raw(31).unwrap();
    let cell = generations
        .extent_cell(segment, extent)
        .with_extent_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_admitted_reference(references.admit_extent(cell))
}

fn generation_counted_segment_reference(generation: u64) -> GenerationCountedPhysicalReference {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(37).unwrap();
    let cell = generations
        .segment_cell(segment)
        .with_segment_generation(PhysicalGeneration::from_raw(generation).unwrap());
    GenerationCountedPhysicalReference::from_segment_cell(cell)
}

fn physical_authority_from_completion(
    completion: forge_store_recovery_physics::RecoveryCompletion,
) -> forge_store_physical_isolation::PhysicalReadStabilityAuthority {
    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_recovery_completion(&completion),
    )
    .unwrap();
    admit_physical_read_stability_authority(&entry).unwrap()
}
