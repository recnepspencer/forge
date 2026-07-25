use std::collections::BTreeSet;

use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalWorkCourtroomRunBinding, PhysicalWorkEvidenceDigest, PhysicalWorkExecutionContext,
    PhysicalWorkFreshReopenEvidence, PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence,
    PhysicalWorkPlatformEvidence, PhysicalWorkRerunEvidence, PhysicalWorkRunEnvironmentEvidence,
    PhysicalWorkSourceBinding,
};

use super::{
    binary_binding::BuiltCourtroomExecutables,
    execution::C6SiegeObservations,
    offline_protocol::OfflineObservation,
    protocol::C6SiegeObservation,
    world::{oracle_bytes, C6SiegeWorld, RECORD_BYTES},
};

#[path = "oracle/pressure.rs"]
mod pressure;
use pressure::{verify_artifacts, verify_residency};

pub(super) struct C6PhysicalWorkCourtroomEvidence {
    run: PhysicalWorkCourtroomRunBinding,
    runner: PhysicalWorkSourceBinding,
    observer: PhysicalWorkSourceBinding,
    child: C6SiegeObservation,
    offline: OfflineObservation,
    reopen: PhysicalWorkFreshReopenEvidence,
    oracle: PhysicalWorkOracleEvidence,
    mutants: Box<[PhysicalWorkMutantLocalization]>,
}

impl C6PhysicalWorkCourtroomEvidence {
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

    pub(super) const fn child(&self) -> C6SiegeObservation {
        self.child
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
}

pub(super) fn verify(
    world: &C6SiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    observations: C6SiegeObservations,
    mutants: Vec<PhysicalWorkMutantLocalization>,
    rerun: PhysicalWorkRerunEvidence,
) -> Result<C6PhysicalWorkCourtroomEvidence, String> {
    verify_processes(&observations)?;
    verify_current_truth(world, &observations)?;
    verify_residency(world, observations.child)?;
    verify_artifacts(world, observations.child, &observations.offline)?;
    verify_reopen(&observations)?;
    verify_mutants(&mutants)?;
    let oracle = PhysicalWorkOracleEvidence::new(
        "courtroom-c:c6-inheritance-siege:independent-physical-truth",
        true,
        oracle_digest(world, &observations)?,
    )
    .map_err(|denial| format!("Courtroom C oracle binding denied: {denial:?}"))?;
    let execution = PhysicalWorkExecutionContext::new(
        0xc651_c006,
        "oversized-world,hot-cold-read,pin-denial,cancellation,dirty-writeback,eviction-refault,close,offline,reopen",
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
    Ok(C6PhysicalWorkCourtroomEvidence {
        run,
        runner: binaries.runner().binding().clone(),
        observer: binaries.observer().binding().clone(),
        child: observations.child,
        offline: observations.offline,
        reopen: observations.reopen,
        oracle,
        mutants: mutants.into_boxed_slice(),
    })
}

fn verify_processes(observations: &C6SiegeObservations) -> Result<(), String> {
    let processes = observations
        .processes
        .iter()
        .map(|process| process.process())
        .collect::<Vec<_>>();
    if processes.iter().collect::<BTreeSet<_>>().len() != processes.len()
        || processes
            != [
                observations.child.process(),
                observations.offline.process(),
                observations.reopen.identity().process(),
            ]
    {
        return Err("Courtroom C did not use three distinct processes".into());
    }
    Ok(())
}

fn verify_current_truth(
    world: &C6SiegeWorld,
    observations: &C6SiegeObservations,
) -> Result<(), String> {
    let child = observations.child;
    let current = observations.offline.current();
    let expected_digest = expected_payload_digest()?;
    if child.store() != current.store()
        || child.records() != world.expected_records()
        || current.records() != world.expected_records()
        || child.payload_bytes() != world.expected_payload_bytes()
        || current.payload_bytes() != world.expected_payload_bytes()
        || current.payload_digest() != expected_digest
    {
        return Err(format!(
            "Courtroom C current truth disagrees with its parent-owned oracle: \
             child={child:?}, offline={current:?}, digest={expected_digest:?}"
        ));
    }
    let encoded = std::fs::read(world.oracle())
        .map_err(|error| format!("cannot reread Courtroom C oracle: {error}"))?;
    if encoded != oracle_bytes() {
        return Err("Courtroom C oracle changed during the child process".into());
    }
    Ok(())
}

fn verify_reopen(observations: &C6SiegeObservations) -> Result<(), String> {
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
    for predicate in ["dirty-clean-without-exact-receipt", "c6-local-scheduler"] {
        if !mutants.iter().any(|mutant| mutant.predicate() == predicate) {
            return Err(format!(
                "Courtroom C mutation evidence omitted `{predicate}`"
            ));
        }
    }
    Ok(())
}

fn expected_payload_digest() -> Result<PhysicalWorkEvidenceDigest, String> {
    digest_payloads(oracle_bytes().chunks_exact(RECORD_BYTES))
}

fn digest_payloads<'payload>(
    payloads: impl IntoIterator<Item = &'payload [u8]>,
) -> Result<PhysicalWorkEvidenceDigest, String> {
    let mut digest = Sha256::new();
    for payload in payloads {
        digest.update((payload.len() as u64).to_le_bytes());
        digest.update(payload);
    }
    PhysicalWorkEvidenceDigest::new(digest.finalize().into())
        .ok_or_else(|| "Courtroom C payload oracle produced an all-zero digest".to_owned())
}

fn oracle_digest(
    world: &C6SiegeWorld,
    observations: &C6SiegeObservations,
) -> Result<PhysicalWorkEvidenceDigest, String> {
    let mut digest = Sha256::new();
    digest.update(b"courtroom-c-c6-inheritance-siege-v1");
    digest.update(oracle_bytes());
    digest.update(observations.child.store());
    digest.update(observations.child.runtime().to_le_bytes());
    digest.update(observations.child.generation().to_le_bytes());
    digest.update(world.expected_records().to_le_bytes());
    digest.update(world.expected_payload_bytes().to_le_bytes());
    digest.update(observations.child.dirty.work_operation.to_le_bytes());
    digest.update(
        observations
            .child
            .dirty
            .first_source_operation
            .to_le_bytes(),
    );
    digest.update(observations.child.dirty.backend_operation.to_le_bytes());
    for artifact in observations.offline.artifacts() {
        digest.update(artifact.path().as_bytes());
        digest.update(artifact.byte_length().to_le_bytes());
        digest.update(artifact.digest());
    }
    PhysicalWorkEvidenceDigest::new(digest.finalize().into())
        .ok_or_else(|| "Courtroom C oracle produced an all-zero digest".to_owned())
}

#[cfg(test)]
mod tests {
    use super::{digest_payloads, expected_payload_digest};
    use crate::courtroom_campaign::c6_inheritance_siege::world::{oracle_bytes, RECORD_BYTES};

    #[test]
    fn payload_oracle_rejects_a_foreign_extra_family() {
        let oracle = oracle_bytes();
        let batch = oracle.chunks_exact(RECORD_BYTES).collect::<Vec<_>>();
        let final_payload = &oracle[..RECORD_BYTES];
        let expected = expected_payload_digest().unwrap();

        let foreign =
            digest_payloads(batch.iter().copied().chain(std::iter::once(final_payload))).unwrap();
        assert_ne!(expected, foreign);
    }
}
