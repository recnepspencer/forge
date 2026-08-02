use worth_store_physical_backend::{BackendDurabilityProfile, SimulatedStrictDurableProfile};
use worth_store_recovery_physics::{
    CheckpointArtifactDurabilityCommitment, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointDurabilityEvidenceSet,
    CheckpointLocatorArtifactCommitment, CheckpointManifest, CheckpointSelectorEvidence,
    CheckpointValidation, LocatedCheckpointCandidate, StoreOwnedCheckpointLocator,
    WalAppendObservationScope, WalAppendReceipt, WalDurabilityObservation, WalLsnRange,
    WalSegmentGeneration, WalSegmentId,
};

pub fn validate(manifest: CheckpointManifest) -> CheckpointValidation {
    CheckpointValidation::validate_located_checkpoint(
        locate(manifest),
        &worth_store_recovery_physics::IntegrityDamageMap::new(),
    )
    .unwrap()
}

pub fn locate(manifest: CheckpointManifest) -> LocatedCheckpointCandidate {
    let locator = recovered_locator(manifest.clone());
    CheckpointSelectorEvidence::from_store_owned_locator(locator)
        .bind_candidate(CheckpointCandidate::from_manifest(
            manifest,
            CheckpointCandidateDiscoverySource::DirectoryListing,
        ))
        .unwrap()
}

pub fn checkpoint_durability(
    validation: &CheckpointValidation,
) -> CheckpointDurabilityEvidenceSet<SimulatedStrictDurableProfile> {
    checkpoint_durability_for_profile::<SimulatedStrictDurableProfile>(validation)
}

pub fn checkpoint_durability_for_profile<P: BackendDurabilityProfile>(
    validation: &CheckpointValidation,
) -> CheckpointDurabilityEvidenceSet<P> {
    let range = validation.manifest().covered_lsn_range().range();
    let manifest_ack = durable_ack_for_digest::<P>(
        range,
        1,
        CheckpointArtifactDurabilityCommitment::manifest(validation).digest(),
    );
    let root_ack = durable_ack_for_digest::<P>(
        range,
        2,
        CheckpointArtifactDurabilityCommitment::root(validation).digest(),
    );
    let frontier_ack = durable_ack_for_digest::<P>(
        range,
        3,
        CheckpointArtifactDurabilityCommitment::page_lsn_frontier(validation).digest(),
    );
    let locator_ack = durable_ack_for_digest::<P>(
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

pub fn recovered_locator(manifest: CheckpointManifest) -> StoreOwnedCheckpointLocator {
    let commitment = CheckpointLocatorArtifactCommitment::superblock_ring_pointer(&manifest, 1);
    let ack = durable_ack_for_digest::<SimulatedStrictDurableProfile>(
        manifest.covered_lsn_range().range(),
        9,
        commitment.digest(),
    );
    StoreOwnedCheckpointLocator::admit(commitment, &ack).unwrap()
}

fn durable_ack_for_digest<P: BackendDurabilityProfile>(
    range: WalLsnRange,
    segment_id: u64,
    digest: &str,
) -> WalDurabilityObservation<P> {
    let scope = WalAppendObservationScope::new(
        WalSegmentId::new(segment_id).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        range,
        digest,
        4096,
    )
    .unwrap();
    let receipt =
        WalAppendReceipt::from_certification_observation(scope, 4096, P::REQUIRED_BARRIERS, None);
    WalDurabilityObservation::from_append_receipt(receipt).unwrap()
}
