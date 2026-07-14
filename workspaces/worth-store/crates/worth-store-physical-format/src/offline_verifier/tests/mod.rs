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

mod support;
use support::*;
