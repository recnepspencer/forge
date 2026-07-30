use sha2::{Digest, Sha256};
use worth_store::physical_runtime::PhysicalWorkEvidenceDigest;

use super::{
    super::{execution::BoundedResidencySiegeObservations, world::BoundedResidencySiegeWorld},
    work_reconciliation,
};

pub(super) fn build(
    world: &BoundedResidencySiegeWorld,
    observations: &BoundedResidencySiegeObservations,
) -> Result<PhysicalWorkEvidenceDigest, String> {
    let mut digest = Sha256::new();
    digest.update(b"courtroom-c-bounded-residency-siege-v7");
    digest.update(world.expectation_digest());
    digest.update(observations.producer.process.get().to_le_bytes());
    digest.update(observations.producer.store);
    digest.update(observations.producer.runtime.to_le_bytes());
    digest.update(observations.producer.generation.to_le_bytes());
    digest.update(observations.child.store());
    digest.update(observations.child.runtime().to_le_bytes());
    digest.update(observations.child.generation().to_le_bytes());
    digest.update(world.expected_records().to_le_bytes());
    digest.update(world.expected_payload_bytes().to_le_bytes());
    digest.update(
        observations
            .child
            .process_allocation
            .largest_successful_request_bytes
            .to_le_bytes(),
    );
    add_pinned_eviction(&mut digest, observations);
    add_speculation(&mut digest, observations);
    add_generation_fencing(&mut digest, observations);
    digest.update(work_reconciliation::digest(
        &observations.child.work_reconciliation,
    ));
    add_dirty_writeback(&mut digest, observations);
    for artifact in observations.offline.artifacts() {
        digest.update(artifact.path().as_bytes());
        digest.update(artifact.byte_length().to_le_bytes());
        digest.update(artifact.digest());
    }
    PhysicalWorkEvidenceDigest::new(digest.finalize().into())
        .ok_or_else(|| "Courtroom C oracle produced an all-zero digest".to_owned())
}

fn add_generation_fencing(digest: &mut Sha256, observations: &BoundedResidencySiegeObservations) {
    for case in [
        observations.child.generation_fencing.read,
        observations.child.generation_fencing.dirty,
        observations.child.generation_fencing.writeback,
    ] {
        digest.update(case.current_generation.to_le_bytes());
        digest.update(case.stale_generation.to_le_bytes());
        for value in [
            case.effects.allocation_admissions,
            case.effects.allocation_releases,
            case.effects.allocation_other,
            case.effects.residency_hits,
            case.effects.residency_faults,
            case.effects.source_loads,
            case.effects.dirty_transitions,
            case.effects.writeback_attempts,
            case.effects.work_declarations,
            case.effects.signal_requests,
            case.effects.scheduler_admissions,
            case.effects.media_attempts,
            case.mutation_invocations,
        ] {
            digest.update(value.to_le_bytes());
        }
        digest.update([denial_code(case.denial), cleanup_code(case.cleanup)]);
    }
}

const fn denial_code(denial: super::super::protocol::BoundedResidencyGenerationDenial) -> u8 {
    match denial {
        super::super::protocol::BoundedResidencyGenerationDenial::StaleGeneration => 1,
        super::super::protocol::BoundedResidencyGenerationDenial::StaleOrForeignFrame => 2,
    }
}

const fn cleanup_code(cleanup: super::super::protocol::BoundedResidencyGenerationCleanup) -> u8 {
    match cleanup {
        super::super::protocol::BoundedResidencyGenerationCleanup::None => 0,
        super::super::protocol::BoundedResidencyGenerationCleanup::LeaseReleased => 1,
        super::super::protocol::BoundedResidencyGenerationCleanup::DirtyReturned => 2,
    }
}

fn add_pinned_eviction(digest: &mut Sha256, observations: &BoundedResidencySiegeObservations) {
    let evidence = observations.child.pinned_eviction;
    digest.update(evidence.forced_evictions.to_le_bytes());
    digest.update(evidence.pinned_frames_before.to_le_bytes());
    digest.update(evidence.pinned_frames_after.to_le_bytes());
    digest.update(evidence.pin_leases_before.to_le_bytes());
    digest.update(evidence.pin_leases_after.to_le_bytes());
    digest.update([u8::from(evidence.bases_preserved)]);
}

fn add_speculation(digest: &mut Sha256, observations: &BoundedResidencySiegeObservations) {
    for kind in [
        observations.child.speculation.prefetch,
        observations.child.speculation.read_ahead,
        observations.child.speculation.write_behind,
    ] {
        digest.update(kind.attempts.to_le_bytes());
        digest.update(kind.admissions.to_le_bytes());
        digest.update(kind.denials.to_le_bytes());
        digest.update(kind.completions.to_le_bytes());
        digest.update(kind.effectful_signal_requests.to_le_bytes());
        digest.update([
            u8::from(kind.signal_family_exact),
            u8::from(kind.foundational_basis_exact),
        ]);
    }
}

fn add_dirty_writeback(digest: &mut Sha256, observations: &BoundedResidencySiegeObservations) {
    let evidence = observations.child.dirty;
    digest.update(evidence.primary_publication.to_le_bytes());
    digest.update(evidence.retry_publication.to_le_bytes());
    digest.update(evidence.primary_candidate_writebacks.to_le_bytes());
    digest.update(evidence.retry_candidate_writebacks.to_le_bytes());
    digest.update(evidence.primary_last_candidate_operation.to_le_bytes());
    digest.update(evidence.retry_last_candidate_operation.to_le_bytes());
}
