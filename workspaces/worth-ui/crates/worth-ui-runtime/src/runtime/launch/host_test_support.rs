use std::borrow::Borrow;

use crate::runtime::allocation_planning::WorthUiAllocationPlanningAdmission;
use crate::runtime::equivalence::WorthUiRuntimeArtifactComparator;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::planning::collect_planning_measurement_basis;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiAllocationPlanning, WorthUiComponentLoweringHook,
    WorthUiExecutionPlanInput, WorthUiPendingActivation, WorthUiPlanLoweringDenial,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial,
    WorthUiRuntimeEquivalenceBasis,
};

use super::host::WorthUiRuntimeHost;

impl WorthUiRuntimeHost {
    pub(crate) fn plan_allocation_for_lowered_input_for_test(
        &self,
        plan_input: WorthUiExecutionPlanInput,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> WorthUiAllocationPlanning {
        let measurement_basis =
            collect_planning_measurement_basis(measurement_basis, allocation_neighborhood, &[]);
        crate::runtime::allocation_planning::WorthUiAllocationPlanner::plan_from_lowered_input(
            WorthUiAllocationPlanningAdmission::from_lowered_input_for_test(
                &plan_input,
                &measurement_basis,
                allocation_neighborhood,
                &measurement_basis
                    .admit_allocation_constraint_set(allocation_neighborhood)
                    .expect("constraint set should admit in lowered-input test path"),
            ),
            plan_input,
        )
    }

    pub(crate) fn plan_allocation_for_pending_and_lowered_input_for_test<P>(
        &self,
        pending_activation: P,
        plan_input: WorthUiExecutionPlanInput,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> WorthUiAllocationPlanning
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        let measurement_basis = collect_planning_measurement_basis(
            measurement_basis,
            allocation_neighborhood,
            pending_activation
                .borrow()
                .staged_replacement()
                .reconciliation_plan()
                .durable_resize_inputs(),
        );
        crate::runtime::allocation_planning::WorthUiAllocationPlanner::plan_from_lowered_input(
            WorthUiAllocationPlanningAdmission::from_pending_activation(
                pending_activation.borrow(),
                &measurement_basis,
                allocation_neighborhood,
                &measurement_basis
                    .admit_allocation_constraint_set(allocation_neighborhood)
                    .expect("constraint set should admit in pending-lowered-input test path"),
            ),
            plan_input,
        )
    }

    pub(crate) fn prepare_execution_plan_input_with_component_hooks_for_test<P>(
        &self,
        pending_activation: P,
        component_hooks: &[WorthUiComponentLoweringHook],
    ) -> Result<WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        WorthUiExecutionPlanInputPreparer::prepare(
            pending_activation.borrow(),
            self.active.frame_epoch(),
            component_hooks,
        )
    }

    pub(crate) fn compare_admitted_replacement_with_basis_for_test(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
        runtime_basis: WorthUiRuntimeEquivalenceBasis,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        WorthUiRuntimeArtifactComparator::for_active_artifact(self.active.active_artifact())
            .with_runtime_basis_for_test(runtime_basis)
            .compare_admitted(admitted)
    }
}
