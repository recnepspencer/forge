use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiFrameBoundary, WorthUiLaneParityReport, WorthUiPendingActivation,
    WorthUiPlanSwapReceipt,
};

#[derive(Debug)]
pub enum WorthUiAllocationCatalogActivationDenial {
    Preparation,
    PlanInput,
    HandleAllocation,
    TopologyAssembly,
    Attempt(crate::runtime::UiCommittedAllocationActivationDenial),
}

impl WorthUiRuntime {
    pub fn activate_admitted_allocation_catalog_at_frame_boundary(
        &mut self,
        pending_activation: WorthUiPendingActivation,
        admitted_catalog: crate::graph::UiAdmittedAllocationCatalogBasisSet,
        boundary: WorthUiFrameBoundary,
        lane_parity_report: Option<WorthUiLaneParityReport>,
    ) -> Result<WorthUiPlanSwapReceipt, WorthUiAllocationCatalogActivationDenial> {
        self.activate_admitted_allocation_catalog_with_boundary_source(
            pending_activation,
            admitted_catalog,
            |_, _, _, _| Ok((boundary, lane_parity_report)),
        )
    }

    pub(crate) fn activate_admitted_allocation_catalog_with_boundary_source<F>(
        &mut self,
        pending_activation: WorthUiPendingActivation,
        admitted_catalog: crate::graph::UiAdmittedAllocationCatalogBasisSet,
        boundary_source: F,
    ) -> Result<WorthUiPlanSwapReceipt, WorthUiAllocationCatalogActivationDenial>
    where
        F: FnOnce(
            &mut WorthUiRuntime,
            &crate::runtime::UiAllocationReceipt,
            &WorthUiExecutionPlan,
            &crate::runtime::WorthUiAllocationPlanning,
        ) -> Result<
            (WorthUiFrameBoundary, Option<WorthUiLaneParityReport>),
            WorthUiAllocationCatalogActivationDenial,
        >,
    {
        let plan_input = self
            .prepare_execution_plan_input(&pending_activation)
            .map_err(|_| WorthUiAllocationCatalogActivationDenial::PlanInput)?;
        let prepared = self
            .prepare_allocation_catalog_activation(&pending_activation, admitted_catalog)
            .map_err(|_| WorthUiAllocationCatalogActivationDenial::Preparation)?;
        let receipt = prepared.primary_receipt().clone();
        let handles = self
            .allocate_runtime_handles(&receipt)
            .map_err(|_| WorthUiAllocationCatalogActivationDenial::HandleAllocation)?;
        let candidate_plan = match self.assemble_execution_plan_topology(&receipt, &handles) {
            Ok(plan) => plan,
            Err(_) => return Err(WorthUiAllocationCatalogActivationDenial::TopologyAssembly),
        };
        let (boundary, lane_parity_report) =
            boundary_source(self, &receipt, &candidate_plan, prepared.primary_planning())?;
        match prepared.activate(
            self,
            pending_activation,
            &plan_input,
            &handles,
            candidate_plan,
            boundary,
            lane_parity_report.as_ref(),
        ) {
            Ok(receipt) => Ok(receipt),
            Err(denial) => Err(WorthUiAllocationCatalogActivationDenial::Attempt(denial)),
        }
    }

    #[cfg(test)]
    pub(crate) fn safe_frame_boundary(&self) -> WorthUiFrameBoundary {
        WorthUiFrameBoundary::safe_to_activate(self.frame_epoch())
    }

    #[cfg(test)]
    pub(crate) fn traversal_frame_boundary_for_test(&self) -> WorthUiFrameBoundary {
        WorthUiFrameBoundary::traversal_in_progress_for_test(self.frame_epoch())
    }

    #[cfg(test)]
    pub(crate) fn safe_frame_boundary_for_epoch_for_test(
        &self,
        frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    ) -> WorthUiFrameBoundary {
        WorthUiFrameBoundary::safe_to_activate(frame_epoch)
    }
}
