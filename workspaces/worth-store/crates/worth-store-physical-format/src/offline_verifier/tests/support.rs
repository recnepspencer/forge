use super::*;

pub(super) fn assert_malformed_section_denies_before_header_decode(
    layout: PersistedPhysicalLayout,
    expected: OfflineVerifierDenialKind,
) {
    let verifier = OfflinePhysicalVerifier::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap(),
        ),
    );

    let denial = verifier.verify(&layout).unwrap_err();

    assert_eq!(denial.kind(), expected);
    assert_eq!(denial.counters().header_decodes(), 0);
    assert_eq!(denial.counters().semantic_decode_attempts(), 0);
}

pub(super) fn malformed_section(mut bytes: Vec<u8>) -> Vec<u8> {
    bytes[0] = 0xff;
    bytes
}

pub(super) struct VerifierFixture {
    pub(super) verifier: OfflinePhysicalVerifier,
    pub(super) layout: PersistedPhysicalLayout,
    pub(super) root: crate::PhysicalRootManifest,
    pub(super) root_manifest: Vec<u8>,
    pub(super) segment_manifest: Vec<u8>,
    pub(super) extent_manifest: Vec<u8>,
    pub(super) free_space_map: Vec<u8>,
    pub(super) page_bytes: Vec<u8>,
    pub(super) extent_bytes: Vec<u8>,
    pub(super) page_cell: crate::PageGenerationCell,
    pub(super) extent_cell: crate::ExtentGenerationCell,
    pub(super) page_id: PhysicalPageId,
    pub(super) slot: PhysicalRecordSlot,
    pub(super) generation: PhysicalGeneration,
    pub(super) byte_order: PhysicalByteOrder,
}

pub(super) fn verifier_fixture() -> VerifierFixture {
    let encoding = PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap();
    let byte_order = encoding.declaration().byte_order();
    let headers = PhysicalHeaderAuthority::for_canonical_physical_format(encoding);
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment_id = PhysicalSegmentId::from_raw(1).unwrap();
    let page_id = PhysicalPageId::from_raw(1).unwrap();
    let extent_id = PhysicalExtentId::from_raw(1).unwrap();
    let slot = PhysicalRecordSlot::from_raw(1).unwrap();
    let generation = PhysicalGeneration::from_raw(7).unwrap();
    let root_reference = PhysicalRootReference::from_raw(1).unwrap();
    let root_cell = generations
        .root_publication_cell(root_reference)
        .with_root_publication_generation(generation);
    let segment_cell = generations
        .segment_cell(segment_id)
        .with_segment_generation(generation);
    let slot_cell = generations
        .slot_cell(segment_id, page_id, slot)
        .with_slot_generation(generation);
    let page_cell = generations
        .page_cell(segment_id, page_id)
        .with_page_generation(generation);
    let extent_cell = generations
        .extent_cell(segment_id, extent_id)
        .with_extent_generation(generation);
    let free_space = generations
        .free_space_slot_cell(
            segment_id,
            page_id,
            slot,
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(generation);
    let root = crate::PhysicalManifestUniverseBuilder::for_canonical_physical_format(root_cell)
        .segment(segment_cell)
        .ordinary_page(slot_cell)
        .extent(extent_cell)
        .free_space_reuse(free_space)
        .publish();
    let root_manifest = OfflineManifestCodec::encode_root_manifest(byte_order, root_cell);
    let segment_manifest = OfflineManifestCodec::encode_segment_manifest(
        byte_order,
        root.segments(),
        root.page_slots(),
    );
    let extent_manifest = OfflineManifestCodec::encode_extent_manifest(
        byte_order,
        root.extents(),
        root.allocation_classes(),
    );
    let free_space_map = OfflineManifestCodec::encode_free_space_map(
        byte_order,
        &[FreeSpaceManifestEntry::new(free_space)],
    );
    let page_bytes = record_page_bytes(headers.clone(), byte_order, page_cell, slot_cell);
    let extent_bytes = extent_record_bytes(byte_order, extent_cell, b"large");
    let layout = PersistedPhysicalLayout::builder()
        .root_manifest(root_manifest.clone())
        .segment_manifest(segment_manifest.clone())
        .extent_manifest(extent_manifest.clone())
        .free_space_map(free_space_map.clone())
        .page(PersistedPageBytes::new(page_cell, page_bytes.clone()))
        .extent(PersistedExtentBytes::new(extent_cell, extent_bytes.clone()))
        .build();
    VerifierFixture {
        verifier: OfflinePhysicalVerifier::for_canonical_physical_format(headers),
        layout,
        root,
        root_manifest,
        segment_manifest,
        extent_manifest,
        free_space_map,
        page_bytes,
        extent_bytes,
        page_cell,
        extent_cell,
        page_id,
        slot,
        generation,
        byte_order,
    }
}

pub(super) fn record_page_bytes(
    headers: PhysicalHeaderAuthority,
    byte_order: PhysicalByteOrder,
    page_cell: crate::PageGenerationCell,
    slot_cell: crate::SlotGenerationCell,
) -> Vec<u8> {
    let authority = PhysicalPageRecordAuthority::for_canonical_physical_format(headers);
    let empty_page = page_bytes(byte_order, page_cell, &[]);
    let header = authority
        .decode_record_page_header(page_cell, &empty_page, PhysicalPageKind::DataPage)
        .unwrap();
    let payload = authority
        .admit_record_page_payload(&empty_page, header.witness())
        .unwrap();
    let append = authority
        .append_record(
            payload,
            crate::SlotAppendRequest::ordinary(slot_cell, b"small"),
        )
        .unwrap();
    page_bytes(byte_order, page_cell, append.page_payload())
}

pub(super) fn page_bytes(
    byte_order: PhysicalByteOrder,
    page_cell: crate::PageGenerationCell,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = crate::header::encode_page_header(
        byte_order,
        PhysicalPageKind::DataPage,
        page_cell,
        payload.len() as u32,
    )
    .to_vec();
    bytes.extend_from_slice(payload);
    bytes
}

pub(super) fn extent_record_bytes(
    byte_order: PhysicalByteOrder,
    extent_cell: crate::ExtentGenerationCell,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes =
        crate::header::encode_extent_frame_header(byte_order, extent_cell, payload.len() as u32)
            .to_vec();
    bytes.extend_from_slice(payload);
    bytes
}
