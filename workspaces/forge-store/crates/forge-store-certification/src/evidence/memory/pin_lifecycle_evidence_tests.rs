use crate::{PinLifecycleEvidenceDenial, PinLifecycleEvidenceReport, PinLifecycleEvidenceRow};
use forge_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, PinnedPageBudget, ResidentFrameAdmission,
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

#[test]
fn explicit_unpin_evidence_consumes_real_receipt() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, b"certified");

    let receipt = table
        .lease_page(admission.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap()
        .unpin()
        .unwrap();
    let evidence = PinLifecycleEvidenceReport::from_explicit_unpin(receipt);

    assert_eq!(
        evidence.row(),
        PinLifecycleEvidenceRow::ExplicitUnpinReceiptObserved
    );
    assert_eq!(evidence.counters().explicit_unpin_count(), 1);
    assert_eq!(evidence.counters().defensive_drop_count(), 0);
}

#[test]
fn closeout_evidence_rows_cannot_claim_explicit_unpin() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, b"dropped");

    {
        let pinned = table
            .lease_page(admission.resident_frame_token())
            .unwrap()
            .pin()
            .unwrap();
        let _ = pinned.view().unwrap();
    }

    let closeout = table.pin_lifecycle_closeout();
    let denial = PinLifecycleEvidenceReport::from_closeout(
        PinLifecycleEvidenceRow::ExplicitUnpinReceiptObserved,
        closeout,
    )
    .unwrap_err();
    let evidence = PinLifecycleEvidenceReport::from_closeout(
        PinLifecycleEvidenceRow::DefensiveDropCleanupObserved,
        closeout,
    )
    .unwrap();

    assert_eq!(denial, PinLifecycleEvidenceDenial::WrongEvidenceRow);
    assert_eq!(
        evidence.row(),
        PinLifecycleEvidenceRow::DefensiveDropCleanupObserved
    );
    assert_eq!(evidence.counters().defensive_drop_count(), 1);
    assert_eq!(evidence.counters().explicit_unpin_count(), 0);
}

#[test]
fn clean_closeout_cannot_certify_unexecuted_lifecycle_rows() {
    let mut table = resident_frame_table();
    let closeout = table.pin_lifecycle_closeout();

    let drop_denial = PinLifecycleEvidenceReport::from_closeout(
        PinLifecycleEvidenceRow::DefensiveDropCleanupObserved,
        closeout,
    )
    .unwrap_err();
    let protected_denial = PinLifecycleEvidenceReport::from_closeout(
        PinLifecycleEvidenceRow::ProtectedFrameMutationDenied,
        closeout,
    )
    .unwrap_err();

    assert_eq!(
        drop_denial,
        PinLifecycleEvidenceDenial::UnprovenLifecycleRow
    );
    assert_eq!(
        protected_denial,
        PinLifecycleEvidenceDenial::UnprovenLifecycleRow
    );
}

#[test]
fn protected_mutation_evidence_requires_recorded_denial_counter() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, b"protected");
    let slot = admission.slot();
    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    std::mem::forget(pinned);

    let denial = table.reuse_frame_slot(slot, load_request_from_payload(b"next"));
    let closeout = table.pin_lifecycle_closeout();
    let evidence = PinLifecycleEvidenceReport::from_closeout(
        PinLifecycleEvidenceRow::ProtectedFrameMutationDenied,
        closeout,
    )
    .unwrap();

    assert_eq!(
        denial.unwrap_err().kind(),
        ResidentFrameDenialKind::ResidentFramePinned
    );
    assert_eq!(evidence.counters().explicit_unpin_count(), 0);
    assert_eq!(evidence.counters().leaked_pin_count(), 1);
    assert_eq!(evidence.counters().denied_protected_mutation_count(), 1);
    assert_eq!(
        evidence.row(),
        PinLifecycleEvidenceRow::ProtectedFrameMutationDenied
    );
}

#[test]
fn leak_report_evidence_consumes_recorded_leak_counters() {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, b"leak-evidence");
    let pinned = table
        .lease_page(admission.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(pinned);

    let leak = table.leaked_pin_report().unwrap();
    let evidence = PinLifecycleEvidenceReport::from_leak_report(leak).unwrap();

    assert_eq!(
        evidence.row(),
        PinLifecycleEvidenceRow::LeakCloseoutObserved
    );
    assert_eq!(evidence.counters().leaked_pin_count(), 1);
    assert_eq!(evidence.counters().active_pinned_pages(), 1);
    assert_eq!(evidence.counters().explicit_unpin_count(), 0);
}

fn admit_payload_frame(table: &mut ResidentFrameTable, payload: &[u8]) -> ResidentFrameAdmission {
    let frame = frame_bytes(7, payload);
    let request = load_request_from_frame(&frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

fn load_request_from_payload(payload: &[u8]) -> ResidentFrameLoadRequest {
    let frame = frame_bytes(7, payload);
    load_request_from_frame(&frame)
}

fn resident_frame_table() -> ResidentFrameTable {
    let readiness = prove_physical_substrate_readiness(
        close_physical_substrate_readiness(accepted_physical_format_readiness()).unwrap(),
    )
    .unwrap();
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
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
    ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(1).unwrap())
}

fn load_request_from_frame(frame_bytes: &[u8]) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_physical_format_physical_frame(
        validated_slot_reference(),
        frame_header_witness(frame_bytes),
    )
    .unwrap()
}

fn validated_slot_reference() -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(7));
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn frame_header_witness(bytes: &[u8]) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(),
            bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
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
