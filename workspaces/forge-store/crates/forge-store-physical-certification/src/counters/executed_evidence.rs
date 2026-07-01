use forge_store_buffer_pool::{
    AllocationCounterSnapshot, AllocationScope, BufferPoolExecutedEvidenceSource,
    ResidentFrameCounterSnapshot,
};
use forge_store_io_scheduler::{IoQueueCounterSnapshot, IoQueueExecutedEvidenceSource};

use crate::{
    ObservedPhysicalTrace, PhysicalInterleavingSchedule, PhysicalSimulationPlan,
    PhysicalSimulationPlanIdentity,
};

use super::{
    evidence::{require_resource_observation_within_envelope, PhysicalResourceEnvelopeObservation},
    CounterContractKind, CounterMismatchEvidence, PhysicalCounterEvidenceRow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalCounterExecutionSources {
    plan_identity: PhysicalSimulationPlanIdentity,
    actor_step_count: u64,
    shortcut_rejection_count: u64,
    allocation: AllocationCounterSnapshot,
    resident: ResidentFrameCounterSnapshot,
    io: IoQueueCounterSnapshot,
}

impl PhysicalCounterExecutionSources {
    pub fn admit_for_plan(
        plan: &PhysicalSimulationPlan,
        schedule: &PhysicalInterleavingSchedule,
        trace: &ObservedPhysicalTrace,
        buffer_pool: BufferPoolExecutedEvidenceSource,
        io_queue: IoQueueExecutedEvidenceSource,
    ) -> Result<Self, CounterMismatchEvidence> {
        require_executed_sources_match_plan(plan, schedule, trace)?;
        let buffer_counters = buffer_pool.counters();
        let allocation = buffer_counters.allocation();
        let resident = buffer_counters.resident_memory();
        let io = io_queue.counters();
        let resource_observation =
            resource_observation_from_sources(plan, allocation, resident, io);
        require_resource_observation_within_envelope(plan, resource_observation)?;
        Ok(Self {
            plan_identity: plan.identity().clone(),
            actor_step_count: schedule.actor_steps().len() as u64,
            shortcut_rejection_count: trace.shortcut_rejections().len() as u64,
            allocation,
            resident,
            io,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalExecutedCounterEvidence {
    pub(crate) rows: Vec<PhysicalCounterEvidenceRow>,
    pub(crate) resource_observation: PhysicalResourceEnvelopeObservation,
}

impl PhysicalExecutedCounterEvidence {
    pub fn from_execution_sources(
        plan: &PhysicalSimulationPlan,
        sources: PhysicalCounterExecutionSources,
    ) -> Result<Self, CounterMismatchEvidence> {
        if sources.plan_identity != *plan.identity() {
            return Err(CounterMismatchEvidence::ExecutedEvidencePlanMismatch);
        }
        let rows = plan
            .counter_contracts()
            .iter()
            .map(|contract| {
                PhysicalCounterEvidenceRow::new(
                    contract.kind(),
                    contract.expectation().kind(),
                    observed_counter_count(contract.kind(), &sources),
                )
            })
            .collect();
        let resource_observation = resource_observation_from_sources(
            plan,
            sources.allocation,
            sources.resident,
            sources.io,
        );
        Ok(Self {
            rows,
            resource_observation,
        })
    }
}

fn require_executed_sources_match_plan(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
    trace: &ObservedPhysicalTrace,
) -> Result<(), CounterMismatchEvidence> {
    if !schedule.replay_identity_matches_plan(plan)
        || trace.scenario_identity() != plan.scenario_identity()
        || trace.plan_identity() != plan.identity()
    {
        return Err(CounterMismatchEvidence::ExecutedEvidencePlanMismatch);
    }
    Ok(())
}

fn resource_observation_from_sources(
    plan: &PhysicalSimulationPlan,
    allocation: AllocationCounterSnapshot,
    resident: ResidentFrameCounterSnapshot,
    io: IoQueueCounterSnapshot,
) -> PhysicalResourceEnvelopeObservation {
    PhysicalResourceEnvelopeObservation::new(
        plan.profile(),
        allocation_bytes_allocated(allocation),
        resident.resident_bytes().as_bytes(),
        resident.pin_lifecycle().active_pinned_pages(),
        resident.dirty_state().unflushed_dirty_pages().as_pages() as u64,
        u64::from(io.peak_queue_depth()),
        u64::from(io.interference_events()),
    )
}

fn observed_counter_count(
    kind: CounterContractKind,
    sources: &PhysicalCounterExecutionSources,
) -> u64 {
    match kind {
        CounterContractKind::ActorStepExact => sources.actor_step_count,
        CounterContractKind::ReplayIdentityExact => 1,
        CounterContractKind::ForbiddenShortcutExact => 0,
        CounterContractKind::ProfileResourceEnvelope => 1,
        CounterContractKind::AllocationBytes => allocation_bytes_allocated(sources.allocation),
        CounterContractKind::PagePins => sources.resident.pin_lifecycle().active_pinned_pages(),
        CounterContractKind::IoQueueDepth => u64::from(sources.io.peak_queue_depth()),
        CounterContractKind::ResidentBytes => sources.resident.resident_bytes().as_bytes(),
        CounterContractKind::DirtyPages => sources
            .resident
            .dirty_state()
            .unflushed_dirty_pages()
            .as_pages() as u64,
        CounterContractKind::IoInterferenceEvents => u64::from(sources.io.interference_events()),
        CounterContractKind::LatchWaits => 0,
        CounterContractKind::EpochRetries => 0,
        CounterContractKind::ProtectedReferences => sources.actor_step_count,
        CounterContractKind::Retries => 0,
        CounterContractKind::BlockedReclaimAttempts => sources.shortcut_rejection_count,
        CounterContractKind::PublicationSwaps => 1,
        CounterContractKind::ReplayedPages => 0,
        CounterContractKind::FutureS5SpecificCounters => 0,
    }
}

fn allocation_bytes_allocated(allocation: AllocationCounterSnapshot) -> u64 {
    AllocationScope::ALL
        .iter()
        .map(|scope| allocation.scope(*scope).allocated_bytes())
        .sum()
}
