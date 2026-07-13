use forge_proof::raw::{ExecuteReadyRecipeTransition, ExecutedRecipe, Transition};

use super::DegradedScanLoweringBasis;
use crate::planning::SelectedDegradedExactScan;
use forge_store_physical_format::PlatformPhysicalDegradedExecutionObservation;

type DegradedScanReadyRecipe = forge_proof::raw::ExecutionReadyRecipe<
    SelectedDegradedExactScan,
    forge_proof::raw::FreshnessScopedBasis<
        forge_proof::raw::CurrentValidity,
        forge_proof::raw::AssumptionBasis<DegradedScanLoweringBasis>,
    >,
>;

type DegradedScanExecutedRecipe = ExecutedRecipe<
    SelectedDegradedExactScan,
    forge_proof::raw::FreshnessScopedBasis<
        forge_proof::raw::CurrentValidity,
        forge_proof::raw::AssumptionBasis<DegradedScanLoweringBasis>,
    >,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct DegradedScanExecution {
    recipe: DegradedScanExecutedRecipe,
    current_materialization: crate::CurrentLayoutMaterialization,
    physical: PlatformPhysicalDegradedExecutionObservation,
}

impl DegradedScanExecution {
    pub(super) fn observe(
        ready: DegradedScanReadyRecipe,
        current_materialization: crate::CurrentLayoutMaterialization,
        physical: PlatformPhysicalDegradedExecutionObservation,
    ) -> Self {
        Self {
            recipe: ExecuteReadyRecipeTransition.transition(ready).into_value(),
            current_materialization,
            physical,
        }
    }
    pub fn selected(&self) -> &SelectedDegradedExactScan {
        self.recipe.payload()
    }
    pub fn basis(&self) -> &DegradedScanLoweringBasis {
        self.recipe.strong_basis().value()
    }
    pub const fn physical_observation(&self) -> &PlatformPhysicalDegradedExecutionObservation {
        &self.physical
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }

    pub const fn observed_rows(&self) -> u64 {
        self.physical.scan().observed_rows()
    }
}
