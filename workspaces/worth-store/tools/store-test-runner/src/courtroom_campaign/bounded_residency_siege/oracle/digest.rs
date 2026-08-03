use sha2::{Digest, Sha256};
use worth_store::physical_runtime::PhysicalWorkEvidenceDigest;

use super::{
    super::{
        c7_crash_campaign::C7CrashCampaignEvidence, execution::BoundedResidencySiegeObservations,
        offline_protocol::OfflineObservation, world::BoundedResidencySiegeWorld,
    },
    work_reconciliation,
};

pub(super) fn build(
    world: &BoundedResidencySiegeWorld,
    observations: &BoundedResidencySiegeObservations,
    c7_campaign: &C7CrashCampaignEvidence,
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
    add_performance(&mut digest, observations);
    for artifact in observations.offline.artifacts() {
        digest.update(artifact.path().as_bytes());
        digest.update(artifact.byte_length().to_le_bytes());
        digest.update(artifact.digest());
    }
    add_c7_campaign(&mut digest, c7_campaign);
    PhysicalWorkEvidenceDigest::new(digest.finalize().into())
        .ok_or_else(|| "Courtroom C oracle produced an all-zero digest".to_owned())
}

fn add_performance(digest: &mut Sha256, observations: &BoundedResidencySiegeObservations) {
    for receipt in &observations.child.performance {
        add_text(digest, receipt.claim().label());
        add_text(digest, receipt.profile().label());
        digest.update((receipt.counters().len() as u64).to_le_bytes());
        for counter in receipt.counters() {
            add_text(digest, counter.name());
            digest.update(counter.observed_count().to_le_bytes());
        }
    }
}

fn add_c7_campaign(digest: &mut Sha256, campaign: &C7CrashCampaignEvidence) {
    digest.update((campaign.cases().len() as u64).to_le_bytes());
    for case in campaign.cases() {
        add_text(digest, case.seam().label());
        add_text(digest, case.checkpoint_order().encoded());
        add_text(digest, case.checkpoint());
        add_observation(digest, case.baseline());
        add_observation(digest, case.observed());
        let reopen = case.reopen();
        digest.update(reopen.identity().store());
        digest.update(reopen.identity().generation().to_le_bytes());
        digest.update(reopen.identity().records().to_le_bytes());
        digest.update([
            u8::from(reopen.posture().inspection_required()),
            u8::from(reopen.posture().residue()),
            u8::from(reopen.posture().recovery_evidence_damaged()),
        ]);
        digest.update(reopen.posture().recovery_obligations().to_le_bytes());
        add_text(digest, case.rerun().program());
        digest.update((case.rerun().arguments().len() as u64).to_le_bytes());
        for argument in case.rerun().arguments() {
            add_text(digest, argument);
        }
    }
}

fn add_observation(digest: &mut Sha256, observation: &OfflineObservation) {
    digest.update(observation.process().get().to_le_bytes());
    let current = observation.current();
    digest.update(current.store());
    digest.update(current.generation().to_le_bytes());
    digest.update(current.records().to_le_bytes());
    digest.update(current.payload_bytes().to_le_bytes());
    digest.update(current.payload_digest().bytes());
    digest.update(observation.recovery_obligations().to_le_bytes());
    digest.update((observation.artifacts().len() as u64).to_le_bytes());
    for artifact in observation.artifacts() {
        add_text(digest, artifact.path());
        digest.update(artifact.byte_length().to_le_bytes());
        digest.update(artifact.digest());
        digest.update((artifact.prefix().len() as u64).to_le_bytes());
        digest.update(artifact.prefix());
        digest.update([u8::from(artifact.is_recovery_obligation())]);
    }
}

fn add_text(digest: &mut Sha256, value: &str) {
    digest.update((value.len() as u64).to_le_bytes());
    digest.update(value.as_bytes());
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
