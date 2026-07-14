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

    #[allow(clippy::too_many_arguments)]
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

    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime) fn activate(
        self,
        runtime: &mut crate::runtime::WorthUiRuntime,
        pending_activation: crate::runtime::WorthUiPendingActivation,
        plan_input: &crate::runtime::WorthUiExecutionPlanInput,
        handle_allocation: &crate::runtime::WorthUiRuntimeHandleAllocation,
        candidate_plan: crate::runtime::WorthUiExecutionPlan,
        boundary: crate::runtime::WorthUiFrameBoundary,
        lane_parity_report: Option<&crate::runtime::WorthUiLaneParityReport>,
    ) -> Result<crate::runtime::WorthUiPlanSwapReceipt, super::UiCommittedAllocationActivationDenial>
    {
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
