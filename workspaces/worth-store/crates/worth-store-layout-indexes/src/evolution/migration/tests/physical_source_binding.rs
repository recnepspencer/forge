use super::super::test_support::{
    current_authority, declaration, migrated_binding, migration_request, publication_plan,
    rollback_request, same_store_unbound_migration_source,
};
use super::super::{
    layout_migration_execution, layout_migration_operation, layout_rollback_execution,
    layout_rollback_operation, LayoutEvolutionDenial, LayoutMigrationExecutionRequest,
    LayoutRollbackExecutionRequest,
};

#[test]
fn migration_rejects_a_same_store_publication_from_an_unbound_source() {
    let authority = current_authority("store.migration.physical.source", "current");
    let declaration = declaration();
    let (source, unrelated_successor) =
        same_store_unbound_migration_source(declaration, &authority, 2_401);
    let migration = layout_migration_operation()
        .plan(migration_request(declaration, source), &authority)
        .into_ready()
        .expect("declared migration should plan");

    let request = LayoutMigrationExecutionRequest::new(migration, unrelated_successor);
    let mut publication =
        worth_store_test_support::harness::physical_isolation::PhysicalRootPublicationFixture::open(
            request.publication_source_root(),
        )
        .unwrap();
    let outcome = layout_migration_execution(&mut publication).execute(request);

    assert!(matches!(
        outcome.into_published(),
        Err(LayoutEvolutionDenial::PhysicalPublicationSourceMismatch { .. })
    ));
}

#[test]
fn rollback_rejects_a_same_store_publication_from_the_pre_migration_source() {
    let authority = current_authority("store.rollback.physical.source", "current");
    let declaration = declaration();
    let migrated = migrated_binding(declaration, &authority);
    let rollback = layout_rollback_operation()
        .plan(
            rollback_request(declaration, migrated.target_binding().clone()),
            &authority,
        )
        .into_ready()
        .expect("published migration should authorize rollback planning");
    let pre_migration_source = publication_plan(&authority, 2_402);

    let request = LayoutRollbackExecutionRequest::new(rollback, pre_migration_source);
    let mut publication =
        worth_store_test_support::harness::physical_isolation::PhysicalRootPublicationFixture::open(
            request.publication_source_root(),
        )
        .unwrap();
    let outcome = layout_rollback_execution(&mut publication).execute(request);

    assert!(matches!(
        outcome.into_published(),
        Err(LayoutEvolutionDenial::PhysicalPublicationSourceMismatch { .. })
    ));
}
