use crate::{
    AllocationClassKind, ManifestDiscoveryAuthority, ManifestDiscoveryDenialKind, PhysicalExtentId,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalReferenceDenialKind, PhysicalRootReference,
    PhysicalSegmentId,
};

#[test]
fn root_manifest_reopen_discovers_manifested_physical_universe() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let segment_cell = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1));
    let page_slot = generations
        .slot_cell(segment(7), page(3), slot(1))
        .with_slot_generation(generation(2));
    let extent_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(3));
    let free_space = generations
        .free_space_slot_cell(
            segment(7),
            page(3),
            slot(1),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(generation(4));
    let root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let manifest = crate::PhysicalManifestUniverseBuilder::s1(root)
        .segment(segment_cell)
        .ordinary_page(page_slot)
        .extent(extent_cell)
        .free_space_reuse(free_space)
        .publish();

    let report = discovery
        .reopen_from_root(&manifest, references.admit_root_publication(root))
        .unwrap();
    let page_validation = discovery
        .locate_page_slot(report, references.admit_page_slot(page_slot))
        .unwrap();
    let extent_validation = discovery
        .locate_extent(report, references.admit_extent(extent_cell))
        .unwrap();
    let free_validation = discovery
        .validate_free_space_reuse(report, references.admit_free_space_reuse(free_space))
        .unwrap();

    assert_eq!(
        page_validation.reference(),
        references.admit_page_slot(page_slot).reference()
    );
    assert_eq!(
        extent_validation.reference(),
        references.admit_extent(extent_cell).reference()
    );
    assert_eq!(
        free_validation.reference(),
        references.admit_free_space_reuse(free_space).reference()
    );
    assert_eq!(report.counters().root_manifest_read_count(), 1);
    assert_eq!(report.counters().segment_manifest_read_count(), 1);
    assert_eq!(report.counters().segment_manifest_entry_count(), 1);
    assert_eq!(report.counters().extent_manifest_entry_count(), 1);
    assert_eq!(report.counters().free_space_map_entry_count(), 1);
    assert_eq!(manifest.publish_counters().root_manifest_publish_count(), 1);
}

#[test]
fn allocation_class_manifest_rows_are_physical_placement_rows() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let manifest = crate::PhysicalManifestUniverseBuilder::s1(root)
        .allocation_class(AllocationClassKind::RootManifest)
        .allocation_class(AllocationClassKind::SegmentManifest)
        .allocation_class(AllocationClassKind::SegmentManifest)
        .allocation_class(AllocationClassKind::ExtentManifest)
        .allocation_class(AllocationClassKind::FreeSpaceMap)
        .publish();

    let report = discovery
        .reopen_from_root(&manifest, references.admit_root_publication(root))
        .unwrap();
    let manifest_allocation_classes: Vec<_> = manifest
        .allocation_classes()
        .iter()
        .map(|entry| entry.allocation_class())
        .collect();

    assert_eq!(
        manifest_allocation_classes,
        vec![
            AllocationClassKind::RootManifest,
            AllocationClassKind::SegmentManifest,
            AllocationClassKind::ExtentManifest,
            AllocationClassKind::FreeSpaceMap,
        ]
    );
    assert_eq!(
        manifest.publish_counters().allocation_class_entry_count(),
        4
    );
    assert_eq!(report.counters().allocation_class_entry_count(), 4);
    assert_eq!(report.counters().root_manifest_entry_count(), 4);
}

#[test]
fn backend_residue_outside_manifests_is_not_discovery_authority() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let segment_cell = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1));
    let residue_slot = generations
        .slot_cell(segment(7), page(3), slot(1))
        .with_slot_generation(generation(2));
    let root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let manifest = crate::PhysicalManifestUniverseBuilder::s1(root)
        .segment(segment_cell)
        .publish();
    let report = discovery
        .reopen_from_root(&manifest, references.admit_root_publication(root))
        .unwrap();
    let admission = references.admit_page_slot(residue_slot);

    let denial = discovery.locate_page_slot(report, admission).unwrap_err();
    let residue_denial = discovery.reject_backend_residue(report, admission);

    assert_eq!(
        denial.kind(),
        ManifestDiscoveryDenialKind::MissingPageSlotManifestMembership
    );
    assert_eq!(
        residue_denial.kind(),
        ManifestDiscoveryDenialKind::BackendResidueDiscoverySource
    );
    assert_eq!(
        residue_denial.counters().backend_residue_rejection_count(),
        1
    );
}

#[test]
fn free_space_reuse_generation_change_stales_old_reference() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let segment_cell = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1));
    let old_free_space = generations
        .free_space_slot_cell(
            segment(7),
            page(3),
            slot(1),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(generation(2));
    let current_free_space = generations
        .free_space_slot_cell(
            segment(7),
            page(3),
            slot(1),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(generation(3));
    let root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let manifest = crate::PhysicalManifestUniverseBuilder::s1(root)
        .segment(segment_cell)
        .free_space_reuse(current_free_space)
        .publish();
    let report = discovery
        .reopen_from_root(&manifest, references.admit_root_publication(root))
        .unwrap();

    let denial = discovery
        .validate_free_space_reuse(report, references.admit_free_space_reuse(old_free_space))
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ManifestDiscoveryDenialKind::ReferenceValidationDenied
    );
    assert_eq!(
        denial.reference_denial().unwrap().kind(),
        PhysicalReferenceDenialKind::StaleFreeSpaceReuseGeneration
    );
}

#[test]
fn stale_root_publication_denies_before_manifest_traversal() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let old_root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(4));
    let current_root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let segment_cell = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1));
    let manifest = crate::PhysicalManifestUniverseBuilder::s1(current_root)
        .segment(segment_cell)
        .publish();

    let denial = discovery
        .reopen_from_root(&manifest, references.admit_root_publication(old_root))
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ManifestDiscoveryDenialKind::ReferenceValidationDenied
    );
    assert_eq!(
        denial.reference_denial().unwrap().kind(),
        PhysicalReferenceDenialKind::StaleRootPublicationGeneration
    );
    assert_eq!(denial.counters().root_manifest_read_count(), 1);
    assert_eq!(denial.counters().root_manifest_entry_count(), 0);
    assert_eq!(denial.counters().segment_manifest_entry_count(), 0);
}

#[test]
fn wrong_root_publication_reference_denies_before_manifest_traversal() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let admitted_other_root = generations
        .root_publication_cell(root_ref(2))
        .with_root_publication_generation(generation(5));
    let current_root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let segment_cell = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1));
    let manifest = crate::PhysicalManifestUniverseBuilder::s1(current_root)
        .segment(segment_cell)
        .publish();

    let denial = discovery
        .reopen_from_root(
            &manifest,
            references.admit_root_publication(admitted_other_root),
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ManifestDiscoveryDenialKind::ReferenceValidationDenied
    );
    assert_eq!(
        denial.reference_denial().unwrap().kind(),
        PhysicalReferenceDenialKind::PlacementMismatch
    );
    assert_eq!(denial.counters().root_manifest_read_count(), 1);
    assert_eq!(denial.counters().root_manifest_entry_count(), 0);
    assert_eq!(denial.counters().segment_manifest_entry_count(), 0);
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn extent(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn root_ref(value: u64) -> PhysicalRootReference {
    PhysicalRootReference::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
