use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationSupportLane,
};

use super::state_access_policy::WorthQueryGraphObligationStateAccessPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationExecutorContract {
    obligation_kind: WorthQueryGraphObligationKind,
    rule_identity: WorthQueryGraphObligationRuleIdentity,
    support_lane: WorthQueryGraphObligationSupportLane,
    execution_budget: WorthQueryGraphObligationExecutionBudget,
    state_access_policy: WorthQueryGraphObligationStateAccessPolicy,
    contract_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationExecutorContract {
    pub fn new(
        obligation_kind: WorthQueryGraphObligationKind,
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        support_lane: WorthQueryGraphObligationSupportLane,
        execution_budget: WorthQueryGraphObligationExecutionBudget,
        state_access_policy: WorthQueryGraphObligationStateAccessPolicy,
    ) -> Self {
        let contract_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationExecutorContract)
                .field_shape(
                    WorthQueryEvidenceTag::new("obligation_kind"),
                    obligation_kind.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("rule"),
                    rule_identity.identity_evidence_digest(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("support_lane"),
                    support_lane.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("execution_budget"),
                    execution_budget.budget_evidence_digest(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("state_access_policy"),
                    state_access_policy.as_str(),
                )
                .seal();
        Self {
            obligation_kind,
            rule_identity,
            support_lane,
            execution_budget,
            state_access_policy,
            contract_digest,
        }
    }

    pub fn obligation_kind(&self) -> WorthQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn rule_identity(&self) -> &WorthQueryGraphObligationRuleIdentity {
        &self.rule_identity
    }

    pub fn support_lane(&self) -> WorthQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn execution_budget(&self) -> &WorthQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn state_access_policy(&self) -> WorthQueryGraphObligationStateAccessPolicy {
        self.state_access_policy
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_str()
    }

    pub(crate) fn contract_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.contract_digest
    }
}
