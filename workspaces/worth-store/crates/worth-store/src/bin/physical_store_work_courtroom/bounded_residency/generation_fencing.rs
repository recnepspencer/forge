use worth_store::physical_runtime::{
    LifecycleGeneration, PhysicalResidencyAllocationBoundaryKind, PhysicalResidencyCertification,
    PhysicalWorkCounterStage, ServingPhysicalRuntime,
};

mod dirty_admission;
mod read_admission;
mod writeback_admission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GenerationFenceDenial {
    StaleGeneration,
    StaleOrForeignFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GenerationFenceCleanup {
    None,
    LeaseReleased,
    DirtyReturned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct GenerationFenceEffects {
    pub(super) allocation_admissions: u64,
    pub(super) allocation_releases: u64,
    pub(super) allocation_other: u64,
    pub(super) residency_hits: u64,
    pub(super) residency_faults: u64,
    pub(super) source_loads: u64,
    pub(super) dirty_transitions: u64,
    pub(super) writeback_attempts: u64,
    pub(super) work_declarations: u64,
    pub(super) signal_requests: u64,
    pub(super) scheduler_admissions: u64,
    pub(super) media_attempts: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct GenerationFenceCaseEvidence {
    pub(super) current_generation: LifecycleGeneration,
    pub(super) stale_generation: LifecycleGeneration,
    pub(super) denial: GenerationFenceDenial,
    pub(super) effects: GenerationFenceEffects,
    pub(super) mutation_invocations: u64,
    pub(super) cleanup: GenerationFenceCleanup,
}

pub(super) struct GenerationFencingEvidence {
    pub(super) read: GenerationFenceCaseEvidence,
    pub(super) dirty: GenerationFenceCaseEvidence,
    pub(super) writeback: GenerationFenceCaseEvidence,
}

struct GenerationBoundarySnapshot {
    allocation_events: usize,
    residency_hits: u64,
    residency_faults: u64,
    source_loads: u64,
    dirty_transitions: u64,
    writeback_attempts: u64,
    work_declarations: u64,
    signal_requests: u64,
    scheduler_admissions: u64,
    media_attempts: u64,
}

pub(super) fn prove(serving: &ServingPhysicalRuntime) -> Result<GenerationFencingEvidence, String> {
    let current = serving.certification_physical_residency();
    let stale = serving.certification_stale_physical_residency();
    let current_generation = current.lifecycle_generation();
    let stale_generation = stale.lifecycle_generation();
    if current_generation.get() == 0
        || stale_generation
            .get()
            .checked_add(1)
            .filter(|successor| *successor == current_generation.get())
            .is_none()
    {
        return Err("bounded-residency generation fence lacked an exact predecessor".to_owned());
    }
    Ok(GenerationFencingEvidence {
        read: read_admission::prove(
            serving,
            &current,
            &stale,
            current_generation,
            stale_generation,
        )?,
        dirty: dirty_admission::prove(
            serving,
            &current,
            &stale,
            current_generation,
            stale_generation,
        )?,
        writeback: writeback_admission::prove(
            serving,
            &current,
            &stale,
            current_generation,
            stale_generation,
        )?,
    })
}

impl GenerationBoundarySnapshot {
    fn capture(
        serving: &ServingPhysicalRuntime,
        residency: &PhysicalResidencyCertification,
    ) -> Result<Self, String> {
        let counters = residency.counters();
        let signal = serving.physical_signal_observation().map_err(|denial| {
            format!("bounded-residency Signal observation was denied: {denial:?}")
        })?;
        Ok(Self {
            allocation_events: residency.allocation_trace().event_count(),
            residency_hits: counters.hits(),
            residency_faults: counters.faults(),
            source_loads: counters.source_loads(),
            dirty_transitions: counters.dirty_transitions(),
            writeback_attempts: serving.residency_observation().writebacks().attempts(),
            work_declarations: serving
                .physical_work_counters()
                .total(PhysicalWorkCounterStage::Declared),
            signal_requests: signal.request_admission_count(),
            scheduler_admissions: serving
                .physical_scheduler_capacity()
                .admitted_reservations(),
            media_attempts: serving.media_counters().attempted_operations(),
        })
    }

    fn effects_since(
        &self,
        serving: &ServingPhysicalRuntime,
        residency: &PhysicalResidencyCertification,
    ) -> Result<GenerationFenceEffects, String> {
        let trace = residency.allocation_trace();
        let mut allocation_admissions = 0_u64;
        let mut allocation_releases = 0_u64;
        let mut allocation_other = 0_u64;
        for event in trace.events().skip(self.allocation_events) {
            match event.kind() {
                PhysicalResidencyAllocationBoundaryKind::Admission => allocation_admissions += 1,
                PhysicalResidencyAllocationBoundaryKind::Release => allocation_releases += 1,
                PhysicalResidencyAllocationBoundaryKind::Denial
                | PhysicalResidencyAllocationBoundaryKind::AllocatorFailure
                | PhysicalResidencyAllocationBoundaryKind::Actualization => allocation_other += 1,
            }
        }
        let after = Self::capture(serving, residency)?;
        Ok(GenerationFenceEffects {
            allocation_admissions,
            allocation_releases,
            allocation_other,
            residency_hits: delta(after.residency_hits, self.residency_hits, "residency hits")?,
            residency_faults: delta(
                after.residency_faults,
                self.residency_faults,
                "residency faults",
            )?,
            source_loads: delta(after.source_loads, self.source_loads, "source loads")?,
            dirty_transitions: delta(
                after.dirty_transitions,
                self.dirty_transitions,
                "dirty transitions",
            )?,
            writeback_attempts: delta(
                after.writeback_attempts,
                self.writeback_attempts,
                "writeback attempts",
            )?,
            work_declarations: delta(
                after.work_declarations,
                self.work_declarations,
                "work declarations",
            )?,
            signal_requests: delta(
                after.signal_requests,
                self.signal_requests,
                "Signal requests",
            )?,
            scheduler_admissions: delta(
                after.scheduler_admissions,
                self.scheduler_admissions,
                "scheduler admissions",
            )?,
            media_attempts: delta(after.media_attempts, self.media_attempts, "media attempts")?,
        })
    }
}

fn delta(after: u64, before: u64, label: &str) -> Result<u64, String> {
    after
        .checked_sub(before)
        .ok_or_else(|| format!("bounded-residency {label} counter regressed"))
}

fn coordinate() -> worth_store_physical_format::RecordFrameCoordinate {
    worth_store_physical_format::RecordFrameCoordinate::new(
        worth_store_physical_format::RecordArtifactFile::BootstrapCatalog,
        8,
        8,
    )
    .expect("bounded-residency generation coordinate is valid")
}
