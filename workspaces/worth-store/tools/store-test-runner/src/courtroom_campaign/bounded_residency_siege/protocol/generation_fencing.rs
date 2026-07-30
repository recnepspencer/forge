#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyGenerationDenial {
    StaleGeneration,
    StaleOrForeignFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyGenerationCleanup {
    None,
    LeaseReleased,
    DirtyReturned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyGenerationFenceEffects
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) allocation_admissions: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) allocation_releases: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) allocation_other: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) residency_hits: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) residency_faults: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) source_loads: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) dirty_transitions: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) writeback_attempts: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) work_declarations: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal_requests: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) scheduler_admissions: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) media_attempts: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyGenerationFenceCase
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) current_generation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) stale_generation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) denial:
        BoundedResidencyGenerationDenial,
    pub(in crate::courtroom_campaign::bounded_residency_siege) effects:
        BoundedResidencyGenerationFenceEffects,
    pub(in crate::courtroom_campaign::bounded_residency_siege) mutation_invocations: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) cleanup:
        BoundedResidencyGenerationCleanup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyGenerationFencingObservation
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) read:
        BoundedResidencyGenerationFenceCase,
    pub(in crate::courtroom_campaign::bounded_residency_siege) dirty:
        BoundedResidencyGenerationFenceCase,
    pub(in crate::courtroom_campaign::bounded_residency_siege) writeback:
        BoundedResidencyGenerationFenceCase,
}
