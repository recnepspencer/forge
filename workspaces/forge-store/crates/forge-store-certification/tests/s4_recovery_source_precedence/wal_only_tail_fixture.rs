use forge_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, PinnedPageBudget, ResidentFrameLoadRequest,
    ResidentFrameTable, ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, ChecksumCoverageMap, ManifestMembershipProof,
    PhysicalBinaryEncodingWitness, PhysicalFormatDeclaration, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalManifestUniverseBuilder, PhysicalPageId, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceScope, PhysicalReferenceValidationWitness,
    PhysicalRootReference, PhysicalSegmentId, RootManifestIntegrityPosture, PHYSICAL_HEADER_LENGTH,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmClaim, ChecksumScopeDeclaration, DeclaredPhysicalChecksum,
    IntegrityEntryAdmission, IntegrityEntryRequest, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionRequest, PhysicalIntegrityEvidenceAuthority,
    PhysicalIntegrityEvidenceProfile, PhysicalQuarantineAuthority, PhysicalScopeAdmission,
    PhysicalScopeAdmissionRequest, ProtectedPhysicalByteView, QuarantineSealRequest,
    ScopedPhysicalValidatorInput, StoreExecutedIntegrityEvidence, WalFrameDamageDenial,
    WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest, WalFrameIntegrityReport,
};
use forge_store_readiness::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
    BufferPoolAuthorityRecap, IntegrityInspectionLifetimeLaw, PhysicalAuthorityRecap,
    ProtectedIntegrityViewCapability, S2BoundedCounterRecap, S2DenialBehaviorRecap,
    S2DeniedBoundaryKind, S2NoMaterializationWitness, S3PhysicalIntegrityReadiness,
    S3PhysicalIntegrityReadinessPayload, ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
use forge_store_recovery_physics::{
    IntegrityVettedWalFrame, RecoveryBlockedByIntegrityDamage, RecoveryIntegrityHandoffReceipt,
    WalLsnRange, WalOnlyTailProof, WalOnlyTailProofDenial, WalSegmentGeneration, WalSegmentId,
    WalSegmentScanRecord, WalTailIntegrityQuarantineHandoff, WalTopologyScan,
};

pub(crate) fn wal_only_tail_proof(range: WalLsnRange) -> WalOnlyTailProof {
    let cursor = WalTopologyScan::from_segment_scan([WalSegmentScanRecord::current(
        WalSegmentId::new(99).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        range,
    )])
    .admit_replay_cursor(WalSegmentGeneration::new(1).unwrap())
    .unwrap();
    WalOnlyTailProof::from_vetted_wal_frame(&vetted_wal_frame(range), &cursor).unwrap()
}

pub(crate) fn wal_only_tail_denial_from_torn_frame(range: WalLsnRange) -> WalOnlyTailProofDenial {
    let cursor = WalTopologyScan::from_segment_scan([WalSegmentScanRecord::current(
        WalSegmentId::new(100).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        range,
    )])
    .admit_replay_cursor(WalSegmentGeneration::new(1).unwrap())
    .unwrap();
    let handoff = quarantined_torn_wal_tail_handoff(range);
    WalOnlyTailProof::from_quarantined_wal_tail(&handoff, &cursor).unwrap_err()
}

fn vetted_wal_frame(range: WalLsnRange) -> IntegrityVettedWalFrame {
    let payload = intact_wal_payload(range);
    let mut table = resident_frame_table();
    let admission = admit_wal_payload_frame(&mut table, &payload);
    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    let protected = ProtectedPhysicalByteView::from_pinned_frame(&pinned.view().unwrap());
    let entry = IntegrityEntryAdmission::from_s3_readiness(s3_readiness()).unwrap();
    let inspection_lease = entry.admit(IntegrityEntryRequest::new(protected)).unwrap();
    let checksum_scope = checksum_scope();
    let integrity_admission = PhysicalIntegrityAdmission::from_entry(inspection_lease)
        .with_checksum_claim(
            ChecksumAlgorithmClaim::declared_text("crc32c"),
            checksum_scope.clone(),
        )
        .unwrap();
    let checked = integrity_admission
        .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
            wal_reference(),
            wal_header_witness(&frame_bytes(1, &payload)),
            PhysicalFrameKind::RecordFrame,
            DeclaredPhysicalChecksum::new(crc32c(&payload).into()),
        ))
        .unwrap();
    let scoped = PhysicalScopeAdmission::admit_frame(
        checked,
        PhysicalScopeAdmissionRequest::frame(
            PhysicalReferenceScope::wal_frame(wal_reference()),
            manifest_membership(),
            RootManifestIntegrityPosture::current_root_admitted(manifest_membership()),
            CheckpointAdjacencyPosture::NotCheckpointAdjacent,
            checksum_coverage_basis(),
        ),
    )
    .unwrap();
    let report = WalFrameIntegrityAuthority::s3()
        .inspect(
            WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(
                ScopedPhysicalValidatorInput::wal_frame(scoped).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_wal_frame(&report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    let receipt =
        forge_store_recovery_physics::RecoveryIntegrityHandoffReceipt::from_executed_evidence(
            &evidence,
        )
        .unwrap();
    IntegrityVettedWalFrame::from_integrity_report(&report, receipt).unwrap()
}

pub(crate) fn quarantined_torn_wal_tail_handoff(
    range: WalLsnRange,
) -> WalTailIntegrityQuarantineHandoff {
    let payload = torn_wal_payload(range);
    let denial = inspect_wal_payload(&payload).unwrap_err();
    let finding =
        forge_store_physical_integrity::ExecutedQuarantineFinding::from_wal_frame_denial(&denial)
            .unwrap();
    let record =
        PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
            .unwrap();
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(
            StoreExecutedIntegrityEvidence::receipt_evidence(&record),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    let receipt =
        RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence(&evidence).unwrap();
    WalTailIntegrityQuarantineHandoff::from_wal_tail_damage_quarantine(&denial, &record, receipt)
        .unwrap()
}

#[allow(dead_code)]
pub(crate) fn recovery_blocking_wal_frame_damage(
    range: WalLsnRange,
) -> RecoveryBlockedByIntegrityDamage {
    recovery_blocking_damage_from_payload(&wal_payload(range, 0, "checksum-fail"))
}

#[allow(dead_code)]
pub(crate) fn recovery_blocking_torn_wal_frame_damage(
    range: WalLsnRange,
) -> RecoveryBlockedByIntegrityDamage {
    recovery_blocking_damage_from_payload(&torn_wal_payload(range))
}

fn recovery_blocking_damage_from_payload(payload: &[u8]) -> RecoveryBlockedByIntegrityDamage {
    let denial = inspect_wal_payload(payload).unwrap_err();
    RecoveryBlockedByIntegrityDamage::damaged_wal_frame(&denial)
}

fn inspect_wal_payload(payload: &[u8]) -> Result<WalFrameIntegrityReport, WalFrameDamageDenial> {
    let mut table = resident_frame_table();
    let admission = admit_wal_payload_frame(&mut table, payload);
    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    let protected = ProtectedPhysicalByteView::from_pinned_frame(&pinned.view().unwrap());
    let entry = IntegrityEntryAdmission::from_s3_readiness(s3_readiness()).unwrap();
    let inspection_lease = entry.admit(IntegrityEntryRequest::new(protected)).unwrap();
    let checksum_scope = checksum_scope();
    let integrity_admission = PhysicalIntegrityAdmission::from_entry(inspection_lease)
        .with_checksum_claim(
            ChecksumAlgorithmClaim::declared_text("crc32c"),
            checksum_scope.clone(),
        )
        .unwrap();
    let checked = integrity_admission
        .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
            wal_reference(),
            wal_header_witness(&frame_bytes(1, payload)),
            PhysicalFrameKind::RecordFrame,
            DeclaredPhysicalChecksum::new(crc32c(payload).into()),
        ))
        .unwrap();
    let scoped = PhysicalScopeAdmission::admit_frame(
        checked,
        PhysicalScopeAdmissionRequest::frame(
            PhysicalReferenceScope::wal_frame(wal_reference()),
            manifest_membership(),
            RootManifestIntegrityPosture::current_root_admitted(manifest_membership()),
            CheckpointAdjacencyPosture::NotCheckpointAdjacent,
            checksum_coverage_basis(),
        ),
    )
    .unwrap();
    WalFrameIntegrityAuthority::s3().inspect(
        WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(
            ScopedPhysicalValidatorInput::wal_frame(scoped).unwrap(),
        )
        .unwrap(),
    )
}

fn intact_wal_payload(range: WalLsnRange) -> Vec<u8> {
    wal_payload(range, 0, "ok")
}

fn torn_wal_payload(range: WalLsnRange) -> Vec<u8> {
    wal_payload(range, 8, "ok")
}

fn wal_payload(range: WalLsnRange, declared_extra: usize, status: &str) -> Vec<u8> {
    let body = format!(
        "range:{}-{}",
        range.start().get(),
        range.end_exclusive().get()
    );
    let declared_len = body.len() + declared_extra;
    format!("WALF|crc32c|{declared_len}|{status}|{body}").into_bytes()
}

fn admit_wal_payload_frame(
    table: &mut ResidentFrameTable,
    payload: &[u8],
) -> forge_store_buffer_pool::ResidentFrameAdmission {
    let frame = frame_bytes(1, payload);
    let request = ResidentFrameLoadRequest::from_s1_physical_frame(
        wal_reference(),
        wal_header_witness(&frame),
    )
    .unwrap();
    let payload = physical_header()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

fn resident_frame_table() -> ResidentFrameTable {
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
        PinnedPageBudget::pages(2).unwrap(),
        DirtyPageBudget::pages(1).unwrap(),
    );
    let entry = S2PhysicalResidencyEntry::from_s1_readiness(s2_readiness())
        .unwrap()
        .with_budget(budget)
        .admit()
        .unwrap();
    ResidentFrameTable::open(entry, ResidentFrameTableCapacity::frames(1).unwrap())
}

fn s3_readiness() -> S3PhysicalIntegrityReadiness {
    let s2 = s2_readiness();
    let facts = s2.facts();
    let payload = S3PhysicalIntegrityReadinessPayload::from_s2_closeout_evidence(
        ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
        VerifierResidentEnvelope::bounded(8192, 2).unwrap(),
        ScrubPlanningAllocationEnvelope::bounded(1024).unwrap(),
        IntegrityInspectionLifetimeLaw::lease_scoped(),
        S2NoMaterializationWitness::observed_zero(0, 0).unwrap(),
        S2BoundedCounterRecap::exact(8192, 1, 0, 1024, 0, 0).unwrap(),
        S2DenialBehaviorRecap::from_named_boundaries(&S2DeniedBoundaryKind::ALL).unwrap(),
        PhysicalAuthorityRecap::from_s1_authority(
            facts.physical_reference_count(),
            facts.header_decode_witness_count(),
            facts.payload_admission_witness_count(),
        )
        .unwrap(),
        BufferPoolAuthorityRecap::s2_authority(true, true, true, true).unwrap(),
    );
    S3PhysicalIntegrityReadiness::from_s2_bounded_residency_closeout(s2, payload).unwrap()
}

fn s2_readiness() -> forge_store_readiness::S2PhysicalSubstrateReadiness {
    prove_s2_physical_substrate_readiness(
        close_s1_physical_substrate_readiness(accepted_s1_readiness()).unwrap(),
    )
    .unwrap()
}

fn accepted_s1_readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_s0_artifacts(
        ROADMAP_2_S1_SCOPE,
        HandoffEvidenceDigestSet::new(
            digest("backend"),
            digest("deferred"),
            digest("harness"),
            digest("terms"),
            digest("audit"),
            digest("complexity"),
            digest("provenance"),
        ),
    )
    .unwrap()
}

fn manifest_membership() -> ManifestMembershipProof {
    let root_cell = PhysicalGenerationAuthority::s1()
        .root_publication_cell(PhysicalRootReference::from_raw(9).unwrap())
        .with_root_publication_generation(generation(1));
    let root = PhysicalManifestUniverseBuilder::s1(root_cell)
        .segment(
            PhysicalGenerationAuthority::s1()
                .segment_cell(segment(1))
                .with_segment_generation(generation(1)),
        )
        .ordinary_page(wal_slot_cell())
        .publish();
    ManifestMembershipProof::from_root(&root, PhysicalReferenceScope::wal_frame(wal_reference()))
        .unwrap()
}

fn checksum_scope() -> ChecksumScopeDeclaration {
    ChecksumScopeDeclaration::for_physical_format(
        PhysicalFormatDeclaration::s1_canonical()
            .unwrap()
            .identity(),
        ChecksumCoverageMap::s1_page_and_frame_crc32c().unwrap(),
    )
    .unwrap()
}

fn checksum_coverage_basis() -> forge_store_physical_integrity::ChecksumCoverageBasis {
    forge_store_physical_integrity::ChecksumAlgorithmId::crc32c()
        .declare_for_scope(checksum_scope())
        .unwrap()
        .coverage_basis()
        .clone()
}

fn wal_header_witness(frame: &[u8]) -> forge_store_physical_format::PhysicalHeaderDecodeWitness {
    physical_header()
        .decode_frame_header(wal_reference(), frame, PhysicalFrameKind::RecordFrame)
        .unwrap()
        .witness()
}

fn physical_header() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap())
}

fn wal_reference() -> PhysicalReferenceValidationWitness {
    PhysicalReferenceAuthority::s1()
        .validate_page_slot(
            PhysicalReferenceAuthority::s1().admit_page_slot(wal_slot_cell()),
            wal_slot_cell(),
        )
        .unwrap()
}

fn wal_slot_cell() -> forge_store_physical_format::SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(1))
}

fn frame_bytes(generation_value: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::RecordFrame.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation_value.to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn crc32c(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0x82f6_3b78 & mask);
        }
    }
    !crc
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:{name}")).unwrap()
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
