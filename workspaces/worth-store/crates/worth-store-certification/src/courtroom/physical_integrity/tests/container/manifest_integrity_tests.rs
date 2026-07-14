use crate::courtroom::harness::test_support::physical_scope_admission_test_support::{
    extent_admission, free_space_slot_admission, page_cell, page_slot_admission, root_admission,
    root_with_extent, root_with_slot, root_with_slot_root_generation, root_with_slot_under_root,
};
use worth_store_physical_format::{
    ManifestMembershipProof, PhysicalGenerationOwner, PhysicalReferenceAuthority,
    PhysicalReferenceKind, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use worth_store_physical_integrity::{
    DerivedManifestOverrideAttempt, ManifestExpectedReference, ManifestIntegrityAuthority,
    ManifestIntegrityDenial, ManifestIntegrityDenialKind, ManifestIntegrityInspectionRequest,
};

#[test]
fn independent_manifest_walks_converge_on_report_counters_and_reference_basis() {
    let root = root_with_slot(1, 2, 3, 7);
    let request = || {
        ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        )
        .with_expected_reference(ManifestExpectedReference::page_slot(
            page_slot_admission(1, 2, 3, 7),
        ))
    };
    let first = ManifestIntegrityAuthority::new()
        .inspect_manifest(request())
        .unwrap();
    let second = ManifestIntegrityAuthority::new()
        .inspect_manifest(request())
        .unwrap();

    assert_eq!(first, second);
    assert_eq!(first.counters().root_manifest_reads(), 1);
    assert_eq!(first.counters().segment_manifest_reads(), 1);
    assert_eq!(first.counters().extent_manifest_reads(), 1);
    assert_eq!(first.counters().allocation_map_reads(), 1);
    assert_eq!(first.counters().free_space_map_reads(), 1);
    assert_eq!(first.counters().manifest_reference_probes(), 1);
    assert_eq!(first.allocation().allocation_entries(), 1);
    assert_eq!(
        first.reference_basis().physical_owners(),
        &[
            root.root_publication().owner(),
            root.segments()[0].segment().owner(),
            root.page_slots()[0].page_slot().owner()
        ]
    );
    assert_eq!(
        first.reference_basis().admitted_scopes(),
        &[PhysicalReferenceScope::manifest_page(page_cell(1, 2, 7))]
    );
}

#[test]
fn manifest_body_denials_are_computed_from_physical_reference_evidence() {
    let root = root_with_slot(1, 2, 3, 7);
    assert_manifest_denial(
        ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        )
        .with_expected_reference(ManifestExpectedReference::page_slot(
            page_slot_admission(1, 2, 3, 6),
        )),
        ManifestIntegrityDenialKind::StaleManifestGeneration,
        Some(
            page_slot_admission(1, 2, 3, 6)
                .reference()
                .generation_owner(),
        ),
        true,
    );
    assert_manifest_denial(
        ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        )
        .with_expected_reference(ManifestExpectedReference::page_slot(
            page_slot_admission(9, 2, 3, 7),
        )),
        ManifestIntegrityDenialKind::WrongSegmentId,
        Some(
            page_slot_admission(9, 2, 3, 7)
                .reference()
                .generation_owner(),
        ),
        true,
    );

    let extent_root = root_with_extent(1, 5, 7);
    assert_manifest_denial(
        ManifestIntegrityInspectionRequest::from_root_publication(
            extent_root.clone(),
            root_admission(&extent_root),
        )
        .with_expected_reference(ManifestExpectedReference::extent(extent_admission(1, 6, 7))),
        ManifestIntegrityDenialKind::MismatchedExtentId,
        Some(extent_admission(1, 6, 7).reference().generation_owner()),
        true,
    );
    assert_manifest_denial(
        ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        )
        .with_expected_reference(ManifestExpectedReference::free_space_reuse(
            free_space_slot_admission(1, 2, 3, 7),
        )),
        ManifestIntegrityDenialKind::DamagedAllocationMap,
        Some(
            free_space_slot_admission(1, 2, 3, 7)
                .reference()
                .generation_owner(),
        ),
        true,
    );
    assert_manifest_denial(
        ManifestIntegrityInspectionRequest::missing_root(),
        ManifestIntegrityDenialKind::MissingRootPage,
        None,
        false,
    );
    assert_manifest_denial(
        ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        )
        .with_backend_residue_fallback(page_slot_admission(1, 2, 3, 7)),
        ManifestIntegrityDenialKind::BackendResidueFallback,
        Some(
            page_slot_admission(1, 2, 3, 7)
                .reference()
                .generation_owner(),
        ),
        false,
    );
}

#[test]
fn denied_manifest_reference_counts_only_performed_probes() {
    let root = root_with_slot(1, 2, 3, 7);
    let denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::page_slot(page_slot_admission(
                9, 2, 3, 7,
            )))
            .with_expected_reference(ManifestExpectedReference::page_slot(page_slot_admission(
                1, 2, 3, 7,
            ))),
        )
        .unwrap_err();

    assert_eq!(denial.kind(), ManifestIntegrityDenialKind::WrongSegmentId);
    assert_eq!(denial.counters().manifest_reference_probes(), 1);
}

#[test]
fn root_posture_variants_distinguish_ambiguous_evidence_without_recovery_choice() {
    let root = root_with_slot(1, 2, 3, 7);
    let report = ManifestIntegrityAuthority::new()
        .inspect_manifest(ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        ))
        .unwrap();
    assert!(matches!(
        report.root().posture(),
        RootManifestIntegrityPosture::CurrentRootAdmitted(_)
    ));

    assert_root_posture(
        ManifestIntegrityInspectionRequest::damaged_root(root_owner(&root)),
        RootManifestIntegrityPosture::DamagedRoot,
    );
    assert_root_posture(
        ManifestIntegrityInspectionRequest::torn_root_pointer(root_owner(&root)),
        RootManifestIntegrityPosture::TornRootPointer,
    );
    assert_root_posture(
        ManifestIntegrityInspectionRequest::multiple_valid_roots(
            root.clone(),
            root_with_slot_under_root(100, 1, 2, 3, 7),
        ),
        RootManifestIntegrityPosture::MultipleValidRoots,
    );

    let old_root = root_with_slot_root_generation(99, 1, 1, 2, 3, 7);
    let new_root = root_with_slot_root_generation(99, 2, 1, 2, 3, 7);
    assert_root_posture(
        ManifestIntegrityInspectionRequest::root_generation_mismatch(
            new_root,
            root_admission(&old_root),
        ),
        RootManifestIntegrityPosture::RootGenerationMismatch,
    );
    assert_root_posture(
        ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        )
        .with_backend_residue_fallback(page_slot_admission(1, 2, 3, 7)),
        RootManifestIntegrityPosture::ResidueRootRejected,
    );
    assert_root_posture(
        ManifestIntegrityInspectionRequest::recovery_blocking_root_damage(root_owner(&root)),
        RootManifestIntegrityPosture::RecoveryBlockingRootDamage,
    );
}

#[test]
fn current_root_posture_requires_root_publication_validation_witness() {
    let root = root_with_slot(1, 2, 3, 7);
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let root_validation = references
        .validate_root_publication(root_admission(&root), root.root_publication())
        .unwrap();
    let page_validation = references
        .validate_page_slot(
            page_slot_admission(1, 2, 3, 7),
            root.page_slots()[0].page_slot(),
        )
        .unwrap();
    let posture = RootManifestIntegrityPosture::current_root_publication(root_validation);

    assert!(posture.admits_scope());
    assert_eq!(
        root_validation.reference().kind(),
        PhysicalReferenceKind::RootPublication
    );
    assert_eq!(
        page_validation.reference().kind(),
        PhysicalReferenceKind::PageSlot
    );
}

#[test]
fn intact_derived_structures_cannot_override_authoritative_manifest_damage() {
    let root = root_with_slot(1, 2, 3, 7);
    let authoritative_denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::free_space_reuse(
                free_space_slot_admission(1, 2, 3, 7),
            )),
        )
        .unwrap_err();
    let override_attempt = DerivedManifestOverrideAttempt::against_authoritative_manifest_denial(
        &authoritative_denial,
        PhysicalReferenceScope::derived_index(page_cell(1, 2, 7)),
    )
    .unwrap();
    let denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_derived_override_attempt(override_attempt),
        )
        .unwrap_err();

    assert_eq!(
        authoritative_denial.kind(),
        ManifestIntegrityDenialKind::DamagedAllocationMap
    );
    assert_eq!(
        denial.kind(),
        ManifestIntegrityDenialKind::SourcePrecedenceViolation
    );
    assert_eq!(
        override_attempt.authoritative_failure().kind(),
        ManifestIntegrityDenialKind::DamagedAllocationMap
    );
    assert_eq!(denial.locality(), authoritative_denial.locality());
    assert_eq!(denial.counters().derived_override_rejections(), 1);
}

#[test]
fn source_precedence_attempt_requires_authoritative_manifest_denial_evidence() {
    let missing_root_denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(ManifestIntegrityInspectionRequest::missing_root())
        .unwrap_err();
    let root = root_with_slot(1, 2, 3, 7);
    let backend_residue_denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_backend_residue_fallback(page_slot_admission(1, 2, 3, 7)),
        )
        .unwrap_err();
    let derived_scope = PhysicalReferenceScope::derived_index(page_cell(1, 2, 7));
    let authoritative_denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::free_space_reuse(
                free_space_slot_admission(1, 2, 3, 7),
            )),
        )
        .unwrap_err();
    let source_precedence_denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_derived_override_attempt(
                DerivedManifestOverrideAttempt::against_authoritative_manifest_denial(
                    &authoritative_denial,
                    derived_scope,
                )
                .unwrap(),
            ),
        )
        .unwrap_err();

    assert_override_attempt_rejected(&missing_root_denial, derived_scope);
    assert_override_attempt_rejected(&backend_residue_denial, derived_scope);
    assert_override_attempt_rejected(
        &authoritative_denial,
        PhysicalReferenceScope::manifest_page(page_cell(1, 2, 7)),
    );
    assert_override_attempt_rejected(&source_precedence_denial, derived_scope);
}

#[test]
fn manifest_membership_remains_the_current_root_admission_basis() {
    let root = root_with_slot(1, 2, 3, 7);
    let scope = PhysicalReferenceScope::manifest_page(page_cell(1, 2, 7));
    let membership = ManifestMembershipProof::from_root(&root, scope).unwrap();
    let report = ManifestIntegrityAuthority::new()
        .inspect_manifest(ManifestIntegrityInspectionRequest::from_root_publication(
            root.clone(),
            root_admission(&root),
        ))
        .unwrap();

    assert_eq!(
        report.root().root_owner(),
        RootManifestIntegrityPosture::current_root_admitted(membership).root_owner()
    );
}

fn assert_manifest_denial(
    request: ManifestIntegrityInspectionRequest,
    expected: ManifestIntegrityDenialKind,
    locality: Option<PhysicalGenerationOwner>,
    current_root: bool,
) {
    let denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(request)
        .unwrap_err();
    assert_eq!(denial.kind(), expected);
    assert_eq!(denial.locality(), locality);
    assert_eq!(denial.posture().admits_scope(), current_root);
}

fn assert_root_posture(
    request: ManifestIntegrityInspectionRequest,
    expected: RootManifestIntegrityPosture,
) {
    let denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(request)
        .unwrap_err();
    assert_eq!(denial.posture(), expected);
}

fn assert_override_attempt_rejected(
    denial: &ManifestIntegrityDenial,
    scope: PhysicalReferenceScope,
) {
    assert!(
        DerivedManifestOverrideAttempt::against_authoritative_manifest_denial(denial, scope)
            .is_none()
    );
}

fn root_owner(root: &worth_store_physical_format::PhysicalRootManifest) -> PhysicalGenerationOwner {
    root.root_publication().owner()
}
