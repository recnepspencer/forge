use crate::application::{
    ForgeQueryDeclarationAdmissionOrLegalityError, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationLegalityChecked, ForgeQueryDeclarationLegalityContract,
};

mod fixtures;

use fixtures::{
    admitted_handle, Declaration, DeferredLegalityFamily, DurableAdmissionFamily,
    IllegalDispositionFamily, IllegalRoleFamily, LegalFamily, MaskedCoverageFamily,
};

#[test]
fn legal_declaration_review_yields_legality_evidence() {
    let handle = admitted_handle("collaborative");
    let declaration = handle
        .declare(Declaration::<LegalFamily>::new("edge:42"))
        .expect("declaration should admit");

    let legal = handle
        .review_legality(declaration)
        .expect("legality review should pass");

    assert_eq!(legal.declaration_family_key(), "split-edge");
    assert!(legal.is_structurally_legal());
    assert_eq!(
        legal.legality_contract(),
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    );
    assert_eq!(
        legal.support_report().declare_status(),
        ForgeQueryDeclarationCapabilityStatus::Admitted
    );
    assert_eq!(
        legal.operating_context_identity_digest(),
        "geometry.collaborative"
    );
    assert_eq!(
        legal.aspect_contract().required(),
        &["selection.active_edge".to_string()]
    );
    assert_eq!(
        legal.reviewed_aspect_coverage().present(),
        &["selection.active_edge".to_string()]
    );
}

#[test]
fn legality_review_rejects_declarations_from_a_different_admitted_world() {
    let left = admitted_handle("collaborative");
    let right = admitted_handle("restricted");
    let declaration = left
        .declare(Declaration::<LegalFamily>::new("edge:42"))
        .expect("declaration should admit");

    match right.review_legality_checked(declaration) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::WrongAdmittedWorld { .. },
        ) => {}
        other => panic!(
            "expected wrong-world denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn legality_review_distinguishes_role_and_surface_failures() {
    let handle = admitted_handle("collaborative");

    let bad_role = handle
        .declare(Declaration::<IllegalRoleFamily>::new("edge:42"))
        .expect("declaration should admit");
    match handle.review_legality_checked(bad_role) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim { .. },
        ) => {}
        other => panic!(
            "expected role denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }

    let bad_surface = handle
        .declare(Declaration::<IllegalDispositionFamily>::new("edge:42"))
        .expect("declaration should admit");
    match handle.review_legality_checked(bad_surface) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::IllegalSurfaceDisposition {
                ..
            },
        ) => {}
        other => panic!(
            "expected disposition denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn legality_boundary_can_defer_even_after_family_admission() {
    let handle = admitted_handle("collaborative");
    let declaration = handle
        .declare(Declaration::<DeferredLegalityFamily>::new("edge:42"))
        .expect("declaration should admit");

    match handle.review_legality_checked(declaration) {
        ForgeQueryDeclarationLegalityChecked::Illegal(
            crate::application::ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary {
                ..
            },
        ) => {}
        other => panic!(
            "expected deferred legality denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn declare_and_review_preserves_admission_vs_legality_split() {
    let handle = admitted_handle("collaborative");

    match handle.declare_and_review(Declaration::<DurableAdmissionFamily>::new("edge:42")) {
        Err(ForgeQueryDeclarationAdmissionOrLegalityError::Admission(admission)) => {
            assert!(matches!(
                admission,
                crate::application::ForgeQueryDeclarationAdmissionError::Deferred(_)
            ));
        }
        _ => panic!("expected admission denial"),
    }

    match handle.declare_and_review(Declaration::<IllegalRoleFamily>::new("edge:42")) {
        Err(ForgeQueryDeclarationAdmissionOrLegalityError::Legality(
            crate::application::ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim { .. },
        )) => {}
        _ => panic!("expected legality denial"),
    }
}

#[test]
fn legality_evidence_preserves_masked_aspect_coverage_without_promoting_it_to_reviewed_presence() {
    let handle = admitted_handle("collaborative");
    let declaration = handle
        .declare(Declaration::<MaskedCoverageFamily>::new("edge:42"))
        .expect("declaration should admit");

    let legal = handle
        .review_legality(declaration)
        .expect("legality review should pass");

    assert_eq!(
        legal.reviewed_aspect_coverage().present(),
        &["selection.active_edge".to_string()]
    );
    assert_eq!(
        legal.reviewed_aspect_coverage().masked(),
        &["selection.active_edge".to_string()]
    );
}
