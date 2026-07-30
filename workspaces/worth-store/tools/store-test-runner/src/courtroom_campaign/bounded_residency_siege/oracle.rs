use std::collections::BTreeSet;

use worth_store::physical_runtime::{
    PhysicalWorkCourtroomRunBinding, PhysicalWorkEvidenceDigest, PhysicalWorkExecutionContext,
    PhysicalWorkFreshReopenEvidence, PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence,
    PhysicalWorkPlatformEvidence, PhysicalWorkRerunEvidence, PhysicalWorkRunEnvironmentEvidence,
    PhysicalWorkScheduleSeed, PhysicalWorkSourceBinding, PhysicalWorkWorkloadSeed,
};

use super::{
    binary_binding::BuiltCourtroomExecutables,
    execution::{BoundedResidencyProducerObservation, BoundedResidencySiegeObservations},
    offline_protocol::OfflineObservation,
    protocol::BoundedResidencySiegeObservation,
    schedule::{RevisionScheduleSeeds, SchedulePerturbationPlan},
    world::BoundedResidencySiegeWorld,
};

#[path = "oracle/allocation.rs"]
mod allocation;
#[path = "oracle/artifact_policy.rs"]
mod artifact_policy;
#[path = "oracle/cancellation.rs"]
mod cancellation;
#[path = "oracle/digest.rs"]
mod digest;
#[path = "oracle/generation_fencing.rs"]
mod generation_fencing;
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

pub(super) struct BoundedResidencyCourtroomEvidence {
    run: PhysicalWorkCourtroomRunBinding,
    runner: PhysicalWorkSourceBinding,
    observer: PhysicalWorkSourceBinding,
    producer: BoundedResidencyProducerObservation,
    child: BoundedResidencySiegeObservation,
    offline: OfflineObservation,
    reopen: PhysicalWorkFreshReopenEvidence,
    oracle: PhysicalWorkOracleEvidence,
    mutants: Box<[PhysicalWorkMutantLocalization]>,
    workload_seed: u64,
    schedule: SchedulePerturbationPlan,
    revision_schedule: RevisionScheduleSeeds,
}

impl BoundedResidencyCourtroomEvidence {
    pub(super) const fn source(&self) -> &PhysicalWorkSourceBinding {
        self.run.source()
    }

    pub(super) const fn writer(&self) -> &PhysicalWorkSourceBinding {
        self.run.binary()
    }

    pub(super) const fn runner(&self) -> &PhysicalWorkSourceBinding {
        &self.runner
    }

    pub(super) const fn observer(&self) -> &PhysicalWorkSourceBinding {
        &self.observer
    }

    pub(super) const fn child(&self) -> &BoundedResidencySiegeObservation {
        &self.child
    }

    pub(super) const fn producer(&self) -> BoundedResidencyProducerObservation {
        self.producer
    }

    pub(super) const fn offline(&self) -> &OfflineObservation {
        &self.offline
    }

    pub(super) const fn reopen(&self) -> PhysicalWorkFreshReopenEvidence {
        self.reopen
    }

    pub(super) const fn oracle(&self) -> &PhysicalWorkOracleEvidence {
        &self.oracle
    }

    pub(super) const fn mutants(&self) -> &[PhysicalWorkMutantLocalization] {
        &self.mutants
    }

    pub(super) const fn run(&self) -> &PhysicalWorkCourtroomRunBinding {
        &self.run
    }

    pub(super) const fn workload_seed(&self) -> u64 {
        self.workload_seed
    }

    pub(super) const fn schedule(&self) -> &SchedulePerturbationPlan {
        &self.schedule
    }

    pub(super) const fn revision_schedule(&self) -> &RevisionScheduleSeeds {
        &self.revision_schedule
    }
}

pub(super) fn verify(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    observations: BoundedResidencySiegeObservations,
    mutants: Vec<PhysicalWorkMutantLocalization>,
    rerun: PhysicalWorkRerunEvidence,
    schedule: SchedulePerturbationPlan,
    revision_schedule: RevisionScheduleSeeds,
) -> Result<BoundedResidencyCourtroomEvidence, String> {
    if observations.child.schedule != *schedule.trace().decisions() {
        return Err("Courtroom C child executed a foreign schedule trace".into());
    }
    verify_processes(&observations)?;
    verify_current_truth(world, &observations)?;
    generation_fencing::verify(
        observations.child.generation_fencing,
        observations.child.generation(),
    )?;
    verify_process_allocation(&observations.child)?;
    verify_residency(world, &observations.child)?;
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
    let exclusive_operation_limit = allocation_operation_limit(&observations.child)?;
    verify_allocation(
        &observations.child.allocation,
        observations.child.store(),
        observations.child.process().get(),
        exclusive_operation_limit,
    )?;
    let artifact_bytes = verify_artifact_manifest(&observations.offline)?;
    verify_world_scale(world, &observations.child, artifact_bytes)?;
    verify_reopen(&observations)?;
    verify_mutants(&mutants)?;
    let oracle = PhysicalWorkOracleEvidence::new(
        "courtroom-c:bounded-residency-siege:independent-physical-truth",
        true,
        digest::build(world, &observations)?,
    )
    .map_err(|denial| format!("Courtroom C oracle binding denied: {denial:?}"))?;
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
        mutants: mutants.into_boxed_slice(),
        workload_seed: world.seed(),
        schedule,
        revision_schedule,
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
    let processes = observations
        .processes
        .iter()
        .map(|process| process.process())
        .collect::<Vec<_>>();
    if processes.iter().collect::<BTreeSet<_>>().len() != processes.len()
        || processes
            != [
                observations.producer.process,
                observations.child.process(),
                observations.offline.process(),
                observations.reopen.identity().process(),
            ]
    {
        return Err("Courtroom C did not use four distinct role processes".into());
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
mod tests {
    use super::verify_mutants;
    use worth_store::physical_runtime::{
        PhysicalWorkEvidenceDigest, PhysicalWorkMutantBinding, PhysicalWorkMutantExecutionContext,
        PhysicalWorkMutantLocalization, PhysicalWorkMutantOutcome, PhysicalWorkMutantSubject,
        PhysicalWorkSourceBinding,
    };

    #[test]
    fn courtroom_requires_each_bounded_residency_mutant() {
        let required = crate::mutation_campaign::bounded_residency_requirements();
        let complete = required
            .iter()
            .map(|requirement| killed_mutant(requirement.identity(), requirement.predicate()))
            .collect::<Vec<_>>();
        assert!(verify_mutants(&complete).is_ok());

        for missing in required {
            let incomplete = complete
                .iter()
                .filter(|mutant| mutant.identity() != missing.identity())
                .cloned()
                .collect::<Vec<_>>();
            let denial = match verify_mutants(&incomplete) {
                Ok(()) => panic!("MUTANT_PREDICATE:bounded-residency-corpus-truncated"),
                Err(denial) => denial,
            };
            assert!(
                denial.contains(missing.predicate()),
                "wrong omission denial: {denial}"
            );
        }
    }

    fn killed_mutant(identity: u16, predicate: &str) -> PhysicalWorkMutantLocalization {
        let source_digest =
            PhysicalWorkEvidenceDigest::new([identity as u8; 32]).expect("nonzero fixture digest");
        let mutant_digest = PhysicalWorkEvidenceDigest::new([(identity + 1) as u8; 32])
            .expect("nonzero fixture digest");
        let subject =
            PhysicalWorkMutantSubject::new(identity, predicate, "current-source.rs").unwrap();
        let execution = PhysicalWorkMutantExecutionContext::new("test", "causal-scenario").unwrap();
        let binary = PhysicalWorkSourceBinding::new("current-test.exe", source_digest).unwrap();
        let binding = PhysicalWorkMutantBinding::new(
            subject,
            source_digest,
            mutant_digest,
            binary,
            execution,
        );
        PhysicalWorkMutantLocalization::new(
            binding,
            PhysicalWorkMutantOutcome::new(true, "exact causal assertion"),
        )
        .unwrap()
    }
}
