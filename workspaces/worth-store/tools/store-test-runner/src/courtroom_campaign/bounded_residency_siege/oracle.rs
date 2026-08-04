use std::{collections::BTreeSet, num::NonZeroU32};

use worth_store::physical_runtime::{
    PhysicalWorkCourtroomRunBinding, PhysicalWorkEvidenceDigest, PhysicalWorkExecutionContext,
    PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence, PhysicalWorkPlatformEvidence,
    PhysicalWorkProcessEvidence, PhysicalWorkRerunEvidence, PhysicalWorkRunEnvironmentEvidence,
    PhysicalWorkScheduleSeed, PhysicalWorkWorkloadSeed,
};

use super::{
    binary_binding::BuiltCourtroomExecutables,
    c7_crash_campaign::C7CrashCampaignEvidence,
    execution::BoundedResidencySiegeObservations,
    protocol::BoundedResidencySiegeObservation,
    schedule::{SchedulePerturbationPlan, SourceClosureScheduleSeeds},
    world::BoundedResidencySiegeWorld,
};

#[path = "oracle/allocation.rs"]
mod allocation;
#[path = "oracle/artifact_policy.rs"]
mod artifact_policy;
#[path = "oracle/c7_campaign.rs"]
mod c7_campaign;
#[path = "oracle/cancellation.rs"]
mod cancellation;
#[path = "oracle/digest.rs"]
mod digest;
#[path = "oracle/dirty_close.rs"]
mod dirty_close;
#[path = "oracle/evidence.rs"]
mod evidence;
#[path = "oracle/generation_fencing.rs"]
mod generation_fencing;
#[path = "oracle/performance.rs"]
mod performance;
#[path = "oracle/pressure.rs"]
mod pressure;
#[path = "oracle/process_allocation.rs"]
mod process_allocation;
#[path = "oracle/speculation.rs"]
mod speculation;
#[path = "oracle/work_reconciliation.rs"]
mod work_reconciliation;
use allocation::verify_allocation;
use artifact_policy::verify_artifact_manifest;
pub(super) use evidence::BoundedResidencyCourtroomEvidence;
use pressure::verify_residency;

#[cfg(test)]
pub(super) fn verify_work_reconciliation(
    child: &BoundedResidencySiegeObservation,
) -> Result<(), String> {
    work_reconciliation::verify(
        &child.work_reconciliation,
        child.store(),
        child.runtime(),
        child.generation(),
    )
}

#[cfg(test)]
pub(super) fn verify_performance(child: &BoundedResidencySiegeObservation) -> Result<(), String> {
    performance::verify(child)
}

pub(super) struct BoundedResidencyCourtroomProofRequest<'evidence> {
    pub(super) world: &'evidence BoundedResidencySiegeWorld,
    pub(super) binaries: &'evidence BuiltCourtroomExecutables,
    pub(super) observations: BoundedResidencySiegeObservations,
    pub(super) controlled_cases: Vec<PhysicalWorkMutantLocalization>,
    pub(super) rerun: PhysicalWorkRerunEvidence,
    pub(super) schedule: SchedulePerturbationPlan,
    pub(super) source_schedule: SourceClosureScheduleSeeds,
    pub(super) termination_campaign: C7CrashCampaignEvidence,
}

impl BoundedResidencyCourtroomProofRequest<'_> {
    fn verify_truth(&self) -> Result<PhysicalWorkOracleEvidence, String> {
        let observations = &self.observations;
        if observations.child.schedule != *self.schedule.serving_decisions() {
            return Err("Courtroom C child executed a foreign schedule trace".into());
        }
        verify_processes(observations)?;
        verify_current_truth(self.world, observations)?;
        generation_fencing::verify(
            observations.child.generation_fencing,
            observations.child.generation(),
        )?;
        verify_process_allocation(&observations.child)?;
        verify_residency(self.world, &observations.child)?;
        speculation::verify(observations.child.speculation)?;
        work_reconciliation::verify(
            &observations.child.work_reconciliation,
            observations.child.store(),
            observations.child.runtime(),
            observations.child.generation(),
        )?;
        cancellation::verify(
            observations.child.cancellation,
            &observations.child.work_reconciliation,
            observations.child.store(),
            observations.child.runtime(),
            observations.child.generation(),
        )?;
        performance::verify(&observations.child)?;
        let exclusive_operation_limit = allocation_operation_limit(&observations.child)?;
        verify_allocation(
            &observations.child.allocation,
            observations.child.store(),
            observations.child.process().get(),
            exclusive_operation_limit,
        )?;
        let artifact_bytes = verify_artifact_manifest(&observations.offline)?;
        verify_world_scale(self.world, &observations.child, artifact_bytes)?;
        verify_reopen(observations)?;
        verify_mutants(&self.controlled_cases)?;
        c7_campaign::verify(&self.termination_campaign, &self.schedule)?;
        PhysicalWorkOracleEvidence::new(
            "courtroom-c:bounded-residency-siege:independent-physical-truth",
            true,
            digest::build(self.world, observations, &self.termination_campaign)?,
        )
        .map_err(|denial| format!("Courtroom C oracle binding denied: {denial:?}"))
    }
}

pub(super) fn verify(
    request: BoundedResidencyCourtroomProofRequest<'_>,
) -> Result<BoundedResidencyCourtroomEvidence, String> {
    let oracle = request.verify_truth()?;
    let BoundedResidencyCourtroomProofRequest {
        world,
        binaries,
        observations,
        controlled_cases,
        rerun,
        schedule,
        source_schedule,
        termination_campaign,
    } = request;
    let execution = PhysicalWorkExecutionContext::new(
        PhysicalWorkWorkloadSeed::new(world.seed()),
        PhysicalWorkScheduleSeed::new(schedule.seed().value()),
        schedule.child_argument(),
        observations.processes.iter().cloned(),
    )
    .map_err(|denial| format!("Courtroom C execution binding denied: {denial:?}"))?;
    let environment = PhysicalWorkRunEnvironmentEvidence::new(
        binaries.feature_graph().clone(),
        PhysicalWorkPlatformEvidence::current(),
        observations.filesystem.clone(),
        rerun,
    );
    let run = PhysicalWorkCourtroomRunBinding::new(
        binaries.source().clone(),
        binaries.writer().binding().clone(),
        execution,
        environment,
    );
    Ok(BoundedResidencyCourtroomEvidence {
        run,
        runner: binaries.runner().binding().clone(),
        observer: binaries.observer().binding().clone(),
        producer: observations.producer,
        child: observations.child,
        offline: observations.offline,
        reopen: observations.reopen,
        oracle,
        mutants: controlled_cases.into_boxed_slice(),
        workload_seed: world.seed(),
        schedule,
        source_schedule,
        crash_campaign: termination_campaign,
    })
}

pub(super) fn verify_process_allocation(
    child: &BoundedResidencySiegeObservation,
) -> Result<(), String> {
    process_allocation::verify(
        child.process_allocation,
        child.process(),
        child.payload_bytes(),
    )
}

fn verify_world_scale(
    world: &BoundedResidencySiegeWorld,
    child: &BoundedResidencySiegeObservation,
    artifact_bytes: u64,
) -> Result<(), String> {
    if artifact_bytes != child.directory_bytes()
        || world.expected_payload_bytes() < world.resident_byte_limit().saturating_mul(32)
        || world.expected_payload_bytes() < world.admitted_byte_limit().saturating_mul(16)
    {
        return Err("Courtroom C durable world was not materially larger than residency".into());
    }
    Ok(())
}

fn verify_processes(observations: &BoundedResidencySiegeObservations) -> Result<(), String> {
    verify_ordinary_process_set(
        &observations.processes,
        [
            observations.producer.process,
            observations.child.process(),
            observations.offline.process(),
            observations.reopen.identity().process(),
        ],
    )
}

fn verify_ordinary_process_set(
    processes: &[PhysicalWorkProcessEvidence],
    expected: [NonZeroU32; 4],
) -> Result<(), String> {
    if processes.len() != expected.len() {
        return Err("Courtroom C ordinary process set was extended or truncated".into());
    }
    let identities = processes
        .iter()
        .map(|process| process.process())
        .collect::<Vec<_>>();
    if identities.iter().collect::<BTreeSet<_>>().len() != identities.len()
        || identities
            .iter()
            .copied()
            .zip(expected)
            .any(|pair| pair.0 != pair.1)
    {
        return Err("Courtroom C process roles were duplicated or reordered".into());
    }
    Ok(())
}

fn verify_current_truth(
    world: &BoundedResidencySiegeWorld,
    observations: &BoundedResidencySiegeObservations,
) -> Result<(), String> {
    let child = &observations.child;
    let producer = observations.producer;
    let verifier = observations.verifier;
    let current = observations.offline.current();
    let expected_bytes = world.expectation_digest();
    let expected_digest = PhysicalWorkEvidenceDigest::new(expected_bytes)
        .ok_or_else(|| "Courtroom C seed-derived digest was all zero".to_owned())?;
    if producer.store != child.store()
        || producer.store != current.store()
        || producer.runtime == child.runtime()
        || producer.generation != child.generation()
        || producer.records != world.producer_records()
        || producer.payload_bytes != world.producer_payload_bytes()
        || producer.expectation_digest != expected_bytes
        || producer.peak_resident_bytes > world.resident_byte_limit()
        || child.records() != world.expected_records()
        || current.records() != world.expected_records()
        || child.payload_bytes() != world.expected_payload_bytes()
        || current.payload_bytes() != world.expected_payload_bytes()
        || verifier.records != world.expected_records()
        || verifier.payload_bytes != world.expected_payload_bytes()
        || verifier.expectation_digest != expected_bytes
        || verifier.seed != world.seed()
    {
        return Err(format!(
            "Courtroom C role-separated truth disagrees with the seed model: \
             producer={producer:?}, child={child:?}, verifier={verifier:?}, \
             offline={current:?}, digest={expected_digest:?}"
        ));
    }
    Ok(())
}

fn verify_reopen(observations: &BoundedResidencySiegeObservations) -> Result<(), String> {
    let identity = observations.reopen.identity();
    let posture = observations.reopen.posture();
    if identity.store() != observations.child.store()
        || identity.store() != observations.offline.current().store()
        || identity.generation() != observations.offline.current().generation()
        || identity.records() != observations.offline.current().records()
        || posture.residue()
        || posture.recovery_evidence_damaged()
        || posture.recovery_obligations() != 0
        || posture.inspection_required()
    {
        return Err("Courtroom C fresh reopen disagrees with offline durable truth".into());
    }
    Ok(())
}

fn verify_mutants(mutants: &[PhysicalWorkMutantLocalization]) -> Result<(), String> {
    if mutants.is_empty() || mutants.iter().any(|mutant| !mutant.killed()) {
        return Err("Courtroom C mutation evidence contains a survivor".into());
    }
    let required = crate::mutation_campaign::bounded_residency_requirements();
    for requirement in required {
        let identity = requirement.identity();
        let predicate = requirement.predicate();
        if !mutants
            .iter()
            .any(|mutant| mutant.identity() == identity && mutant.predicate() == predicate)
        {
            return Err(format!(
                "Courtroom C mutation evidence omitted mutant {identity} `{predicate}`"
            ));
        }
    }
    Ok(())
}

fn allocation_operation_limit(child: &BoundedResidencySiegeObservation) -> Result<u64, String> {
    [
        child.dirty.retry_last_candidate_operation,
        child.reads.last_operation,
        child.cancellation.pre_dispatch.operation,
        child.cancellation.post_dispatch.operation,
    ]
    .into_iter()
    .max()
    .and_then(|operation| operation.checked_add(1))
    .ok_or_else(|| "Courtroom C could not bound allocation operation attribution".to_owned())
}

#[cfg(test)]
#[path = "oracle/tests.rs"]
mod tests;
