use forge_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration, BufferPoolBudget,
    DirtyPageBudget, PinnedPageBudget, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::{
    FramedRecordView, PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageRecordAuthority,
    PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceValidationWitness, PhysicalSegmentId, SlotAppendRequest, SlotGenerationCell,
    PHYSICAL_HEADER_LENGTH,
};
use forge_store_readiness::{
    close_physical_substrate_readiness, prove_physical_substrate_readiness,
};

pub(crate) fn record_view_table_without_conflicts() -> ResidentFrameTable {
    resident_frame_table()
}

pub(crate) fn resident_frame_table() -> ResidentFrameTable {
    let readiness = prove_physical_substrate_readiness(
        close_physical_substrate_readiness(
            AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
                ROADMAP_2_S1_SCOPE,
                HandoffEvidenceDigestSet::new(
                    StableDigest::new("sha256:backend").unwrap(),
                    StableDigest::new("sha256:deferred").unwrap(),
                    StableDigest::new("sha256:harness").unwrap(),
                    StableDigest::new("sha256:terms").unwrap(),
                    StableDigest::new("sha256:audit").unwrap(),
                    StableDigest::new("sha256:complexity").unwrap(),
                    StableDigest::new("sha256:provenance").unwrap(),
                ),
            )
            .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
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

pub(crate) fn allocation_admission(bytes: u64) -> AllocationAdmission {
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

pub(crate) fn admit_payload_frame(
    table: &mut ResidentFrameTable,
    generation_value: u64,
    page_value: u64,
    payload: &[u8],
) -> forge_store_buffer_pool::ResidentFrameAdmission {
    let frame = record_frame_bytes(generation_value, payload);
    let request = load_request_from_frame(generation_value, page_value, &frame);
    let payload = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, payload).unwrap()
}

fn load_request_from_frame(
    generation_value: u64,
    page_value: u64,
    frame: &[u8],
) -> ResidentFrameLoadRequest {
    ResidentFrameLoadRequest::from_physical_format_physical_frame(
        validated_slot_reference(generation_value, page_value),
        frame_header_witness(generation_value, page_value, frame),
    )
    .unwrap()
}

pub(crate) fn framed_record(
    generation_value: u64,
    page_value: u64,
    payload: &[u8],
) -> FramedRecordView<'static> {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = page_cell(&generations, 5, page_value);
    let slot_cell = slot_cell(&generations, generation_value, page_value);
    let empty_page = page_bytes(generation(5), &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, payload),
        )
        .unwrap();
    let reopened_page = page_bytes(generation(5), append.page_payload());
    let reopened_page = Box::leak(reopened_page.into_boxed_slice());
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
        .decode_record_page_header(
            cell,
            bytes,
            forge_store_physical_format::PhysicalPageKind::DataPage,
        )
        .unwrap();
    records
        .admit_record_page_payload(bytes, header.witness())
        .unwrap()
}

fn validated_slot_reference(
    generation_value: u64,
    page_value: u64,
) -> PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = slot_cell(&generations, generation_value, page_value);
    references
        .validate_page_slot(references.admit_page_slot(cell), cell)
        .unwrap()
}

fn frame_header_witness(
    generation_value: u64,
    page_value: u64,
    frame: &[u8],
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validated_slot_reference(generation_value, page_value),
            frame,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::for_canonical_physical_format(header_authority())
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap())
}

fn page_cell(
    generations: &PhysicalGenerationAuthority,
    page_generation: u64,
    page_value: u64,
) -> PageGenerationCell {
    generations
        .page_cell(segment(1), page(page_value))
        .with_page_generation(generation(page_generation))
}

fn slot_cell(
    generations: &PhysicalGenerationAuthority,
    slot_generation: u64,
    page_value: u64,
) -> SlotGenerationCell {
    generations
        .slot_cell(segment(1), page(page_value), slot(3))
        .with_slot_generation(generation(slot_generation))
}

fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(forge_store_physical_format::PhysicalPageKind::DataPage.tag());
    write_physical_header_tail(&mut bytes, generation, payload);
    bytes
}

fn record_frame_bytes(generation_value: u64, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalFrameKind::RecordFrame.tag());
    write_physical_header_tail(&mut bytes, generation(generation_value), payload);
    bytes
}

fn write_physical_header_tail(bytes: &mut Vec<u8>, generation: PhysicalGeneration, payload: &[u8]) {
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
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
