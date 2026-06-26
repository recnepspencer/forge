use std::fmt;

use super::{
    close_covered_derived_product_migration_sweep, status_rows_from_migrated_family_closeouts,
    CoveredDerivedProductMigrationError, CoveredDerivedProductMigrationSweepCloseout,
    MigratedDerivedProductFamilyCloseout,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

pub fn close_required_product_migration_sweep_from_execution_receipt(
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<CoveredDerivedProductMigrationSweepCloseout, RequiredProductMigrationSweepError> {
    if execution_receipt.selected_plan_digest() != selected_plan.selected_plan_digest() {
        return Err(RequiredProductMigrationSweepError::SelectedPlanMismatch);
    }

    let migrated_closeouts = DerivedTopologyProductFamilyIdentity::REQUIRED
        .iter()
        .copied()
        .map(|family| family_closeout_from_receipt(family, execution_receipt))
        .collect::<Result<Vec<_>, _>>()?;
    let migrated_refs = migrated_closeouts.iter().collect::<Vec<_>>();
    let status_rows = status_rows_from_migrated_family_closeouts(
        &migrated_refs,
        &required_sweep_residue_digest(selected_plan, execution_receipt),
    );
    close_covered_derived_product_migration_sweep(selected_plan, status_rows)
        .map_err(RequiredProductMigrationSweepError::CoveredSweep)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredProductMigrationSweepError {
    SelectedPlanMismatch,
    MissingRequiredFamilyReceipt(DerivedTopologyProductFamilyIdentity),
    CoveredSweep(CoveredDerivedProductMigrationError),
}

impl fmt::Display for RequiredProductMigrationSweepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelectedPlanMismatch => write!(
                f,
                "required product migration sweep must consume the matching selected plan"
            ),
            Self::MissingRequiredFamilyReceipt(family) => write!(
                f,
                "required product migration sweep missed execution receipt for {}",
                family.as_str()
            ),
            Self::CoveredSweep(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for RequiredProductMigrationSweepError {}

fn family_closeout_from_receipt(
    family: DerivedTopologyProductFamilyIdentity,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> Result<MigratedDerivedProductFamilyCloseout, RequiredProductMigrationSweepError> {
    if let Some(row) = execution_receipt
        .executed_rows()
        .iter()
        .find(|row| row.family_identity() == family)
    {
        return Ok(
            MigratedDerivedProductFamilyCloseout::from_executed_product_row(execution_receipt, row),
        );
    }
    if let Some(row) = execution_receipt
        .unaffected_rows()
        .iter()
        .find(|row| row.family_identity() == family)
    {
        return Ok(
            MigratedDerivedProductFamilyCloseout::from_unaffected_product_row(
                execution_receipt,
                row,
            ),
        );
    }
    Err(RequiredProductMigrationSweepError::MissingRequiredFamilyReceipt(family))
}

fn required_sweep_residue_digest(
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> String {
    super::super::catalog::catalog_digest([
        "worth-topo:required-product-migration-sweep-residue:v1".to_string(),
        format!("selected-plan:{}", selected_plan.selected_plan_digest()),
        format!("execution:{}", execution_receipt.execution_receipt_digest()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
    use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
        admitted_legality_support, admitted_query_support, catalog_closeout,
        loop_cycles_touched_closure, unrelated_geometry_touched_closure,
    };
    use crate::derived_topology::invalidation_plan::selection::{
        DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
    };

    #[test]
    fn receipt_synthetic_required_sweep_cannot_close_as_migrated_product_proof() {
        let plan = selected_plan(loop_cycles_touched_closure("required-sweep"));
        let receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan(&plan).unwrap();

        assert_eq!(
            close_required_product_migration_sweep_from_execution_receipt(&plan, &receipt)
                .unwrap_err(),
            RequiredProductMigrationSweepError::CoveredSweep(
                CoveredDerivedProductMigrationError::RequiredFamilyProofNotFamilySpecific
            )
        );
    }

    #[test]
    fn required_sweep_rejects_execution_receipt_from_different_plan() {
        let loop_plan = selected_plan(loop_cycles_touched_closure("required-sweep-loop"));
        let geometry_plan = selected_plan(unrelated_geometry_touched_closure());
        let geometry_receipt =
            DerivedInvalidationExecutionReceipt::execute_selected_plan(&geometry_plan).unwrap();

        let error = close_required_product_migration_sweep_from_execution_receipt(
            &loop_plan,
            &geometry_receipt,
        )
        .unwrap_err();

        assert_eq!(
            error,
            RequiredProductMigrationSweepError::SelectedPlanMismatch
        );
    }

    fn selected_plan(
        touched_closure: crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure,
    ) -> DerivedInvalidationSelectedPlan {
        DerivedInvalidationSelectedPlan::lower(
            &catalog_closeout(),
            &touched_closure,
            &admitted_query_support(),
            &admitted_legality_support(),
            DerivedInvalidationDensityPolicy::Sparse,
        )
        .unwrap()
    }
}
