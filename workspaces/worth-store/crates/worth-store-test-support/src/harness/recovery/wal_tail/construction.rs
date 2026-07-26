use worth_store_buffer_pool::{
    PhysicalFrameAccess, PhysicalFrameKey, PhysicalOperationAllocationScope,
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalSpeculativeWorkKind,
};
use worth_store_contracts::{
    BoundedCounterRecap, BufferPoolAuthorityRecap, DenialBehaviorRecap, DeniedBoundaryKind,
    IntegrityInspectionLifetimeLaw, NoMaterializationWitness, PhysicalAuthorityRecap,
    PhysicalIntegrityReadinessPayload, ProtectedIntegrityViewCapability,
    ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
use worth_store_physical_format::{
    store_namespace::{
        ProposedStoreIdentity, StableStoreIdentity, StoreNamespaceIdentityRecord,
        StoreNamespaceVersion,
    },
    CheckpointAdjacencyPosture, ChecksumCoverageMap, ManifestMembershipProof,
    PhysicalBinaryEncodingWitness, PhysicalFormatDeclaration, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalGenerationOwner,
    PhysicalHeaderAuthority, PhysicalManifestUniverseBuilder, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceScope, PhysicalReferenceValidationWitness,
    PhysicalRootReference, PhysicalSegmentId, RecordArtifactFile, RecordFrameCoordinate,
    RootManifestIntegrityPosture, SlotGenerationCell,
};
use worth_store_physical_integrity::{
    ChecksumAlgorithmClaim, ChecksumScopeDeclaration, DeclaredPhysicalChecksum,
    IntegrityEntryAdmission, IntegrityEntryRequest, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionRequest, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    ProtectedPhysicalByteView, ScopedPhysicalValidatorInput, WalFrameDamageDenial,
    WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest, WalFrameIntegrityReport,
};
use worth_store_recovery_physics::WalLsnRange;

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
    let frame = frame_bytes(cell, payload);
    let store = physical_residency_store();
    let pool = PhysicalResidencyPool::open(store, wal_residency_limits()).unwrap();
    let allocation = pool
        .begin_operation(
            PhysicalOperationAllocationScope::Recovery,
            std::num::NonZeroU64::MIN,
        )
        .unwrap();
    let key = PhysicalFrameKey::new(store, frame_coordinate(owner, frame.len()));
    let PhysicalFrameAccess::Fault(fault) = pool.access_frame(&allocation, key).unwrap() else {
        panic!("a fresh WAL inspection pool must issue the sole frame fault");
    };
    let lease = fault
        .load(|target| {
            target.copy_from_slice(&frame);
            Ok::<_, std::convert::Infallible>(())
        })
        .unwrap();
    let protected = ProtectedPhysicalByteView::from_physical_frame(&lease);
    let entry =
        IntegrityEntryAdmission::from_integrity_model_payload(physical_integrity_model_payload())
            .unwrap();
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
            wal_header_witness(&frame, reference),
            PhysicalFrameKind::RecordFrame,
            DeclaredPhysicalChecksum::new(crc32c(&frame).into()),
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

pub fn physical_integrity_model_payload() -> PhysicalIntegrityReadinessPayload {
    PhysicalIntegrityReadinessPayload::from_physical_substrate_closeout_evidence(
        ProtectedIntegrityViewCapability::protected_views(1).unwrap(),
        VerifierResidentEnvelope::bounded(8192, 2).unwrap(),
        ScrubPlanningAllocationEnvelope::bounded(1024).unwrap(),
        IntegrityInspectionLifetimeLaw::lease_scoped(),
        NoMaterializationWitness::observed_zero(0, 0).unwrap(),
        BoundedCounterRecap::exact(8192, 1, 0, 1024, 0, 0).unwrap(),
        DenialBehaviorRecap::from_named_boundaries(&DeniedBoundaryKind::ALL).unwrap(),
        PhysicalAuthorityRecap::from_physical_format_authority(4, 2, 2).unwrap(),
        BufferPoolAuthorityRecap::physical_substrate_authority(true, true, true, true).unwrap(),
    )
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

fn checksum_coverage_basis() -> worth_store_physical_integrity::ChecksumCoverageBasis {
    worth_store_physical_integrity::ChecksumAlgorithmId::crc32c()
        .declare_for_scope(checksum_scope())
        .unwrap()
        .coverage_basis()
        .clone()
}

fn wal_header_witness(
    frame: &[u8],
    reference: PhysicalReferenceValidationWitness,
) -> worth_store_physical_format::PhysicalHeaderDecodeWitness {
    physical_header()
        .decode_frame_header(reference, frame, PhysicalFrameKind::RecordFrame)
        .unwrap()
        .witness()
}

fn physical_header() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
}

fn wal_reference_for_cell(cell: SlotGenerationCell) -> PhysicalReferenceValidationWitness {
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .validate_page_slot(
            PhysicalReferenceAuthority::for_canonical_physical_format().admit_page_slot(cell),
            cell,
        )
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

fn frame_bytes(cell: SlotGenerationCell, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(
        usize::from(worth_store_physical_format::PHYSICAL_HEADER_LENGTH) + payload.len(),
    );
    bytes.extend_from_slice(&physical_header().encode_record_frame_header(
        cell,
        u32::try_from(payload.len()).expect("test payload length should fit the physical format"),
    ));
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

fn physical_residency_store() -> StableStoreIdentity {
    StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([0x54; 16]).unwrap(),
    )
    .published_identity()
}

fn wal_residency_limits() -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Speculation;

    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(25_600))
        .resident_bytes(nonzero_bytes(8192))
        .metadata_bytes(nonzero_bytes(8192))
        .frame_entries(nonzero_count(2))
        .pinned_frames(nonzero_count(2))
        .pin_leases(nonzero_count(2))
        .dirty_frames(nonzero_count(1))
        .dirty_replacement_bytes(nonzero_bytes(8192))
        .operation_bytes(nonzero_bytes(1024))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(1024))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(1024))
        .scope_bytes(Scope::Recovery, nonzero_bytes(1024))
        .scope_bytes(Scope::Scrub, nonzero_bytes(1024))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(1024))
        .scope_bytes(Scope::Verification, nonzero_bytes(1024))
        .scope_bytes(Scope::Blob, nonzero_bytes(1024))
        .speculative_frames(Speculation::Prefetch, nonzero_count(2))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(2))
        .speculative_frames(Speculation::WriteBehind, nonzero_count(1))
        .admit(std::num::NonZeroU64::MIN)
        .unwrap()
}

fn nonzero_bytes(value: u64) -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(value).unwrap()
}

fn frame_coordinate(owner: PhysicalGenerationOwner, frame_bytes: usize) -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(
        RecordArtifactFile::Segment {
            segment: owner
                .segment_id()
                .expect("WAL frame owner has a segment")
                .get(),
            generation: owner.generation().get(),
        },
        0,
        u32::try_from(frame_bytes).expect("fixture frame length fits the physical coordinate"),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{inspect_wal_payload, intact_wal_payload, torn_wal_payload, wal_payload};
    use worth_store_recovery_physics::{LogSequenceNumber, WalLsnRange};

    #[test]
    fn intact_wal_frame_checksum_covers_header_and_payload() {
        let range = range();
        let inspection = inspect_wal_payload(&intact_wal_payload(range));

        assert!(
            inspection.is_ok(),
            "intact WAL frame should admit: {inspection:?}"
        );
    }

    #[test]
    fn damaged_and_torn_wal_frames_remain_denied() {
        let range = range();

        assert!(inspect_wal_payload(&wal_payload(range, 0, "checksum-fail")).is_err());
        assert!(inspect_wal_payload(&torn_wal_payload(range)).is_err());
    }

    fn range() -> WalLsnRange {
        WalLsnRange::new(LogSequenceNumber::new(40), LogSequenceNumber::new(50))
            .expect("test WAL range is ordered")
    }
}
