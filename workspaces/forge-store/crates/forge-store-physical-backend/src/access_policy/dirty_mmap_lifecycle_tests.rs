use super::*;
use crate::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
    BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    PhysicalAlignmentClass, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalPublicationState, PhysicalRecordSlot, PhysicalReference,
    PhysicalReferenceAuthority, PhysicalReferenceValidationWitness, PhysicalSegmentId,
    SlotGenerationCell, PHYSICAL_HEADER_LENGTH,
};
use forge_store_readiness::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
};
use forge_store_security::admitted_store_internal_security_scope_for_io_qos_test;

#[test]
fn real_buffer_pool_mmap_dirty_state_blocks_direct_io_admission() {
    let backend = backend_with_all_access_modes();
    let reference = test_reference();
    let lifecycle = mmap_dirty_lifecycle_from_buffer_pool();
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);

    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            AccessPolicyRequest::direct_io_read()
                .for_physical_reference(reference)
                .with_security_scope(test_security_scope())
                .with_buffer_lifecycle(lifecycle)
                .with_page_cache_policy(page_cache_policy(&backend))
                .with_alignment_requirement(
                    proof
                        .direct_io_page_and_sector_aligned(
                            reference,
                            lifecycle,
                            4096,
                            PhysicalAlignmentClass::page_start_4k(),
                            PhysicalAlignmentClass::extent_start_4k(),
                        )
                        .expect("test backend carries direct I/O alignment assumptions"),
                ),
        )
        .expect_err("real mmap dirty state denies direct I/O before execution");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::DirtyMmapPageBlocksDirectIo
    );
}

#[test]
fn real_buffer_pool_mmap_dirty_state_blocks_mixed_direct_io_participant() {
    let backend = backend_with_all_access_modes();
    let reference = test_reference();
    let lifecycle = mmap_dirty_lifecycle_from_buffer_pool();
    let scope = test_security_scope();
    let transition =
        MixedAccessTransition::new(StoreAccessMode::Buffered, StoreAccessMode::DirectIo);
    let proof = StoreAccessPolicyProofAuthority::for_admitted_backend(&backend);

    let denial = AccessPolicyAdmission::for_backend(&backend)
        .admit(
            AccessPolicyRequest::mixed_read(transition)
                .for_physical_reference(reference)
                .with_security_scope(scope)
                .with_buffer_lifecycle(lifecycle)
                .with_page_cache_policy(page_cache_policy(&backend))
                .with_alignment_requirement(
                    proof
                        .direct_io_page_and_sector_aligned(
                            reference,
                            lifecycle,
                            4096,
                            PhysicalAlignmentClass::page_start_4k(),
                            PhysicalAlignmentClass::extent_start_4k(),
                        )
                        .expect("test backend carries direct I/O alignment assumptions"),
                )
                .with_coherence_basis(
                    proof
                        .mixed_coherence(transition, reference, scope)
                        .expect("test backend carries mixed access coherence assumptions"),
                ),
        )
        .expect_err("real mmap dirty state blocks mixed direct-I/O before execution");

    assert_eq!(
        denial.kind(),
        AccessPolicyDenialKind::DirtyMmapPageBlocksDirectIo
    );
}

fn mmap_dirty_lifecycle_from_buffer_pool() -> AccessPolicyBufferLifecycle {
    let mut table = resident_frame_table();
    let frame = frame_bytes(7, b"mmap-dirty-lifecycle");
    let request = load_request_from_frame(7, 2, &frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    let admission = table.admit_resident_frame_bytes(request, payload).unwrap();

    table.mark_dirty(admission.resident_frame_token()).unwrap();
    table
        .mark_mmap_dirty(admission.resident_frame_token())
        .unwrap()
        .access_policy_lifecycle_proof()
}

fn resident_frame_table() -> ResidentFrameTable {
    let readiness = prove_physical_substrate_readiness(
        close_physical_substrate_readiness(accepted_physical_format_readiness()).unwrap(),
    )
    .unwrap();
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
        forge_store_buffer_pool::PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(1).unwrap(),
    );
    let admitted = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        readiness.physical_substrate_snapshot(),
    )
    .unwrap()
    .with_budget(budget)
    .admit()
    .unwrap();
    ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(1).unwrap())
}

fn page_cache_policy(backend: &crate::AdmittedBackendCapabilityWitness) -> PageCachePolicyProof {
    StoreAccessPolicyProofAuthority::for_admitted_backend(backend)
        .page_cache_policy()
        .expect("test backend carries Store-admitted page-cache posture")
}

fn backend_with_all_access_modes() -> crate::AdmittedBackendCapabilityWitness {
    let support = BackendCapabilitySupportSet::all_supported().with_posture(
        BackendCapabilityKind::DirectIo,
        BackendCapabilitySupportPosture::Supported,
    );
    let assumptions = BackendMediaAssumptionSet::platform_file_defaults()
        .with_direct_io_alignment()
        .with_sector_atomicity()
        .with_page_cache_policy()
        .with_mmap_coherence()
        .with_mixed_access_coherence();
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            support,
            assumptions,
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend()
                .with_sector_alignment()
                .with_security_posture(),
        ))
        .expect("test backend capability admits")
}

fn load_request_from_frame(
    generation_value: u64,
    page_value: u64,
    frame_bytes: &[u8],
) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_physical_format_physical_frame(
        validated_slot_reference(generation_value, page_value),
        frame_header_witness(generation_value, page_value, frame_bytes),
    )
    .unwrap()
}

fn frame_header_witness(
    generation_value: u64,
    page_value: u64,
    bytes: &[u8],
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(generation_value, page_value),
            bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn test_security_scope() -> AccessPolicySecurityScope {
    let admitted = admitted_store_internal_security_scope_for_io_qos_test();
    AccessPolicySecurityScope::from_current_store_scope(admitted.witnesses())
}

fn test_reference() -> PhysicalReference {
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(reference_cell(7, 2))
        .reference()
}

fn validated_slot_reference(
    generation_value: u64,
    page_value: u64,
) -> PhysicalReferenceValidationWitness {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = reference_cell(generation_value, page_value);
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn reference_cell(generation_value: u64, page_value: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value))
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap())
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
