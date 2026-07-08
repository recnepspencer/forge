use crate::evidence::{UiAllocationConstraintSet, UiAllocationNeighborhood, UiMeasurementBasis};
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputWitness;
use crate::runtime::{
    WorthUiExecutionPlanInput, WorthUiPendingActivation, WorthUiPlanLoweringBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WorthUiAllocationPlanningAdmission {
    measurement_basis: UiMeasurementBasis,
    allocation_neighborhood: UiAllocationNeighborhood,
    allocation_constraint_set: UiAllocationConstraintSet,
    expected_lowered_witness: WorthUiExecutionPlanInputWitness,
}

impl WorthUiAllocationPlanningAdmission {
    pub(crate) fn from_pending_activation(
        pending_activation: &WorthUiPendingActivation,
        measurement_basis: &UiMeasurementBasis,
        allocation_neighborhood: &UiAllocationNeighborhood,
        allocation_constraint_set: &UiAllocationConstraintSet,
    ) -> Self {
        Self {
            measurement_basis: measurement_basis.clone(),
            allocation_neighborhood: allocation_neighborhood.clone(),
            allocation_constraint_set: allocation_constraint_set.clone(),
            expected_lowered_witness: WorthUiExecutionPlanInputWitness::from_pending_activation(
                pending_activation,
            ),
        }
    }

    pub(crate) fn from_lowered_input_for_test(
        lowered_input: &WorthUiExecutionPlanInput,
        measurement_basis: &UiMeasurementBasis,
        allocation_neighborhood: &UiAllocationNeighborhood,
        allocation_constraint_set: &UiAllocationConstraintSet,
    ) -> Self {
        Self {
            measurement_basis: measurement_basis.clone(),
            allocation_neighborhood: allocation_neighborhood.clone(),
            allocation_constraint_set: allocation_constraint_set.clone(),
            expected_lowered_witness: WorthUiExecutionPlanInputWitness::from_execution_plan_input(
                lowered_input,
            ),
        }
    }

    pub(crate) fn measurement_basis(&self) -> &UiMeasurementBasis {
        &self.measurement_basis
    }

    pub(crate) fn allocation_neighborhood(&self) -> &UiAllocationNeighborhood {
        &self.allocation_neighborhood
    }

    pub(crate) fn allocation_constraint_set(&self) -> &UiAllocationConstraintSet {
        &self.allocation_constraint_set
    }

    pub(crate) fn lowered_input_matches(&self, lowered_input: &WorthUiExecutionPlanInput) -> bool {
        self.expected_lowered_witness
            .matches_execution_plan_input(lowered_input)
    }

    pub(crate) fn expected_lowering_basis(&self) -> &WorthUiPlanLoweringBasis {
        self.expected_lowered_witness.basis()
    }

    pub(crate) fn expected_lowered_witness_digest(&self) -> u64 {
        self.expected_lowered_witness.digest()
    }
}
