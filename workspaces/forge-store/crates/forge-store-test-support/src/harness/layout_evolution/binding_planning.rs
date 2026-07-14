use forge_store_layout_indexes::evolution::migration::{
    layout_migration_operation, layout_rollback_operation, LayoutBindingAdmissionCaseId,
    LayoutInterruptionPolicy, LayoutMigrationRequest, LayoutRollbackRequest,
    MigrationPlanningCaseId, RollbackPlanningCaseId,
};
use forge_store_layout_indexes::{ObserveOwnerCase, OwnerCaseObservation};

use super::world;

pub(super) fn observe() -> (
    Vec<OwnerCaseObservation<LayoutBindingAdmissionCaseId>>,
    Vec<OwnerCaseObservation<MigrationPlanningCaseId>>,
    Vec<OwnerCaseObservation<RollbackPlanningCaseId>>,
) {
    (
        observe_binding(),
        observe_migration_planning(),
        observe_rollback_planning(),
    )
}

fn observe_binding() -> Vec<OwnerCaseObservation<LayoutBindingAdmissionCaseId>> {
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let authority = world::authority("store.layout_evolution.binding");
    let foreign_authority = world::authority("store.layout_evolution.binding.foreign");

    let admitted = world::binding_outcome(
        declaration,
        world::admitted_family(declaration.family_declaration(), &authority),
        authority.clone(),
        world::compatibility(declaration),
        world::physical_inputs(&authority, "binding-admitted", 11_001).old_candidate,
    );
    let family_mismatch = world::binding_outcome(
        declaration,
        world::admitted_family(world::other_declared_family(), &authority),
        authority.clone(),
        world::compatibility(declaration),
        world::physical_inputs(&authority, "binding-family", 11_002).old_candidate,
    );
    let store_mismatch = world::binding_outcome(
        declaration,
        world::admitted_family(declaration.family_declaration(), &authority),
        foreign_authority.clone(),
        world::compatibility(declaration),
        world::physical_inputs(&foreign_authority, "binding-store", 11_003).old_candidate,
    );
    let physical_source_mismatch = world::binding_outcome(
        declaration,
        world::admitted_family(declaration.family_declaration(), &authority),
        authority.clone(),
        world::compatibility(declaration),
        world::physical_inputs(&foreign_authority, "binding-physical", 11_004).old_candidate,
    );
    let compatibility_mismatch = world::binding_outcome(
        declaration,
        world::admitted_family(declaration.family_declaration(), &authority),
        authority.clone(),
        world::foreign_compatibility(),
        world::physical_inputs(&authority, "binding-compatibility", 11_005).old_candidate,
    );

    [
        admitted,
        family_mismatch,
        store_mismatch,
        physical_source_mismatch,
        compatibility_mismatch,
    ]
    .into_iter()
    .map(|outcome| outcome.owner_case_observation())
    .collect()
}

fn observe_migration_planning() -> Vec<OwnerCaseObservation<MigrationPlanningCaseId>> {
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let authority = world::authority("store.layout_evolution.migration_planning");
    let replacement = world::authority("store.layout_evolution.migration_planning.rebind");
    let source = world::source_binding(declaration, &authority);

    let ready = layout_migration_operation().plan(
        LayoutMigrationRequest::new(declaration, source.clone(), source.admitted_family()),
        &authority,
    );
    let foreign_declaration = world::declaration_for_family(
        world::other_declared_family(),
        LayoutInterruptionPolicy::ResumeDeclaredMigration,
    );
    let foreign = world::source_binding(foreign_declaration, &authority);
    let denied = layout_migration_operation().plan(
        LayoutMigrationRequest::new(declaration, foreign.clone(), foreign.admitted_family()),
        &authority,
    );
    let rebind = layout_migration_operation().plan(
        LayoutMigrationRequest::new(declaration, source.clone(), source.admitted_family()),
        &replacement,
    );

    [ready, denied, rebind]
        .into_iter()
        .map(|outcome| outcome.owner_case_observation())
        .collect()
}

fn observe_rollback_planning() -> Vec<OwnerCaseObservation<RollbackPlanningCaseId>> {
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let authority = world::authority("store.layout_evolution.rollback_planning");
    let replacement = world::authority("store.layout_evolution.rollback_planning.rebind");
    let migrated = world::execute_migration(declaration, &authority, "rollback-plan", 11_101);
    let target = migrated.target_binding().clone();

    let ready = layout_rollback_operation().plan(
        LayoutRollbackRequest::new(declaration, target.clone(), target.admitted_family()),
        &authority,
    );
    let foreign_declaration = world::declaration_for_family(
        world::other_declared_family(),
        LayoutInterruptionPolicy::ResumeDeclaredMigration,
    );
    let foreign_migrated = world::execute_migration(
        foreign_declaration,
        &authority,
        "rollback-plan-foreign",
        11_102,
    );
    let foreign = foreign_migrated.target_binding().clone();
    let denied = layout_rollback_operation().plan(
        LayoutRollbackRequest::new(declaration, foreign.clone(), foreign.admitted_family()),
        &authority,
    );
    let rebind = layout_rollback_operation().plan(
        LayoutRollbackRequest::new(declaration, target.clone(), target.admitted_family()),
        &replacement,
    );

    [ready, denied, rebind]
        .into_iter()
        .map(|outcome| outcome.owner_case_observation())
        .collect()
}
