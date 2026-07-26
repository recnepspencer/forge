use worth_store_buffer_pool::{
    PhysicalFrameAccess, PhysicalFrameKey, PhysicalOperationAllocationScope,
    PhysicalResidencyLimits, PhysicalResidencyPool, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::{
    store_namespace::{
        ProposedStoreIdentity, StableStoreIdentity, StoreNamespaceIdentityRecord,
        StoreNamespaceVersion,
    },
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalGenerationOwner, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageKind, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceValidationWitness, PhysicalRootManifest,
    PhysicalRootReference, PhysicalSegmentId, RecordArtifactFile, RecordFrameCoordinate,
    SlotGenerationCell,
};
use worth_store_physical_integrity::ProtectedPhysicalByteView;

pub(super) fn with_protected_page_view(
    payload: &[u8],
    cell: PageGenerationCell,
    run: impl FnOnce(ProtectedPhysicalByteView<'_>, PhysicalHeaderDecodeWitness),
) {
    let page = page_bytes(cell, payload);
    let witness = header_authority()
        .decode_page_header(cell, &page, PhysicalPageKind::DataPage)
        .unwrap()
        .witness();
    with_protected_physical_bytes(&page, cell.owner().page_id().unwrap().get(), |protected| {
        run(protected, witness);
    });
}

pub(super) fn with_protected_frame_view(
    payload: &[u8],
    validation: PhysicalReferenceValidationWitness,
    run: impl FnOnce(ProtectedPhysicalByteView<'_>, PhysicalHeaderDecodeWitness),
) {
    let frame = frame_bytes(slot_cell_for_owner(validation.owner()), payload);
    let witness = header_authority()
        .decode_frame_header(validation, &frame, PhysicalFrameKind::RecordFrame)
        .unwrap()
        .witness();
    with_protected_physical_bytes(
        &frame,
        validation.owner().page_id().unwrap().get(),
        |protected| run(protected, witness),
    );
}

fn with_protected_physical_bytes(
    bytes: &[u8],
    page: u64,
    run: impl FnOnce(ProtectedPhysicalByteView<'_>),
) {
    let store = physical_residency_store();
    let pool = PhysicalResidencyPool::open(store, physical_residency_limits()).unwrap();
    let allocation = pool
        .begin_operation(
            PhysicalOperationAllocationScope::Recovery,
            std::num::NonZeroU64::MIN,
        )
        .unwrap();
    let key = PhysicalFrameKey::new(store, frame_coordinate(page, bytes.len()));
    let PhysicalFrameAccess::Fault(fault) = pool.access_frame(&allocation, key).unwrap() else {
        panic!("a fresh recovery fixture pool must issue the sole frame fault");
    };
    let lease = fault
        .load(|target| {
            target.copy_from_slice(bytes);
            Ok::<_, std::convert::Infallible>(())
        })
        .unwrap();
    run(ProtectedPhysicalByteView::from_physical_frame(&lease));
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

fn physical_residency_store() -> StableStoreIdentity {
    StoreNamespaceIdentityRecord::new(
        StoreNamespaceVersion::CURRENT,
        ProposedStoreIdentity::from_nonzero_bytes([0x52; 16]).unwrap(),
    )
    .published_identity()
}

fn physical_residency_limits() -> PhysicalResidencyLimits {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Speculation;

    PhysicalResidencyLimits::builder()
        .total_bytes(nonzero_bytes(25_088))
        .resident_bytes(nonzero_bytes(8192))
        .metadata_bytes(nonzero_bytes(8192))
        .frame_entries(nonzero_count(4))
        .pinned_frames(nonzero_count(4))
        .pin_leases(nonzero_count(4))
        .dirty_frames(nonzero_count(1))
        .dirty_replacement_bytes(nonzero_bytes(8192))
        .operation_bytes(nonzero_bytes(512))
        .scope_bytes(Scope::ForegroundRead, nonzero_bytes(512))
        .scope_bytes(Scope::ForegroundWrite, nonzero_bytes(512))
        .scope_bytes(Scope::Recovery, nonzero_bytes(512))
        .scope_bytes(Scope::Scrub, nonzero_bytes(512))
        .scope_bytes(Scope::Maintenance, nonzero_bytes(512))
        .scope_bytes(Scope::Verification, nonzero_bytes(512))
        .scope_bytes(Scope::Blob, nonzero_bytes(512))
        .speculative_frames(Speculation::Prefetch, nonzero_count(4))
        .speculative_frames(Speculation::ReadAhead, nonzero_count(4))
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

fn frame_coordinate(page: u64, frame_bytes: usize) -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(
        RecordArtifactFile::RootRoutingBlock {
            generation: 7,
            block: page,
        },
        0,
        u32::try_from(frame_bytes).expect("fixture frame length fits the physical coordinate"),
    )
    .unwrap()
}
