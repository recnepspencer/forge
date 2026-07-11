use forge_proof::raw::{ExecuteReadyRecipeTransition, ExecutedRecipe, Transition};

use crate::execution::counter_snapshot::S8AccessPathCounterSnapshot;
use forge_store_budgets::CounterEvidenceStrength;

use super::admitted_counters::S8AdmittedExecutedCounters;
use super::amplification_receipt::S8AccessPathAmplificationReceipt;
use super::attempt_cost::S8AccessAttemptCostReceipt;
use super::lowered_plan::{S8AccessLoweringBasis, S8LoweredAccessPayload};
use super::observed_counters::S8ObservedAccessPathCounters;
use super::performance_receipt::S8StoreLayoutPerformanceReceipt;
use super::planned_vs_observed::S8PlannedVsObservedCounterReceipt;
use super::ready_plan::S8ExecutionReadyAccessReceipt;

type ExecutedAccessRecipe = ExecutedRecipe<
    S8LoweredAccessPayload,
    forge_proof::raw::FreshnessScopedBasis<
        forge_proof::raw::CurrentValidity,
        forge_proof::raw::AssumptionBasis<S8AccessLoweringBasis>,
    >,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct S8ExecutedAccessReceipt {
    recipe: ExecutedAccessRecipe,
    observed: S8ObservedAccessPathCounters,
}

impl S8ExecutedAccessReceipt {
    pub(crate) fn observe(
        ready: S8ExecutionReadyAccessReceipt,
        observed: S8AdmittedExecutedCounters,
    ) -> Self {
        let recipe = ExecuteReadyRecipeTransition
            .transition(ready.recipe())
            .into_value();
        Self {
            recipe,
            observed: observed.observed(),
        }
    }

    pub fn selected(&self) -> crate::planning::S8SelectedAccessPlan {
        self.recipe.payload().selected()
    }

    pub fn path_kind(&self) -> super::path_kind::S8AccessPathKind {
        self.recipe.payload().path_kind()
    }

    pub const fn observed(&self) -> S8AccessPathCounterSnapshot {
        self.observed.snapshot()
    }

    pub const fn counter_strength(&self) -> CounterEvidenceStrength {
        self.observed.strength()
    }

    pub fn basis(&self) -> S8AccessLoweringBasis {
        *self.recipe.strong_basis().value()
    }

    pub fn planned_vs_observed(&self) -> S8PlannedVsObservedCounterReceipt {
        S8PlannedVsObservedCounterReceipt::from_executed(self)
    }

    pub fn amplification_receipt(&self) -> S8AccessPathAmplificationReceipt {
        S8AccessPathAmplificationReceipt::new(self.path_kind(), self.observed())
    }

    pub fn performance_receipt(&self) -> S8StoreLayoutPerformanceReceipt {
        S8StoreLayoutPerformanceReceipt::new(
            self.selected().fingerprint(),
            self.planned_vs_observed(),
            self.amplification_receipt(),
            self.counter_strength(),
        )
    }

    pub fn spent_cost_receipt(&self) -> S8AccessAttemptCostReceipt {
        S8AccessAttemptCostReceipt::ObservedExecutionCost(self.performance_receipt())
    }
}
