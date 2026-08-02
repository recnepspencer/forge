use super::test_support::{
    binding, current_authority, declaration, migration_request, other_family_binding,
    source_binding_for_declaration, version,
};
use super::{
    layout_backward_read_compatibility, layout_backward_read_compatibility_cases,
    LayoutBackwardReadRequest, LayoutBackwardReadView,
};
use worth_store_compatibility::{ArtifactCompatibilityWindow, ArtifactFormatVersion};

mod planning_coverage;

#[test]
fn compatible_old_layout_reads_through_explicit_compatibility_lane() {
    let declaration = declaration();
    let authority = current_authority("store.compatibility.backward", "current");
    let binding = source_binding_for_declaration(declaration, authority);
    let compatibility = declaration
        .compatibility_window()
        .artifact_window()
        .admit_backward_read(binding.bound_version().format_version())
        .expect("declared source format should be backward readable");

    let outcome = layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
        declaration,
        &binding,
        compatibility,
    ));

    assert!(matches!(
        outcome.view(),
        LayoutBackwardReadView::Admitted(_)
    ));
}

#[test]
fn compatibility_witness_from_another_window_cannot_authorize_layout_reading() {
    let declaration = declaration();
    let authority = current_authority("store.compatibility.window", "current");
    let binding = source_binding_for_declaration(declaration, authority);
    let other_window = ArtifactCompatibilityWindow::new(
        ArtifactFormatVersion(4),
        ArtifactFormatVersion(5),
        ArtifactFormatVersion(6),
    )
    .unwrap();
    let compatibility = other_window
        .admit_backward_read(ArtifactFormatVersion(5))
        .unwrap();

    let outcome = layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
        declaration,
        &binding,
        compatibility,
    ));

    assert!(matches!(
        outcome.view(),
        LayoutBackwardReadView::Denied(
            super::LayoutEvolutionDenial::CompatibilityAdmissionMismatch
        )
    ));
}

#[test]
fn equal_looking_format_witness_cannot_substitute_for_the_bound_version() {
    let declaration = declaration();
    let authority = current_authority("store.compatibility.binding", "current");
    let binding = source_binding_for_declaration(declaration, authority);
    let compatibility = declaration
        .compatibility_window()
        .artifact_window()
        .admit_backward_read(ArtifactFormatVersion(6))
        .unwrap();

    let outcome = layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
        declaration,
        &binding,
        compatibility,
    ));

    assert!(matches!(
        outcome.view(),
        LayoutBackwardReadView::Denied(
            super::LayoutEvolutionDenial::CompatibilityBindingVersionMismatch { .. }
        )
    ));
}

#[test]
fn binding_from_another_declaration_cannot_smuggle_an_undeclared_semantic_version() {
    let declaration = declaration();
    let foreign_declaration = super::LayoutEvolutionDeclaration::new(
        declaration.family(),
        declaration.layout_version(),
        declaration.compatibility_window(),
        version(5, 9, 9),
        declaration.migration_target(),
        declaration.rollback_source(),
        declaration.rollback_target(),
        declaration.interruption_policy(),
    );
    let authority = current_authority("store.compatibility.semantic", "current");
    let binding = source_binding_for_declaration(foreign_declaration, authority);
    let compatibility = declaration
        .compatibility_window()
        .artifact_window()
        .admit_backward_read(binding.bound_version().format_version())
        .unwrap();

    let outcome = layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
        declaration,
        &binding,
        compatibility,
    ));

    assert!(matches!(
        outcome.view(),
        LayoutBackwardReadView::Denied(
            super::LayoutEvolutionDenial::UndeclaredCompatibleLayoutVersion { .. }
        )
    ));
}

#[test]
fn backward_read_owner_declares_exactly_the_cases_ordinary_requests_emit() {
    use std::collections::BTreeSet;

    let declaration = declaration();
    let authority = current_authority("store.compatibility.case-coverage", "current");
    let binding = source_binding_for_declaration(declaration, authority.clone());
    let admitted = declaration
        .compatibility_window()
        .artifact_window()
        .admit_backward_read(ArtifactFormatVersion(5))
        .unwrap();
    let wrong_binding = declaration
        .compatibility_window()
        .artifact_window()
        .admit_backward_read(ArtifactFormatVersion(6))
        .unwrap();
    let other_window = ArtifactCompatibilityWindow::new(
        ArtifactFormatVersion(4),
        ArtifactFormatVersion(5),
        ArtifactFormatVersion(6),
    )
    .unwrap()
    .admit_backward_read(ArtifactFormatVersion(5))
    .unwrap();
    let foreign_declaration = super::LayoutEvolutionDeclaration::new(
        declaration.family(),
        declaration.layout_version(),
        declaration.compatibility_window(),
        version(5, 9, 9),
        declaration.migration_target(),
        declaration.rollback_source(),
        declaration.rollback_target(),
        declaration.interruption_policy(),
    );
    let foreign_binding = source_binding_for_declaration(foreign_declaration, authority);

    let observed = [
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            admitted,
        )),
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            other_window,
        )),
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            wrong_binding,
        )),
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &foreign_binding,
            admitted,
        )),
    ]
    .into_iter()
    .map(|outcome| outcome.case_id())
    .collect::<BTreeSet<_>>();

    assert_eq!(
        observed,
        layout_backward_read_compatibility_cases().collect::<BTreeSet<_>>()
    );
}

#[test]
fn generic_binding_admission_cannot_mint_a_target_published_version() {
    let current = current_authority("store.migration.binding.source_only", "current");
    let admitted = source_binding_for_declaration(declaration(), current);

    assert_eq!(admitted.bound_version(), declaration().migration_source());
    assert_eq!(
        admitted.observed_version(),
        declaration().migration_source()
    );
    assert_ne!(admitted.bound_version(), declaration().migration_target());
}
