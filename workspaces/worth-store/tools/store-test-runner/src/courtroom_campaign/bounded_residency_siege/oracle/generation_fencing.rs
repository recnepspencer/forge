use super::super::protocol::{
    BoundedResidencyGenerationCleanup, BoundedResidencyGenerationDenial,
    BoundedResidencyGenerationFenceCase, BoundedResidencyGenerationFenceEffects,
    BoundedResidencyGenerationFencingObservation,
};

pub(super) fn verify(
    evidence: BoundedResidencyGenerationFencingObservation,
    world_generation: u64,
) -> Result<(), String> {
    verify_case(
        "read",
        evidence.read,
        world_generation,
        BoundedResidencyGenerationDenial::StaleGeneration,
        BoundedResidencyGenerationFenceEffects {
            allocation_admissions: 0,
            allocation_releases: 0,
            allocation_other: 0,
            residency_hits: 0,
            residency_faults: 0,
            source_loads: 0,
            dirty_transitions: 0,
            writeback_attempts: 0,
            work_declarations: 0,
            signal_requests: 0,
            scheduler_admissions: 0,
            media_attempts: 0,
        },
        BoundedResidencyGenerationCleanup::None,
    )?;
    verify_case(
        "dirty admission",
        evidence.dirty,
        world_generation,
        BoundedResidencyGenerationDenial::StaleOrForeignFrame,
        BoundedResidencyGenerationFenceEffects {
            allocation_admissions: 0,
            allocation_releases: 2,
            allocation_other: 0,
            residency_hits: 0,
            residency_faults: 0,
            source_loads: 0,
            dirty_transitions: 0,
            writeback_attempts: 0,
            work_declarations: 0,
            signal_requests: 0,
            scheduler_admissions: 0,
            media_attempts: 0,
        },
        BoundedResidencyGenerationCleanup::LeaseReleased,
    )?;
    verify_case(
        "writeback admission",
        evidence.writeback,
        world_generation,
        BoundedResidencyGenerationDenial::StaleGeneration,
        BoundedResidencyGenerationFenceEffects {
            allocation_admissions: 0,
            allocation_releases: 0,
            allocation_other: 0,
            residency_hits: 0,
            residency_faults: 0,
            source_loads: 0,
            dirty_transitions: 0,
            writeback_attempts: 0,
            work_declarations: 0,
            signal_requests: 0,
            scheduler_admissions: 0,
            media_attempts: 0,
        },
        BoundedResidencyGenerationCleanup::DirtyReturned,
    )
}

fn verify_case(
    label: &str,
    case: BoundedResidencyGenerationFenceCase,
    world_generation: u64,
    denial: BoundedResidencyGenerationDenial,
    effects: BoundedResidencyGenerationFenceEffects,
    cleanup: BoundedResidencyGenerationCleanup,
) -> Result<(), String> {
    if case.current_generation != world_generation
        || case.current_generation == 0
        || case.stale_generation.checked_add(1) != Some(case.current_generation)
        || case.denial != denial
        || case.effects != effects
        || case.mutation_invocations != 0
        || case.cleanup != cleanup
    {
        return Err(format!(
            "Courtroom C {label} generation fence was not first-boundary exact: {case:?}"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "generation_fencing/tests.rs"]
mod tests;
