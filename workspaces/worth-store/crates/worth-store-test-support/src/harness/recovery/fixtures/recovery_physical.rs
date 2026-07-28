use std::num::NonZeroU64;

use worth_store_physical_format::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalGenerationOwner, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageKind, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceValidationWitness, PhysicalRootManifest,
    PhysicalRootReference, PhysicalSegmentId, SlotGenerationCell,
};
use worth_store_physical_integrity::{
    IntegrityEntryAdmission, IntegrityEntryRequest, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionSeed, ProtectedPhysicalByteView,
};

use crate::harness::physical_residency::{
    PhysicalResidencyStoreWorld, SUCCESSOR_SCOPE_ALLOCATION_BYTES,
};

pub(super) fn with_protected_page_view(
    payload: &[u8],
    cell: PageGenerationCell,
    run: impl FnOnce(
        PhysicalIntegrityAdmissionSeed<'_, '_>,
        ProtectedPhysicalByteView<'_>,
        PhysicalHeaderDecodeWitness,
    ),
) {
    let page = page_bytes(cell, payload);
    let witness = header_authority()
        .decode_page_header(cell, &page, PhysicalPageKind::DataPage)
        .unwrap()
        .witness();
    with_protected_physical_bytes(&page, |seed, protected| run(seed, protected, witness));
}

pub(super) fn with_protected_frame_view(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
    run: impl FnOnce(
        PhysicalIntegrityAdmissionSeed<'_, '_>,
        ProtectedPhysicalByteView<'_>,
        PhysicalHeaderDecodeWitness,
    ),
) {
    let frame = frame_bytes(slot_cell_for_owner(validation.owner()), payload);
    let witness = header_authority()
        .decode_frame_header(validation, &frame, PhysicalFrameKind::RecordFrame)
        .unwrap()
        .witness();
    with_protected_physical_bytes(&frame, |seed, protected| run(seed, protected, witness));
}

fn with_protected_physical_bytes(
    bytes: &[u8],
    run: impl FnOnce(
        PhysicalIntegrityAdmissionSeed<'_, '_>,
        ProtectedPhysicalByteView<'_>,
    ),
) {
    let world = PhysicalResidencyStoreWorld::initialize("recovery-integrity-entry").unwrap();
    world
        .with_record_chunk(bytes, |serving, chunk| {
            let verification = serving
                .physical_allocations()
                .admit_verification(
                    NonZeroU64::new(SUCCESSOR_SCOPE_ALLOCATION_BYTES)
                        .expect("fixture allocation is nonzero"),
                )
                .expect("real Store verification allocation admits");
            let protected = ProtectedPhysicalByteView::from_store_chunk(&chunk);
            let lease =
                IntegrityEntryAdmission::admit(IntegrityEntryRequest::new(protected, verification))
                    .expect("matching real Store integrity entry admits");
            run(PhysicalIntegrityAdmission::from_entry(lease), protected);
        })
        .unwrap();
    assert!(!world.close().residency().requires_inspection());
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
