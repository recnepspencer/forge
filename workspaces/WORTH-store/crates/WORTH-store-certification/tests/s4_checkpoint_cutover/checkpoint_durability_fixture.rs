use worth_store_physical_backend::{
    BackendDurabilityBarrierAuthority, SimulatedStrictDurabilityAuthority,
    SimulatedStrictDurableProfile, WalDurabilityBarrier,
};
use worth_store_recovery_physics::{
    AcknowledgmentPrecondition, CheckpointArtifactDurabilityCommitment, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointDurabilityEvidenceSet,
    CheckpointLocatorArtifactCommitment, CheckpointManifest, CheckpointSelectorEvidence,
    CheckpointValidation, DurableAckReceipt, LocatedCheckpointCandidate,
    StoreOwnedCheckpointLocator, WalAppendPlan, WalDurabilityObservationSequence, WalLsnRange,
    WalSegmentGeneration, WalSegmentId,
};

pub(crate) fn validate(manifest: CheckpointManifest) -> CheckpointValidation {
    CheckpointValidation::validate_located_checkpoint(
        locate(manifest),
        &worth_store_recovery_physics::IntegrityDamageMap::new(),
    )
    .unwrap()
}

pub(crate) fn locate(manifest: CheckpointManifest) -> LocatedCheckpointCandidate {
    let locator = recovered_locator(manifest.clone());
    CheckpointSelectorEvidence::from_store_owned_locator(locator)
        .bind_candidate(CheckpointCandidate::from_manifest(
            manifest,
            CheckpointCandidateDiscoverySource::DirectoryListing,
        ))
        .unwrap()
}

pub(crate) fn checkpoint_durability(
    validation: &CheckpointValidation,
) -> CheckpointDurabilityEvidenceSet<SimulatedStrictDurableProfile> {
    let range = validation.manifest().covered_lsn_range().range();
    let manifest_ack = durable_ack_for_digest(
        range,
        1,
        CheckpointArtifactDurabilityCommitment::manifest(validation).digest(),
    );
    let root_ack = durable_ack_for_digest(
        range,
        2,
        CheckpointArtifactDurabilityCommitment::root(validation).digest(),
    );
    let frontier_ack = durable_ack_for_digest(
        range,
        3,
        CheckpointArtifactDurabilityCommitment::page_lsn_frontier(validation).digest(),
    );
    let locator_ack = durable_ack_for_digest(
        range,
        4,
        CheckpointArtifactDurabilityCommitment::locator(validation).digest(),
    );
    CheckpointDurabilityEvidenceSet::admit(
        validation,
        &manifest_ack,
        &root_ack,
        &frontier_ack,
        &locator_ack,
    )
    .unwrap()
}

pub(crate) fn recovered_locator(manifest: CheckpointManifest) -> StoreOwnedCheckpointLocator {
    let commitment = CheckpointLocatorArtifactCommitment::superblock_ring_pointer(&manifest, 1);
    let ack = durable_ack_for_digest(manifest.covered_lsn_range().range(), 9, commitment.digest());
    StoreOwnedCheckpointLocator::admit(commitment, &ack).unwrap()
}

pub(crate) fn durable_ack_for_digest(
    range: WalLsnRange,
    segment_id: u64,
    digest: &str,
) -> DurableAckReceipt<SimulatedStrictDurableProfile> {
    let plan = WalAppendPlan::<SimulatedStrictDurableProfile>::new(
        WalSegmentId::new(segment_id).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        range,
        digest,
        4096,
    )
    .unwrap();
    let progress = plan.record_written_bytes(4096);
    let barrier = SimulatedStrictDurabilityAuthority::new()
        .certify_completed_barrier(
            progress.durability_scope(),
            WalDurabilityBarrier::SimulatedDurableCommit,
        )
        .unwrap();
    let receipt = WalDurabilityObservationSequence::new(progress)
        .observe(worth_store_recovery_physics::WalDurabilityObservation::Completed(barrier))
        .unwrap()
        .finish()
        .unwrap();
    DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(receipt).unwrap(),
    )
}
