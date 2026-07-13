use forge_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration,
    BackgroundEnvelopeAdmission, BackgroundEnvelopeRequest, BackgroundWorkBudgetSnapshot,
    FixedMetadataReservation,
};
use forge_store_contracts::{
    DurableArtifactFamilyId, StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
};
use forge_store_physical_backend::{
    BackendDurabilityBarrierAuthority, SimulatedStrictDurabilityAuthority,
    SimulatedStrictDurableProfile, WalDurabilityBarrier,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReferenceAuthority,
    PhysicalReferenceScope, PhysicalRootReference, PhysicalSegmentId,
};
use forge_store_physical_integrity::{
    ExecutedQuarantineFinding, PhysicalQuarantineAuthority, QuarantineRecord, QuarantineSealRequest,
};
use forge_store_recovery_physics::{
    AcknowledgmentPrecondition, AdmittedRecoverySource, BackendResidueKind,
    BackendResidueRejection, CheckpointArtifactDurabilityCommitment, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointCoveredLsnRange, CheckpointCutoverLayoutReport,
    CheckpointCutoverReceipt, CheckpointDurabilityEvidenceSet, CheckpointIntervalContract,
    CheckpointManifest, CheckpointPageLsnFrontier, CheckpointPublicationPlan,
    CheckpointRedoBoundary, CheckpointRootPosture, CheckpointSelectorEvidence,
    CheckpointValidation, ContiguousWalTailProof, DurableAckReceipt, IntegrityDamageMap,
    LogSequenceNumber, PageLsn, RecoveryCandidateDiscoveryTrace, RecoveryMemoryEnvelope,
    RecoverySourceCandidate, RecoveryStoreFootprint, SharpCheckpointCertificationMode,
    StoreOwnedCheckpointLocator, WalAppendPlan, WalDurabilityObservationSequence, WalLsnRange,
    WalSegmentGeneration, WalSegmentId, WalTailRedoSource, WalTailReplayBudget,
};
use forge_store_wal::{
    admit_replay_cursor, AdmittedReplayTailCursor, WalSegmentScanRecord, WalTopologyScan,
};

pub struct Phase22Fixture {
    pub checkpoint_receipt: CheckpointCutoverReceipt,
    pub checkpoint_report: forge_store_recovery_physics::CheckpointCutoverLayoutReport,
    pub replay_cursor: AdmittedReplayTailCursor,
}

pub fn fixture() -> Phase22Fixture {
    let manifest = checkpoint_manifest();
    let locator_commitment =
        forge_store_recovery_physics::CheckpointLocatorArtifactCommitment::manifest_pointer(
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

    Phase22Fixture {
        checkpoint_receipt,
        checkpoint_report,
        replay_cursor,
    }
}

pub fn admitted_source_with_residue() -> AdmittedRecoverySource {
    bounded_source_admission().source().clone()
}

pub fn bounded_source_admission() -> forge_store_recovery_physics::BoundedRecoverySourceAdmission {
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
            forge_store_recovery_physics::CheckpointBaseAdmission::from_validated_checkpoint(
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

pub const fn recovery_family_id() -> DurableArtifactFamilyId {
    DurableArtifactFamilyId::WalRecoveryDecision
}

fn recovery_budget() -> forge_store_recovery_physics::RecoveryBudget {
    forge_store_recovery_physics::RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(32),
        WalTailReplayBudget::max_frames(32)
            .with_max_scanned_segments(4)
            .with_max_page_redos(32),
        recovery_memory_envelope(),
    )
    .with_store_footprint(RecoveryStoreFootprint::admitted_persisted_pages(64))
    .with_checkpoint_discovery_candidates(4)
}

fn validated_checkpoint() -> CheckpointValidation {
    let manifest = checkpoint_manifest();
    let locator_commitment =
        forge_store_recovery_physics::CheckpointLocatorArtifactCommitment::manifest_pointer(
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

pub fn wal_range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(LogSequenceNumber::new(start), LogSequenceNumber::new(end)).unwrap()
}

fn trace(label: &str, order: u64) -> RecoveryCandidateDiscoveryTrace {
    RecoveryCandidateDiscoveryTrace::new("phase22-profile", label, order)
}

fn test_scope(seed: &str) -> PhysicalReferenceScope {
    PhysicalReferenceScope::derived_index(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .page_cell(segment(seed_basis(seed) + 1), page(seed_basis(seed) + 11))
            .with_page_generation(generation(seed_basis(seed) + 5)),
    )
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .unwrap()
}

fn validated_scalar_value(
    contract: &AspectContract,
    raw_value: &str,
) -> forge_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
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

fn recovery_memory_envelope() -> RecoveryMemoryEnvelope {
    RecoveryMemoryEnvelope::from_admitted(admit_background()).unwrap()
}

fn admit_background() -> forge_store_buffer_pool::AdmittedBackgroundEnvelope {
    let mut admission = BackgroundEnvelopeAdmission::new();
    let mut allocation = allocation_admission();
    admission
        .admit(
            BackgroundEnvelopeRequest::recovery_planning()
                .resident_frames(1)
                .resident_bytes(128)
                .pin_pages_for_bounded_step(1)
                .allocation_bytes(128)
                .finish(),
            BackgroundWorkBudgetSnapshot::foreground_reserved(16, 4, 0, 16),
            &mut allocation,
        )
        .unwrap()
}

fn allocation_admission() -> AllocationAdmission {
    AllocationAdmission::from_declaration(
        AllocationEnvelopeDeclaration::declare()
            .foreground(bytes(512))
            .maintenance(bytes(512))
            .recovery(bytes(512))
            .scrub(bytes(512))
            .import_export(bytes(512))
            .streaming(bytes(512))
            .fixed_metadata(FixedMetadataReservation::constant_bytes(64).unwrap())
            .seal()
            .unwrap(),
    )
}

fn bytes(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}

fn seed_basis(seed: &str) -> u64 {
    seed.bytes().enumerate().fold(17_u64, |acc, (index, byte)| {
        acc + ((index as u64 + 1) * byte as u64)
    })
}
