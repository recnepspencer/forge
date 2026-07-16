use crate::evidence::UiMeasurementBasis;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputWitness;
use crate::runtime::{
    WorthUiExecutionPlanInput, WorthUiPendingActivation, WorthUiPlanLoweringBasis,
};

#[derive(Clone, Debug)]
pub(crate) struct WorthUiAllocationPlanningAdmission {
    constraint_basis: crate::graph::UiAdmittedAllocationConstraintBasis,
    portal_allocation_input: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
    expected_lowered_witness: WorthUiExecutionPlanInputWitness,
}

impl WorthUiAllocationPlanningAdmission {
    pub(crate) fn constraint_basis(&self) -> &crate::graph::UiAdmittedAllocationConstraintBasis {
        &self.constraint_basis
    }

    pub(crate) fn from_pending_activation(
        pending_activation: &WorthUiPendingActivation,
        constraint_basis: crate::graph::UiAdmittedAllocationConstraintBasis,
    ) -> Self {
        Self {
            constraint_basis,
            portal_allocation_input: None,
            expected_lowered_witness: WorthUiExecutionPlanInputWitness::from_pending_activation(
                pending_activation,
            ),
        }
    }

    pub(crate) fn from_execution_plan_input(
        lowered_input: &WorthUiExecutionPlanInput,
        constraint_basis: crate::graph::UiAdmittedAllocationConstraintBasis,
        portal_allocation_input: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
    ) -> Self {
        Self {
            constraint_basis,
            portal_allocation_input,
            expected_lowered_witness: WorthUiExecutionPlanInputWitness::from_execution_plan_input(
                lowered_input,
            ),
        }
    }

    pub(crate) fn measurement_basis(&self) -> &UiMeasurementBasis {
        self.constraint_basis.measurement_basis()
    }

    pub(crate) fn portal_allocation_input(
        &self,
    ) -> Option<&crate::runtime::UiPortalAllocationPlanningBasis> {
        self.portal_allocation_input.as_ref()
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
