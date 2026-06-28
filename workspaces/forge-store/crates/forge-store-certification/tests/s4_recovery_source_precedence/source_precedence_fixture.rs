#[path = "wal_only_tail_fixture.rs"]
mod wal_only_tail_fixture;

use forge_store_physical_backend::{
    BackendDurabilityBarrierAuthority, SimulatedStrictDurabilityAuthority,
    SimulatedStrictDurableProfile, WalDurabilityBarrier,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRootReference,
    PhysicalSegmentId,
};
use forge_store_recovery_physics::{
    AcknowledgmentPrecondition, AdmittedCompactionCutoverDurability,
    AdmittedCompactionCutoverRecord, CheckpointArtifactDurabilityCommitment,
    CheckpointBaseAdmission, CheckpointCandidate, CheckpointCandidateDiscoverySource,
    CheckpointCoveredLsnRange, CheckpointCutoverReceipt, CheckpointDurabilityEvidenceSet,
    CheckpointLocatorArtifactCommitment, CheckpointManifest, CheckpointPageLsnFrontier,
    CheckpointPublicationPlan, CheckpointRedoBoundary, CheckpointRootPosture,
    CheckpointSelectorEvidence, CheckpointValidation, CompactionGenerationIdentity,
    CompactionVisibleProductEvidence, CompactionVisibleProductEvidenceDenial,
    ContiguousWalTailProof, DurableAckReceipt, IntegrityDamageMap, LogSequenceNumber, PageLsn,
    RecoverableOldCompactionGeneration, RecoveryCandidateDiscoveryTrace,
    SharpCheckpointCertificationMode, StoreOwnedCheckpointLocator, WalAppendPlan,
    WalDurabilityObservationSequence, WalLsnRange, WalOnlyTailProofDenial, WalSegmentGeneration,
    WalSegmentId, WalTailRedoSource,
};

pub(crate) fn trace(label: &str, order: u64) -> RecoveryCandidateDiscoveryTrace {
    RecoveryCandidateDiscoveryTrace::new("strict-test-profile", label, order)
}

pub(crate) fn checkpoint_base(
    start: u64,
    end: u64,
    redo: u64,
    order: u64,
) -> (CheckpointBaseAdmission, CheckpointCutoverReceipt) {
    let validation = validate(manifest(start, end, redo));
    let durability = checkpoint_durability(&validation);
    let plan = CheckpointPublicationPlan::<SimulatedStrictDurableProfile>::plan_cutover(
        validation.clone(),
        durability,
    )
    .unwrap();
    let receipt = CheckpointCutoverReceipt::publish(plan);
    let base = CheckpointBaseAdmission::from_validated_checkpoint(
        &validation,
        &receipt,
        trace("checkpoint-base", order),
    )
    .unwrap();
    (base, receipt)
}

pub(crate) fn wal_tail_for_checkpoint(
    receipt: &CheckpointCutoverReceipt,
    end: u64,
    order: u64,
) -> WalTailRedoSource {
    let start = receipt.covered_lsn_range().range().end_exclusive().get();
    let tail = ContiguousWalTailProof::prove(receipt, wal_range(start, end)).unwrap();
    WalTailRedoSource::from_contiguous_tail(receipt, tail, trace("wal-tail", order)).unwrap()
}

pub(crate) fn wal_only_tail(start: u64, end: u64, order: u64) -> WalTailRedoSource {
    let range = wal_range(start, end);
    let proof = wal_only_tail_fixture::wal_only_tail_proof(range);
    WalTailRedoSource::wal_only(proof, trace("wal-only-tail", order))
}

pub(crate) fn wal_only_tail_denial_from_torn_frame(start: u64, end: u64) -> WalOnlyTailProofDenial {
    wal_only_tail_fixture::wal_only_tail_denial_from_torn_frame(wal_range(start, end))
}

pub(crate) fn compaction_visible_product_evidence(
    generation_value: u64,
) -> CompactionVisibleProductEvidence {
    let generation = CompactionGenerationIdentity::new(generation_value);
    let (old_generation_base, cutover_receipt) = checkpoint_base(10, 20, 19, 1);
    let cutover = AdmittedCompactionCutoverRecord::from_checkpoint_cutover_receipt(
        generation,
        &cutover_receipt,
    );
    let old_generation = RecoverableOldCompactionGeneration::from_checkpoint_base_admission(
        generation,
        &old_generation_base,
    );
    let durability = AdmittedCompactionCutoverDurability::from_durable_ack_receipt(
        generation,
        &cutover,
        &compaction_cutover_durability_ack(&cutover_receipt, &cutover),
    )
    .unwrap();
    CompactionVisibleProductEvidence::admit(generation, cutover, old_generation, durability)
        .unwrap()
}

pub(crate) fn admitted_compaction_cutover_for_generation(
    generation_value: u64,
) -> (
    CompactionGenerationIdentity,
    AdmittedCompactionCutoverRecord,
) {
    let generation = CompactionGenerationIdentity::new(generation_value);
    let (_, cutover_receipt) = checkpoint_base(10, 20, 19, 1);
    let cutover = AdmittedCompactionCutoverRecord::from_checkpoint_cutover_receipt(
        generation,
        &cutover_receipt,
    );
    (generation, cutover)
}

pub(crate) fn compaction_generation_mismatch_denial() -> CompactionVisibleProductEvidenceDenial {
    let expected = CompactionGenerationIdentity::new(7);
    let observed = CompactionGenerationIdentity::new(8);
    let (old_generation_base, cutover_receipt) = checkpoint_base(10, 20, 19, 1);
    let cutover = AdmittedCompactionCutoverRecord::from_checkpoint_cutover_receipt(
        observed,
        &cutover_receipt,
    );
    let old_generation = RecoverableOldCompactionGeneration::from_checkpoint_base_admission(
        expected,
        &old_generation_base,
    );
    let durability = AdmittedCompactionCutoverDurability::from_durable_ack_receipt(
        observed,
        &cutover,
        &compaction_cutover_durability_ack(&cutover_receipt, &cutover),
    )
    .unwrap();

    CompactionVisibleProductEvidence::admit(expected, cutover, old_generation, durability)
        .unwrap_err()
}

pub(crate) fn compaction_cutover_basis_mismatch_denial() -> CompactionVisibleProductEvidenceDenial {
    let generation = CompactionGenerationIdentity::new(7);
    let (_, cutover_receipt) = checkpoint_base(10, 20, 19, 1);
    let mismatched_old_generation_base = checkpoint_base(20, 30, 29, 2).0;
    let cutover = AdmittedCompactionCutoverRecord::from_checkpoint_cutover_receipt(
        generation,
        &cutover_receipt,
    );
    let old_generation = RecoverableOldCompactionGeneration::from_checkpoint_base_admission(
        generation,
        &mismatched_old_generation_base,
    );
    let durability = AdmittedCompactionCutoverDurability::from_durable_ack_receipt(
        generation,
        &cutover,
        &compaction_cutover_durability_ack(&cutover_receipt, &cutover),
    )
    .unwrap();

    CompactionVisibleProductEvidence::admit(generation, cutover, old_generation, durability)
        .unwrap_err()
}

pub(crate) fn compaction_durability_artifact_mismatch_denial(
) -> CompactionVisibleProductEvidenceDenial {
    let generation = CompactionGenerationIdentity::new(7);
    let (_, cutover_receipt) = checkpoint_base(10, 20, 19, 1);
    let cutover = AdmittedCompactionCutoverRecord::from_checkpoint_cutover_receipt(
        generation,
        &cutover_receipt,
    );
    let ack = durable_ack_for_digest(
        cutover_receipt.covered_lsn_range().range(),
        77,
        "s4-compaction-cutover:wrong-artifact",
    );
    AdmittedCompactionCutoverDurability::from_durable_ack_receipt(generation, &cutover, &ack)
        .unwrap_err()
}

pub(crate) fn compaction_durability_range_mismatch_denial() -> CompactionVisibleProductEvidenceDenial
{
    let generation = CompactionGenerationIdentity::new(7);
    let (_, cutover_receipt) = checkpoint_base(10, 20, 19, 1);
    let cutover = AdmittedCompactionCutoverRecord::from_checkpoint_cutover_receipt(
        generation,
        &cutover_receipt,
    );
    let ack = durable_ack_for_digest(wal_range(30, 40), 78, cutover.artifact_digest());
    AdmittedCompactionCutoverDurability::from_durable_ack_receipt(generation, &cutover, &ack)
        .unwrap_err()
}

pub(crate) fn manifest(start: u64, end: u64, redo: u64) -> CheckpointManifest {
    CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(root_reference()),
        frontier(redo),
        CheckpointCoveredLsnRange::new(lsn(start), lsn(end)).unwrap(),
        CheckpointRedoBoundary::from_page_lsn(PageLsn::from_lsn(lsn(redo))),
        SharpCheckpointCertificationMode::certified(),
    )
    .unwrap()
}

pub(crate) fn wal_range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(lsn(start), lsn(end)).unwrap()
}

pub(crate) fn page_lsn(value: u64) -> PageLsn {
    PageLsn::from_lsn(lsn(value))
}

fn validate(manifest: CheckpointManifest) -> CheckpointValidation {
    CheckpointValidation::validate_located_checkpoint(locate(manifest), &IntegrityDamageMap::new())
        .unwrap()
}

fn locate(
    manifest: CheckpointManifest,
) -> forge_store_recovery_physics::LocatedCheckpointCandidate {
    let locator = recovered_locator(manifest.clone());
    CheckpointSelectorEvidence::from_store_owned_locator(locator)
        .bind_candidate(CheckpointCandidate::from_manifest(
            manifest,
            CheckpointCandidateDiscoverySource::DirectoryListing,
        ))
        .unwrap()
}

fn checkpoint_durability(
    validation: &CheckpointValidation,
) -> CheckpointDurabilityEvidenceSet<SimulatedStrictDurableProfile> {
    let range = validation.manifest().covered_lsn_range().range();
    CheckpointDurabilityEvidenceSet::admit(
        validation,
        &durable_ack_for_digest(
            range,
            1,
            CheckpointArtifactDurabilityCommitment::manifest(validation).digest(),
        ),
        &durable_ack_for_digest(
            range,
            2,
            CheckpointArtifactDurabilityCommitment::root(validation).digest(),
        ),
        &durable_ack_for_digest(
            range,
            3,
            CheckpointArtifactDurabilityCommitment::page_lsn_frontier(validation).digest(),
        ),
        &durable_ack_for_digest(
            range,
            4,
            CheckpointArtifactDurabilityCommitment::locator(validation).digest(),
        ),
    )
    .unwrap()
}

fn recovered_locator(manifest: CheckpointManifest) -> StoreOwnedCheckpointLocator {
    let commitment = CheckpointLocatorArtifactCommitment::superblock_ring_pointer(&manifest, 1);
    let ack = durable_ack_for_digest(manifest.covered_lsn_range().range(), 9, commitment.digest());
    StoreOwnedCheckpointLocator::admit(commitment, &ack).unwrap()
}

fn compaction_cutover_durability_ack(
    receipt: &CheckpointCutoverReceipt,
    cutover: &AdmittedCompactionCutoverRecord,
) -> DurableAckReceipt<SimulatedStrictDurableProfile> {
    durable_ack_for_digest(
        receipt.covered_lsn_range().range(),
        77,
        cutover.artifact_digest(),
    )
}

fn durable_ack_for_digest(
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
        .observe(forge_store_recovery_physics::WalDurabilityObservation::Completed(barrier))
        .unwrap()
        .finish()
        .unwrap();
    DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(receipt).unwrap(),
    )
}

fn frontier(redo: u64) -> CheckpointPageLsnFrontier {
    CheckpointPageLsnFrontier::from_pages([(page_cell(), PageLsn::from_lsn(lsn(redo)))]).unwrap()
}

fn page_cell() -> forge_store_physical_format::PageGenerationCell {
    PhysicalGenerationAuthority::s1()
        .page_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .with_page_generation(PhysicalGeneration::from_raw(1).unwrap())
}

fn root_reference() -> PhysicalRootReference {
    PhysicalRootReference::from_raw(1).unwrap()
}

fn lsn(value: u64) -> LogSequenceNumber {
    LogSequenceNumber::new(value)
}
