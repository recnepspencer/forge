use crate::courtroom::harness::test_support::physical_scope_admission_test_support::{
    extent_validation, frame_request, mismatched_checksum_request, page_cell, page_request,
    root_with_extent, root_with_slot, root_with_slot_under_root, scope_membership, validation,
    with_checked_frame, with_checked_page,
};
use worth_store_physical_format::{
    CheckpointAdjacencyPosture, ManifestMembershipProof, PhysicalReferenceScope,
    PhysicalScopeFamily, RootManifestIntegrityPosture,
};
use worth_store_physical_integrity::{
    GenerationIntegrityReport, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    PhysicalScopeBasis, PhysicalScopeDenialKind, ScopedPhysicalValidatorInput,
};

#[test]
fn slot_reuse_generation_changes_basis_and_stale_reference_denies() {
    let first = admitted_frame_basis(validation(1, 2, 3, 7), root_with_slot(1, 2, 3, 7));
    let second = admitted_frame_basis(validation(1, 2, 3, 8), root_with_slot(1, 2, 3, 8));
    assert_ne!(first, second);

    with_checked_frame(b"slot-reuse", validation(1, 2, 3, 8), |checked| {
        let stale = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, stale);
        let request = frame_request(&checked, stale, membership);
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
    });
}

#[test]
fn locally_checksummed_wrong_scope_denies_with_precise_reason() {
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
    assert_wrong_extent_scope(
        PhysicalReferenceScope::chunk_like(extent_validation(1, 6, 7)),
        root_with_extent(1, 6, 7),
        PhysicalScopeDenialKind::WrongExtent,
    );

    with_checked_frame(b"manifest-mismatch", validation(1, 2, 3, 7), |checked| {
        let expected = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let actual = PhysicalReferenceScope::frame(validation(1, 4, 3, 7));
        let actual_root = root_with_slot(1, 4, 3, 7);
        let membership = scope_membership(&actual_root, actual);
        let request = frame_request(&checked, expected, membership);
        let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();
        assert_eq!(denial.kind(), PhysicalScopeDenialKind::WrongManifestScope);
        assert!(denial.intact_wrong_scope().is_some());
    });
}

#[test]
fn root_checkpoint_and_checksum_scope_deny_before_family_validation() {
    with_checked_frame(b"scope-posture", validation(1, 2, 3, 7), |checked| {
        let scope = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let wrong_root = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::DamagedRoot,
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let denial = PhysicalScopeAdmission::admit_frame(checked.clone(), wrong_root).unwrap_err();
        assert_eq!(denial.kind(), PhysicalScopeDenialKind::WrongRootPosture);

        let wrong_checkpoint = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            CheckpointAdjacencyPosture::MismatchedCheckpointAdjacency,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let denial =
            PhysicalScopeAdmission::admit_frame(checked.clone(), wrong_checkpoint).unwrap_err();
        assert_eq!(
            denial.kind(),
            PhysicalScopeDenialKind::WrongCheckpointAdjacency
        );

        let denial = PhysicalScopeAdmission::admit_frame(
            checked,
            mismatched_checksum_request(scope, membership),
        )
        .unwrap_err();
        assert_eq!(
            denial.kind(),
            PhysicalScopeDenialKind::ChecksumScopeMismatch
        );
        assert!(denial.checksum_mismatch().is_some());
    });
}

#[test]
fn copied_root_posture_from_different_membership_denies_scope_admission() {
    with_checked_frame(b"copied-root-posture", validation(1, 2, 3, 7), |checked| {
        let scope = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let other_root = root_with_slot_under_root(100, 9, 2, 3, 7);
        let other_scope = PhysicalReferenceScope::frame(validation(9, 2, 3, 7));
        let other_membership = scope_membership(&other_root, other_scope);
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(other_membership),
            CheckpointAdjacencyPosture::NotApplicable,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();
        assert_eq!(denial.kind(), PhysicalScopeDenialKind::WrongRootPosture);
    });
}

#[test]
fn stale_manifest_entry_cannot_mint_membership_for_new_generation() {
    let stale_root = root_with_slot(1, 2, 3, 7);
    let reused_slot_scope = PhysicalReferenceScope::frame(validation(1, 2, 3, 8));

    assert!(ManifestMembershipProof::from_root(&stale_root, reused_slot_scope).is_err());
}

#[test]
fn incompatible_checked_form_and_scope_family_deny_before_family_validation() {
    with_checked_page(b"wrong-family-page", page_cell(1, 2, 7), |checked| {
        let frame_scope = PhysicalReferenceScope::frame(validation(1, 2, 3, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, frame_scope);
        let request = page_request(&checked, frame_scope, membership);
        let denial = PhysicalScopeAdmission::admit_page(checked, request).unwrap_err();
        assert_eq!(denial.kind(), PhysicalScopeDenialKind::WrongPhysicalFamily);
    });

    with_checked_frame(b"wrong-family-frame", validation(1, 2, 3, 7), |checked| {
        let page_scope = PhysicalReferenceScope::page(page_cell(1, 2, 7));
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, page_scope);
        let request = frame_request(&checked, page_scope, membership);
        let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();
        assert_eq!(denial.kind(), PhysicalScopeDenialKind::WrongPhysicalFamily);
    });
}

#[test]
fn family_validators_consume_scope_admission_for_each_physical_family() {
    let mut validator = CountingFamilyValidator::default();
    validate_page_family(&mut validator, PhysicalScopeFamily::Page);
    validate_frame_family(&mut validator, PhysicalScopeFamily::Frame);
    validate_frame_family(&mut validator, PhysicalScopeFamily::WalFrame);
    validate_page_family(&mut validator, PhysicalScopeFamily::Manifest);
    validate_chunk_family(&mut validator);
    validate_page_family(&mut validator, PhysicalScopeFamily::DerivedIndex);
    assert_eq!(validator.invocations, 6);
}

#[test]
fn family_validator_inputs_reject_wrong_scope_family() {
    let validation = validation(1, 2, 3, 7);
    let root = root_with_slot(1, 2, 3, 7);
    with_checked_frame(b"wrong-validator-family", validation, |checked| {
        let scope = PhysicalReferenceScope::frame(validation);
        let membership = scope_membership(&root, scope);
        let request = frame_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();

        let denial = ScopedPhysicalValidatorInput::wal_frame(admission).unwrap_err();
        assert_eq!(denial.kind(), PhysicalScopeDenialKind::WrongPhysicalFamily);
    });

    let cell = page_cell(1, 2, 7);
    let root = root_with_slot(1, 2, 3, 7);
    with_checked_page(b"wrong-validator-family", cell, |checked| {
        let scope = PhysicalReferenceScope::manifest_page(cell);
        let membership = scope_membership(&root, scope);
        let request = page_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_page(checked, request).unwrap();

        let denial = ScopedPhysicalValidatorInput::page(admission).unwrap_err();
        assert_eq!(denial.kind(), PhysicalScopeDenialKind::WrongPhysicalFamily);
    });
}

fn admitted_frame_basis(
    validation: worth_store_physical_format::PhysicalReferenceValidationWitness,
    root: worth_store_physical_format::PhysicalRootManifest,
) -> PhysicalScopeBasis {
    let mut basis = None;
    with_checked_frame(b"slot-reuse", validation, |checked| {
        let scope = PhysicalReferenceScope::frame(validation);
        let membership = scope_membership(&root, scope);
        let request = frame_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        basis = Some(admission.basis().clone());
    });
    basis.unwrap()
}

fn assert_wrong_frame_scope(
    scope: PhysicalReferenceScope,
    root: worth_store_physical_format::PhysicalRootManifest,
    expected: PhysicalScopeDenialKind,
) {
    with_checked_frame(b"wrong-frame-scope", validation(1, 2, 3, 7), |checked| {
        let membership = scope_membership(&root, scope);
        let request = frame_request(&checked, scope, membership);
        let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();
        assert_eq!(denial.kind(), expected);
    });
}

fn assert_wrong_extent_scope(
    scope: PhysicalReferenceScope,
    root: worth_store_physical_format::PhysicalRootManifest,
    expected: PhysicalScopeDenialKind,
) {
    with_checked_frame(
        b"wrong-extent-scope",
        extent_validation(1, 5, 7),
        |checked| {
            let membership = scope_membership(&root, scope);
            let request = frame_request(&checked, scope, membership);
            let denial = PhysicalScopeAdmission::admit_frame(checked, request).unwrap_err();
            assert_eq!(denial.kind(), expected);
        },
    );
}

fn validate_frame_family(validator: &mut CountingFamilyValidator, family: PhysicalScopeFamily) {
    let validation = validation(1, 2, 3, 7);
    let root = root_with_slot(1, 2, 3, 7);
    with_checked_frame(b"frame-family", validation, |checked| {
        let scope = match family {
            PhysicalScopeFamily::Frame => PhysicalReferenceScope::frame(validation),
            PhysicalScopeFamily::WalFrame => PhysicalReferenceScope::wal_frame(validation),
            _ => unreachable!(),
        };
        let membership = scope_membership(&root, scope);
        let request = frame_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        match family {
            PhysicalScopeFamily::Frame => {
                validator.validate_frame(ScopedPhysicalValidatorInput::frame(admission).unwrap());
            }
            PhysicalScopeFamily::WalFrame => {
                validator.validate_wal_frame(
                    ScopedPhysicalValidatorInput::wal_frame(admission).unwrap(),
                );
            }
            _ => unreachable!(),
        }
    });
}

fn validate_chunk_family(validator: &mut CountingFamilyValidator) {
    let validation = extent_validation(1, 5, 7);
    let root = root_with_extent(1, 5, 7);
    with_checked_frame(b"chunk-family", validation, |checked| {
        let scope = PhysicalReferenceScope::chunk_like(validation);
        let membership = scope_membership(&root, scope);
        let request = frame_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        validator.validate_chunk(ScopedPhysicalValidatorInput::chunk_like(admission).unwrap());
    });
}

fn validate_page_family(validator: &mut CountingFamilyValidator, family: PhysicalScopeFamily) {
    let cell = page_cell(1, 2, 7);
    let root = root_with_slot(1, 2, 3, 7);
    with_checked_page(b"page-family", cell, move |checked| {
        let scope = match family {
            PhysicalScopeFamily::Page => PhysicalReferenceScope::page(cell),
            PhysicalScopeFamily::Manifest => PhysicalReferenceScope::manifest_page(cell),
            PhysicalScopeFamily::DerivedIndex => PhysicalReferenceScope::derived_index(cell),
            _ => unreachable!(),
        };
        let membership = scope_membership(&root, scope);
        let request = page_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_page(checked, request).unwrap();
        match family {
            PhysicalScopeFamily::Page => {
                validator.validate_page(ScopedPhysicalValidatorInput::page(admission).unwrap());
            }
            PhysicalScopeFamily::Manifest => {
                validator
                    .validate_manifest(ScopedPhysicalValidatorInput::manifest(admission).unwrap());
            }
            PhysicalScopeFamily::DerivedIndex => {
                validator.validate_derived_index(
                    ScopedPhysicalValidatorInput::derived_index(admission).unwrap(),
                );
            }
            _ => unreachable!(),
        }
    });
}

#[derive(Default)]
struct CountingFamilyValidator {
    invocations: u32,
}

impl CountingFamilyValidator {
    fn validate_page(&mut self, input: ScopedPhysicalValidatorInput<'_>) {
        assert_eq!(input.family(), PhysicalScopeFamily::Page);
        self.invocations += 1;
    }

    fn validate_frame(&mut self, input: ScopedPhysicalValidatorInput<'_>) {
        assert_eq!(input.family(), PhysicalScopeFamily::Frame);
        self.invocations += 1;
    }

    fn validate_wal_frame(&mut self, input: ScopedPhysicalValidatorInput<'_>) {
        assert_eq!(input.family(), PhysicalScopeFamily::WalFrame);
        self.invocations += 1;
    }

    fn validate_manifest(&mut self, input: ScopedPhysicalValidatorInput<'_>) {
        assert_eq!(input.family(), PhysicalScopeFamily::Manifest);
        self.invocations += 1;
    }

    fn validate_chunk(&mut self, input: ScopedPhysicalValidatorInput<'_>) {
        assert_eq!(input.family(), PhysicalScopeFamily::ChunkLike);
        self.invocations += 1;
    }

    fn validate_derived_index(&mut self, input: ScopedPhysicalValidatorInput<'_>) {
        assert_eq!(input.family(), PhysicalScopeFamily::DerivedIndex);
        self.invocations += 1;
    }
}
