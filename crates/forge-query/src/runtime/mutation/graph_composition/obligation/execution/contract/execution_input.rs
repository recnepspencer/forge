use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryGraphObligationRegistration;

use super::execution_context::ForgeQueryGraphObligationExecutionContext;
use super::executor_contract::ForgeQueryGraphObligationExecutorContract;
use super::state_access_policy::ForgeQueryGraphObligationStateAccessPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationExecutionInput {
    selection_digest: String,
    selected_registration: ForgeQueryGraphObligationRegistration,
    executor_contract: ForgeQueryGraphObligationExecutorContract,
    execution_context: ForgeQueryGraphObligationExecutionContext,
    input_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationExecutionInput {
    pub fn from_selected_registration(
        selection_digest: impl Into<String>,
        selected_registration: ForgeQueryGraphObligationRegistration,
    ) -> Self {
        let executor_contract = ForgeQueryGraphObligationExecutorContract::new(
            selected_registration.kind(),
            selected_registration.rule_identity().clone(),
            selected_registration.support_posture().lane(),
            selected_registration.execution_budget().clone(),
            ForgeQueryGraphObligationStateAccessPolicy::DeclaredBudgetOnly,
        );
        Self::new(
            selection_digest,
            selected_registration,
            executor_contract,
            ForgeQueryGraphObligationExecutionContext::default(),
        )
    }

    pub fn from_selected_registration_with_context(
        selection_digest: impl Into<String>,
        selected_registration: ForgeQueryGraphObligationRegistration,
        execution_context: ForgeQueryGraphObligationExecutionContext,
    ) -> Self {
        let executor_contract = ForgeQueryGraphObligationExecutorContract::new(
            selected_registration.kind(),
            selected_registration.rule_identity().clone(),
            selected_registration.support_posture().lane(),
            selected_registration.execution_budget().clone(),
            ForgeQueryGraphObligationStateAccessPolicy::DeclaredBudgetOnly,
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
        selected_registration: ForgeQueryGraphObligationRegistration,
        executor_contract: ForgeQueryGraphObligationExecutorContract,
        execution_context: ForgeQueryGraphObligationExecutionContext,
    ) -> Self {
        let selection_digest = selection_digest.into();
        let input_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationExecutionInput)
                .field_value(ForgeQueryEvidenceTag::new("selection"), &selection_digest)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("registration"),
                    selected_registration.registration_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("executor_contract"),
                    executor_contract.contract_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution_context"),
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

    pub fn selected_registration(&self) -> &ForgeQueryGraphObligationRegistration {
        &self.selected_registration
    }

    pub fn executor_contract(&self) -> &ForgeQueryGraphObligationExecutorContract {
        &self.executor_contract
    }

    pub fn execution_context(&self) -> &ForgeQueryGraphObligationExecutionContext {
        &self.execution_context
    }

    pub fn input_digest(&self) -> &str {
        self.input_digest.as_str()
    }

    pub(crate) fn input_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.input_digest
    }
}
