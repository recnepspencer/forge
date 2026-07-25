use worth_store_layout_indexes::evolution::migration::{
    layout_migration_execution, layout_rollback_execution, layout_rollback_operation,
    LayoutInterruptionPolicy, LayoutMigrationExecutionCaseId, LayoutMigrationExecutionRequest,
    LayoutRollbackExecutionCaseId, LayoutRollbackExecutionRequest, LayoutRollbackPlan,
    LayoutRollbackRequest,
};
use worth_store_layout_indexes::{ObserveOwnerCase, OwnerCaseObservation};

use super::world;
use crate::harness::physical_isolation::publication;

pub(super) fn observe() -> (
    Vec<OwnerCaseObservation<LayoutMigrationExecutionCaseId>>,
    Vec<OwnerCaseObservation<LayoutRollbackExecutionCaseId>>,
) {
    (observe_migration(), observe_rollback())
}

fn observe_migration() -> Vec<OwnerCaseObservation<LayoutMigrationExecutionCaseId>> {
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let authority = world::authority("store.layout_evolution.migration_execution");
    let foreign = world::authority("store.layout_evolution.migration_execution.foreign");

    let published = execute_migration_request(
        LayoutMigrationExecutionRequest::new(
            world::migration_plan(declaration, &authority),
            world::publication_plan(&authority, 12_001),
        ),
        None,
    );
    let store_mismatch = execute_migration_request(
        LayoutMigrationExecutionRequest::new(
            world::migration_plan(declaration, &authority),
            world::publication_plan(&foreign, 12_002),
        ),
        None,
    );
    let prior_inputs = world::physical_inputs(&authority, 12_003);
    let prior_publication = publication::publish_inputs(&prior_inputs);
    let source_mismatch = execute_migration_request(
        LayoutMigrationExecutionRequest::new(
            world::migration_plan(declaration, &authority),
            world::successor_publication_plan(&prior_publication, &authority, 12_004),
        ),
        None,
    );
    let stale_inputs = world::physical_inputs(&authority, 12_005);
    let physical_denial = execute_migration_request(
        LayoutMigrationExecutionRequest::new(
            world::migration_plan(declaration, &authority),
            publication::admitted_copy_on_write_plan(&stale_inputs),
        ),
        Some(stale_inputs.new_root),
    );

    [published, store_mismatch, source_mismatch, physical_denial]
        .into_iter()
        .map(|outcome| outcome.owner_case_observation())
        .collect()
}

fn execute_migration_request(
    request: LayoutMigrationExecutionRequest,
    current_root: Option<worth_store_physical_isolation::CurrentPhysicalRoot>,
) -> worth_store_layout_indexes::evolution::migration::LayoutMigrationExecutionOutcome {
    let root = current_root.unwrap_or_else(|| request.publication_source_root());
    let mut runtime =
        crate::harness::physical_isolation::PhysicalRootPublicationFixture::open(root).unwrap();
    layout_migration_execution(&mut runtime).execute(request)
}

fn observe_rollback() -> Vec<OwnerCaseObservation<LayoutRollbackExecutionCaseId>> {
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let authority = world::authority("store.layout_evolution.rollback_execution");
    let foreign = world::authority("store.layout_evolution.rollback_execution.foreign");

    let published = rollback_request(declaration, &authority, 12_101);
    let store_mismatch = rollback_request(declaration, &authority, 12_102);
    let source_mismatch = rollback_request(declaration, &authority, 12_103);
    let physical_denial = rollback_request(declaration, &authority, 12_104);

    let published = execute_rollback_request(published.0, None);
    let store_mismatch = LayoutRollbackExecutionRequest::new(
        store_mismatch.1,
        world::publication_plan(&foreign, 12_105),
    );
    let store_mismatch = execute_rollback_request(store_mismatch, None);
    let source_mismatch = LayoutRollbackExecutionRequest::new(
        source_mismatch.1,
        world::publication_plan(&authority, 12_106),
    );
    let source_mismatch = execute_rollback_request(source_mismatch, None);
    let stale_current = physical_denial.2.old_root;
    let physical_denial = execute_rollback_request(physical_denial.0, Some(stale_current));

    [published, store_mismatch, source_mismatch, physical_denial]
        .into_iter()
        .map(|outcome| outcome.owner_case_observation())
        .collect()
}

fn rollback_request(
    declaration: worth_store_layout_indexes::evolution::migration::LayoutEvolutionDeclaration,
    authority: &worth_store_authority::StoreCurrentAuthorityWitness,
    generation: u64,
) -> (
    LayoutRollbackExecutionRequest,
    LayoutRollbackPlan,
    publication::PublicationInputs,
) {
    let (migrated, inputs) =
        world::execute_migration_with_inputs(declaration, authority, generation);
    let target = migrated.target_binding().clone();
    let plan = layout_rollback_operation()
        .plan(
            LayoutRollbackRequest::new(declaration, target.clone(), target.admitted_family()),
            authority,
        )
        .into_ready()
        .unwrap();
    let publication =
        world::successor_publication_plan(migrated.publication(), authority, generation + 1);
    (
        LayoutRollbackExecutionRequest::new(plan.clone(), publication),
        plan,
        inputs,
    )
}

fn execute_rollback_request(
    request: LayoutRollbackExecutionRequest,
    current_root: Option<worth_store_physical_isolation::CurrentPhysicalRoot>,
) -> worth_store_layout_indexes::evolution::migration::LayoutRollbackExecutionOutcome {
    let root = current_root.unwrap_or_else(|| request.publication_source_root());
    let mut runtime =
        crate::harness::physical_isolation::PhysicalRootPublicationFixture::open(root).unwrap();
    layout_rollback_execution(&mut runtime).execute(request)
}
