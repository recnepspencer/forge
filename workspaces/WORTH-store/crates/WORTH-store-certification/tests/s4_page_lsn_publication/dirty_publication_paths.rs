use worth_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, DirtyPublicationReceipt, PinnedPageBudget,
    ResidentFrameAdmission, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeWitness,
    PhysicalPageId, PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};
use worth_store_readiness::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
};

pub fn scheduled_dirty_publication(payload: &[u8]) -> DirtyPublicationReceipt {
    scheduled_dirty_publication_for_page(payload, 2)
}

pub fn scheduled_dirty_publication_for_page(
    payload: &[u8],
    page_value: u64,
) -> DirtyPublicationReceipt {
    let mut table = resident_frame_table();
    let admission = admit_payload_frame(&mut table, payload, page_value);
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    table.record_dirty_write_scheduled(plan).unwrap()
}

fn resident_frame_table() -> ResidentFrameTable {
    let readiness = prove_s2_physical_substrate_readiness(
        close_s1_physical_substrate_readiness(accepted_s1_readiness()).unwrap(),
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
    ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(2).unwrap())
}

fn admit_payload_frame(
    table: &mut ResidentFrameTable,
    payload: &[u8],
    page_value: u64,
) -> ResidentFrameAdmission {
    let frame = frame_bytes(7, payload);
    let request = load_request_from_frame(7, page_value, &frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

fn load_request_from_frame(
    generation_value: u64,
    page_value: u64,
    frame_bytes: &[u8],
) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_s1_physical_frame(
        validated_slot_reference(generation_value, page_value),
        frame_header_witness(generation_value, page_value, frame_bytes),
    )
    .unwrap()
}

fn validated_slot_reference(
    generation_value: u64,
    page_value: u64,
) -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
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
    PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap())
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
