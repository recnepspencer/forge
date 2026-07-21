use std::borrow::Borrow;

use crate::runtime::planning::execution_plan_input::WorthUiExecutionPlanInputPreparer;
use crate::runtime::replacement::equivalence::WorthUiRuntimeArtifactComparator;
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
                Err(candidate) => {
                    let report =
                        crate::runtime::UiAllocationReceiptDenialReport::candidate_planning_denied(
                            &candidate,
                        );
                    return crate::runtime::UiAllocationReceiptCommitOutcome::Denied(Box::new(
                        crate::runtime::UiAllocationReceiptCommitDenial::CandidatePlanningDenied(
                            Box::new(report),
                        ),
                    ));
                }
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

    pub(crate) fn prepare_execution_plan_input<P>(
        &self,
        pending_activation: P,
    ) -> Result<WorthUiExecutionPlanInput, WorthUiPlanLoweringDenial>
    where
        P: Borrow<WorthUiPendingActivation>,
    {
        WorthUiExecutionPlanInputPreparer::prepare(
            pending_activation.borrow(),
            self.active.frame_epoch(),
            &[],
            self.active_application_lowering_authority
                .query_binding_plan(),
        )
    }

    pub(crate) fn detached_execution_plan_lowering_facts_for_test(
        &self,
        candidate: &UiAllocationCandidate,
        plan_input: WorthUiExecutionPlanInput,
    ) -> crate::runtime::planning::WorthUiExecutionPlanLoweringFacts {
        self.execution_plan_lowering_facts_below_authority_for_test(
            self.detached_allocation_lowering_input_for_test(candidate),
            plan_input,
        )
    }

    pub(crate) fn execution_plan_lowering_authority_from_committed_input_for_test(
        &self,
        pending_activation: WorthUiPendingActivation,
        committed_input: crate::runtime::UiCommittedAllocationLoweringInput,
    ) -> Result<
        crate::runtime::planning::WorthUiExecutionPlanLoweringAuthority,
        crate::runtime::planning::WorthUiExecutionPlanLoweringAuthorityDenial,
    > {
        crate::runtime::planning::WorthUiExecutionPlanLoweringAuthority::seal(
            pending_activation,
            committed_input,
            self.active.frame_epoch(),
        )
    }

    pub(crate) fn execution_plan_lowering_facts_below_authority_for_test(
        &self,
        committed_input: crate::runtime::UiCommittedAllocationLoweringInput,
        plan_input: WorthUiExecutionPlanInput,
    ) -> crate::runtime::planning::WorthUiExecutionPlanLoweringFacts {
        crate::runtime::planning::facts_below_authority(
            self.active_application_lowering_authority.clone(),
            committed_input,
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
            self.active_application_lowering_authority
                .query_binding_plan(),
        )
    }

    pub(crate) fn prepare_reconstructive_plan_input_for_test(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
        component_hooks: &[WorthUiComponentLoweringHook],
    ) -> WorthUiExecutionPlanInput {
        WorthUiExecutionPlanInputPreparer::prepare_launch_with_component_hooks(
            admitted.artifact_bundle().artifact(),
            admitted.artifact_bundle().artifact_digest(),
            self.active.frame_epoch(),
            self.active_application_lowering_authority
                .query_binding_plan(),
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
