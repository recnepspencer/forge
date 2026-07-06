use crate::{
    AllocationClassKind, PhysicalCellReuseDomain, PhysicalExtentId, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalReferenceDenialKind, PhysicalRootReference, PhysicalSegmentId, PhysicalVocabularyError,
};

#[test]
fn page_slot_reference_validates_against_slot_generation() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();

    let cell = generations
        .slot_cell(segment(7), page(11), slot(3))
        .with_slot_generation(generation(9));

    let admitted = references.admit_page_slot(cell);
    let validated = references.validate_page_slot(admitted, cell).unwrap();

    assert_eq!(validated.reference().segment_id(), Some(segment(7)));
    assert_eq!(validated.reference().page_id(), Some(page(11)));
    assert_eq!(validated.reference().slot(), Some(slot(3)));
    assert_eq!(validated.reference().generation(), generation(9));
    assert_eq!(validated.owner(), cell.owner());
    assert_eq!(validated.counters().validation_attempt_count(), 1);
    assert_eq!(validated.counters().page_slot_validation_count(), 1);
    assert_eq!(validated.counters().segment_id_check_count(), 1);
    assert_eq!(validated.counters().page_id_check_count(), 1);
    assert_eq!(validated.counters().slot_check_count(), 1);
    assert_eq!(validated.counters().generation_check_count(), 1);
    assert_eq!(validated.counters().stale_generation_rejection_count(), 0);
}

#[test]
fn generation_owner_domains_are_distinct_for_each_physical_cell_family() {
    let generations = PhysicalGenerationAuthority::s1();

    let slot_owner = generations
        .slot_cell(segment(7), page(11), slot(3))
        .with_slot_generation(generation(1))
        .owner();
    let extent_owner = generations
        .extent_cell(segment(7), extent(44))
        .with_extent_generation(generation(1))
        .owner();
    let free_space_owner = generations
        .free_space_slot_cell(
            segment(7),
            page(11),
            slot(3),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .expect("ordinary record page is valid for free-space slot reuse")
        .with_free_space_generation(generation(1))
        .owner();
    let root_owner = generations
        .root_publication_cell(root_reference(6))
        .with_root_publication_generation(generation(1))
        .owner();
    let page_owner = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(1))
        .owner();
    let segment_owner = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1))
        .owner();

    assert_eq!(slot_owner.domain(), PhysicalCellReuseDomain::SlotAllocation);
    assert_eq!(
        extent_owner.domain(),
        PhysicalCellReuseDomain::ExtentAllocation
    );
    assert_eq!(
        free_space_owner.domain(),
        PhysicalCellReuseDomain::FreeSpaceReuse
    );
    assert_eq!(
        root_owner.domain(),
        PhysicalCellReuseDomain::RootPublication
    );
    assert_eq!(page_owner.domain(), PhysicalCellReuseDomain::Page);
    assert_eq!(segment_owner.domain(), PhysicalCellReuseDomain::Segment);
    assert_ne!(slot_owner, page_owner);
    assert_ne!(extent_owner, segment_owner);
}

#[test]
fn admitted_reference_carries_same_generation_owner_as_source_cell() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let cell = generations
        .free_space_slot_cell(
            segment(7),
            page(11),
            slot(3),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .expect("ordinary record page is valid for free-space slot reuse")
        .with_free_space_generation(generation(4));

    let admitted = references.admit_free_space_reuse(cell);

    assert_eq!(admitted.owner(), cell.owner());
    assert_eq!(
        admitted.owner().domain(),
        PhysicalCellReuseDomain::FreeSpaceReuse
    );
    assert_eq!(
        admitted.owner().allocation_class(),
        Some(AllocationClassKind::OrdinaryRecordPage)
    );
}

#[test]
fn stale_slot_generation_denies_before_decode() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();

    let admitted = references.admit_page_slot(
        generations
            .slot_cell(segment(7), page(11), slot(3))
            .with_slot_generation(generation(9)),
    );
    let reused_cell = generations
        .slot_cell(segment(7), page(11), slot(3))
        .with_slot_generation(generation(10));

    let denial = references
        .validate_page_slot(admitted, reused_cell)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalReferenceDenialKind::StaleSlotGeneration
    );
    assert!(denial.stale_reference().is_some());
    assert_eq!(denial.counters().page_slot_validation_count(), 1);
    assert_eq!(denial.counters().generation_check_count(), 1);
    assert_eq!(denial.counters().stale_generation_rejection_count(), 1);
}

#[test]
fn stale_extent_generation_denies_before_decode() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();

    let admitted = references.admit_extent(
        generations
            .extent_cell(segment(7), extent(44))
            .with_extent_generation(generation(2)),
    );
    let reused_cell = generations
        .extent_cell(segment(7), extent(44))
        .with_extent_generation(generation(3));

    let denial = references
        .validate_extent(admitted, reused_cell)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalReferenceDenialKind::StaleExtentGeneration
    );
    assert!(denial.stale_reference().is_some());
    assert_eq!(denial.counters().extent_validation_count(), 1);
    assert_eq!(denial.counters().extent_id_check_count(), 1);
    assert_eq!(denial.counters().generation_check_count(), 1);
    assert_eq!(denial.counters().stale_generation_rejection_count(), 1);
}

#[test]
fn stale_free_space_reuse_generation_denies_before_decode() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();

    let admitted = references.admit_free_space_reuse(
        generations
            .free_space_slot_cell(
                segment(7),
                page(11),
                slot(3),
                AllocationClassKind::OrdinaryRecordPage,
            )
            .expect("ordinary record page is valid for free-space slot reuse")
            .with_free_space_generation(generation(4)),
    );
    let reused_cell = generations
        .free_space_slot_cell(
            segment(7),
            page(11),
            slot(3),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .expect("ordinary record page is valid for free-space slot reuse")
        .with_free_space_generation(generation(5));

    let denial = references
        .validate_free_space_reuse(admitted, reused_cell)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalReferenceDenialKind::StaleFreeSpaceReuseGeneration
    );
    assert!(denial.stale_reference().is_some());
    assert_eq!(denial.counters().free_space_reuse_validation_count(), 1);
    assert_eq!(denial.counters().allocation_class_check_count(), 1);
    assert_eq!(denial.counters().generation_check_count(), 1);
    assert_eq!(denial.counters().stale_generation_rejection_count(), 1);
}

#[test]
fn stale_root_publication_generation_denies_before_decode() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();

    let admitted = references.admit_root_publication(
        generations
            .root_publication_cell(root_reference(6))
            .with_root_publication_generation(generation(12)),
    );
    let republished_root = generations
        .root_publication_cell(root_reference(6))
        .with_root_publication_generation(generation(13));

    let denial = references
        .validate_root_publication(admitted, republished_root)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalReferenceDenialKind::StaleRootPublicationGeneration
    );
    assert!(denial.stale_reference().is_some());
    assert_eq!(denial.counters().root_publication_validation_count(), 1);
    assert_eq!(denial.counters().root_reference_check_count(), 1);
    assert_eq!(denial.counters().generation_check_count(), 1);
    assert_eq!(denial.counters().stale_generation_rejection_count(), 1);
}

#[test]
fn placement_mismatch_denies_separately_from_stale_generation() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();

    let admitted = references.admit_page_slot(
        generations
            .slot_cell(segment(7), page(11), slot(3))
            .with_slot_generation(generation(9)),
    );
    let different_slot = generations
        .slot_cell(segment(7), page(11), slot(4))
        .with_slot_generation(generation(9));

    let denial = references
        .validate_page_slot(admitted, different_slot)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalReferenceDenialKind::PlacementMismatch
    );
    assert!(denial.stale_reference().is_none());
    assert_eq!(denial.counters().page_slot_validation_count(), 1);
    assert_eq!(denial.counters().placement_mismatch_rejection_count(), 1);
    assert_eq!(denial.counters().generation_check_count(), 0);
}

#[test]
fn wrong_reference_kind_records_exact_rejection_counter() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();

    let extent_admission = references.admit_extent(
        generations
            .extent_cell(segment(7), extent(44))
            .with_extent_generation(generation(2)),
    );
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(3))
        .with_slot_generation(generation(2));

    let denial = references
        .validate_page_slot(extent_admission, slot_cell)
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalReferenceDenialKind::WrongReferenceKind
    );
    assert!(denial.stale_reference().is_none());
    assert_eq!(denial.counters().page_slot_validation_count(), 1);
    assert_eq!(denial.counters().wrong_kind_rejection_count(), 1);
    assert_eq!(denial.counters().generation_check_count(), 0);
}

#[test]
fn free_space_slot_cell_rejects_extent_allocation_class_before_generation() {
    let generations = PhysicalGenerationAuthority::s1();

    let denial = generations
        .free_space_slot_cell(
            segment(7),
            page(11),
            slot(3),
            AllocationClassKind::LargeRecordExtent,
        )
        .unwrap_err();

    assert_eq!(
        denial,
        PhysicalVocabularyError::InvalidFreeSpaceReuseAllocationClass
    );
}

#[test]
fn free_space_extent_cell_rejects_page_allocation_class_before_generation() {
    let generations = PhysicalGenerationAuthority::s1();

    let denial = generations
        .free_space_extent_cell(
            segment(7),
            extent(44),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap_err();

    assert_eq!(
        denial,
        PhysicalVocabularyError::InvalidFreeSpaceReuseAllocationClass
    );
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("test segment id is non-zero")
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("test page id is non-zero")
}

fn extent(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).expect("test extent id is non-zero")
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("test slot is non-zero")
}

fn root_reference(value: u64) -> PhysicalRootReference {
    PhysicalRootReference::from_raw(value).expect("test root reference is non-zero")
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).expect("test generation is non-zero")
}
