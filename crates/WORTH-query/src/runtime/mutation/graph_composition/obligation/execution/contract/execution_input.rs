use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::WorthQueryGraphObligationRegistration;

use super::execution_context::WorthQueryGraphObligationExecutionContext;
use super::executor_contract::WorthQueryGraphObligationExecutorContract;
use super::state_access_policy::WorthQueryGraphObligationStateAccessPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutionInput {
    selection_digest: String,
    selected_registration: WorthQueryGraphObligationRegistration,
    executor_contract: WorthQueryGraphObligationExecutorContract,
    execution_context: WorthQueryGraphObligationExecutionContext,
    input_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationExecutionInput {
    pub fn from_selected_registration(
        selection_digest: impl Into<String>,
        selected_registration: WorthQueryGraphObligationRegistration,
    ) -> Self {
        let executor_contract = WorthQueryGraphObligationExecutorContract::new(
            selected_registration.kind(),
            selected_registration.rule_identity().clone(),
            selected_registration.support_posture().lane(),
            selected_registration.execution_budget().clone(),
            WorthQueryGraphObligationStateAccessPolicy::DeclaredBudgetOnly,
        );
        Self::new(
            selection_digest,
            selected_registration,
            executor_contract,
            WorthQueryGraphObligationExecutionContext::default(),
        )
    }

    pub fn from_selected_registration_with_context(
        selection_digest: impl Into<String>,
        selected_registration: WorthQueryGraphObligationRegistration,
        execution_context: WorthQueryGraphObligationExecutionContext,
    ) -> Self {
        let executor_contract = WorthQueryGraphObligationExecutorContract::new(
            selected_registration.kind(),
            selected_registration.rule_identity().clone(),
            selected_registration.support_posture().lane(),
            selected_registration.execution_budget().clone(),
            WorthQueryGraphObligationStateAccessPolicy::DeclaredBudgetOnly,
        );
        Self::new(
            selection_digest,
            selected_registration,
            executor_contract,
            execution_context,
        )
    }

    pub fn new(
        selection_digest: impl Into<String>,
        selected_registration: WorthQueryGraphObligationRegistration,
        executor_contract: WorthQueryGraphObligationExecutorContract,
        execution_context: WorthQueryGraphObligationExecutionContext,
    ) -> Self {
        let selection_digest = selection_digest.into();
        let input_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationExecutionInput)
                .field_value(WorthQueryEvidenceTag::new("selection"), &selection_digest)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("registration"),
                    selected_registration.registration_evidence_digest(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("executor_contract"),
                    executor_contract.contract_evidence_digest(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("execution_context"),
                    execution_context.context_evidence_digest(),
                )
                .seal();
        Self {
            selection_digest,
            selected_registration,
            executor_contract,
            execution_context,
            input_digest,
        }
    }

    pub fn selection_digest(&self) -> &str {
        &self.selection_digest
    }

    pub fn selected_registration(&self) -> &WorthQueryGraphObligationRegistration {
        &self.selected_registration
    }

    pub fn executor_contract(&self) -> &WorthQueryGraphObligationExecutorContract {
        &self.executor_contract
    }

    pub fn execution_context(&self) -> &WorthQueryGraphObligationExecutionContext {
        &self.execution_context
    }

    pub fn input_digest(&self) -> &str {
        self.input_digest.as_str()
    }

    pub(crate) fn input_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.input_digest
    }
}
