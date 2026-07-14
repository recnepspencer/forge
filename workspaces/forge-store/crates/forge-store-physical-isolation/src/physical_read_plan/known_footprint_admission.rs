use crate::{
    CurrentGenerationPhysicalReference, CurrentPhysicalRoot, PhysicalReadStabilityAuthority,
};

use super::{
    admit_seed_stable_read_plan, PhysicalReadPlanAdmissionDenial, PhysicalReadPlanReleaseSemantics,
    PostProtectionPhysicalReadObservation, ProtectedPhysicalReferenceSet, PublishedReaderHazard,
    ReadPlanAdmissionScratchArena, StablePhysicalReadPlan, TraversalAdmissionGuard,
    UnprotectedReadIntent,
};

pub(crate) fn admit_known_footprint_read<I>(
    authority: &PhysicalReadStabilityAuthority,
    root: CurrentPhysicalRoot,
    references: I,
    resident_bytes: u64,
    reference_capacity: usize,
) -> Result<StablePhysicalReadPlan, PhysicalReadPlanAdmissionDenial>
where
    I: IntoIterator<Item = CurrentGenerationPhysicalReference>,
    I::IntoIter: ExactSizeIterator,
{
    let scratch =
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(reference_capacity);
    let protected = ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
        references, scratch,
    )?;
    let observed_references = protected.clone();
    let intent = UnprotectedReadIntent::for_known_footprint(root, protected, resident_bytes)
        .with_release_semantics(PhysicalReadPlanReleaseSemantics::reader_releases_all());
    let hazard = PublishedReaderHazard::publish(authority, intent)?;
    let observed = PostProtectionPhysicalReadObservation::from_authority_after_hazard_publication(
        authority,
        &hazard,
        root,
        observed_references,
    )?;
    let validated = hazard
        .observe_authority_after_publication(authority, observed)?
        .validate()?;
    let traversal = TraversalAdmissionGuard::from_validated_root(validated).admit(
        ReadPlanAdmissionScratchArena::for_protected_reference_capacity(reference_capacity),
    )?;
    admit_seed_stable_read_plan(traversal.into_cursor().finish())
}
