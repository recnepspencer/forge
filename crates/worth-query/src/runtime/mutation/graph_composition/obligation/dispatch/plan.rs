use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::super::error::WorthQueryGraphObligationDispatchError;
use super::super::kind::WorthQueryGraphObligationKind;
use super::super::rule_identity::WorthQueryGraphObligationRuleIdentity;
use super::super::verdict::WorthQueryGraphObligationVerdict;
use crate::runtime::{
    WorthQueryGraphCompositionDomainInvariantDenial,
    WorthQueryGraphCompositionDomainInvariantSummary, WorthQueryGraphObligationExecutionBudget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDispatchPlan {
    kind: WorthQueryGraphObligationKind,
    rule_identity: WorthQueryGraphObligationRuleIdentity,
    execution_budget: WorthQueryGraphObligationExecutionBudget,
    verdict: WorthQueryGraphObligationVerdict,
    plan_digest: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDispatchPlanDraft {
    kind: WorthQueryGraphObligationKind,
    rule_identity:
        Result<WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationDispatchError>,
    execution_budget: WorthQueryGraphObligationExecutionBudget,
}

impl WorthQueryGraphObligationDispatchPlan {
    pub fn blocking_invariant(
        rule_name: impl Into<String>,
    ) -> WorthQueryGraphObligationDispatchPlanDraft {
        WorthQueryGraphObligationDispatchPlanDraft::new(
            WorthQueryGraphObligationKind::BlockingInvariant,
            rule_name,
        )
    }

    pub fn schema_contract_validator(
        rule_name: impl Into<String>,
    ) -> WorthQueryGraphObligationDispatchPlanDraft {
        WorthQueryGraphObligationDispatchPlanDraft::new(
            WorthQueryGraphObligationKind::SchemaContractValidator,
            rule_name,
        )
    }

    pub fn advisory(rule_name: impl Into<String>) -> WorthQueryGraphObligationDispatchPlanDraft {
        WorthQueryGraphObligationDispatchPlanDraft::new(
            WorthQueryGraphObligationKind::AdvisoryObligation,
            rule_name,
        )
    }

    pub fn preflight_sequencing(
        rule_name: impl Into<String>,
    ) -> WorthQueryGraphObligationDispatchPlanDraft {
        WorthQueryGraphObligationDispatchPlanDraft::new(
            WorthQueryGraphObligationKind::PreflightSequencingObligation,
            rule_name,
        )
    }

    pub fn capability_gap_screen(
        rule_name: impl Into<String>,
    ) -> WorthQueryGraphObligationDispatchPlanDraft {
        WorthQueryGraphObligationDispatchPlanDraft::new(
            WorthQueryGraphObligationKind::CapabilityGapScreen,
            rule_name,
        )
    }

    pub fn operating_context_gate(
        rule_name: impl Into<String>,
    ) -> WorthQueryGraphObligationDispatchPlanDraft {
        WorthQueryGraphObligationDispatchPlanDraft::new(
            WorthQueryGraphObligationKind::OperatingContextGate,
            rule_name,
        )
    }

    pub fn kind(&self) -> WorthQueryGraphObligationKind {
        self.kind
    }

    pub fn rule_identity(&self) -> &WorthQueryGraphObligationRuleIdentity {
        &self.rule_identity
    }

    pub fn verdict(&self) -> &WorthQueryGraphObligationVerdict {
        &self.verdict
    }

    pub fn execution_budget(&self) -> &WorthQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn plan_digest(&self) -> &str {
        self.plan_digest.as_str()
    }

    pub(crate) fn plan_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.plan_digest
    }

    pub fn graph_composition_domain_invariant_denial(
        &self,
        domain_invariant_summary: WorthQueryGraphCompositionDomainInvariantSummary,
    ) -> Option<WorthQueryGraphCompositionDomainInvariantDenial> {
        if !self.verdict.is_blocking() {
            return None;
        }
        if !matches!(
            self.kind,
            WorthQueryGraphObligationKind::BlockingInvariant
                | WorthQueryGraphObligationKind::SchemaContractValidator
        ) {
            return None;
        }
        let context = self.verdict.context()?;
        Some(
            WorthQueryGraphCompositionDomainInvariantDenial::from_contributed(
                self.rule_identity.domain_invariant_family(),
                context.to_string(),
                domain_invariant_summary,
            ),
        )
    }
}

impl WorthQueryGraphObligationDispatchPlanDraft {
    fn new(kind: WorthQueryGraphObligationKind, rule_name: impl Into<String>) -> Self {
        Self {
            kind,
            rule_identity: WorthQueryGraphObligationRuleIdentity::new(
                "graph-obligation",
                rule_name,
                "v1",
            ),
            execution_budget:
                WorthQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
        }
    }

    pub fn with_rule_identity(
        mut self,
        rule_identity: WorthQueryGraphObligationRuleIdentity,
    ) -> Self {
        self.rule_identity = Ok(rule_identity);
        self
    }

    pub fn with_execution_budget(
        mut self,
        execution_budget: WorthQueryGraphObligationExecutionBudget,
    ) -> Self {
        self.execution_budget = execution_budget;
        self
    }

    pub fn verdict(
        self,
        verdict: WorthQueryGraphObligationVerdict,
    ) -> Result<WorthQueryGraphObligationDispatchPlan, WorthQueryGraphObligationDispatchError> {
        let rule_identity = self.rule_identity?;
        let plan_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationDispatchPlan)
                .field_shape(WorthQueryEvidenceTag::new("kind"), self.kind.as_str())
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("rule"),
                    rule_identity.identity_evidence_digest(),
                )
                .field_shape(WorthQueryEvidenceTag::new("verdict"), verdict.as_str())
                .optional_value(WorthQueryEvidenceTag::new("context"), verdict.context())
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("execution_budget"),
                    self.execution_budget.budget_evidence_digest(),
                )
                .seal();
        Ok(WorthQueryGraphObligationDispatchPlan {
            kind: self.kind,
            rule_identity,
            execution_budget: self.execution_budget,
            verdict,
            plan_digest,
        })
    }
}
