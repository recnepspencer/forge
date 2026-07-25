use std::collections::BTreeSet;

use worth_store_compatibility::ArtifactFormatVersion;

use super::test_support::{
    binding, current_authority, declaration, migrated_binding, migration_execution_request,
    migration_execution_request_for_publication, migration_request, version,
};
use super::{
    layout_migration_interruption_cases, layout_migration_operation, LayoutCompatibilityWindow,
    LayoutEvolutionDeclaration, LayoutInterruptedMigrationDisposition, LayoutInterruptionPolicy,
    LayoutMigrationPlan, LayoutReadCompatibilityPosture, LayoutWriteCompatibilityPosture,
};

#[test]
fn independently_reconstructed_resume_plan_accepts_the_same_interruption() {
    let first_authority = current_authority("store.migration.replay", "current");
    let first = plan(declaration(), &first_authority);
    let interruption = first.interruption_state();
    drop(first);

    let replay_authority = current_authority("store.migration.replay", "current");
    let replayed = plan(declaration(), &replay_authority);
    let disposition = replayed
        .resume_or_rollback(interruption)
        .into_result()
        .expect("the same durable declaration and authority must reconstruct the same plan");

    assert!(matches!(
        disposition,
        LayoutInterruptedMigrationDisposition::Resume(state)
            if state.fingerprint()
                == super::LayoutInterruptionFingerprint::plan(replayed.fingerprint())
    ));
}

#[test]
fn independently_reconstructed_rollback_plan_preserves_rollback_posture() {
    let first_authority = current_authority("store.migration.rollback.replay", "current");
    let declared = rollback_declaration();
    let interruption = migrated_binding(declared, &first_authority).interruption_state();

    let replay_authority = current_authority("store.migration.rollback.replay", "current");
    let replayed = migration_execution_request(declared, &replay_authority);
    let disposition = replayed
        .resume_or_rollback(interruption)
        .into_result()
        .expect("rollback policy must survive independent plan reconstruction");

    let LayoutInterruptedMigrationDisposition::Rollback(request) = disposition else {
        panic!("rollback policy reconstructed as resume");
    };
    assert!(super::layout_rollback_operation()
        .plan(request, &replay_authority)
        .into_ready()
        .is_ok());
}

#[test]
fn interruption_owner_declares_exactly_the_cases_ordinary_plans_emit() {
    let authority = current_authority("store.migration.interruption.cases", "current");
    let resume = plan(declaration(), &authority);
    let rollback = plan(rollback_declaration(), &authority);
    let target_interruption =
        migrated_binding(rollback_declaration(), &authority).interruption_state();
    let target_execution = migration_execution_request(rollback_declaration(), &authority);

    let observed = [
        resume
            .resume_or_rollback(resume.interruption_state())
            .case_id(),
        rollback
            .resume_or_rollback(rollback.interruption_state())
            .case_id(),
        target_execution
            .resume_or_rollback(target_interruption)
            .case_id(),
        rollback
            .resume_or_rollback(resume.interruption_state())
            .case_id(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let declared = layout_migration_interruption_cases().collect::<BTreeSet<_>>();

    assert_eq!(observed, declared);
}

#[test]
fn interruption_boundary_rejects_foreign_authority() {
    let authority = current_authority("store.migration.boundary", "current");
    let planned = plan(declaration(), &authority);
    let foreign = current_authority("store.migration.boundary.foreign", "current");

    let foreign_state = migrated_binding(declaration(), &foreign).interruption_state();
    assert!(matches!(
        planned.resume_or_rollback(foreign_state).into_result(),
        Err(super::LayoutEvolutionDenial::InterruptStateDoesNotMatchPlan { .. })
    ));
}

#[test]
fn target_publication_interruption_rejects_a_different_physical_execution() {
    let authority = current_authority("store.migration.execution.binding", "current");
    let declared = rollback_declaration();
    let first = migration_execution_request_for_publication(declared, &authority, 2_101);
    let mut publication =
        worth_store_test_support::harness::physical_isolation::PhysicalRootPublicationFixture::open(
            first.publication_source_root(),
        )
        .unwrap();
    let interruption = super::layout_migration_execution(&mut publication)
        .execute(first)
        .into_published()
        .expect("first physical migration must publish")
        .interruption_state();
    let different = migration_execution_request_for_publication(declared, &authority, 2_102);

    assert!(matches!(
        different.resume_or_rollback(interruption).into_result(),
        Err(super::LayoutEvolutionDenial::InterruptStateDoesNotMatchPlan { .. })
    ));
}

fn plan(
    declared: LayoutEvolutionDeclaration,
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
) -> LayoutMigrationPlan {
    layout_migration_operation()
        .plan(
            migration_request(
                declared,
                binding(version(5, 1, 0), version(5, 1, 0), authority.clone()),
            ),
            authority,
        )
        .into_ready()
        .expect("ordinary migration request must produce a ready plan")
}

fn rollback_declaration() -> LayoutEvolutionDeclaration {
    LayoutEvolutionDeclaration::new(
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
    )
}
