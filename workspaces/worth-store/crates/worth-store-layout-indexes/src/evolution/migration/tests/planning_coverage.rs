use super::*;

#[test]
fn migration_requires_rebind_when_current_authority_changes() {
    let bound = current_authority("store.new.rebind", "bound");
    let current = current_authority("store.new.rebind.current", "current");

    let outcome = super::super::layout_migration_operation().plan(
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
fn migration_rejects_cross_scope_family_bindings() {
    let current = current_authority("store.evolution.scope", "current");
    let source = binding(
        declaration().migration_source(),
        declaration().migration_source(),
        current.clone(),
    );

    for current_family in [
        super::super::test_support::admitted_family_for_scope(
            declaration().family().declaration(),
            &current,
            worth_store_security::StoreKeyScope::TenantEnvelope,
            worth_store_security::StoreTenantScope::StoreInternal,
        ),
        super::super::test_support::admitted_family_for_scope(
            declaration().family().declaration(),
            &current,
            worth_store_security::StoreKeyScope::StoreManagedRoot,
            worth_store_security::StoreTenantScope::MultiTenantPhysicalBoundary,
        ),
    ] {
        let migration = super::super::layout_migration_operation().plan(
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
}

#[test]
fn migration_declares_exactly_the_cases_ordinary_planning_emits() {
    use std::collections::BTreeSet;

    let current = current_authority("store.migration.case.coverage", "current");
    let rebound = current_authority("store.migration.case.coverage.rebound", "rebound");
    let observed = [
        super::super::layout_migration_operation().plan(
            migration_request(
                declaration(),
                binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        ),
        super::super::layout_migration_operation().plan(
            migration_request(
                declaration(),
                other_family_binding(version(5, 1, 0), version(5, 1, 0), current.clone()),
            ),
            &current,
        ),
        super::super::layout_migration_operation().plan(
            migration_request(
                declaration(),
                binding(version(5, 1, 0), version(5, 1, 0), rebound),
            ),
            &current,
        ),
    ]
    .into_iter()
    .map(|outcome| outcome.case_id())
    .collect::<BTreeSet<_>>();

    assert_eq!(
        observed,
        super::super::migration_planning_cases().collect::<BTreeSet<_>>()
    );
}
