use worth_store_buffer_pool::{
    BufferPoolBudget, DirtyPageBudget, PinnedPageBudget, ResidentFrameLoadRequest,
    ResidentFrameTable, ResidentFrameTableCapacity, ResidentMemoryBudget, S2PhysicalResidencyEntry,
};
use worth_store_physical_format::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalGenerationOwner, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageKind, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceValidationWitness, PhysicalRootManifest,
    PhysicalRootReference, PhysicalSegmentId, SlotGenerationCell,
};
use worth_store_physical_integrity::ProtectedPhysicalByteView;

use super::s4_recovery_readiness_fixture::physical_substrate_readiness;

pub(super) fn with_protected_payload_view(
    payload: &[u8],
    run: impl FnOnce(ProtectedPhysicalByteView<'_>),
) {
    let mut table = resident_frame_table();
    let frame = admit_payload_frame(&mut table, 7, 2, payload);
    let lease = table.lease_page(frame.resident_frame_token()).unwrap();
    let pinned = lease.pin().unwrap();
    let view = pinned.view().unwrap();
    run(ProtectedPhysicalByteView::from_pinned_frame(&view));
}

pub(super) fn page_payload_with_record(payload: &[u8]) -> Vec<u8> {
    let records =
        worth_store_physical_format::PhysicalPageRecordAuthority::for_canonical_physical_format(
            header_authority(),
        );
    let cell = page_cell(1, 2, 7);
    let empty = page_bytes(cell, &[]);
    let header = records
        .decode_record_page_header(cell, &empty, PhysicalPageKind::DataPage)
        .unwrap();
    let admitted = records
        .admit_record_page_payload(&empty, header.witness())
        .unwrap();
    records
        .append_record(
            admitted,
            worth_store_physical_format::SlotAppendRequest::ordinary(
                slot_cell(1, 2, 3, 7),
                payload,
            ),
        )
        .unwrap()
        .page_payload()
        .to_vec()
}

pub(super) fn root_with_slot(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalRootManifest {
    worth_store_physical_format::PhysicalManifestUniverseBuilder::for_canonical_physical_format(
        root_publication(99),
    )
    .segment(segment_cell(segment))
    .ordinary_page(slot_cell(segment, page, slot, generation))
    .publish()
}

pub(super) fn validation(
    segment: u64,
    page: u64,
    slot: u64,
    generation: u64,
) -> PhysicalReferenceValidationWitness {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = slot_cell(segment, page, slot, generation);
    references
        .validate_page_slot(references.admit_page_slot(cell), cell)
        .unwrap()
}

pub(super) fn page_witness(
    payload: &[u8],
    cell: PageGenerationCell,
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_page_header(cell, &page_bytes(cell, payload), PhysicalPageKind::DataPage)
        .unwrap()
        .witness()
}

pub(super) fn frame_witness(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
) -> PhysicalHeaderDecodeWitness {
    header_authority()
        .decode_frame_header(
            validation,
            &frame_bytes(slot_cell_for_owner(validation.owner()), payload),
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap()
        .witness()
}

pub(super) fn page_cell(segment: u64, page: u64, generation: u64) -> PageGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment_id(segment), page_id(page))
        .with_page_generation(physical_generation(generation))
}

pub(super) fn slot_cell(segment: u64, page: u64, slot: u64, generation: u64) -> SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(segment_id(segment), page_id(page), record_slot(slot))
        .with_slot_generation(physical_generation(generation))
}

fn resident_frame_table() -> ResidentFrameTable {
    let admitted = S2PhysicalResidencyEntry::from_physical_substrate_snapshot(
        physical_substrate_readiness().physical_substrate_snapshot(),
    )
    .unwrap()
    .with_budget(BufferPoolBudget::declare(
        ResidentMemoryBudget::bytes(8192).unwrap(),
        PinnedPageBudget::pages(4).unwrap(),
        DirtyPageBudget::pages(1).unwrap(),
    ))
    .admit()
    .unwrap();
    ResidentFrameTable::open(admitted, ResidentFrameTableCapacity::frames(1).unwrap())
}

fn admit_payload_frame(
    table: &mut ResidentFrameTable,
    generation: u64,
    page: u64,
    payload: &[u8],
) -> worth_store_buffer_pool::ResidentFrameAdmission {
    let frame = frame_bytes(slot_cell(1, page, 3, generation), payload);
    let request = ResidentFrameLoadRequest::from_physical_format_physical_frame(
        validation(1, page, 3, generation),
        frame_witness(payload, validation(1, page, 3, generation)),
    )
    .unwrap();
    let view = header_authority()
        .payload_view(&frame, request.header())
        .unwrap();
    table.admit_resident_frame_bytes(request, view).unwrap()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
    )
}

fn segment_cell(segment: u64) -> worth_store_physical_format::SegmentGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .segment_cell(segment_id(segment))
        .with_segment_generation(physical_generation(1))
}

fn root_publication(root: u64) -> worth_store_physical_format::RootPublicationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(PhysicalRootReference::from_raw(root).unwrap())
        .with_root_publication_generation(physical_generation(1))
}

fn frame_bytes(cell: SlotGenerationCell, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(
        usize::from(worth_store_physical_format::PHYSICAL_HEADER_LENGTH) + payload.len(),
    );
    bytes.extend_from_slice(&header_authority().encode_record_frame_header(
        cell,
        u32::try_from(payload.len()).expect("test payload length should fit the physical format"),
    ));
    bytes.extend_from_slice(payload);
    bytes
}

fn page_bytes(cell: PageGenerationCell, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(
        usize::from(worth_store_physical_format::PHYSICAL_HEADER_LENGTH) + payload.len(),
    );
    bytes.extend_from_slice(&header_authority().encode_page_header(
        cell,
        PhysicalPageKind::DataPage,
        u32::try_from(payload.len()).expect("test payload length should fit the physical format"),
    ));
    bytes.extend_from_slice(payload);
    bytes
}

fn slot_cell_for_owner(owner: PhysicalGenerationOwner) -> SlotGenerationCell {
    slot_cell(
        owner.segment_id().expect("slot owner segment").get(),
        owner.page_id().expect("slot owner page").get(),
        u64::from(owner.slot().expect("slot owner slot").get()),
        owner.generation().get(),
    )
}

fn segment_id(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page_id(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn record_slot(value: u64) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value as u16).unwrap()
}

fn physical_generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
