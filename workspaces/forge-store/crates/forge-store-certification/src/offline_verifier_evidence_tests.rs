use crate::{
    offline_observer_requires_physical_references, OfflineVerifierObserver, PhysicalLayoutParity,
    PhysicalOfflineVerifierEvidenceReport, PhysicalOfflineVerifierEvidenceRow,
    RuntimeLayoutObserver, RuntimeVerifierRelationship,
};
use forge_store_physical_format::{
    AllocationClassKind, FreeSpaceManifestEntry, ManifestDiscoveryAuthority, OfflineManifestCodec,
    OfflinePhysicalVerifier, OfflineVerifierDenialKind, PersistedExtentBytes, PersistedPageBytes,
    PersistedPhysicalLayout, PhysicalBinaryEncodingWitness, PhysicalByteOrder, PhysicalExtentId,
    PhysicalFrameKind, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority, PhysicalPublicationState,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalRootManifest, PhysicalRootReference,
    PhysicalSegmentId, SlotAppendRequest, PHYSICAL_HEADER_LENGTH,
};

#[test]
fn offline_verifier_layout_match_emits_parity_trace() {
    let fixture = FullLayoutFixture::new(1);
    let offline_report = fixture.verifier.verify(&fixture.layout).unwrap();
    let offline = OfflineVerifierObserver::from_report(&offline_report);
    let runtime = RuntimeLayoutObserver::from_manifest_report(fixture.runtime_report());

    offline_observer_requires_physical_references(&offline).unwrap();
    let parity = PhysicalLayoutParity::compare(runtime, offline).unwrap();
    let evidence = PhysicalOfflineVerifierEvidenceReport::from_parity_report(
        PhysicalOfflineVerifierEvidenceRow::RuntimeLayoutMatch,
        parity.clone(),
    )
    .unwrap();

    assert!(parity.matches());
    assert_eq!(
        parity.parity_trace().relationship(),
        RuntimeVerifierRelationship::RuntimeMustMatchVerifier
    );
    assert_eq!(evidence.observed_reference_count(), 4);
}

#[test]
fn minimal_offline_verifier_manifest_smoke_uses_real_report() {
    let fixture = FullLayoutFixture::new(1);
    let offline_report = fixture.verifier.verify(&fixture.layout).unwrap();

    let evidence = PhysicalOfflineVerifierEvidenceReport::from_verifier_report(
        PhysicalOfflineVerifierEvidenceRow::MinimalManifestSmoke,
        &offline_report,
    )
    .unwrap();

    assert_eq!(evidence.observed_reference_count(), 4);
    assert_eq!(evidence.semantic_decode_attempts(), 0);
}

#[test]
fn offline_verifier_runtime_disagreement_reported() {
    let runtime_fixture = FullLayoutFixture::new(1);
    let offline_fixture = FullLayoutFixture::new(2);
    let offline_report = offline_fixture
        .verifier
        .verify(&offline_fixture.layout)
        .unwrap();
    let runtime = RuntimeLayoutObserver::from_manifest_report(runtime_fixture.runtime_report());
    let offline = OfflineVerifierObserver::from_report(&offline_report);

    let denial = PhysicalLayoutParity::compare(runtime, offline).unwrap_err();
    let evidence = PhysicalOfflineVerifierEvidenceReport::from_parity_denial(
        PhysicalOfflineVerifierEvidenceRow::RuntimeDisagreementReported,
        denial,
    )
    .unwrap();

    assert_eq!(evidence.observed_reference_count(), 4);
    assert_eq!(
        denial.parity_trace().relationship(),
        RuntimeVerifierRelationship::RuntimeMustDisagreeWithVerifier
    );
}

#[test]
fn extent_header_denial_certifies_from_real_offline_verifier_denial() {
    let mut fixture = FullLayoutFixture::new(1);
    fixture.extent_bytes[0] = 0xff;
    fixture.layout = fixture.layout_with_current_bytes();

    let denial = fixture.verifier.verify(&fixture.layout).unwrap_err();
    let evidence = PhysicalOfflineVerifierEvidenceReport::from_verifier_denial(
        PhysicalOfflineVerifierEvidenceRow::MinimalManifestSmoke,
        denial.clone(),
    )
    .unwrap();

    assert_eq!(denial.kind(), OfflineVerifierDenialKind::ExtentRecordDenied);
    assert_eq!(evidence.semantic_decode_attempts(), 0);
}

#[test]
fn missing_persisted_extent_denial_does_not_certify_as_header_evidence() {
    let mut fixture = FullLayoutFixture::new(1);
    fixture.layout = fixture.layout_without_extent_bytes();

    let denial = fixture.verifier.verify(&fixture.layout).unwrap_err();
    let evidence = PhysicalOfflineVerifierEvidenceReport::from_verifier_denial(
        PhysicalOfflineVerifierEvidenceRow::MinimalManifestSmoke,
        denial.clone(),
    );

    assert_eq!(
        denial.kind(),
        OfflineVerifierDenialKind::MissingPersistedExtent
    );
    assert!(evidence.is_err());
}

struct FullLayoutFixture {
    verifier: OfflinePhysicalVerifier,
    layout: PersistedPhysicalLayout,
    root: PhysicalRootManifest,
    root_cell: forge_store_physical_format::RootPublicationCell,
    root_manifest: Vec<u8>,
    segment_manifest: Vec<u8>,
    extent_manifest: Vec<u8>,
    free_space_map: Vec<u8>,
    page_bytes: Vec<u8>,
    extent_bytes: Vec<u8>,
    page_cell: forge_store_physical_format::PageGenerationCell,
    extent_cell: forge_store_physical_format::ExtentGenerationCell,
}

impl FullLayoutFixture {
    fn new(root_reference: u64) -> Self {
        let encoding = PhysicalBinaryEncodingWitness::s1_canonical().unwrap();
        let byte_order = encoding.declaration().byte_order();
        let headers = PhysicalHeaderAuthority::s1(encoding);
        let generations = PhysicalGenerationAuthority::s1();
        let generation = PhysicalGeneration::from_raw(1).unwrap();
        let root_reference = PhysicalRootReference::from_raw(root_reference).unwrap();
        let segment_id = PhysicalSegmentId::from_raw(1).unwrap();
        let page_id = PhysicalPageId::from_raw(1).unwrap();
        let extent_id = PhysicalExtentId::from_raw(1).unwrap();
        let slot = PhysicalRecordSlot::from_raw(1).unwrap();
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
        let root = forge_store_physical_format::PhysicalManifestUniverseBuilder::s1(root_cell)
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
        Self {
            verifier: OfflinePhysicalVerifier::s1(headers),
            layout: PersistedPhysicalLayout::builder()
                .root_manifest(root_manifest.as_slice())
                .segment_manifest(segment_manifest.as_slice())
                .extent_manifest(extent_manifest.as_slice())
                .free_space_map(free_space_map.as_slice())
                .page(PersistedPageBytes::new(page_cell, page_bytes.as_slice()))
                .extent(PersistedExtentBytes::new(
                    extent_cell,
                    extent_bytes.as_slice(),
                ))
                .build(),
            root,
            root_cell,
            root_manifest,
            segment_manifest,
            extent_manifest,
            free_space_map,
            page_bytes,
            extent_bytes,
            page_cell,
            extent_cell,
        }
    }

    fn layout_with_current_bytes(&self) -> PersistedPhysicalLayout {
        self.layout_builder_with_manifests()
            .page(PersistedPageBytes::new(
                self.page_cell,
                self.page_bytes.as_slice(),
            ))
            .extent(PersistedExtentBytes::new(
                self.extent_cell,
                self.extent_bytes.as_slice(),
            ))
            .build()
    }

    fn layout_without_extent_bytes(&self) -> PersistedPhysicalLayout {
        self.layout_builder_with_manifests()
            .page(PersistedPageBytes::new(
                self.page_cell,
                self.page_bytes.as_slice(),
            ))
            .build()
    }

    fn runtime_report(&self) -> forge_store_physical_format::ManifestDiscoveryReport<'_> {
        ManifestDiscoveryAuthority::s1()
            .reopen_from_root(
                &self.root,
                PhysicalReferenceAuthority::s1().admit_root_publication(self.root_cell),
            )
            .unwrap()
    }

    fn layout_builder_with_manifests(
        &self,
    ) -> forge_store_physical_format::PersistedPhysicalLayoutBuilder {
        PersistedPhysicalLayout::builder()
            .root_manifest(self.root_manifest.as_slice())
            .segment_manifest(self.segment_manifest.as_slice())
            .extent_manifest(self.extent_manifest.as_slice())
            .free_space_map(self.free_space_map.as_slice())
    }
}

fn record_page_bytes(
    headers: PhysicalHeaderAuthority,
    byte_order: PhysicalByteOrder,
    page_cell: forge_store_physical_format::PageGenerationCell,
    slot_cell: forge_store_physical_format::SlotGenerationCell,
) -> Vec<u8> {
    let authority = PhysicalPageRecordAuthority::s1(headers);
    let empty_page = page_bytes(byte_order, page_cell.generation(), &[]);
    let header = authority
        .decode_record_page_header(page_cell, &empty_page, PhysicalPageKind::DataPage)
        .unwrap();
    let payload = authority
        .admit_record_page_payload(&empty_page, header.witness())
        .unwrap();
    let append = authority
        .append_record(payload, SlotAppendRequest::ordinary(slot_cell, b"small"))
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
        &byte_order
            .write_u16(forge_store_physical_format::PhysicalFormatVersion::s1_initial().value()),
    );
    bytes.extend_from_slice(&byte_order.write_u16(PHYSICAL_HEADER_LENGTH));
    bytes.extend_from_slice(&byte_order.write_u32(payload_len as u32));
    bytes.extend_from_slice(&byte_order.write_u64(generation.get()));
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&byte_order.write_u32(0));
    bytes.extend_from_slice(&byte_order.write_u64(0));
    bytes
}
