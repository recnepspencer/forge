use std::collections::BTreeSet;

use super::super::super::super::catalog::DerivedTopologyProductFamilyIdentity;
use super::super::super::super::migrated_products::{
    close_covered_derived_product_migration_sweep, status_rows_from_migrated_family_closeouts,
    CoveredDerivedProductMigrationError, CoveredDerivedProductMigrationSweepCloseout,
    MigratedDerivedProductFamilyCloseout,
};
use super::super::super::super::selection::DerivedInvalidationSelectedPlan;
use super::selected_execution::selected_family_identities;

pub(in super::super) fn full_phase_six_closeout(
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
                "phase-six-all-family-residue:{}",
                selected_plan.selected_plan_digest()
            ),
        ),
    )
    .expect("full covered product migration sweep")
}

pub(in super::super) fn partial_phase_six_closeout(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> Result<CoveredDerivedProductMigrationSweepCloseout, CoveredDerivedProductMigrationError> {
    let selected_closeouts = selected_family_identities(selected_plan)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|family| family_closeout_bound_to_selected_plan(family, selected_plan))
        .collect::<Vec<_>>();
    let selected_refs = selected_closeouts.iter().collect::<Vec<_>>();
    close_covered_derived_product_migration_sweep(
        selected_plan,
        status_rows_from_migrated_family_closeouts(&selected_refs, "partial-residue"),
    )
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
