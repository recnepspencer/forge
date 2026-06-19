use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphObligationExecutionBudget, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationSupportLane,
};

use super::state_access_policy::ForgeQueryGraphObligationStateAccessPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationExecutorContract {
    obligation_kind: ForgeQueryGraphObligationKind,
    rule_identity: ForgeQueryGraphObligationRuleIdentity,
    support_lane: ForgeQueryGraphObligationSupportLane,
    execution_budget: ForgeQueryGraphObligationExecutionBudget,
    state_access_policy: ForgeQueryGraphObligationStateAccessPolicy,
    contract_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationExecutorContract {
    pub fn new(
        obligation_kind: ForgeQueryGraphObligationKind,
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        support_lane: ForgeQueryGraphObligationSupportLane,
        execution_budget: ForgeQueryGraphObligationExecutionBudget,
        state_access_policy: ForgeQueryGraphObligationStateAccessPolicy,
    ) -> Self {
        let contract_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationExecutorContract)
                .field_shape(
                    ForgeQueryEvidenceTag::new("obligation_kind"),
                    obligation_kind.as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("rule"),
                    rule_identity.identity_evidence_digest(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("support_lane"),
                    support_lane.as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution_budget"),
                    execution_budget.budget_evidence_digest(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("state_access_policy"),
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

    pub fn obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn rule_identity(&self) -> &ForgeQueryGraphObligationRuleIdentity {
        &self.rule_identity
    }

    pub fn support_lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn execution_budget(&self) -> &ForgeQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn state_access_policy(&self) -> ForgeQueryGraphObligationStateAccessPolicy {
        self.state_access_policy
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_str()
    }

    pub(crate) fn contract_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.contract_digest
    }
}
