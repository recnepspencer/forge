use worth_proof::TransitionOutcome;
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_physical_backend::{
    BackendDurabilityBarrierAuthority, SimulatedStrictDurabilityAuthority,
    SimulatedStrictDurableProfile, WalDurabilityBarrier,
};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference,
};
use worth_store_physical_integrity::{
    ExecutedQuarantineFinding, PhysicalQuarantineAuthority, QuarantineRecord, QuarantineSealRequest,
};
use worth_store_recovery_physics::{
    AcknowledgmentPrecondition, AdmittedRecoverySource, BackendResidueKind,
    BackendResidueRejection, CheckpointArtifactDurabilityCommitment, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointCoveredLsnRange, CheckpointCutoverLayoutReport,
    CheckpointCutoverReceipt, CheckpointDurabilityEvidenceSet, CheckpointIntervalContract,
    CheckpointManifest, CheckpointPageLsnFrontier, CheckpointPublicationPlan,
    CheckpointRedoBoundary, CheckpointRootPosture, CheckpointSelectorEvidence,
    CheckpointValidation, ContiguousWalTailProof, DurableAckReceipt, IntegrityDamageMap,
    LogSequenceNumber, PageLsn, RecoverySourceCandidate, RecoveryStoreFootprint,
    SharpCheckpointCertificationMode, StoreOwnedCheckpointLocator, WalAppendPlan,
    WalDurabilityObservationSequence, WalLsnRange, WalSegmentGeneration, WalSegmentId,
    WalTailRedoSource, WalTailReplayBudget,
};
use worth_store_wal::{
    admit_replay_cursor, AdmittedReplayTailCursor, WalSegmentScanRecord, WalTopologyScan,
};

mod fixture_primitives;

pub use fixture_primitives::wal_range;
use fixture_primitives::*;

pub struct BTreeRecoveryFixture {
    pub checkpoint_receipt: CheckpointCutoverReceipt,
    pub checkpoint_report: worth_store_recovery_physics::CheckpointCutoverLayoutReport,
    pub replay_cursor: AdmittedReplayTailCursor,
}

pub fn fixture() -> BTreeRecoveryFixture {
    let manifest = checkpoint_manifest();
    let locator_commitment =
        worth_store_recovery_physics::CheckpointLocatorArtifactCommitment::manifest_pointer(
            &manifest,
        );
    let locator = StoreOwnedCheckpointLocator::admit(
        locator_commitment.clone(),
        &durable_ack(1, 1, wal_range(10, 30), locator_commitment.digest(), 64),
    )
    .expect("locator");
    let located = CheckpointSelectorEvidence::from_store_owned_locator(locator)
        .bind_candidate(CheckpointCandidate::from_manifest(
            manifest.clone(),
            CheckpointCandidateDiscoverySource::DirectoryListing,
        ))
        .expect("candidate");
    let validation =
        CheckpointValidation::validate_located_checkpoint(located, &IntegrityDamageMap::new())
            .expect("validation");
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
    .expect("durability");
    let plan = CheckpointPublicationPlan::plan_cutover(validation.clone(), durability).unwrap();
    let checkpoint_receipt = CheckpointCutoverReceipt::publish(plan);
    let checkpoint_report = CheckpointCutoverLayoutReport::from_receipt(&checkpoint_receipt);

    let replay_cursor = admit_replay_cursor(
        WalTopologyScan::from_segment_scan([WalSegmentScanRecord::current(
            WalSegmentId::new(1).unwrap(),
            WalSegmentGeneration::new(1).unwrap(),
            wal_range(30, 45),
        )]),
        WalSegmentGeneration::new(1).unwrap(),
    )
    .expect("replay cursor");

    BTreeRecoveryFixture {
        checkpoint_receipt,
        checkpoint_report,
        replay_cursor,
    }
}

pub fn admitted_source_with_residue() -> AdmittedRecoverySource {
    bounded_source_admission().source().clone()
}

pub fn bounded_source_admission() -> worth_store_recovery_physics::BoundedRecoverySourceAdmission {
    recovery_budget()
        .source_precedence_graph("phase22-profile")
        .discover(RecoverySourceCandidate::backend_residue(
            BackendResidueRejection::new(
                BackendResidueKind::BackendDirectoryResidue,
                trace("residue", 2),
            ),
        ))
        .expect("residue candidate")
        .discover(RecoverySourceCandidate::checkpoint_base(
            worth_store_recovery_physics::CheckpointBaseAdmission::from_validated_checkpoint(
                &validated_checkpoint(),
                &fixture().checkpoint_receipt,
                trace("checkpoint", 1),
            )
            .expect("checkpoint base"),
        ))
        .expect("checkpoint candidate")
        .discover(RecoverySourceCandidate::wal_tail(
            WalTailRedoSource::from_contiguous_tail(
                &fixture().checkpoint_receipt,
                ContiguousWalTailProof::prove(&fixture().checkpoint_receipt, wal_range(30, 45))
                    .unwrap(),
                trace("wal-tail", 3),
            )
            .expect("tail"),
        ))
        .expect("wal tail candidate")
        .admit_sources()
}

pub fn authoritative_quarantine_record(seed: &str) -> QuarantineRecord {
    let finding = ExecutedQuarantineFinding::authoritative_quarantine(test_scope(seed));
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .expect("authoritative quarantine")
}

pub fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

pub fn current_security_scope(
    identity_key: &str,
    value: &str,
) -> worth_store_security::StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, value);
    let key_scope = worth_store_security::StoreKeyScope::WalCheckpointEnvelope;
    let tenant_scope = worth_store_security::StoreTenantScope::StoreInternal;
    let authenticity = worth_store_security::StoreAuthenticityRequirement::not_required();
    let custody = worth_store_security::StoreCustodyPosture::InternalStoreCustody;
    let expectation = worth_store_security::StoreSecurityScopeAdmissionExpectation::new(
        key_scope,
        tenant_scope,
        authenticity,
        custody,
    );
    match worth_store_security::admit_store_security_scope(
        worth_store_security::StoreSecurityScopeAdmissionRequest::new(
            &authority,
            key_scope,
            worth_store_security::StoreKeyVersionPosture::Current,
            tenant_scope,
            authenticity,
            custody,
            expectation,
        ),
    ) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("recovery test security scope should admit: {outcome:?}"),
    }
}

pub const fn recovery_family_id() -> DurableArtifactFamilyId {
    DurableArtifactFamilyId::WalRecoveryDecision
}

fn recovery_budget() -> worth_store_recovery_physics::RecoveryBudget {
    worth_store_recovery_physics::RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(32),
        WalTailReplayBudget::max_frames(32)
            .with_max_scanned_segments(4)
            .with_max_page_redos(32),
        recovery_memory_allocation(),
    )
    .with_store_footprint(RecoveryStoreFootprint::admitted_persisted_pages(64))
    .with_checkpoint_discovery_candidates(4)
}

fn validated_checkpoint() -> CheckpointValidation {
    let manifest = checkpoint_manifest();
    let locator_commitment =
        worth_store_recovery_physics::CheckpointLocatorArtifactCommitment::manifest_pointer(
            &manifest,
        );
    let locator = StoreOwnedCheckpointLocator::admit(
        locator_commitment.clone(),
        &durable_ack(1, 1, wal_range(10, 30), locator_commitment.digest(), 64),
    )
    .unwrap();
    let located = CheckpointSelectorEvidence::from_store_owned_locator(locator)
        .bind_candidate(CheckpointCandidate::from_manifest(
            manifest,
            CheckpointCandidateDiscoverySource::DirectoryListing,
        ))
        .unwrap();
    CheckpointValidation::validate_located_checkpoint(located, &IntegrityDamageMap::new()).unwrap()
}

fn checkpoint_manifest() -> CheckpointManifest {
    let redo_lsn = PageLsn::from_lsn(LogSequenceNumber::new(20));
    CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(
            PhysicalReferenceAuthority::for_canonical_physical_format()
                .admit_root_publication(
                    PhysicalGenerationAuthority::for_canonical_physical_format()
                        .root_publication_cell(PhysicalRootReference::from_raw(7).unwrap())
                        .with_root_publication_generation(PhysicalGeneration::from_raw(1).unwrap()),
                )
                .reference(),
        ),
        CheckpointPageLsnFrontier::from_pages([(
            PhysicalGenerationAuthority::for_canonical_physical_format()
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
    let barrier = SimulatedStrictDurabilityAuthority::new()
        .certify_completed_barrier(
            progress.durability_scope(),
            WalDurabilityBarrier::SimulatedDurableCommit,
        )
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
