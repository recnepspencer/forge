use super::{UiCommittedAllocationActivationIdentity, UiCommittedAllocationValidation};

#[derive(Debug, PartialEq)]
pub(crate) struct UiCommittedAllocationActivationAttempt {
    pub(super) catalog: crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
    pub(super) ledger_transition:
        crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
    pub(super) committed: crate::runtime::UiCommittedAllocationReplan,
    pub(super) activation:
        crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
    pub(super) identity: UiCommittedAllocationActivationIdentity,
}

pub(crate) struct UiCommittedAllocationActivationInput<'a> {
    pub pending_activation: crate::runtime::WorthUiPendingActivation,
    pub plan_input: &'a crate::runtime::WorthUiExecutionPlanInput,
    pub handle_allocation: &'a crate::runtime::WorthUiRuntimeHandleAllocation,
    pub candidate_plan: crate::runtime::WorthUiExecutionPlan,
    pub boundary: crate::runtime::WorthUiFrameBoundary,
    pub lane_parity_report: Option<&'a crate::runtime::WorthUiLaneParityReport>,
}

impl UiCommittedAllocationActivationAttempt {
    pub(in crate::runtime) fn new(
        catalog: crate::runtime::invalidation_narrowing::UiAllocationActivationCatalog,
        ledger_transition: crate::runtime::allocation_receipt::UiAllocationCatalogLedgerTransition,
        committed: crate::runtime::UiCommittedAllocationReplan,
        activation: crate::runtime::allocation_receipt::UiCommittedAllocationCatalogActivation,
    ) -> Self {
        let identity =
            UiCommittedAllocationActivationIdentity::seal(&activation, &ledger_transition);
        Self {
            catalog,
            ledger_transition,
            committed,
            activation,
            identity,
        }
    }

    pub(in crate::runtime) fn primary_receipt(&self) -> &crate::runtime::UiAllocationReceipt {
        self.committed
            .receipts()
            .first()
            .expect("receipt-ledger sealing admits only non-empty catalogs")
    }

    pub(in crate::runtime) fn primary_planning(
        &self,
    ) -> &crate::runtime::WorthUiAllocationPlanning {
        self.catalog.activation_candidate().planning()
    }

    fn prepare(
        self,
        pending_activation: crate::runtime::WorthUiPendingActivation,
        plan_input: &crate::runtime::WorthUiExecutionPlanInput,
        handle_allocation: &crate::runtime::WorthUiRuntimeHandleAllocation,
        invalidation_authority: &crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
        candidate_plan: &crate::runtime::WorthUiExecutionPlan,
        lane_parity_report: Option<&crate::runtime::WorthUiLaneParityReport>,
    ) -> Result<UiCommittedAllocationValidation, super::UiCommittedAllocationActivationDenial> {
        UiCommittedAllocationValidation::prepare(
            pending_activation,
            plan_input,
            handle_allocation,
            self,
            invalidation_authority,
            candidate_plan,
            lane_parity_report,
        )
    }

    pub(in crate::runtime) fn activate(
        self,
        runtime: &mut crate::runtime::WorthUiRuntime,
        input: UiCommittedAllocationActivationInput<'_>,
    ) -> Result<crate::runtime::WorthUiPlanSwapReceipt, super::UiCommittedAllocationActivationDenial>
    {
        let UiCommittedAllocationActivationInput {
            pending_activation,
            plan_input,
            handle_allocation,
            candidate_plan,
            boundary,
            lane_parity_report,
        } = input;
        let attempt_identity = self.identity.clone();
        let invalidation_authority =
            runtime
                .allocation_invalidation_index
                .try_borrow()
                .map_err(|_| {
                    super::UiCommittedAllocationActivationDenial::preparation(
                    attempt_identity,
                    super::UiCommittedAllocationActivationCounters::default(),
                    super::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable,
                )
                })?;
        let validated = self.prepare(
            pending_activation,
            plan_input,
            handle_allocation,
            &invalidation_authority,
            &candidate_plan,
            lane_parity_report,
        )?;
        drop(invalidation_authority);
        super::publication::publish_validated_committed_allocation(
            runtime,
            validated,
            candidate_plan,
            boundary,
        )
    }
}
