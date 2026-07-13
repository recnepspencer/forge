use crate::{
    AllocationClassKind, FreeSpaceManifestEntry, OfflineManifestCodec, OfflinePhysicalVerifier,
    OfflineVerifierDenialKind, PersistedExtentBytes, PersistedPageBytes, PersistedPhysicalLayout,
    PhysicalBinaryEncodingWitness, PhysicalByteOrder, PhysicalExtentId, PhysicalFrameKind,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId,
    PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalRootReference, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

#[test]
fn minimal_offline_verifier_manifest_smoke_walks_persisted_bytes() {
    let fixture = verifier_fixture();

    let report = fixture.verifier.verify(&fixture.layout).unwrap();

    assert_eq!(report.traversal().root_count(), 1);
    assert_eq!(report.traversal().segment_count(), 1);
    assert_eq!(report.traversal().page_slot_count(), 1);
    assert_eq!(report.traversal().extent_count(), 1);
    assert_eq!(report.traversal().free_space_count(), 1);
    assert_eq!(report.layout().discovered_references().len(), 4);
    assert_eq!(report.semantic_decode_attempts(), 0);
    assert_eq!(report.counters().root_candidates_inspected(), 1);
    assert_eq!(report.counters().header_decodes(), 2);
    assert_eq!(report.counters().slot_directory_entries(), 1);
    assert_eq!(report.counters().extent_membership_checks(), 1);
    assert_eq!(report.counters().free_space_entries_checked(), 1);
}

#[test]
fn missing_and_ambiguous_roots_deny_before_manifest_decode() {
    let fixture = verifier_fixture();
    let missing = PersistedPhysicalLayout::builder()
        .segment_manifest(fixture.segment_manifest)
        .extent_manifest(fixture.extent_manifest)
        .free_space_map(fixture.free_space_map)
        .build();

    let denial = fixture.verifier.verify(&missing).unwrap_err();
    assert_eq!(
        denial.kind(),
        OfflineVerifierDenialKind::MissingRootManifest
    );
    assert_eq!(denial.counters().semantic_decode_attempts(), 0);

    let ambiguous = PersistedPhysicalLayout::builder()
        .root_manifest(fixture.root_manifest.clone())
        .root_manifest(fixture.root_manifest)
        .build();

    let denial = fixture.verifier.verify(&ambiguous).unwrap_err();
    assert_eq!(
        denial.kind(),
        OfflineVerifierDenialKind::AmbiguousRootManifest
    );
    assert_eq!(denial.counters().root_candidates_inspected(), 2);
}

#[test]
fn backend_residue_denies_before_page_header_walk() {
    let mut fixture = verifier_fixture();
    let residue = PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_extent(fixture.extent_cell);
    fixture.layout = PersistedPhysicalLayout::builder()
        .root_manifest(fixture.root_manifest)
        .segment_manifest(fixture.segment_manifest)
        .extent_manifest(fixture.extent_manifest)
        .free_space_map(fixture.free_space_map)
        .backend_residue_reference(residue)
        .build();

    let denial = fixture.verifier.verify(&fixture.layout).unwrap_err();

    assert_eq!(
        denial.kind(),
        OfflineVerifierDenialKind::BackendResidueDiscoverySource
    );
    assert_eq!(denial.counters().backend_residue_rejections(), 1);
    assert_eq!(denial.counters().header_decodes(), 0);
    assert_eq!(denial.counters().semantic_decode_attempts(), 0);
}

#[test]
fn malformed_membership_denies_before_header_decode() {
    let mut fixture = verifier_fixture();
    let bad_segment = PhysicalSegmentId::from_raw(99).unwrap();
    let bad_slot = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(bad_segment, fixture.page_id, fixture.slot)
        .with_slot_generation(fixture.generation);
    let bad_segment_manifest = OfflineManifestCodec::encode_segment_manifest(
        fixture.byte_order,
        fixture.root.segments(),
        &[crate::SegmentPageManifestEntry::new(bad_slot)],
    );
    fixture.layout = PersistedPhysicalLayout::builder()
        .root_manifest(fixture.root_manifest)
        .segment_manifest(bad_segment_manifest)
        .extent_manifest(fixture.extent_manifest)
        .free_space_map(fixture.free_space_map)
        .build();

    let denial = fixture.verifier.verify(&fixture.layout).unwrap_err();

    assert_eq!(
        denial.kind(),
        OfflineVerifierDenialKind::MalformedManifestMembership
    );
    assert_eq!(denial.counters().header_decodes(), 0);
    assert_eq!(denial.counters().semantic_decode_attempts(), 0);
}

#[test]
fn malformed_manifest_sections_deny_before_header_decode() {
    let fixture = verifier_fixture();

    assert_malformed_section_denies_before_header_decode(
        PersistedPhysicalLayout::builder()
            .root_manifest(malformed_section(fixture.root_manifest.clone()))
            .segment_manifest(fixture.segment_manifest.clone())
            .extent_manifest(fixture.extent_manifest.clone())
            .free_space_map(fixture.free_space_map.clone())
            .build(),
        OfflineVerifierDenialKind::MalformedRootManifest,
    );
    assert_malformed_section_denies_before_header_decode(
        PersistedPhysicalLayout::builder()
            .root_manifest(fixture.root_manifest.clone())
            .segment_manifest(malformed_section(fixture.segment_manifest.clone()))
            .extent_manifest(fixture.extent_manifest.clone())
            .free_space_map(fixture.free_space_map.clone())
            .build(),
        OfflineVerifierDenialKind::MalformedSegmentManifest,
    );
    assert_malformed_section_denies_before_header_decode(
        PersistedPhysicalLayout::builder()
            .root_manifest(fixture.root_manifest.clone())
            .segment_manifest(fixture.segment_manifest.clone())
            .extent_manifest(malformed_section(fixture.extent_manifest.clone()))
            .free_space_map(fixture.free_space_map.clone())
            .build(),
        OfflineVerifierDenialKind::MalformedExtentManifest,
    );
    assert_malformed_section_denies_before_header_decode(
        PersistedPhysicalLayout::builder()
            .root_manifest(fixture.root_manifest)
            .segment_manifest(fixture.segment_manifest)
            .extent_manifest(fixture.extent_manifest)
            .free_space_map(malformed_section(fixture.free_space_map))
            .build(),
        OfflineVerifierDenialKind::MalformedFreeSpaceMap,
    );
}

#[test]
fn bad_page_header_denies_before_semantic_decode() {
    let mut fixture = verifier_fixture();
    let mut page_bytes = fixture.page_bytes.clone();
    page_bytes[0] = 0xff;
    fixture.layout = PersistedPhysicalLayout::builder()
        .root_manifest(fixture.root_manifest)
        .segment_manifest(fixture.segment_manifest)
        .extent_manifest(fixture.extent_manifest)
        .free_space_map(fixture.free_space_map)
        .page(PersistedPageBytes::new(fixture.page_cell, page_bytes))
        .extent(PersistedExtentBytes::new(
            fixture.extent_cell,
            fixture.extent_bytes,
        ))
        .build();

    let denial = fixture.verifier.verify(&fixture.layout).unwrap_err();

    assert_eq!(denial.kind(), OfflineVerifierDenialKind::HeaderDecodeDenied);
    assert!(denial.header_denial().is_some());
    assert_eq!(denial.counters().semantic_decode_attempts(), 0);
}

#[test]
fn bad_extent_header_denies_before_semantic_decode() {
    let mut fixture = verifier_fixture();
    let mut extent_bytes = fixture.extent_bytes.clone();
    extent_bytes[0] = 0xff;
    fixture.layout = PersistedPhysicalLayout::builder()
        .root_manifest(fixture.root_manifest)
        .segment_manifest(fixture.segment_manifest)
        .extent_manifest(fixture.extent_manifest)
        .free_space_map(fixture.free_space_map)
        .page(PersistedPageBytes::new(
            fixture.page_cell,
            fixture.page_bytes,
        ))
        .extent(PersistedExtentBytes::new(fixture.extent_cell, extent_bytes))
        .build();

    let denial = fixture.verifier.verify(&fixture.layout).unwrap_err();

    assert_eq!(denial.kind(), OfflineVerifierDenialKind::ExtentRecordDenied);
    assert!(denial
        .extent_denial()
        .and_then(|extent_denial| extent_denial.header_denial())
        .is_some());
    assert_eq!(denial.counters().semantic_decode_attempts(), 0);
}

fn assert_malformed_section_denies_before_header_decode(
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

fn malformed_section(mut bytes: Vec<u8>) -> Vec<u8> {
    bytes[0] = 0xff;
    bytes
}

struct VerifierFixture {
    verifier: OfflinePhysicalVerifier,
    layout: PersistedPhysicalLayout,
    root: crate::PhysicalRootManifest,
    root_manifest: Vec<u8>,
    segment_manifest: Vec<u8>,
    extent_manifest: Vec<u8>,
    free_space_map: Vec<u8>,
    page_bytes: Vec<u8>,
    extent_bytes: Vec<u8>,
    page_cell: crate::PageGenerationCell,
    extent_cell: crate::ExtentGenerationCell,
    page_id: PhysicalPageId,
    slot: PhysicalRecordSlot,
    generation: PhysicalGeneration,
    byte_order: PhysicalByteOrder,
}

fn verifier_fixture() -> VerifierFixture {
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
    let extent_bytes = extent_record_bytes(byte_order, generation, b"large");
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

fn record_page_bytes(
    headers: PhysicalHeaderAuthority,
    byte_order: PhysicalByteOrder,
    page_cell: crate::PageGenerationCell,
    slot_cell: crate::SlotGenerationCell,
) -> Vec<u8> {
    let authority = PhysicalPageRecordAuthority::for_canonical_physical_format(headers);
    let empty_page = page_bytes(byte_order, page_cell.generation(), &[]);
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
    page_bytes(byte_order, page_cell.generation(), append.page_payload())
}

fn page_bytes(
    byte_order: PhysicalByteOrder,
    generation: PhysicalGeneration,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = header_bytes(
        byte_order,
        PhysicalPageKind::DataPage.tag(),
        generation,
        payload.len(),
    );
    bytes.extend_from_slice(payload);
    bytes
}

fn extent_record_bytes(
    byte_order: PhysicalByteOrder,
    generation: PhysicalGeneration,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = header_bytes(
        byte_order,
        PhysicalFrameKind::ExtentRecordFrame.tag(),
        generation,
        payload.len(),
    );
    bytes.extend_from_slice(payload);
    bytes
}

fn header_bytes(
    byte_order: PhysicalByteOrder,
    tag: u8,
    generation: PhysicalGeneration,
    payload_len: usize,
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload_len);
    bytes.push(tag);
    bytes.extend_from_slice(
        &byte_order.write_u16(crate::PhysicalFormatVersion::initial_format_version().value()),
    );
    bytes.extend_from_slice(&byte_order.write_u16(PHYSICAL_HEADER_LENGTH));
    bytes.extend_from_slice(&byte_order.write_u32(payload_len as u32));
    bytes.extend_from_slice(&byte_order.write_u64(generation.get()));
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&byte_order.write_u32(0));
    bytes.extend_from_slice(&byte_order.write_u64(0));
    bytes
}
