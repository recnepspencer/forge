use crate::{
    AccessPolicyBufferLifecycleKind, BufferPoolBudget, DirtyPageBudget, PinnedPageBudget,
    ResidentFrameDenialKind, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};
use forge_store_readiness::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
};
use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn explicit_pin_view_and_unpin_produce_normal_lifecycle_receipt() {
    let mut table = resident_frame_table(8192, 1);
    let frame_bytes = frame_bytes(7, b"resident-payload");
    let request = load_request_from_frame(7, 2, &frame_bytes);
    let payload = header_authority()
        .payload_view(&frame_bytes, request.header())
        .unwrap();
    let admission = table.admit_resident_frame_bytes(request, payload).unwrap();

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    let view = pinned.view().unwrap();

    assert_eq!(view.as_bytes(), b"resident-payload");
    let receipt = pinned.unpin().unwrap();
    assert_eq!(receipt.identity(), admission.identity());
    assert_eq!(receipt.counters().successful_pin_count(), 1);
    assert_eq!(receipt.counters().explicit_unpin_count(), 1);
    assert_eq!(receipt.counters().active_pinned_pages(), 0);
    assert_eq!(table.pin_counters().defensive_drop_count(), 0);
}

#[test]
fn pinned_page_lease_produces_access_policy_lifecycle_proof() {
    let mut table = resident_frame_table(8192, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"access-policy-proof");

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    let proof = pinned.access_policy_lifecycle_proof();

    assert_eq!(proof.kind(), AccessPolicyBufferLifecycleKind::PinnedS2Lease);
    let _ = pinned.unpin().unwrap();
}

#[test]
fn resident_bytes_are_not_viewable_without_byte_admission() {
    let mut table = resident_frame_table(8192, 1);
    let request = load_request(7, 2, 8);
    let admission = table.admit_frame(request).unwrap();

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    let denial = pinned.view().unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::ResidentBytesNotAdmitted
    );
    let _ = pinned.unpin().unwrap();
}

#[test]
fn leaked_pin_remains_protected_and_is_reported_at_closeout() {
    let mut table = resident_frame_table(8192, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"leaked");
    let slot = admission.slot();

    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    std::mem::forget(pinned);

    let denied = table
        .reuse_frame_slot(slot, load_request(8, 3, 6))
        .unwrap_err();
    let leak = table.leaked_pin_report().unwrap();

    assert_eq!(denied.kind(), ResidentFrameDenialKind::ResidentFramePinned);
    assert_eq!(leak.leaked_pin_count(), 1);
    assert_eq!(leak.pin_counters().leaked_pin_count(), 1);
    assert_eq!(leak.pin_counters().denied_protected_mutation_count(), 1);
    assert_eq!(table.pin_counters().active_pinned_pages(), 1);
    assert_eq!(table.pin_counters().denied_protected_mutation_count(), 1);
    assert_eq!(table.pin_counters().leaked_pin_count(), 1);
}

#[test]
fn dropped_pin_defensively_cleans_without_normal_receipt() {
    let mut table = resident_frame_table(8192, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"drop-cleanup");

    {
        let lease = table.lease_page(admission.resident_frame_token()).unwrap();
        let pinned = lease.pin().unwrap();
        let view = pinned.view().unwrap();
        assert_eq!(view.as_bytes(), b"drop-cleanup");
    }

    assert_eq!(table.pin_counters().defensive_drop_count(), 1);
    assert_eq!(table.pin_counters().explicit_unpin_count(), 0);
    assert_eq!(table.pin_counters().active_pinned_pages(), 0);
    assert!(table.leaked_pin_report().is_none());
}

#[test]
fn panic_unwind_defensively_cleans_without_normal_receipt() {
    let mut table = resident_frame_table(8192, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"panic-cleanup");

    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        let lease = table.lease_page(admission.resident_frame_token()).unwrap();
        let pinned = lease.pin().unwrap();
        let view = pinned.view().unwrap();
        assert_eq!(view.as_bytes(), b"panic-cleanup");
        panic!("phase 3 panic lifecycle pressure");
    }));

    assert!(panic_result.is_err());
    assert_eq!(table.pin_counters().defensive_drop_count(), 1);
    assert_eq!(table.pin_counters().explicit_unpin_count(), 0);
    assert_eq!(table.pin_counters().active_pinned_pages(), 0);
    assert!(table.leaked_pin_report().is_none());
}

#[test]
fn mismatched_payload_admission_cannot_enter_resident_bytes() {
    let mut table = resident_frame_table(8192, 1);
    let request = load_request(7, 2, 5);
    let other_frame = frame_bytes(8, b"other");
    let other_request = load_request_from_frame(8, 2, &other_frame);
    let mismatched_payload = header_authority()
        .payload_view(&other_frame, other_request.header())
        .unwrap();

    let denial = table
        .admit_resident_frame_bytes(request, mismatched_payload)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::ResidentPayloadWitnessMismatch
    );
    assert_eq!(table.counters().resident_bytes().as_bytes(), 0);
}

fn admit_payload_frame(
    table: &mut ResidentFrameTable,
    generation_value: u64,
    page_value: u64,
    payload: &[u8],
) -> crate::ResidentFrameAdmission {
    let frame = frame_bytes(generation_value, payload);
    let request = load_request_from_frame(generation_value, page_value, &frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

fn resident_frame_table(resident_bytes: u64, frame_count: u32) -> ResidentFrameTable {
    let readiness = prove_physical_substrate_readiness(
        close_physical_substrate_readiness(accepted_physical_format_readiness()).unwrap(),
    )
    .unwrap();
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(resident_bytes).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(2).unwrap(),
    );
    let admitted = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        readiness.physical_substrate_snapshot(),
    )
    .unwrap()
    .with_budget(budget)
    .admit()
    .unwrap();
    ResidentFrameTable::open(
        admitted,
        ResidentFrameTableCapacity::frames(frame_count).unwrap(),
    )
}

fn load_request(
    generation_value: u64,
    page_value: u64,
    payload_len: usize,
) -> ResidentFrameLoadRequest {
    let frame = frame_bytes(generation_value, &vec![0xAB; payload_len]);
    load_request_from_frame(generation_value, page_value, &frame)
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

fn validated_slot_reference(
    generation_value: u64,
    page_value: u64,
) -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(generation_value));
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
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
