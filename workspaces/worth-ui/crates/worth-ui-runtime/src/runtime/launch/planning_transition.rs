use std::borrow::Borrow;

use crate::graph::UiGraphSnapshot;
use crate::obligations::selection::UiSelectedObligationSet;
use crate::runtime::execution::WorthUiExecutionLaneInput;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::handle_allocation::WorthUiRuntimeHandleAllocator;
use crate::runtime::planning::{
    construct_planning_lane_input, plan_allocation_for_pending_activation,
    WorthUiPlanningLaneAdmissionDenial, WorthUiPlanningLaneInput,
};
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiPendingActivation, WorthUiPlanLoweringDenial,
    WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial,
};

use super::host::WorthUiRuntimeHost;

impl WorthUiRuntimeHost {
    pub(crate) fn prepare_execution_plan_input<P>(
        &self,
        pending_activation: P,
    ) -> Result<crate::runtime::WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        WorthUiExecutionPlanInputPreparer::prepare(
            pending_activation.borrow(),
            self.active.frame_epoch(),
            &[],
        )
    }

    pub fn allocate_runtime_handles_from_lane_input(
        &self,
        input: WorthUiExecutionLaneInput<'_>,
    ) -> Result<WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial> {
        WorthUiRuntimeHandleAllocator::allocate(input.allocation_planning())
    }

    pub fn allocate_runtime_handles(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
    ) -> Result<WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial> {
        self.allocate_runtime_handles_from_lane_input(WorthUiExecutionLaneInput::new(
            allocation_planning,
        ))
    }

    pub fn plan_allocation_from_lane_input<P>(
        &self,
        input: WorthUiPlanningLaneInput<P>,
    ) -> WorthUiAllocationPlanning
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        plan_allocation_for_pending_activation(
            self,
            input.pending_activation(),
            input.measurement_basis(),
            input.allocation_neighborhood(),
        )
    }

    pub fn admit_planning_lane_input<P>(
        &self,
        pending_activation: P,
        graph_snapshot: &UiGraphSnapshot,
        measurement_basis: crate::evidence::UiMeasurementBasis,
        selected_obligations: &UiSelectedObligationSet,
    ) -> Result<WorthUiPlanningLaneInput<P>, WorthUiPlanningLaneAdmissionDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        let allocation_neighborhood = selected_obligations
            .admit_allocation_neighborhood(graph_snapshot, &measurement_basis)?;
        construct_planning_lane_input(
            pending_activation,
            measurement_basis,
            allocation_neighborhood,
        )
        .map_err(Into::into)
    }

    pub fn plan_allocation<P>(
        &self,
        pending_activation: P,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> WorthUiAllocationPlanning
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        self.plan_allocation_from_lane_input(WorthUiPlanningLaneInput::new(
            pending_activation,
            measurement_basis.clone(),
            allocation_neighborhood.clone(),
        ))
    }
}
