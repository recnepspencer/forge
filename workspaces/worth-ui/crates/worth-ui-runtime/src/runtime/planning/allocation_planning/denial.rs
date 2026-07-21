use crate::evidence::{UiConstraintPropagationDenial, UiMeasurementBasisDenial};
use crate::runtime::WorthUiAllocationPlanningCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAllocationPlanningDenialReason {
    MeasurementBasisDenied,
    ConstraintSetDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAllocationPlanningDenial {
    reason: WorthUiAllocationPlanningDenialReason,
    measurement_basis_denial: Option<UiMeasurementBasisDenial>,
    constraint_set_denial: Option<UiConstraintPropagationDenial>,
    counters: WorthUiAllocationPlanningCounters,
}

impl WorthUiAllocationPlanningDenial {
    pub(crate) fn new(
        reason: WorthUiAllocationPlanningDenialReason,
        measurement_basis_denial: Option<UiMeasurementBasisDenial>,
        constraint_set_denial: Option<UiConstraintPropagationDenial>,
        mut counters: WorthUiAllocationPlanningCounters,
    ) -> Self {
        counters.record_denial();
        Self {
            reason,
            measurement_basis_denial,
            constraint_set_denial,
            counters,
        }
    }

    pub fn reason(&self) -> WorthUiAllocationPlanningDenialReason {
        self.reason
    }

    pub fn measurement_basis_denial(&self) -> Option<&UiMeasurementBasisDenial> {
        self.measurement_basis_denial.as_ref()
    }

    pub fn constraint_set_denial(&self) -> Option<&UiConstraintPropagationDenial> {
        self.constraint_set_denial.as_ref()
    }

    pub fn counters(&self) -> WorthUiAllocationPlanningCounters {
        self.counters
    }
}
