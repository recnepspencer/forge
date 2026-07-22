use worth_store_buffer_pool::{
    AllocationAdmission, AllocationByteBudget, AllocationEnvelopeDeclaration, BufferPoolBudget,
    DirtyPageBudget, PinnedPageBudget, ResidentFrameLoadRequest, ResidentFrameTable,
    ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use worth_store_contracts::PhysicalSubstrateReadinessSnapshot;
use worth_store_physical_format::{
    FramedRecordView, PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageRecordAuthority, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceValidationWitness, PhysicalSegmentId,
    SlotAppendRequest, SlotGenerationCell,
};

pub(crate) fn record_view_table_without_conflicts() -> ResidentFrameTable {
    resident_frame_table()
}

pub(crate) fn resident_frame_table() -> ResidentFrameTable {
    let budget = BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(1).unwrap(),
    );
    let admitted =
        S2PhysicalResidencyEntry::from_physical_substrate_snapshot(algorithm_model_snapshot())
            .unwrap()
            .with_budget(budget)
            .admit()
            .unwrap();
    ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(1).unwrap()).unwrap()
}

fn algorithm_model_snapshot() -> PhysicalSubstrateReadinessSnapshot {
    PhysicalSubstrateReadinessSnapshot::from_exact_counts(true, 4, 2, 2, 3, 1, 9)
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
            worth_store_buffer_pool::FixedMetadataReservation::constant_bytes(1).unwrap(),
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
) -> worth_store_buffer_pool::ResidentFrameAdmission {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let frame = crate::physical_fixture_encoding::record_frame_bytes(
        slot_cell(&generations, generation_value, page_value),
        payload,
    );
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
    let empty_page = crate::physical_fixture_encoding::data_page_bytes(page_cell, &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, payload),
        )
        .unwrap();
    let reopened_page =
        crate::physical_fixture_encoding::data_page_bytes(page_cell, append.page_payload());
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
) -> worth_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(
            cell,
            bytes,
            worth_store_physical_format::PhysicalPageKind::DataPage,
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
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
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
