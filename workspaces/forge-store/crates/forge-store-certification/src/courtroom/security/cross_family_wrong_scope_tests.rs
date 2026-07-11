use crate::courtroom::harness::test_support::physical_scope_admission_test_support::{
    extent_validation, mismatched_checksum_request, page_cell, root_with_extent, root_with_slot,
    scope_membership, validation, with_checked_frame, with_checked_page,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, PhysicalScopeFamily,
    RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    GenerationIntegrityReport, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    PhysicalScopeDenial, PhysicalScopeDenialKind,
};

#[test]
fn repeated_cross_family_replays_stop_at_scope_admission() {
    for family in [
        PhysicalScopeFamily::Frame,
        PhysicalScopeFamily::WalFrame,
        PhysicalScopeFamily::ChunkLike,
    ] {
        let first = deny_page_bytes_as_scope(family);
        let second = deny_page_bytes_as_scope(family);

        assert_same_scope_admission_denial(first, second);
    }

    for family in [
        PhysicalScopeFamily::Page,
        PhysicalScopeFamily::Manifest,
        PhysicalScopeFamily::DerivedIndex,
    ] {
        let first = deny_frame_bytes_as_scope(family);
        let second = deny_frame_bytes_as_scope(family);

        assert_same_scope_admission_denial(first, second);
    }
}

#[test]
fn checksummed_wrong_scope_denials_remain_typed_before_family_validation() {
    assert_wrong_frame_scope(
        PhysicalReferenceScope::frame(validation(9, 2, 3, 7)),
        root_with_slot(9, 2, 3, 7),
        PhysicalScopeDenialKind::WrongSegment,
    );
    assert_wrong_frame_scope(
        PhysicalReferenceScope::frame(validation(1, 4, 3, 7)),
        root_with_slot(1, 4, 3, 7),
        PhysicalScopeDenialKind::WrongPage,
    );
    assert_wrong_chunk_scope(
        PhysicalReferenceScope::chunk_like(extent_validation(1, 6, 7)),
        root_with_extent(1, 6, 7),
        PhysicalScopeDenialKind::WrongExtent,
    );

    let manifest_denial = manifest_scope_mismatch_denial();
    assert_eq!(
        manifest_denial.kind(),
        PhysicalScopeDenialKind::WrongManifestScope
    );
    assert!(manifest_denial.intact_wrong_scope().is_some());

    assert_eq!(
        checkpoint_adjacency_denial().kind(),
        PhysicalScopeDenialKind::WrongCheckpointAdjacency
    );
    assert_eq!(
        root_posture_denial().kind(),
        PhysicalScopeDenialKind::WrongRootPosture
    );
    assert_eq!(
        checksum_scope_denial().kind(),
        PhysicalScopeDenialKind::ChecksumScopeMismatch
    );
    assert!(checksum_scope_denial().checksum_mismatch().is_some());

    let derived_index_denial = derived_index_authority_basis_denial();
    assert_eq!(
        derived_index_denial.kind(),
        PhysicalScopeDenialKind::WrongPage
    );
    assert!(matches!(
        derived_index_denial.generation_report(),
        Some(GenerationIntegrityReport::MisplacedPhysicalIdentity { .. })
    ));
}

#[test]
fn physical_substrate_resident_generation_cannot_refresh_physical_format_durable_generation() {
    with_checked_frame(
        b"resident-generation-is-not-durable",
        validation(1, 2, 3, 8),
        |checked| {
            let stale_scope = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
            let stale_root = root_with_slot(1, 2, 3, 7);
            let membership = scope_membership(&stale_root, stale_scope);
            let request = PhysicalScopeAdmissionRequest::frame(
                stale_scope,
                membership,
                RootManifestIntegrityPosture::current_root_admitted(membership),
                CheckpointAdjacencyPosture::NotApplicable,
                checked.gate_evidence().coverage_basis().clone(),
            );
            let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();

            assert_eq!(
                denial.kind(),
                PhysicalScopeDenialKind::StalePhysicalGeneration
            );
            assert_eq!(
                denial.generation_report(),
                Some(GenerationIntegrityReport::StalePhysicalGeneration {
                    expected: validation(1, 2, 3, 7).owner(),
                    actual: validation(1, 2, 3, 8).owner(),
                })
            );
        },
    );
}

fn deny_page_bytes_as_scope(family: PhysicalScopeFamily) -> PhysicalScopeDenial {
    let mut denial = None;
    let cell = page_cell(1, 2, 7);
    with_checked_page(b"page-as-wrong-family", cell, |checked| {
        let scope = frame_backed_scope(family);
        let root = root_for_scope(scope);
        let membership = scope_membership(&root, scope);
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        denial = Some(PhysicalScopeAdmission::admit_page(checked, request).unwrap_err());
    });
    denial.unwrap()
}

fn deny_frame_bytes_as_scope(family: PhysicalScopeFamily) -> PhysicalScopeDenial {
    let mut denial = None;
    with_checked_frame(
        b"frame-as-wrong-family",
        validation(1, 2, 3, 7),
        |checked| {
            let scope = page_backed_scope(family);
            let root = root_for_scope(scope);
            let membership = scope_membership(&root, scope);
            let request = PhysicalScopeAdmissionRequest::page(
                scope,
                membership,
                RootManifestIntegrityPosture::current_root_admitted(membership),
                checked.gate_evidence().coverage_basis().clone(),
            );
            denial = Some(PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err());
        },
    );
    denial.unwrap()
}

fn assert_same_scope_admission_denial(first: PhysicalScopeDenial, second: PhysicalScopeDenial) {
    assert_eq!(first.kind(), PhysicalScopeDenialKind::WrongPhysicalFamily);
    assert_eq!(first.kind(), second.kind());
    assert_eq!(first.expected_scope(), second.expected_scope());
    assert_eq!(first.actual_scope(), second.actual_scope());
    assert!(first.generation_report().is_none());
    assert!(second.generation_report().is_none());
    assert!(first.checksum_mismatch().is_none());
    assert!(second.checksum_mismatch().is_none());
}

fn assert_wrong_frame_scope(
    scope: PhysicalReferenceScope,
    root: forge_store_physical_format::PhysicalRootManifest,
    expected: PhysicalScopeDenialKind,
) {
    with_checked_frame(b"wrong-frame-scope", validation(1, 2, 3, 7), |checked| {
        let membership = scope_membership(&root, scope);
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();
        assert_eq!(denial.kind(), expected);
        assert!(denial.generation_report().is_some());
    });
}

fn assert_wrong_chunk_scope(
    scope: PhysicalReferenceScope,
    root: forge_store_physical_format::PhysicalRootManifest,
    expected: PhysicalScopeDenialKind,
) {
    with_checked_frame(b"wrong-chunk-scope", validation(1, 2, 3, 7), |checked| {
        let membership = scope_membership(&root, scope);
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();
        assert_eq!(denial.kind(), expected);
        assert!(matches!(
            denial.generation_report(),
            Some(GenerationIntegrityReport::MisplacedPhysicalIdentity { .. })
        ));
    });
}

fn manifest_scope_mismatch_denial() -> PhysicalScopeDenial {
    let mut denial = None;
    with_checked_frame(b"manifest-mismatch", validation(1, 2, 3, 7), |checked| {
        let expected = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let actual = PhysicalReferenceScope::frame(validation(1, 4, 3, 7));
        let actual_root = root_with_slot(1, 4, 3, 7);
        let membership = scope_membership(&actual_root, actual);
        let request = PhysicalScopeAdmissionRequest::frame(
            expected,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        denial = Some(PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err());
    });
    denial.unwrap()
}

fn checkpoint_adjacency_denial() -> PhysicalScopeDenial {
    let mut denial = None;
    with_checked_frame(b"checkpoint-mismatch", validation(1, 2, 3, 7), |checked| {
        let scope = PhysicalReferenceScope::wal_frame(validation(1, 2, 3, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::MismatchedCheckpointAdjacency,
            checked.gate_evidence().coverage_basis().clone(),
        );
        denial = Some(PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err());
    });
    denial.unwrap()
}

fn root_posture_denial() -> PhysicalScopeDenial {
    let mut denial = None;
    with_checked_frame(b"root-posture", validation(1, 2, 3, 7), |checked| {
        let scope = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::DamagedRoot,
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        denial = Some(PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err());
    });
    denial.unwrap()
}

fn checksum_scope_denial() -> PhysicalScopeDenial {
    let mut denial = None;
    with_checked_frame(b"checksum-scope", validation(1, 2, 3, 7), |checked| {
        let scope = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        denial = Some(
            PhysicalScopeAdmission::admit_frame(
                checked,
                mismatched_checksum_request(scope, membership),
            )
            .unwrap_err(),
        );
    });
    denial.unwrap()
}

fn derived_index_authority_basis_denial() -> PhysicalScopeDenial {
    let mut denial = None;
    let actual_cell = page_cell(1, 2, 7);
    with_checked_page(b"derived-index-basis", actual_cell, |checked| {
        let claimed_scope = PhysicalReferenceScope::derived_index(page_cell(1, 4, 7));
        let root = root_with_slot(1, 4, 3, 7);
        let membership = scope_membership(&root, claimed_scope);
        let request = PhysicalScopeAdmissionRequest::page(
            claimed_scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            checked.gate_evidence().coverage_basis().clone(),
        );
        denial = Some(PhysicalScopeAdmission::admit_page(checked, request).unwrap_err());
    });
    denial.unwrap()
}

fn frame_backed_scope(family: PhysicalScopeFamily) -> PhysicalReferenceScope {
    match family {
        PhysicalScopeFamily::Frame => PhysicalReferenceScope::frame(validation(1, 2, 3, 7)),
        PhysicalScopeFamily::WalFrame => PhysicalReferenceScope::wal_frame(validation(1, 2, 3, 7)),
        PhysicalScopeFamily::ChunkLike => {
            PhysicalReferenceScope::chunk_like(extent_validation(1, 5, 7))
        }
        _ => unreachable!(),
    }
}

fn page_backed_scope(family: PhysicalScopeFamily) -> PhysicalReferenceScope {
    let cell = page_cell(1, 2, 7);
    match family {
        PhysicalScopeFamily::Page => PhysicalReferenceScope::page(cell),
        PhysicalScopeFamily::Manifest => PhysicalReferenceScope::manifest_page(cell),
        PhysicalScopeFamily::DerivedIndex => PhysicalReferenceScope::derived_index(cell),
        _ => unreachable!(),
    }
}

fn root_for_scope(
    scope: PhysicalReferenceScope,
) -> forge_store_physical_format::PhysicalRootManifest {
    match scope.family() {
        PhysicalScopeFamily::ChunkLike => root_with_extent(1, 5, 7),
        _ => root_with_slot(1, 2, 3, 7),
    }
}
