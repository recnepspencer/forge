use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiFrameBoundary, WorthUiLaneParityReport, WorthUiPendingActivation,
    WorthUiPlanSwapReceipt,
};

#[derive(Debug)]
pub enum WorthUiAllocationCatalogActivationDenial {
    Preparation(WorthUiAllocationCatalogPreparationStage),
    PlanInput,
    HandleAllocation,
    Freshness(crate::runtime::UiAllocationFreshnessConsumptionDenial),
    TopologyAssembly(crate::runtime::WorthUiPlanTopologyDenial),
    CertificationBoundary(&'static str),
    Attempt(Box<crate::runtime::UiCommittedAllocationActivationDenial>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAllocationCatalogPreparationStage {
    PlanningAdmission,
    CatalogPlanning,
    ReceiptRecomputePending,
    ReceiptCatalogBindingCardinalityMismatch,
    ReceiptCatalogBindingIdentityMismatch,
    ReceiptCatalogActivationAuthority,
    ReceiptCandidatePlanningDenied,
    ReceiptReuseDenied,
    ReceiptAuthorityCounterExhausted,
    ReceiptEvidenceCounterExhausted,
    UnexpectedCommittedReceipt,
}

impl WorthUiRuntime {
    pub(crate) fn activate_admitted_allocation_catalog_at_frame_boundary(
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
            .map_err(|denial| {
                let stage = match denial {
                    crate::runtime::launch::UiAllocationCatalogPreparationDenial::PlanningAdmission(_) => WorthUiAllocationCatalogPreparationStage::PlanningAdmission,
                    crate::runtime::launch::UiAllocationCatalogPreparationDenial::CatalogPlanning(_) => WorthUiAllocationCatalogPreparationStage::CatalogPlanning,
                    crate::runtime::launch::UiAllocationCatalogPreparationDenial::ReceiptCommit(outcome) => match outcome.as_ref() {
                        crate::runtime::UiAllocationReceiptCommitOutcome::RecomputePending(_) => WorthUiAllocationCatalogPreparationStage::ReceiptRecomputePending,
                        crate::runtime::UiAllocationReceiptCommitOutcome::Denied(denial) => match denial.as_ref() {
                            crate::runtime::UiAllocationReceiptCommitDenial::CatalogBindingCardinalityMismatch => WorthUiAllocationCatalogPreparationStage::ReceiptCatalogBindingCardinalityMismatch,
                            crate::runtime::UiAllocationReceiptCommitDenial::CatalogBindingIdentityMismatch { .. } => WorthUiAllocationCatalogPreparationStage::ReceiptCatalogBindingIdentityMismatch,
                            crate::runtime::UiAllocationReceiptCommitDenial::CatalogActivationAuthority(_) => WorthUiAllocationCatalogPreparationStage::ReceiptCatalogActivationAuthority,
                            crate::runtime::UiAllocationReceiptCommitDenial::CandidatePlanningDenied(_) => WorthUiAllocationCatalogPreparationStage::ReceiptCandidatePlanningDenied,
                            crate::runtime::UiAllocationReceiptCommitDenial::ReuseDenied(_) => WorthUiAllocationCatalogPreparationStage::ReceiptReuseDenied,
                            crate::runtime::UiAllocationReceiptCommitDenial::AuthorityCounterExhausted(_) => WorthUiAllocationCatalogPreparationStage::ReceiptAuthorityCounterExhausted,
                            crate::runtime::UiAllocationReceiptCommitDenial::EvidenceCounterExhausted => WorthUiAllocationCatalogPreparationStage::ReceiptEvidenceCounterExhausted,
                        },
                        crate::runtime::UiAllocationReceiptCommitOutcome::Committed(_) => WorthUiAllocationCatalogPreparationStage::UnexpectedCommittedReceipt,
                    },
                };
                WorthUiAllocationCatalogActivationDenial::Preparation(stage)
            })?;
        let receipt = prepared.primary_receipt().clone();
        let lowering_input = receipt
            .lowering_input()
            .map_err(WorthUiAllocationCatalogActivationDenial::Freshness)?;
        let handles = self
            .allocate_runtime_handles(&receipt)
            .map_err(|_| WorthUiAllocationCatalogActivationDenial::HandleAllocation)?;
        let candidate_plan = self
            .assemble_execution_plan_topology(&lowering_input, &handles)
            .map_err(WorthUiAllocationCatalogActivationDenial::TopologyAssembly)?;
        let (boundary, lane_parity_report) =
            boundary_source(self, &receipt, &candidate_plan, prepared.primary_planning())?;
        match prepared.activate(
            self,
            super::committed_allocation_attempt::UiCommittedAllocationActivationInput {
                pending_activation,
                plan_input: &plan_input,
                handle_allocation: &handles,
                candidate_plan,
                boundary,
                lane_parity_report: lane_parity_report.as_ref(),
            },
        ) {
            Ok(receipt) => Ok(receipt),
            Err(denial) => Err(WorthUiAllocationCatalogActivationDenial::Attempt(Box::new(
                denial,
            ))),
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
