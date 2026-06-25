use super::super::super::catalog::DerivedTopologyProductFamilyIdentity;
use super::super::super::inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
use super::super::super::migrated_products::{
    close_covered_derived_product_migration_sweep, status_rows_from_migrated_family_closeouts,
    CoveredDerivedProductMigrationError, CoveredDerivedProductMigrationSweepCloseout,
    MigratedDerivedProductFamilyCloseout,
};
use super::super::super::operator_cutover::DerivedInvalidationPhaseEightSeed;
use super::super::super::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    loop_cycles_touched_closure,
};
use super::super::super::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};
use super::super::DerivedInvalidationDeletionSourceFirewall;

pub(in super::super) fn selected_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("phase-eight-deletion-closeout"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("phase eight selected plan")
}

pub(in super::super) fn phase_eight_seed(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> DerivedInvalidationPhaseEightSeed {
    DerivedInvalidationPhaseEightSeed::from_deletion_closeout_test_parts(
        selected_plan.selected_plan_digest(),
    )
}

pub(in super::super) fn mismatched_phase_eight_seed() -> DerivedInvalidationPhaseEightSeed {
    DerivedInvalidationPhaseEightSeed::from_deletion_closeout_test_parts("other-selected-plan")
}

pub(in super::super) fn full_migration_sweep(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> CoveredDerivedProductMigrationSweepCloseout {
    let migrated_closeouts = DerivedTopologyProductFamilyIdentity::REQUIRED
        .iter()
        .copied()
        .map(|family| family_closeout_bound_to_selected_plan(family, selected_plan))
        .collect::<Vec<_>>();
    let migrated_refs = migrated_closeouts.iter().collect::<Vec<_>>();
    close_covered_derived_product_migration_sweep(
        selected_plan,
        status_rows_from_migrated_family_closeouts(
            &migrated_refs,
            &format!(
                "phase-eight-all-family-residue:{}",
                selected_plan.selected_plan_digest()
            ),
        ),
    )
    .expect("full covered product migration sweep")
}

pub(in super::super) fn partial_migration_sweep(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> Result<CoveredDerivedProductMigrationSweepCloseout, CoveredDerivedProductMigrationError> {
    let selected_closeouts = selected_plan
        .selected_rows()
        .iter()
        .map(|row| family_closeout_bound_to_selected_plan(row.family_identity(), selected_plan))
        .collect::<Vec<_>>();
    let selected_refs = selected_closeouts.iter().collect::<Vec<_>>();
    close_covered_derived_product_migration_sweep(
        selected_plan,
        status_rows_from_migrated_family_closeouts(&selected_refs, "partial-residue"),
    )
}

pub(in super::super) fn inventory_closeout() -> DerivedInvalidationAuthorityInventoryCloseout {
    DerivedInvalidationAuthorityInventoryCloseout::close(
        current_derived_invalidation_authority_inventory(),
    )
    .expect("current authority inventory closeout")
}

pub(in super::super) fn dirty_firewall() -> DerivedInvalidationDeletionSourceFirewall {
    DerivedInvalidationDeletionSourceFirewall::from_sources_for_tests([(
        "dirty-phase-eight-source.rs",
        "operator_dirty_products",
    )])
}

pub(in super::super) fn clean_firewall() -> DerivedInvalidationDeletionSourceFirewall {
    DerivedInvalidationDeletionSourceFirewall::from_sources_for_tests([])
}

fn family_closeout_bound_to_selected_plan(
    family: DerivedTopologyProductFamilyIdentity,
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> MigratedDerivedProductFamilyCloseout {
    let family_key = family.as_str();
    let selected_digest = selected_plan.selected_plan_digest();
    MigratedDerivedProductFamilyCloseout::new(
        family,
        selected_digest,
        &format!("execution-receipt:{selected_digest}:{family_key}"),
        &format!("executed-row:{selected_digest}:{family_key}"),
        &format!("product-output:{selected_digest}:{family_key}"),
        &format!("old-authority-residue:{selected_digest}:{family_key}"),
        &format!("counters:{selected_digest}:{family_key}"),
    )
}
