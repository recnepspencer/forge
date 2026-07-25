use worth_store_compatibility::{ArtifactCompatibilityWindow, ArtifactFormatVersion};
use worth_store_layout_indexes::evolution::migration::{
    layout_backward_read_compatibility, layout_migration_execution, layout_rollback_execution,
    layout_rollback_operation, LayoutBackwardReadCompatibilityCaseId, LayoutBackwardReadRequest,
    LayoutCompatibilityWindow, LayoutEvolutionDeclaration, LayoutInterruptionPolicy,
    LayoutMigrationExecutionRequest, LayoutMigrationInterruptionCaseId,
    LayoutReadCompatibilityPosture, LayoutRollbackExecutionRequest,
    LayoutRollbackInterruptionCaseId, LayoutRollbackRequest, LayoutWriteCompatibilityPosture,
};
use worth_store_layout_indexes::{ObserveOwnerCase, OwnerCaseObservation};

use super::world;

type InterruptionCompatibilityObservations = (
    Vec<OwnerCaseObservation<LayoutMigrationInterruptionCaseId>>,
    Vec<OwnerCaseObservation<LayoutRollbackInterruptionCaseId>>,
    Vec<OwnerCaseObservation<LayoutBackwardReadCompatibilityCaseId>>,
);

pub(super) fn observe() -> InterruptionCompatibilityObservations {
    (
        observe_migration_interruption(),
        observe_rollback_interruption(),
        observe_backward_read(),
    )
}

fn observe_migration_interruption() -> Vec<OwnerCaseObservation<LayoutMigrationInterruptionCaseId>>
{
    let authority = world::authority("store.layout_evolution.migration_interruption");
    let resume_declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let resume_plan = world::migration_plan(resume_declaration, &authority);
    let resume = resume_plan.resume_or_rollback(resume_plan.interruption_state());

    let rollback_declaration =
        world::declaration(LayoutInterruptionPolicy::RollbackDeclaredMigration);
    let rollback_plan = world::migration_plan(rollback_declaration, &authority);
    let remain = rollback_plan.resume_or_rollback(rollback_plan.interruption_state());

    let request = LayoutMigrationExecutionRequest::new(
        world::migration_plan(rollback_declaration, &authority),
        world::publication_plan(&authority, 13_001),
    );
    let mut publication = crate::harness::physical_isolation::PhysicalRootPublicationFixture::open(
        request.publication_source_root(),
    )
    .unwrap();
    let target = layout_migration_execution(&mut publication)
        .execute(request)
        .into_published()
        .unwrap();
    let replay = LayoutMigrationExecutionRequest::new(
        world::migration_plan(rollback_declaration, &authority),
        world::publication_plan(&authority, 13_001),
    );
    let rollback = replay.resume_or_rollback(target.interruption_state());

    let different = LayoutMigrationExecutionRequest::new(
        world::migration_plan(rollback_declaration, &authority),
        world::publication_plan(&authority, 13_002),
    );
    let denied = different.resume_or_rollback(target.interruption_state());

    [resume, remain, rollback, denied]
        .into_iter()
        .map(|outcome| outcome.owner_case_observation())
        .collect()
}

fn observe_rollback_interruption() -> Vec<OwnerCaseObservation<LayoutRollbackInterruptionCaseId>> {
    let authority = world::authority("store.layout_evolution.rollback_interruption");
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let (migrated, _inputs) = world::execute_migration_with_inputs(declaration, &authority, 13_101);
    let target = migrated.target_binding().clone();
    let plan = layout_rollback_operation()
        .plan(
            LayoutRollbackRequest::new(declaration, target.clone(), target.admitted_family()),
            &authority,
        )
        .into_ready()
        .unwrap();
    let request = LayoutRollbackExecutionRequest::new(
        plan,
        world::successor_publication_plan(migrated.publication(), &authority, 13_102),
    );
    let source_state = request.interruption_state();
    let matching = request.clone();
    let resume = matching.classify_interruption(source_state);

    let mut publication = crate::harness::physical_isolation::PhysicalRootPublicationFixture::open(
        request.publication_source_root(),
    )
    .unwrap();
    let target_state = layout_rollback_execution(&mut publication)
        .execute(request)
        .into_published()
        .unwrap()
        .interruption_state();
    let matching = rollback_request_for(declaration, &authority, 13_101);
    let published = matching.classify_interruption(target_state.clone());
    let different_plan = layout_rollback_operation()
        .plan(
            LayoutRollbackRequest::new(declaration, target.clone(), target.admitted_family()),
            &authority,
        )
        .into_ready()
        .unwrap();
    let different = LayoutRollbackExecutionRequest::new(
        different_plan,
        world::successor_publication_plan(migrated.publication(), &authority, 13_103),
    );
    let denied = different.classify_interruption(target_state);

    [resume, published, denied]
        .into_iter()
        .map(|outcome| outcome.owner_case_observation())
        .collect()
}

fn rollback_request_for(
    declaration: LayoutEvolutionDeclaration,
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
    generation: u64,
) -> LayoutRollbackExecutionRequest {
    let (migrated, _inputs) =
        world::execute_migration_with_inputs(declaration, authority, generation);
    let target = migrated.target_binding().clone();
    let plan = layout_rollback_operation()
        .plan(
            LayoutRollbackRequest::new(declaration, target.clone(), target.admitted_family()),
            authority,
        )
        .into_ready()
        .unwrap();
    LayoutRollbackExecutionRequest::new(
        plan,
        world::successor_publication_plan(migrated.publication(), authority, generation + 1),
    )
}

fn observe_backward_read() -> Vec<OwnerCaseObservation<LayoutBackwardReadCompatibilityCaseId>> {
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let authority = world::authority("store.layout_evolution.backward_read");
    let binding = world::source_binding(declaration, &authority);
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
    let wrong_window = ArtifactCompatibilityWindow::new(
        ArtifactFormatVersion(4),
        ArtifactFormatVersion(5),
        ArtifactFormatVersion(6),
    )
    .unwrap()
    .admit_backward_read(ArtifactFormatVersion(5))
    .unwrap();
    let foreign_declaration = LayoutEvolutionDeclaration::from_admitted_family(
        world::admitted_family(world::declared_family(), &authority),
        declaration.layout_version(),
        LayoutCompatibilityWindow::new(
            ArtifactFormatVersion(5),
            ArtifactFormatVersion(7),
            ArtifactFormatVersion(7),
            LayoutReadCompatibilityPosture::ReadOldWriteNew,
            LayoutWriteCompatibilityPosture::WriteNewDuringRollingUpgrade,
        )
        .unwrap(),
        world::version(5, 9, 9),
        declaration.migration_target(),
        declaration.rollback_source(),
        declaration.rollback_target(),
        declaration.interruption_policy(),
    );
    let foreign_binding = world::source_binding(foreign_declaration, &authority);

    [
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            admitted,
        )),
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            wrong_window,
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
    .map(|outcome| outcome.owner_case_observation())
    .collect()
}
