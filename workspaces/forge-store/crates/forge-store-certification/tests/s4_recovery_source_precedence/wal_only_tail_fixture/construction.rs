use forge_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, PinnedPageBudget, ResidentFrameLoadRequest,
    ResidentFrameTable, ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, BufferPoolAuthorityRecap, HandoffEvidenceDigestSet,
    IntegrityInspectionLifetimeLaw, PhysicalAuthorityRecap, ProtectedIntegrityViewCapability,
    BoundedCounterRecap, DenialBehaviorRecap, DeniedBoundaryKind, NoMaterializationWitness,
    PhysicalIntegrityReadinessPayload, ScrubPlanningAllocationEnvelope, StableDigest,
    VerifierResidentEnvelope, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, ChecksumCoverageMap, ManifestMembershipProof,
    PhysicalBinaryEncodingWitness, PhysicalFormatDeclaration, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalGenerationOwner,
    PhysicalHeaderAuthority, PhysicalManifestUniverseBuilder, PhysicalPageId,
    PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceScope, PhysicalReferenceValidationWitness, PhysicalRootReference,
    PhysicalSegmentId, RootManifestIntegrityPosture, SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};
use forge_store_physical_integrity::{
    ChecksumAlgorithmClaim, ChecksumScopeDeclaration, DeclaredPhysicalChecksum,
    IntegrityEntryAdmission, IntegrityEntryRequest, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionRequest, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    ProtectedPhysicalByteView, ScopedPhysicalValidatorInput, WalFrameDamageDenial,
    WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest, WalFrameIntegrityReport,
};
use forge_store_readiness::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
    PhysicalIntegrityReadiness,
};
use forge_store_recovery_physics::WalLsnRange;

pub(super) fn inspect_wal_payload(
    payload: &[u8],
) -> Result<WalFrameIntegrityReport, WalFrameDamageDenial> {
    inspect_wal_payload_for_owner(payload, wal_slot_cell().owner())
}

pub(super) fn inspect_wal_payload_for_owner(
    payload: &[u8],
    owner: PhysicalGenerationOwner,
) -> Result<WalFrameIntegrityReport, WalFrameDamageDenial> {
    let cell = wal_slot_cell_for_owner(owner);
    let reference = wal_reference_for_cell(cell);
    let mut table = resident_frame_table();
    let admission = admit_wal_payload_frame(&mut table, payload, reference);
    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    let protected = ProtectedPhysicalByteView::from_pinned_frame(&pinned.view().unwrap());
    let entry = IntegrityEntryAdmission::from_physical_integrity_payload(physical_integrity_readiness().payload()).unwrap();
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
            reference,
            wal_header_witness(&frame_bytes(owner.generation().get(), payload), reference),
            PhysicalFrameKind::RecordFrame,
            DeclaredPhysicalChecksum::new(crc32c(payload).into()),
        ))
        .unwrap();
    let scoped = PhysicalScopeAdmission::admit_frame(
        checked,
        PhysicalScopeAdmissionRequest::frame(
            PhysicalReferenceScope::wal_frame(reference),
            manifest_membership_for_scope(PhysicalReferenceScope::wal_frame(reference), cell),
            RootManifestIntegrityPosture::current_root_admitted(manifest_membership_for_scope(
                PhysicalReferenceScope::wal_frame(reference),
                cell,
            )),
            CheckpointAdjacencyPosture::NotCheckpointAdjacent,
            checksum_coverage_basis(),
        ),
    )
    .unwrap();
    WalFrameIntegrityAuthority::new().inspect(
        WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(
            ScopedPhysicalValidatorInput::wal_frame(scoped).unwrap(),
        )
        .unwrap(),
    )
}

pub(super) fn intact_wal_payload(range: WalLsnRange) -> Vec<u8> {
    wal_payload(range, 0, "ok")
}

pub(super) fn torn_wal_payload(range: WalLsnRange) -> Vec<u8> {
    wal_payload(range, 8, "ok")
}

pub(super) fn wal_payload(range: WalLsnRange, declared_extra: usize, status: &str) -> Vec<u8> {
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
    reference: PhysicalReferenceValidationWitness,
) -> forge_store_buffer_pool::ResidentFrameAdmission {
    let frame = frame_bytes(reference.owner().generation().get(), payload);
    let request = ResidentFrameLoadRequest::from_physical_format_physical_frame(
        reference,
        wal_header_witness(&frame, reference),
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
    let entry = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        physical_substrate_readiness().physical_substrate_snapshot(),
    )
    .unwrap()
    .with_budget(budget)
    .admit()
    .unwrap();
    ResidentFrameTable::open(entry, ResidentFrameTableCapacity::frames(1).unwrap())
}

pub(crate) fn physical_integrity_readiness() -> PhysicalIntegrityReadiness {
    let s2 = physical_substrate_readiness();
    let facts = s2.facts();
    let payload = PhysicalIntegrityReadinessPayload::from_physical_substrate_closeout_evidence(
        ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
        VerifierResidentEnvelope::bounded(8192, 2).unwrap(),
        ScrubPlanningAllocationEnvelope::bounded(1024).unwrap(),
        IntegrityInspectionLifetimeLaw::lease_scoped(),
        NoMaterializationWitness::observed_zero(0, 0).unwrap(),
        BoundedCounterRecap::exact(8192, 1, 0, 1024, 0, 0).unwrap(),
        DenialBehaviorRecap::from_named_boundaries(&DeniedBoundaryKind::ALL).unwrap(),
        PhysicalAuthorityRecap::from_physical_format_authority(
            facts.physical_reference_count(),
            facts.header_decode_witness_count(),
            facts.payload_admission_witness_count(),
        )
        .unwrap(),
        BufferPoolAuthorityRecap::physical_substrate_authority(true, true, true, true).unwrap(),
    );
    PhysicalIntegrityReadiness::from_physical_substrate_bounded_residency_closeout(s2, payload).unwrap()
}

fn physical_substrate_readiness() -> forge_store_readiness::PhysicalSubstrateReadiness {
    prove_physical_substrate_readiness(
        close_physical_substrate_readiness(accepted_physical_format_readiness()).unwrap(),
    )
    .unwrap()
}

fn accepted_physical_format_readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
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

fn manifest_membership_for_scope(
    scope: PhysicalReferenceScope,
    cell: SlotGenerationCell,
) -> ManifestMembershipProof {
    let root_cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(PhysicalRootReference::from_raw(9).unwrap())
        .with_root_publication_generation(generation(1));
    let root = PhysicalManifestUniverseBuilder::for_canonical_physical_format(root_cell)
        .segment(
            PhysicalGenerationAuthority::for_canonical_physical_format()
                .segment_cell(cell.segment_id())
                .with_segment_generation(cell.generation()),
        )
        .ordinary_page(cell)
        .publish();
    ManifestMembershipProof::from_root(&root, scope).unwrap()
}

fn checksum_scope() -> ChecksumScopeDeclaration {
    ChecksumScopeDeclaration::for_physical_format(
        PhysicalFormatDeclaration::physical_format_canonical()
            .unwrap()
            .identity(),
        ChecksumCoverageMap::physical_format_page_and_frame_crc32c().unwrap(),
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

fn wal_header_witness(
    frame: &[u8],
    reference: PhysicalReferenceValidationWitness,
) -> forge_store_physical_format::PhysicalHeaderDecodeWitness {
    physical_header()
        .decode_frame_header(reference, frame, PhysicalFrameKind::RecordFrame)
        .unwrap()
        .witness()
}

fn physical_header() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap())
}

fn wal_reference_for_cell(cell: SlotGenerationCell) -> PhysicalReferenceValidationWitness {
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .validate_page_slot(PhysicalReferenceAuthority::for_canonical_physical_format().admit_page_slot(cell), cell)
        .unwrap()
}

fn wal_slot_cell() -> SlotGenerationCell {
    wal_slot_cell_for_owner(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .slot_cell(segment(1), page(2), slot(3))
            .with_slot_generation(generation(1))
            .owner(),
    )
}

fn wal_slot_cell_for_owner(owner: PhysicalGenerationOwner) -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            owner.segment_id().unwrap(),
            owner.page_id().unwrap(),
            owner.slot().unwrap(),
        )
        .with_slot_generation(owner.generation())
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
