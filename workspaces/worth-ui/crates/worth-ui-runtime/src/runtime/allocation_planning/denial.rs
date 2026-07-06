use crate::evidence::{UiConstraintPropagationDenial, UiMeasurementBasisDenial};
use crate::runtime::{
    WorthUiAllocationPlanningCounters, WorthUiPlanLoweringBasis, WorthUiPlanLoweringDenial,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAllocationPlanningDenialReason {
    MeasurementBasisDenied,
    ConstraintSetDenied,
    LoweringAdmissionMismatch,
    PlanLoweringDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAllocationPlanningLoweringMismatch {
    expected_basis: WorthUiPlanLoweringBasis,
    observed_basis: WorthUiPlanLoweringBasis,
    expected_witness_digest: u64,
    observed_witness_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAllocationPlanningDenial {
    reason: WorthUiAllocationPlanningDenialReason,
    measurement_basis_denial: Option<UiMeasurementBasisDenial>,
    constraint_set_denial: Option<UiConstraintPropagationDenial>,
    lowering_mismatch: Option<WorthUiAllocationPlanningLoweringMismatch>,
    plan_lowering_denial: Option<WorthUiPlanLoweringDenial>,
    counters: WorthUiAllocationPlanningCounters,
}

impl WorthUiAllocationPlanningDenial {
    pub(crate) fn new(
        reason: WorthUiAllocationPlanningDenialReason,
        measurement_basis_denial: Option<UiMeasurementBasisDenial>,
        constraint_set_denial: Option<UiConstraintPropagationDenial>,
        lowering_mismatch: Option<WorthUiAllocationPlanningLoweringMismatch>,
        plan_lowering_denial: Option<WorthUiPlanLoweringDenial>,
        mut counters: WorthUiAllocationPlanningCounters,
    ) -> Self {
        counters.record_denial();
        Self {
            reason,
            measurement_basis_denial,
            constraint_set_denial,
            lowering_mismatch,
            plan_lowering_denial,
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

    pub fn lowering_mismatch(&self) -> Option<&WorthUiAllocationPlanningLoweringMismatch> {
        self.lowering_mismatch.as_ref()
    }

    pub fn plan_lowering_denial(&self) -> Option<&WorthUiPlanLoweringDenial> {
        self.plan_lowering_denial.as_ref()
    }

    pub fn counters(&self) -> WorthUiAllocationPlanningCounters {
        self.counters
    }
}

impl WorthUiAllocationPlanningLoweringMismatch {
    pub(crate) fn new(
        expected_basis: WorthUiPlanLoweringBasis,
        observed_basis: WorthUiPlanLoweringBasis,
        expected_witness_digest: u64,
        observed_witness_digest: u64,
    ) -> Self {
        Self {
            expected_basis,
            observed_basis,
            expected_witness_digest,
            observed_witness_digest,
        }
    }

    pub fn expected(&self) -> &WorthUiPlanLoweringBasis {
        &self.expected_basis
    }

    pub fn observed(&self) -> &WorthUiPlanLoweringBasis {
        &self.observed_basis
    }

    pub fn expected_witness_digest(&self) -> u64 {
        self.expected_witness_digest
    }

    pub fn observed_witness_digest(&self) -> u64 {
        self.observed_witness_digest
    }
}
