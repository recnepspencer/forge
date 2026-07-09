use crate::courtroom::harness::test_support::physical_scope_admission_test_support::{
    free_space_slot_admission, page_cell, page_request, page_slot_admission, root_admission,
    root_with_slot, root_with_slot_under_root, scope_membership, with_checked_page,
};
use worth_store_physical_format::PhysicalReferenceScope;
use worth_store_physical_integrity::{
    DerivedDamageClassification, DerivedIndexIntegrityAuthority,
    DerivedIndexIntegrityInspectionRequest, IndexPageIntegrityDenial, IndexPageIntegrityDenialKind,
    IndexPageIntegrityReport, ManifestExpectedReference, ManifestIntegrityAuthority,
    ManifestIntegrityInspectionRequest, ManifestReferenceBasis, PhysicalScopeAdmission,
    RebuildabilityPrerequisite, ScopedPhysicalValidatorInput,
};

#[test]
fn damaged_derived_index_with_intact_authority_classifies_rebuildably_with_parity() {
    let first = inspect_damaged_derived_index_with_authority(7);
    let second = inspect_damaged_derived_index_with_authority(7);

    assert_eq!(first, second);
    match first.damage_classification() {
        DerivedDamageClassification::RebuildableDerived(damage) => {
            assert_eq!(
                damage.damaged_scope(),
                PhysicalReferenceScope::derived_index(page_cell(1, 2, 7))
            );
            let authority_owner = page_slot_admission(1, 2, 3, 7)
                .reference()
                .generation_owner();
            assert_eq!(damage.rebuild_input().authority_owner(), authority_owner);
            assert_eq!(damage.prerequisites().authority_owner(), authority_owner);
        }
        other => panic!("expected rebuildable derived damage, got {other:?}"),
    }
    assert_eq!(first.counters().derived_scope_checks(), 1);
    assert_eq!(first.counters().authority_basis_checks(), 1);
    assert_eq!(first.counters().generation_link_checks(), 1);
    assert_eq!(first.counters().rebuildable_classifications(), 1);
    assert_eq!(first.counters().skipped_semantic_index_lookups(), 1);
}

#[test]
fn damaged_authority_missing_basis_and_stale_generation_do_not_classify_rebuildably() {
    let damaged_authority = inspect_with_damaged_authority();
    assert_eq!(
        damaged_authority.kind(),
        IndexPageIntegrityDenialKind::DamagedAuthority
    );
    assert!(damaged_authority.authority_damage().is_some());
    assert_eq!(damaged_authority.counters().authority_damage_denials(), 1);

    let missing_basis = inspect_without_authority_basis();
    assert_eq!(
        missing_basis.kind(),
        IndexPageIntegrityDenialKind::MissingAuthorityBasis
    );
    assert!(missing_basis.indeterminate_damage().is_some());
    assert_eq!(
        missing_basis
            .indeterminate_damage()
            .unwrap()
            .missing_prerequisite(),
        RebuildabilityPrerequisite::CurrentAuthorityBasis
    );
    assert_eq!(missing_basis.counters().indeterminate_classifications(), 1);

    let stale = inspect_damaged_derived_index_against_authority_generation(6, 7);
    assert_eq!(
        stale.kind(),
        IndexPageIntegrityDenialKind::StaleIndexGeneration
    );
    assert_eq!(
        stale.expected_owner(),
        Some(
            page_slot_admission(1, 2, 3, 7)
                .reference()
                .generation_owner()
        )
    );
    assert_eq!(stale.actual_owner(), Some(page_cell(1, 2, 6).owner()));
    assert_eq!(
        stale.indeterminate_damage().unwrap().missing_prerequisite(),
        RebuildabilityPrerequisite::GenerationLink
    );
}

#[test]
fn missing_generation_link_is_indeterminate_not_rebuildable() {
    let denial = inspect_damaged_derived_index_against_unrelated_authority();

    assert_eq!(
        denial.kind(),
        IndexPageIntegrityDenialKind::MissingGenerationLink
    );
    assert!(denial.indeterminate_damage().is_some());
    assert_eq!(
        denial
            .indeterminate_damage()
            .unwrap()
            .missing_prerequisite(),
        RebuildabilityPrerequisite::GenerationLink
    );
    assert_eq!(denial.counters().indeterminate_classifications(), 1);
    assert_eq!(denial.counters().rebuildable_classifications(), 0);
}

#[test]
fn copied_same_owner_basis_from_different_root_is_indeterminate_not_rebuildable() {
    let denial = inspect_damaged_derived_index_against_same_owner_different_root();
    let admitted_root = root_with_slot(1, 2, 3, 7).root_publication().owner();
    let copied_root = root_with_slot_under_root(100, 1, 2, 3, 7)
        .root_publication()
        .owner();

    assert_eq!(
        denial.kind(),
        IndexPageIntegrityDenialKind::MismatchedAuthorityRoot
    );
    assert_eq!(denial.expected_owner(), Some(admitted_root));
    assert_eq!(denial.actual_owner(), Some(copied_root));
    assert!(denial.indeterminate_damage().is_some());
    assert_eq!(
        denial
            .indeterminate_damage()
            .unwrap()
            .missing_prerequisite(),
        RebuildabilityPrerequisite::ExecutedManifestAuthority
    );
    assert_eq!(denial.counters().generation_link_checks(), 1);
    assert_eq!(denial.counters().indeterminate_classifications(), 1);
    assert_eq!(denial.counters().rebuildable_classifications(), 0);
}

#[test]
fn intact_derived_index_remains_derived_and_does_not_create_rebuild_input() {
    let report = inspect_intact_derived_index_with_authority();

    match report.damage_classification() {
        DerivedDamageClassification::IntactIndexPage(boundary) => {
            assert_eq!(
                boundary.scope(),
                PhysicalReferenceScope::derived_index(page_cell(1, 2, 7))
            );
        }
        other => panic!("expected intact derived index page, got {other:?}"),
    }
    assert_eq!(report.counters().skipped_semantic_index_lookups(), 0);
}

pub(crate) fn inspect_damaged_derived_index_with_authority(
    generation: u64,
) -> IndexPageIntegrityReport {
    let authority_basis = manifest_basis_for_page_generation(generation);
    inspect_damaged_derived_index(page_generation_payload(false), generation, authority_basis)
        .unwrap()
}

fn inspect_damaged_derived_index_against_authority_generation(
    derived_generation: u64,
    authority_generation: u64,
) -> IndexPageIntegrityDenial {
    let authority_basis = manifest_basis_for_page_generation(authority_generation);
    inspect_damaged_derived_index(
        page_generation_payload(false),
        derived_generation,
        authority_basis,
    )
    .unwrap_err()
}

fn inspect_damaged_derived_index_against_unrelated_authority() -> IndexPageIntegrityDenial {
    let authority_basis = manifest_basis_for_unrelated_page();
    inspect_damaged_derived_index(page_generation_payload(false), 7, authority_basis).unwrap_err()
}

fn inspect_damaged_derived_index_against_same_owner_different_root() -> IndexPageIntegrityDenial {
    let authority_basis = manifest_basis_for_same_owner_different_root();
    inspect_damaged_derived_index(page_generation_payload(false), 7, authority_basis).unwrap_err()
}

pub(crate) fn inspect_intact_derived_index_with_authority() -> IndexPageIntegrityReport {
    let authority_basis = manifest_basis_for_page_generation(7);
    inspect_damaged_derived_index(page_generation_payload(true), 7, authority_basis).unwrap()
}

pub(crate) fn inspect_with_damaged_authority() -> IndexPageIntegrityDenial {
    let authority_denial = damaged_authority_denial();
    let mut denial = None;
    with_derived_index_input(page_generation_payload(false), 7, |input| {
        let request =
            DerivedIndexIntegrityInspectionRequest::with_damaged_authority(input, authority_denial)
                .unwrap();
        denial = Some(
            DerivedIndexIntegrityAuthority::s3()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

pub(crate) fn inspect_without_authority_basis() -> IndexPageIntegrityDenial {
    let mut denial = None;
    with_derived_index_input(page_generation_payload(false), 7, |input| {
        let request =
            DerivedIndexIntegrityInspectionRequest::without_authority_basis(input).unwrap();
        denial = Some(
            DerivedIndexIntegrityAuthority::s3()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

fn inspect_damaged_derived_index(
    payload: Vec<u8>,
    generation: u64,
    authority_basis: ManifestReferenceBasis,
) -> Result<IndexPageIntegrityReport, IndexPageIntegrityDenial> {
    let mut result = None;
    with_derived_index_input(payload, generation, |input| {
        let request =
            DerivedIndexIntegrityInspectionRequest::from_admitted_scope(input, authority_basis)
                .unwrap();
        result = Some(DerivedIndexIntegrityAuthority::s3().inspect(request));
    });
    result.unwrap()
}

fn with_derived_index_input(
    payload: Vec<u8>,
    generation: u64,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    let cell = page_cell(1, 2, generation);
    with_checked_page(&payload, cell, |checked| {
        let scope = PhysicalReferenceScope::derived_index(cell);
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let request = page_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_page(checked, request).unwrap();
        run(ScopedPhysicalValidatorInput::derived_index(admission).unwrap());
    });
}

fn manifest_basis_for_page_generation(generation: u64) -> ManifestReferenceBasis {
    let root = root_with_slot(1, 2, 3, generation);
    ManifestIntegrityAuthority::s3()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::page_slot(
                page_slot_admission(1, 2, 3, generation),
            )),
        )
        .unwrap()
        .reference_basis()
        .clone()
}

fn manifest_basis_for_unrelated_page() -> ManifestReferenceBasis {
    let root = root_with_slot(1, 9, 3, 7);
    ManifestIntegrityAuthority::s3()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::page_slot(
                page_slot_admission(1, 9, 3, 7),
            )),
        )
        .unwrap()
        .reference_basis()
        .clone()
}

fn manifest_basis_for_same_owner_different_root() -> ManifestReferenceBasis {
    let root = root_with_slot_under_root(100, 1, 2, 3, 7);
    ManifestIntegrityAuthority::s3()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::page_slot(
                page_slot_admission(1, 2, 3, 7),
            )),
        )
        .unwrap()
        .reference_basis()
        .clone()
}

pub(crate) fn damaged_authority_denial() -> worth_store_physical_integrity::ManifestIntegrityDenial
{
    let root = root_with_slot(1, 2, 3, 7);
    ManifestIntegrityAuthority::s3()
        .inspect_manifest(
            ManifestIntegrityInspectionRequest::from_root_publication(
                root.clone(),
                root_admission(&root),
            )
            .with_expected_reference(ManifestExpectedReference::free_space_reuse(
                free_space_slot_admission(1, 2, 3, 7),
            )),
        )
        .unwrap_err()
}

fn page_generation_payload(intact: bool) -> Vec<u8> {
    if intact {
        b"DIDX:index-page".to_vec()
    } else {
        b"BIDX:index-page".to_vec()
    }
}
