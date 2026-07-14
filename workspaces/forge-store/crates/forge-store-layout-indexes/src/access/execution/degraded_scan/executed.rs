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
    counter_receipt: super::DegradedScanCounterReceipt,
}

impl DegradedScanExecution {
    pub(super) fn observe(
        ready: DegradedScanReadyRecipe,
        current_materialization: crate::CurrentLayoutMaterialization,
        physical: PlatformPhysicalDegradedExecutionObservation,
        observed_rows: u16,
    ) -> Result<Self, crate::CounterEnvelopeViolation> {
        let counter_receipt = super::DegradedScanCounterReceipt::issue(
            ready.payload().fingerprint(),
            observed_rows,
            physical.allocation_events(),
        )?;
        Ok(Self {
            recipe: ExecuteReadyRecipeTransition.transition(ready).into_value(),
            current_materialization,
            physical,
            counter_receipt,
        })
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

    pub const fn counter_receipt(&self) -> &super::DegradedScanCounterReceipt {
        &self.counter_receipt
    }
}

pub(in crate::access::execution::degraded_scan) fn execute_ready(
    ready: super::DegradedScanReady,
    physical: &mut forge_store_physical_format::PhysicalStoreRuntime,
) -> Result<DegradedScanExecution, crate::PhysicalDegradedExecutionDenial> {
    let selected = ready.selected();
    let expected = selected.admitted_family().authority_identity();
    let actual = physical.store_identity().authority_identity();
    if actual != expected {
        return Err(
            crate::PhysicalDegradedExecutionDenial::StoreAuthorityMismatch { expected, actual },
        );
    }
    let contract = physical
        .admit_degraded_exact_scan(
            selected.intent().budget_rows().unwrap_or(0),
            selected.budget_receipt(),
        )
        .map_err(crate::PhysicalDegradedExecutionDenial::Admission)?;
    let observation = physical
        .execute_admitted_degraded_exact_scan(contract)
        .map_err(|denial| crate::PhysicalDegradedExecutionDenial::Physical(Box::new(denial)))?;
    let observed_rows = admit_observed_row_count(observation.scan().observed_rows())?;
    let (recipe, current) = ready.into_parts();
    DegradedScanExecution::observe(recipe, current, observation, observed_rows)
        .map_err(crate::PhysicalDegradedExecutionDenial::CounterEnvelope)
}

fn admit_observed_row_count(
    observed_rows: u64,
) -> Result<u16, crate::PhysicalDegradedExecutionDenial> {
    u16::try_from(observed_rows).map_err(|_| {
        crate::PhysicalDegradedExecutionDenial::CounterDomainOverflow { observed_rows }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn counter_domain_overflow_is_a_typed_execution_denial() {
        let observed_rows = u64::from(u16::MAX) + 1;
        let denial = super::admit_observed_row_count(observed_rows).unwrap_err();

        assert_eq!(
            denial,
            crate::PhysicalDegradedExecutionDenial::CounterDomainOverflow { observed_rows }
        );
    }
}
