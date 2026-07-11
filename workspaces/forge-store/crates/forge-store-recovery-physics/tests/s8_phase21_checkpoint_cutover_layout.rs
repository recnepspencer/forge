use forge_store_physical_backend::{
    BackendDurabilityBarrierAuthority, SimulatedStrictDurabilityAuthority,
    SimulatedStrictDurableProfile, WalDurabilityBarrier,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRootReference,
    PhysicalSegmentId,
};
use forge_store_recovery_physics::{
    ensure_recovery_entry_allowed, reject_locator_projection, AcknowledgmentPrecondition,
    CheckpointArtifactDurabilityCommitment, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointCoveredLsnRange, CheckpointCutoverLayoutReport,
    CheckpointCutoverReceipt, CheckpointDurabilityEvidenceSet, CheckpointLocatorArtifactCommitment,
    CheckpointManifest, CheckpointPageLsnFrontier, CheckpointPublicationPlan,
    CheckpointRecoveryManifestLayoutReport, CheckpointRedoBoundary, CheckpointRootPosture,
    CheckpointSelectorEvidence, CheckpointValidation, DurableAckReceipt, IntegrityDamageMap,
    LogSequenceNumber, PageLsn, RecoveryLayoutAccessDenialKind, SharpCheckpointCertificationMode,
    StoreOwnedCheckpointLocator, WalAppendPlan, WalDurabilityObservationSequence, WalLsnRange,
    WalSegmentGeneration, WalSegmentId,
};

#[test]
fn phase21_recovery_manifest_and_cutover_rules_consume_checkpoint_authority() {
    let manifest = checkpoint_manifest();
    let locator_commitment = CheckpointLocatorArtifactCommitment::manifest_pointer(&manifest);
    let locator = StoreOwnedCheckpointLocator::admit(
        locator_commitment.clone(),
        &durable_ack(1, 1, wal_range(10, 30), locator_commitment.digest(), 64),
    )
    .expect("locator admission");
    let located = CheckpointSelectorEvidence::from_store_owned_locator(locator)
        .bind_candidate(CheckpointCandidate::from_manifest(
            manifest.clone(),
            CheckpointCandidateDiscoverySource::DirectoryListing,
        ))
        .expect("located candidate");
    let validation =
        CheckpointValidation::validate_located_checkpoint(located, &IntegrityDamageMap::new())
            .expect("checkpoint validation");

    let manifest_report = CheckpointRecoveryManifestLayoutReport::from_validation(&validation);
    assert_eq!(manifest_report.checkpoint_id(), validation.checkpoint_id());
    assert_eq!(
        manifest_report.covered_lsn_range(),
        validation.manifest().covered_lsn_range()
    );
    assert_eq!(
        manifest_report.counters().manifest_validation_count(),
        validation.counters().manifest_validation_count()
    );

    let durability = CheckpointDurabilityEvidenceSet::<SimulatedStrictDurableProfile>::admit(
        &validation,
        &artifact_ack(
            2,
            CheckpointArtifactDurabilityCommitment::manifest(&validation),
        ),
        &artifact_ack(3, CheckpointArtifactDurabilityCommitment::root(&validation)),
        &artifact_ack(
            4,
            CheckpointArtifactDurabilityCommitment::page_lsn_frontier(&validation),
        ),
        &artifact_ack(
            5,
            CheckpointArtifactDurabilityCommitment::locator(&validation),
        ),
    )
    .expect("checkpoint durability evidence");
    let plan = CheckpointPublicationPlan::plan_cutover(validation.clone(), durability)
        .expect("checkpoint publication plan");
    let cutover_report =
        CheckpointCutoverLayoutReport::from_receipt(&CheckpointCutoverReceipt::publish(plan));
    assert_eq!(cutover_report.checkpoint_id(), validation.checkpoint_id());
    assert_eq!(
        cutover_report.covered_lsn_range(),
        validation.manifest().covered_lsn_range()
    );
    assert_eq!(
        cutover_report.counters().manifest_validation_count(),
        validation.counters().manifest_validation_count()
    );

    ensure_recovery_entry_allowed(&IntegrityDamageMap::new())
        .expect("empty damage map must allow recovery entry");
    let denial = reject_locator_projection(validation.locator())
        .expect_err("locator projection must not stand in for checkpoint authority");
    assert_eq!(
        denial.kind(),
        RecoveryLayoutAccessDenialKind::LocatorProjectionCannotStandInForCheckpointAuthority
    );
}

fn checkpoint_manifest() -> CheckpointManifest {
    let redo_lsn = PageLsn::from_lsn(LogSequenceNumber::new(20));
    CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(PhysicalRootReference::from_raw(7).unwrap()),
        CheckpointPageLsnFrontier::from_pages([(
            PhysicalGenerationAuthority::s1()
                .page_cell(segment(1), page(1))
                .with_page_generation(generation(3)),
            redo_lsn,
        )])
        .unwrap(),
        CheckpointCoveredLsnRange::new(LogSequenceNumber::new(10), LogSequenceNumber::new(30))
            .unwrap(),
        CheckpointRedoBoundary::from_page_lsn(redo_lsn),
        SharpCheckpointCertificationMode::certified(),
    )
    .unwrap()
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}

fn wal_range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(LogSequenceNumber::new(start), LogSequenceNumber::new(end)).unwrap()
}

fn durable_ack(
    segment_id: u64,
    generation: u64,
    lsn_range: WalLsnRange,
    frame_digest: impl Into<String>,
    expected_bytes: u64,
) -> DurableAckReceipt<SimulatedStrictDurableProfile> {
    let plan = WalAppendPlan::<SimulatedStrictDurableProfile>::new(
        WalSegmentId::new(segment_id).unwrap(),
        WalSegmentGeneration::new(generation).unwrap(),
        lsn_range,
        frame_digest,
        expected_bytes,
    )
    .unwrap();
    let progress = plan.record_written_bytes(expected_bytes);
    let scope = progress.durability_scope();
    let barrier = SimulatedStrictDurabilityAuthority::new()
        .certify_completed_barrier(scope, WalDurabilityBarrier::SimulatedDurableCommit)
        .unwrap();
    let receipt = WalDurabilityObservationSequence::new(progress)
        .completed(barrier)
        .unwrap()
        .finish()
        .unwrap();
    DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(receipt).unwrap(),
    )
}

fn artifact_ack(
    segment_id: u64,
    commitment: CheckpointArtifactDurabilityCommitment,
) -> DurableAckReceipt<SimulatedStrictDurableProfile> {
    durable_ack(segment_id, 1, wal_range(10, 30), commitment.digest(), 64)
}
