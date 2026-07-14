use super::test_support::{
    admitted_family_for_scope, binding, current_authority, declaration, migrated_binding,
    migration_request, other_family_binding, other_family_migrated_binding, rollback_request,
    source_binding_for_declaration, version,
};
use super::{
    layout_backward_read_compatibility, layout_backward_read_compatibility_cases,
    layout_migration_operation, layout_rollback_operation, LayoutBackwardReadRequest,
    LayoutBackwardReadView, LayoutCompatibilityWindow, LayoutInterruptedMigrationDisposition,
    LayoutInterruptionPolicy, LayoutReadCompatibilityPosture, LayoutWriteCompatibilityPosture,
};
use forge_store_compatibility::{ArtifactCompatibilityWindow, ArtifactFormatVersion};
use forge_store_security::{StoreKeyScope, StoreTenantScope};

mod counter_tests;
mod physical_source_binding;
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

#[test]
fn interrupted_migration_resumes_or_rolls_back_according_to_declaration() {
    let current = current_authority("store.new.migration", "current");
    let plan = match layout_migration_operation()
        .plan(
            migration_request(
                declaration(),
                binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        )
        .into_ready()
    {
        Ok(plan) => plan,
        outcome => panic!("migration plan should be ready: {outcome:?}"),
    };

    let resumed = plan
        .resume_or_rollback(plan.interruption_state())
        .into_result()
        .expect("resume path should stay admitted");
    assert!(matches!(
        resumed,
        LayoutInterruptedMigrationDisposition::Resume(_)
    ));

    let rollback_decl = super::LayoutEvolutionDeclaration::new(
        declaration().family(),
        declaration().layout_version(),
        LayoutCompatibilityWindow::new(
            ArtifactFormatVersion(5),
            ArtifactFormatVersion(7),
            ArtifactFormatVersion(7),
            LayoutReadCompatibilityPosture::ReadOldWriteNew,
            LayoutWriteCompatibilityPosture::WriteNewDuringRollingUpgrade,
        )
        .unwrap(),
        declaration().migration_source(),
        declaration().migration_target(),
        declaration().rollback_source(),
        declaration().rollback_target(),
        LayoutInterruptionPolicy::RollbackDeclaredMigration,
    );
    let rollback_plan = match layout_migration_operation()
        .plan(
            migration_request(
                rollback_decl,
                binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        )
        .into_ready()
    {
        Ok(plan) => plan,
        outcome => panic!("rollback-interrupt plan should be ready: {outcome:?}"),
    };
    let rollback = rollback_plan
        .resume_or_rollback(rollback_plan.interruption_state())
        .into_result()
        .expect("rollback path should stay admitted");
    assert!(matches!(
        rollback,
        LayoutInterruptedMigrationDisposition::RemainAtSource(_)
    ));
}

#[test]
fn interruption_state_rejects_declaration_drift_with_same_migration_pair() {
    let current = current_authority("store.new.migration.drift", "current");
    let base_plan = match layout_migration_operation()
        .plan(
            migration_request(
                declaration(),
                binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        )
        .into_ready()
    {
        Ok(plan) => plan,
        outcome => panic!("base migration plan should be ready: {outcome:?}"),
    };

    let drifted_decl = super::LayoutEvolutionDeclaration::new(
        declaration().family(),
        declaration().layout_version(),
        declaration().compatibility_window(),
        declaration().migration_source(),
        declaration().migration_target(),
        declaration().rollback_source(),
        version(4, 8, 0),
        LayoutInterruptionPolicy::RollbackDeclaredMigration,
    );
    let drifted_plan = match layout_migration_operation()
        .plan(
            migration_request(
                drifted_decl,
                binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        )
        .into_ready()
    {
        Ok(plan) => plan,
        outcome => panic!("drifted migration plan should be ready: {outcome:?}"),
    };

    let replay = drifted_plan.resume_or_rollback(base_plan.interruption_state());
    assert!(matches!(
        replay.into_result(),
        Err(super::LayoutEvolutionDenial::InterruptStateDoesNotMatchPlan { .. })
    ));
}

#[test]
fn rollback_preserves_authority_and_requires_rebind_after_authority_change() {
    let current = current_authority("store.new.rollback", "current");
    let migrated = migrated_binding(declaration(), &current);
    let request = rollback_request(declaration(), migrated.target_binding().clone());

    let plan = match layout_rollback_operation()
        .plan(request, &current)
        .into_ready()
    {
        Ok(plan) => plan,
        outcome => panic!("rollback plan should be ready: {outcome:?}"),
    };

    assert_eq!(plan.authority().identity(), current.identity());
    assert_eq!(plan.rollback_target(), version(5, 1, 0));

    let replacement_authority = current_authority("store.new.rollback.rebound", "replacement");
    let rebound = layout_rollback_operation().plan(
        rollback_request(declaration(), migrated.target_binding().clone()),
        &replacement_authority,
    );
    assert!(matches!(
        rebound.view(),
        super::RollbackPlanningView::LoweringRebindRequired(_)
    ));
}

#[test]
fn rollback_rejects_binding_from_wrong_family_even_when_versions_match() {
    let current = current_authority("store.new.rollback.family", "current");
    let request = rollback_request(
        declaration(),
        other_family_migrated_binding(&current)
            .target_binding()
            .clone(),
    );

    let outcome = layout_rollback_operation().plan(request, &current);
    assert!(matches!(
        outcome.view(),
        super::RollbackPlanningView::DeclarationDenied(
            super::LayoutEvolutionDenial::FamilyMismatch { .. }
        )
    ));
}
