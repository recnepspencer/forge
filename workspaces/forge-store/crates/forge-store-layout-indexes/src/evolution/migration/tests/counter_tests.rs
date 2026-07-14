use super::super::test_support::{
    current_authority, declaration, migrated_binding, rollback_publication_plan, rollback_request,
    version,
};
use super::super::{
    layout_rollback_execution, layout_rollback_operation, LayoutRollbackExecutionRequest,
};

#[test]
fn migration_and_rollback_publish_exact_version_binding_counters() {
    let current = current_authority("store.new.evolution.execution", "current");
    let migrated = migrated_binding(declaration(), &current);
    assert_eq!(migrated.source_binding().bound_version(), version(5, 1, 0));
    assert_eq!(migrated.target_binding().bound_version(), version(7, 2, 1));
    assert_eq!(migrated.counters().target_bindings_published(), 1);
    assert_eq!(migrated.counters().physical_publication().root_swaps(), 1);

    let rollback = layout_rollback_operation()
        .plan(
            rollback_request(declaration(), migrated.target_binding().clone()),
            &current,
        )
        .into_ready()
        .expect("executed migration must authorize declared rollback planning");
    let request = LayoutRollbackExecutionRequest::new(
        rollback,
        rollback_publication_plan(&current, "layout-rollback-publication", 1_903),
    );
    let mut publication =
        forge_store_physical_isolation::PhysicalRootPublicationRuntime::from_current_root(
            request.publication_source_root(),
        );
    let receipt = layout_rollback_execution(&mut publication)
        .execute(request)
        .into_published()
        .expect("rollback must publish through physical copy-on-write");
    assert_eq!(receipt.source_binding().bound_version(), version(7, 2, 1));
    assert_eq!(receipt.target_binding().bound_version(), version(5, 1, 0));
    assert_eq!(receipt.counters().rollback_bindings_published(), 1);
    assert_eq!(receipt.counters().physical_publication().root_swaps(), 1);
}
