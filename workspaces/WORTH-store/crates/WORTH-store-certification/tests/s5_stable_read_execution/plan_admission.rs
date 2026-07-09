use worth_store_physical_isolation::{
    admit_seed_stable_read_plan, CurrentGenerationPhysicalReference,
    PhysicalReadPlanReleaseSemantics, PostProtectionPhysicalReadObservation,
    ProtectedPhysicalReferenceSet, PublishedReaderHazard, ReadPlanAdmissionScratchArena,
    StablePhysicalReadPlan, TraversalAdmissionGuard, UnprotectedReadIntent,
};

pub(crate) fn admit_plan(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    root: worth_store_physical_isolation::CurrentPhysicalRoot,
    references: ProtectedPhysicalReferenceSet,
    resident_bytes: u64,
    scratch_capacity: usize,
) -> StablePhysicalReadPlan {
    let observed_references = references.clone();
    let intent = UnprotectedReadIntent::for_known_footprint(root, references, resident_bytes)
        .with_release_semantics(PhysicalReadPlanReleaseSemantics::reader_releases_all());
    let hazard = PublishedReaderHazard::publish(authority, intent).unwrap();
    let observed = post_protection_observation(authority, &hazard, root, observed_references);
    let validated = hazard
        .observe_authority_after_publication(authority, observed)
        .unwrap()
        .validate()
        .unwrap();
    let receipt = TraversalAdmissionGuard::from_validated_root(validated)
        .admit(ReadPlanAdmissionScratchArena::for_protected_reference_capacity(scratch_capacity))
        .unwrap();
    admit_seed_stable_read_plan(receipt.into_cursor().finish()).unwrap()
}

pub(crate) fn protected_set<const N: usize>(
    references: [CurrentGenerationPhysicalReference; N],
    scratch_capacity: usize,
) -> ProtectedPhysicalReferenceSet {
    ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
        references,
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(scratch_capacity),
    )
    .unwrap()
}

fn post_protection_observation(
    authority: &worth_store_physical_isolation::PhysicalReadStabilityAuthority,
    hazard: &PublishedReaderHazard,
    root: worth_store_physical_isolation::CurrentPhysicalRoot,
    references: ProtectedPhysicalReferenceSet,
) -> PostProtectionPhysicalReadObservation {
    PostProtectionPhysicalReadObservation::from_authority_after_hazard_publication(
        authority, hazard, root, references,
    )
    .unwrap()
}
