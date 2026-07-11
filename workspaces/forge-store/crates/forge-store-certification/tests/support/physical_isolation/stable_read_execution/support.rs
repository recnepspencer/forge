use forge_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationRequest,
    AllocationScope, BoundedCopyRecordView, BufferPoolBudget, DirtyPageBudget, PinnedPageBudget,
    RecordViewMaterializationProfile, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    FramedRecordView, PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority,
    PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, SlotAppendRequest, SlotGenerationCell,
    PHYSICAL_HEADER_LENGTH,
};
use forge_store_physical_isolation::CurrentGenerationPhysicalReference;
use forge_store_readiness::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
};

pub(crate) fn bounded_copy_for_record(payload: &'static [u8]) -> BoundedCopyRecordView {
    bounded_copy_for_slot(segment(1), page(2), slot(3), generation(7), payload)
}

pub(crate) fn bounded_copy_for_reference(
    reference: CurrentGenerationPhysicalReference,
    payload: &'static [u8],
) -> BoundedCopyRecordView {
    let owner = reference.owner();
    bounded_copy_for_slot(
        owner.segment_id().unwrap(),
        owner.page_id().unwrap(),
        owner.slot().unwrap(),
        owner.generation(),
        payload,
    )
}

pub(crate) fn admit_payload_frame_for_reference(
    table: &mut ResidentFrameTable,
    reference: CurrentGenerationPhysicalReference,
    payload: &[u8],
) -> forge_store_buffer_pool::ResidentFrameAdmission {
    let owner = reference.owner();
    admit_payload_frame(
        table,
        owner.segment_id().unwrap(),
        owner.page_id().unwrap(),
        owner.slot().unwrap(),
        owner.generation(),
        payload,
    )
}

fn bounded_copy_for_slot(
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
    payload: &'static [u8],
) -> BoundedCopyRecordView {
    let mut table = resident_frame_table(8192, 1);
    let admission = admit_payload_frame(
        &mut table,
        segment_id,
        page_id,
        slot_id,
        slot_generation,
        payload,
    );
    let framed = framed_record(segment_id, page_id, slot_id, slot_generation, payload);
    let mut allocation = allocation_admission(payload.len() as u64);
    let lease = table.lease_page(admission.resident_frame_token()).unwrap();
    let mut pinned = lease.pin().unwrap();
    let zero_copy = pinned
        .zero_copy_record_view(framed, RecordViewMaterializationProfile::PhysicalBytesOnly)
        .unwrap();
    let request =
        AllocationRequest::copied_payload(AllocationScope::Foreground, payload.len() as u64)
            .unwrap();
    let grant = allocation.admit(request).unwrap();
    let receipt = allocation.record_allocation(grant).unwrap();
    zero_copy.bounded_copy(receipt).unwrap()
}

pub(crate) fn payload_admission_for_frame(
    generation_value: u64,
    payload: &'static [u8],
) -> forge_store_physical_format::PhysicalPayloadViewAdmission<'static> {
    payload_admission_for_slot(
        segment(1),
        page(2),
        slot(3),
        generation(generation_value),
        payload,
    )
}

pub(crate) fn payload_admission_for_reference(
    reference: CurrentGenerationPhysicalReference,
    payload: &'static [u8],
) -> forge_store_physical_format::PhysicalPayloadViewAdmission<'static> {
    let owner = reference.owner();
    payload_admission_for_slot(
        owner.segment_id().unwrap(),
        owner.page_id().unwrap(),
        owner.slot().unwrap(),
        owner.generation(),
        payload,
    )
}

fn payload_admission_for_slot(
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
    payload: &'static [u8],
) -> forge_store_physical_format::PhysicalPayloadViewAdmission<'static> {
    let frame = Box::leak(frame_bytes(slot_generation.get(), payload).into_boxed_slice());
    let request = load_request_from_frame(segment_id, page_id, slot_id, slot_generation, frame);
    header_authority()
        .payload_view(frame, request.header())
        .unwrap()
}

fn admit_payload_frame(
    table: &mut ResidentFrameTable,
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
    payload: &[u8],
) -> forge_store_buffer_pool::ResidentFrameAdmission {
    let frame = frame_bytes(slot_generation.get(), payload);
    let request = load_request_from_frame(segment_id, page_id, slot_id, slot_generation, &frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

pub(crate) fn resident_frame_table(resident_bytes: u64, frame_count: u32) -> ResidentFrameTable {
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

fn framed_record(
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
    payload: &'static [u8],
) -> FramedRecordView<'static> {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = page_cell(&generations, segment_id, generation(5), page_id);
    let slot_cell = slot_cell(&generations, segment_id, page_id, slot_id, slot_generation);
    let empty_page = page_bytes(generation(5), &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, payload),
        )
        .unwrap();
    let reopened_page =
        Box::leak(page_bytes(generation(5), append.page_payload()).into_boxed_slice());
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .unwrap();
    records
        .locate_record(
            admitted_page(&records, page_cell, reopened_page),
            validation,
        )
        .unwrap()
        .record_view()
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    cell: PageGenerationCell,
    bytes: &'a [u8],
) -> forge_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(cell, bytes, PhysicalPageKind::DataPage)
        .unwrap();
    records
        .admit_record_page_payload(bytes, header.witness())
        .unwrap()
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::for_canonical_physical_format(header_authority())
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap())
}

fn load_request_from_frame(
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
    frame_bytes: &[u8],
) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_physical_format_physical_frame(
        validated_slot_reference(segment_id, page_id, slot_id, slot_generation),
        frame_header_witness(segment_id, page_id, slot_id, slot_generation, frame_bytes),
    )
    .unwrap()
}

fn validated_slot_reference(
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
) -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .slot_cell(segment_id, page_id, slot_id)
        .with_slot_generation(slot_generation);
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn frame_header_witness(
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
    frame_bytes: &[u8],
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(segment_id, page_id, slot_id, slot_generation),
            frame_bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn allocation_admission(bytes: u64) -> AllocationAdmission {
    let envelopes = AllocationEnvelopeDeclaration::declare()
        .foreground(AllocationByteBudget::bytes(bytes).unwrap())
        .maintenance(AllocationByteBudget::bytes(bytes).unwrap())
        .recovery(AllocationByteBudget::bytes(bytes).unwrap())
        .scrub(AllocationByteBudget::bytes(bytes).unwrap())
        .import_export(AllocationByteBudget::bytes(bytes).unwrap())
        .streaming(AllocationByteBudget::bytes(bytes).unwrap())
        .fixed_metadata(
            forge_store_buffer_pool::FixedMetadataReservation::constant_bytes(1).unwrap(),
        )
        .seal()
        .unwrap();
    AllocationAdmission::from_declaration(envelopes)
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
    StableDigest::new(format!("sha256:s5-stable-read-execution-{name}")).unwrap()
}

fn page_cell(
    generations: &PhysicalGenerationAuthority,
    segment_id: PhysicalSegmentId,
    page_generation: PhysicalGeneration,
    page_id: PhysicalPageId,
) -> PageGenerationCell {
    generations
        .page_cell(segment_id, page_id)
        .with_page_generation(page_generation)
}

fn slot_cell(
    generations: &PhysicalGenerationAuthority,
    segment_id: PhysicalSegmentId,
    page_id: PhysicalPageId,
    slot_id: PhysicalRecordSlot,
    slot_generation: PhysicalGeneration,
) -> SlotGenerationCell {
    generations
        .slot_cell(segment_id, page_id, slot_id)
        .with_slot_generation(slot_generation)
}

fn frame_bytes(generation: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::RecordFrame.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
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
