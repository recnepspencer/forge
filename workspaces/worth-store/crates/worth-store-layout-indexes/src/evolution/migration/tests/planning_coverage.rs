use super::*;

#[test]
fn migration_requires_rebind_when_current_authority_changes() {
    let bound = current_authority("store.new.rebind", "bound");
    let current = current_authority("store.new.rebind.current", "current");

    let outcome = layout_migration_operation().plan(
        migration_request(
            declaration(),
            binding(version(5, 1, 0), version(5, 1, 0), bound),
        ),
        &current,
    );

    assert!(matches!(
        outcome.view(),
        super::super::MigrationPlanningView::LoweringRebindRequired(_)
    ));
}

#[test]
fn migration_and_rollback_require_rebind_across_tenant_or_key_scope() {
    let current = current_authority("store.evolution.scope", "current");
    let source = binding(
        declaration().migration_source(),
        declaration().migration_source(),
        current.clone(),
    );

    for current_family in [
        admitted_family_for_scope(
            declaration().family().declaration(),
            &current,
            StoreKeyScope::TenantEnvelope,
            StoreTenantScope::StoreInternal,
        ),
        admitted_family_for_scope(
            declaration().family().declaration(),
            &current,
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::MultiTenantPhysicalBoundary,
        ),
    ] {
        let migration = layout_migration_operation().plan(
            super::super::LayoutMigrationRequest::new(
                declaration(),
                source.clone(),
                current_family,
            ),
            &current,
        );
        assert!(matches!(
            migration.view(),
            super::super::MigrationPlanningView::LoweringRebindRequired(_)
        ));
    }

    let migrated = migrated_binding(declaration(), &current);
    for current_family in [
        admitted_family_for_scope(
            declaration().family().declaration(),
            &current,
            StoreKeyScope::TenantEnvelope,
            StoreTenantScope::StoreInternal,
        ),
        admitted_family_for_scope(
            declaration().family().declaration(),
            &current,
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::MultiTenantPhysicalBoundary,
        ),
    ] {
        let rollback = layout_rollback_operation().plan(
            super::super::LayoutRollbackRequest::new(
                declaration(),
                migrated.target_binding().clone(),
                current_family,
            ),
            &current,
        );
        assert!(matches!(
            rollback.view(),
            super::super::RollbackPlanningView::LoweringRebindRequired(_)
        ));
    }
}

#[test]
fn migration_and_rollback_declare_exactly_the_cases_ordinary_planning_emits() {
    use std::collections::BTreeSet;

    let current = current_authority("store.migration.case.coverage", "current");
    let rebound = current_authority("store.migration.case.coverage.rebound", "rebound");

    let migration_observed = [
        layout_migration_operation().plan(
            migration_request(
                declaration(),
                binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        ),
        layout_migration_operation().plan(
            migration_request(
                declaration(),
                other_family_binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        ),
        layout_migration_operation().plan(
            migration_request(
                declaration(),
                binding(version(5, 1, 0), version(5, 1, 0), rebound.clone()),
            ),
            &current,
        ),
    ]
    .into_iter()
    .map(|outcome| outcome.case_id())
    .collect::<BTreeSet<_>>();

    let rollback_observed = [
        layout_rollback_operation().plan(
            rollback_request(
                declaration(),
                migrated_binding(declaration(), &current)
                    .target_binding()
                    .clone(),
            ),
            &current,
        ),
        layout_rollback_operation().plan(
            rollback_request(
                declaration(),
                other_family_migrated_binding(&current)
                    .target_binding()
                    .clone(),
            ),
            &current,
        ),
        layout_rollback_operation().plan(
            rollback_request(
                declaration(),
                migrated_binding(declaration(), &rebound)
                    .target_binding()
                    .clone(),
            ),
            &current,
        ),
    ]
    .into_iter()
    .map(|outcome| outcome.case_id())
    .collect::<BTreeSet<_>>();

    assert_eq!(
        migration_observed,
        super::super::migration_planning_cases().collect::<BTreeSet<_>>()
    );
    assert_eq!(
        rollback_observed,
        super::super::rollback_planning_cases().collect::<BTreeSet<_>>()
    );
}
