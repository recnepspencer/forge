use std::borrow::Borrow;

use crate::runtime::allocation_planning::WorthUiAllocationPlanningAdmission;
use crate::runtime::equivalence::WorthUiRuntimeArtifactComparator;
use crate::runtime::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::{
    UiAllocationCandidate, WorthUiAdmittedReplacementCandidate, WorthUiComponentLoweringHook,
    WorthUiExecutionPlanInput, WorthUiPendingActivation, WorthUiPlanLoweringDenial,
    WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial,
    WorthUiRuntimeEquivalenceBasis,
};

use super::runtime_instance::WorthUiRuntime;

impl WorthUiRuntime {
    pub(crate) fn committed_allocation_scope_count_for_test(&self) -> usize {
        self.allocation_receipt_ledger.committed_scope_count()
    }
    pub(crate) fn commit_allocation_candidate_for_test(
        &self,
        candidate: UiAllocationCandidate,
    ) -> crate::runtime::UiAllocationReceiptCommitOutcome {
        let candidate =
            match crate::runtime::allocation_receipt::UiNonPortalReceiptLawCandidate::admit(
                candidate,
            ) {
                Ok(candidate) => candidate,
                Err(candidate) => return crate::runtime::UiAllocationReceiptCommitOutcome::Denied(
                    Box::new(crate::runtime::UiAllocationReceiptCommitDenial::CandidatePlanningDenied(
                        Box::new(
                        crate::runtime::UiAllocationReceiptDenialReport::candidate_planning_denied(
                            &candidate,
                        ),
                        ),
                    )),
                ),
            };
        self.allocation_receipt_ledger
            .commit_non_portal_receipt_law_candidate(candidate)
    }

    /// Test convenience still crosses the production receipt-commit seam.
    pub(crate) fn detached_allocation_receipt_for_test(
        &self,
        candidate: &UiAllocationCandidate,
    ) -> crate::runtime::UiAllocationReceipt {
        let candidate = crate::runtime::allocation_receipt::UiNonPortalReceiptLawCandidate::admit(
            candidate.clone(),
        )
        .expect("detached receipt fixture cannot carry portal allocation authority");
        crate::runtime::allocation_receipt::detached_non_portal_receipt(candidate)
            .expect("admitted test planning must commit through the production receipt seam")
    }

    pub(crate) fn detached_allocation_lowering_input_for_test(
        &self,
        candidate: &UiAllocationCandidate,
    ) -> crate::runtime::UiCommittedAllocationLoweringInput {
        self.detached_allocation_receipt_for_test(candidate)
            .lowering_input()
            .expect("freshly committed test receipt must admit execution lowering")
    }

    pub(crate) fn plan_allocation_for_lowered_input_for_test(
        &self,
        plan_input: WorthUiExecutionPlanInput,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> UiAllocationCandidate {
        crate::runtime::planning::candidate_from_test_planning(
            crate::runtime::allocation_planning::WorthUiAllocationPlanner::plan_from_lowered_input(
                WorthUiAllocationPlanningAdmission::from_execution_plan_input(
                    &plan_input,
                    measurement_basis
                        .admit_allocation_constraint_basis(allocation_neighborhood)
                        .expect("constraint basis should admit in lowered-input test path"),
                    None,
                ),
                plan_input,
            ),
        )
    }

    pub(crate) fn plan_allocation_for_pending_and_lowered_input_for_test<P>(
        &self,
        pending_activation: P,
        plan_input: WorthUiExecutionPlanInput,
        measurement_basis: &crate::evidence::UiMeasurementBasis,
        allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    ) -> UiAllocationCandidate
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        crate::runtime::planning::candidate_from_test_planning(
            crate::runtime::allocation_planning::WorthUiAllocationPlanner::plan_from_lowered_input(
                WorthUiAllocationPlanningAdmission::from_pending_activation(
                    pending_activation.borrow(),
                    measurement_basis
                        .admit_allocation_constraint_basis(allocation_neighborhood)
                        .expect("constraint basis should admit in pending-lowered-input test path"),
                ),
                plan_input,
            ),
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
