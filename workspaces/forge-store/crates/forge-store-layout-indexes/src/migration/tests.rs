use forge_proof::TransitionOutcome;

use super::test_support::{binding, current_authority, declaration, other_family_binding, version};
use super::{
    layout_migration, LayoutCompatibilityWindow, LayoutInterruptedMigrationDisposition,
    LayoutInterruptionPolicy, LayoutReadCompatibilityPosture, LayoutRollbackRequest,
    LayoutWriteCompatibilityPosture,
};
use forge_store_compatibility::ArtifactFormatVersion;

#[test]
fn compatible_old_layout_reads_through_explicit_compatibility_lane() {
    let declaration = declaration();

    assert!(layout_migration()
        .require_backward_compatible_read(version(5, 1, 0), declaration)
        .is_ok());
}

#[test]
fn incompatible_layouts_deny_with_typed_reason() {
    let declaration = declaration();

    let denial = layout_migration()
        .require_backward_compatible_read(version(4, 0, 9), declaration)
        .unwrap_err();

    assert!(matches!(
        denial,
        super::LayoutEvolutionDenial::IncompatibleSourceVersion { .. }
    ));
}

#[test]
fn same_format_with_undeclared_semantic_version_does_not_count_as_compatible_read() {
    let declaration = declaration();

    let denial = layout_migration()
        .require_backward_compatible_read(version(5, 9, 9), declaration)
        .unwrap_err();

    assert!(matches!(
        denial,
        super::LayoutEvolutionDenial::UndeclaredCompatibleLayoutVersion { .. }
    ));
}

#[test]
fn interrupted_migration_resumes_or_rolls_back_according_to_declaration() {
    let current = current_authority("store.s8.migration", "current");
    let plan = match layout_migration().plan_migration(
        super::LayoutMigrationRequest::new(
            declaration(),
            binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
        ),
        &current,
    ).into_transition_outcome() {
        TransitionOutcome::Success(plan) => plan,
        outcome => panic!("migration plan should be ready: {outcome:?}"),
    };

    let resumed = match plan.resume_or_rollback(plan.interruption_state()) {
        TransitionOutcome::Success(resumed) => resumed,
        outcome => panic!("resume path should stay admitted: {outcome:?}"),
    };
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
    let rollback_plan = match layout_migration().plan_migration(
        super::LayoutMigrationRequest::new(
            rollback_decl,
            binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
        ),
        &current,
    ).into_transition_outcome() {
        TransitionOutcome::Success(plan) => plan,
        outcome => panic!("rollback-interrupt plan should be ready: {outcome:?}"),
    };
    let rollback = match rollback_plan.resume_or_rollback(rollback_plan.interruption_state()) {
        TransitionOutcome::Success(rollback) => rollback,
        outcome => panic!("rollback path should stay admitted: {outcome:?}"),
    };
    assert!(matches!(
        rollback,
        LayoutInterruptedMigrationDisposition::Rollback(_)
    ));
}

#[test]
fn interruption_state_rejects_declaration_drift_with_same_migration_pair() {
    let current = current_authority("store.s8.migration.drift", "current");
    let base_plan = match layout_migration().plan_migration(
        super::LayoutMigrationRequest::new(
            declaration(),
            binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
        ),
        &current,
    ).into_transition_outcome() {
        TransitionOutcome::Success(plan) => plan,
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
    let drifted_plan = match layout_migration().plan_migration(
        super::LayoutMigrationRequest::new(
            drifted_decl,
            binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
        ),
        &current,
    ).into_transition_outcome() {
        TransitionOutcome::Success(plan) => plan,
        outcome => panic!("drifted migration plan should be ready: {outcome:?}"),
    };

    let replay = drifted_plan.resume_or_rollback(base_plan.interruption_state());
    assert!(matches!(
        replay,
        TransitionOutcome::Denied(
            super::LayoutEvolutionDenial::InterruptStateDoesNotMatchPlan { .. }
        )
    ));
}

#[test]
fn rollback_preserves_authority_and_rejects_stale_projection_truth() {
    let current = current_authority("store.s8.rollback", "current");
    let request = LayoutRollbackRequest::new(
        declaration(),
        binding(version(7, 2, 1), version(7, 2, 1), current.clone()),
    );

    let plan = match layout_migration()
        .plan_rollback(request, &current)
        .into_transition_outcome()
    {
        TransitionOutcome::Success(plan) => plan,
        outcome => panic!("rollback plan should be ready: {outcome:?}"),
    };

    assert_eq!(plan.authority().identity(), current.identity());
    assert_eq!(plan.rollback_target(), version(5, 1, 0));

    let stale_request = LayoutRollbackRequest::new(
        declaration(),
        binding(version(6, 9, 9), version(7, 2, 1), current.clone()),
    );
    let stale = layout_migration()
        .plan_rollback(stale_request, &current)
        .into_transition_outcome();
    assert!(matches!(stale, TransitionOutcome::Stale(_)));
}

#[test]
fn rollback_rejects_binding_from_wrong_family_even_when_versions_match() {
    let current = current_authority("store.s8.rollback.family", "current");
    let request = LayoutRollbackRequest::new(
        declaration(),
        other_family_binding(version(7, 2, 1), version(7, 2, 1), current.clone()),
    );

    let outcome = layout_migration()
        .plan_rollback(request, &current)
        .into_transition_outcome();
    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(super::LayoutEvolutionDenial::FamilyMismatch { .. })
    ));
}

#[test]
fn migration_requires_rebind_when_current_authority_changes() {
    let bound = current_authority("store.s8.rebind", "bound");
    let current = current_authority("store.s8.rebind.current", "current");

    let outcome = layout_migration().plan_migration(
        super::LayoutMigrationRequest::new(
            declaration(),
            binding(version(5, 1, 0), version(5, 1, 0), bound),
        ),
        &current,
    ).into_transition_outcome();

    assert!(matches!(outcome, TransitionOutcome::RebindRequired(_)));
}
