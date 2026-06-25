use super::support::{
    execution_receipt, full_migration_sweep, inventory_closeout, operator_closeout,
    partial_migration_sweep, selected_plan_for_closeout,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::deletion_closeout::{
    close_derived_invalidation_deletion,
    close_derived_invalidation_deletion_with_source_firewall_for_tests,
    DerivedInvalidationDeletionSourceFirewall,
};
use crate::derived_topology::invalidation_plan::migrated_products::{
    CoveredDerivedProductMigrationError, CoveredDerivedProductMigrationStatus,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::catalog_closeout;

#[test]
fn milestone_ten_closeout_binds_every_upstream_receipt_and_seeds_milestone_eleven() {
    let catalog_closeout = catalog_closeout();
    let selected_plan = selected_plan_for_closeout(&catalog_closeout);
    let execution_receipt = execution_receipt(&selected_plan);
    let migration_sweep = full_migration_sweep(&selected_plan);
    let operator_closeout = operator_closeout(&migration_sweep, &selected_plan, &execution_receipt);
    let deletion_closeout = close_derived_invalidation_deletion_with_source_firewall_for_tests(
        operator_closeout.phase_eight_seed(),
        &migration_sweep,
        &inventory_closeout(),
        DerivedInvalidationDeletionSourceFirewall::from_sources_for_tests([]),
    )
    .expect("deletion closeout");

    let closeout = super::super::close_derived_invalidation_milestone_ten(
        &catalog_closeout,
        &selected_plan,
        &execution_receipt,
        &migration_sweep,
        &operator_closeout,
        &deletion_closeout,
    )
    .expect("milestone ten closeout");

    assert_eq!(
        closeout.selected_plan_digest(),
        selected_plan.selected_plan_digest()
    );
    assert_eq!(
        closeout.execution_receipt_digest(),
        execution_receipt.execution_receipt_digest()
    );
    assert_eq!(
        closeout.product_summary().rows().len(),
        DerivedTopologyProductFamilyIdentity::REQUIRED.len()
    );
    assert!(closeout.product_summary().rows().iter().all(|row| {
        row.migration_status() == CoveredDerivedProductMigrationStatus::Migrated
            && row.ordinary_invalidation_consumable()
    }));
    assert!(
        closeout
            .product_summary()
            .rows()
            .iter()
            .any(|row| row.selected_by_touched_closure()
                && row.selected_row_digest().is_some()
                && !row.consumed_graph_facts_digest().is_empty()),
        "closeout must explain selected product routing by touched closure facts"
    );
    assert!(closeout
        .product_summary()
        .rows()
        .iter()
        .filter(|row| row.selected_by_touched_closure())
        .all(|row| row.query_receipt_bound_count() > 0 && row.legality_receipt_bound_count() > 0));
    assert!(closeout
        .performance_proof()
        .slope_cases()
        .iter()
        .any(|case| case.label() == "semantic_delta_bounded_execution"
            && case.observed_work_count()
                == execution_receipt.counters().executed_product_count()));
    assert!(closeout
        .performance_proof()
        .slope_cases()
        .iter()
        .any(|case| case.label() == "product_catalog_closed_once"
            && case.touched_or_declared_bound()
                == DerivedTopologyProductFamilyIdentity::REQUIRED.len()));
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert_eq!(closeout.counters().caller_owned_graph_work_count(), 0);
    assert_eq!(
        closeout.milestone_eleven_seed().lookup_readiness(),
        super::super::DerivedInvalidationMilestoneElevenLookupReadiness::TopologyDerivedReceiptsReadySpatialEvidenceNotBound
    );
    assert_eq!(
        closeout
            .milestone_eleven_seed()
            .topology_derived_product_receipts()
            .len(),
        execution_receipt.executed_rows().len()
    );
    assert!(closeout
        .milestone_eleven_seed()
        .can_bootstrap_lookup_without_raw_scan());
}

#[test]
fn milestone_ten_current_source_closes_after_old_ordinary_authority_is_cut() {
    let catalog_closeout = catalog_closeout();
    let selected_plan = selected_plan_for_closeout(&catalog_closeout);
    let execution_receipt = execution_receipt(&selected_plan);
    let migration_sweep = full_migration_sweep(&selected_plan);
    let operator_closeout = operator_closeout(&migration_sweep, &selected_plan, &execution_receipt);

    let deletion_closeout = close_derived_invalidation_deletion(
        operator_closeout.phase_eight_seed(),
        &migration_sweep,
        &inventory_closeout(),
    )
    .expect("current source should close once old ordinary authority is cut");

    assert_eq!(
        deletion_closeout
            .counters()
            .source_firewall_violation_count(),
        0
    );
    assert_eq!(deletion_closeout.source_firewall().violations(), &[]);
}

#[test]
fn milestone_ten_rejects_partial_product_migration() {
    let catalog_closeout = catalog_closeout();
    let selected_plan = selected_plan_for_closeout(&catalog_closeout);
    let partial_migration_sweep = partial_migration_sweep(&selected_plan);

    let error = partial_migration_sweep.expect_err("partial product sweep must not close");

    assert_eq!(
        error,
        CoveredDerivedProductMigrationError::RequiredFamilyNotOrdinaryConsumable
    );
}
