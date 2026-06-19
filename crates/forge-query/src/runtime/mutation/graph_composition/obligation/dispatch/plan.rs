use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::super::error::ForgeQueryGraphObligationDispatchError;
use super::super::kind::ForgeQueryGraphObligationKind;
use super::super::rule_identity::ForgeQueryGraphObligationRuleIdentity;
use super::super::verdict::ForgeQueryGraphObligationVerdict;
use crate::runtime::{
    ForgeQueryGraphCompositionDomainInvariantDenial,
    ForgeQueryGraphCompositionDomainInvariantSummary, ForgeQueryGraphObligationExecutionBudget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationDispatchPlan {
    kind: ForgeQueryGraphObligationKind,
    rule_identity: ForgeQueryGraphObligationRuleIdentity,
    execution_budget: ForgeQueryGraphObligationExecutionBudget,
    verdict: ForgeQueryGraphObligationVerdict,
    plan_digest: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationDispatchPlanDraft {
    kind: ForgeQueryGraphObligationKind,
    rule_identity:
        Result<ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationDispatchError>,
    execution_budget: ForgeQueryGraphObligationExecutionBudget,
}

impl ForgeQueryGraphObligationDispatchPlan {
    pub fn blocking_invariant(
        rule_name: impl Into<String>,
    ) -> ForgeQueryGraphObligationDispatchPlanDraft {
        ForgeQueryGraphObligationDispatchPlanDraft::new(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            rule_name,
        )
    }

    pub fn schema_contract_validator(
        rule_name: impl Into<String>,
    ) -> ForgeQueryGraphObligationDispatchPlanDraft {
        ForgeQueryGraphObligationDispatchPlanDraft::new(
            ForgeQueryGraphObligationKind::SchemaContractValidator,
            rule_name,
        )
    }

    pub fn advisory(rule_name: impl Into<String>) -> ForgeQueryGraphObligationDispatchPlanDraft {
        ForgeQueryGraphObligationDispatchPlanDraft::new(
            ForgeQueryGraphObligationKind::AdvisoryObligation,
            rule_name,
        )
    }

    pub fn preflight_sequencing(
        rule_name: impl Into<String>,
    ) -> ForgeQueryGraphObligationDispatchPlanDraft {
        ForgeQueryGraphObligationDispatchPlanDraft::new(
            ForgeQueryGraphObligationKind::PreflightSequencingObligation,
            rule_name,
        )
    }

    pub fn capability_gap_screen(
        rule_name: impl Into<String>,
    ) -> ForgeQueryGraphObligationDispatchPlanDraft {
        ForgeQueryGraphObligationDispatchPlanDraft::new(
            ForgeQueryGraphObligationKind::CapabilityGapScreen,
            rule_name,
        )
    }

    pub fn operating_context_gate(
        rule_name: impl Into<String>,
    ) -> ForgeQueryGraphObligationDispatchPlanDraft {
        ForgeQueryGraphObligationDispatchPlanDraft::new(
            ForgeQueryGraphObligationKind::OperatingContextGate,
            rule_name,
        )
    }

    pub fn kind(&self) -> ForgeQueryGraphObligationKind {
        self.kind
    }

    pub fn rule_identity(&self) -> &ForgeQueryGraphObligationRuleIdentity {
        &self.rule_identity
    }

    pub fn verdict(&self) -> &ForgeQueryGraphObligationVerdict {
        &self.verdict
    }

    pub fn execution_budget(&self) -> &ForgeQueryGraphObligationExecutionBudget {
        &self.execution_budget
    }

    pub fn plan_digest(&self) -> &str {
        self.plan_digest.as_str()
    }

    pub(crate) fn plan_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.plan_digest
    }

    pub fn graph_composition_domain_invariant_denial(
        &self,
        domain_invariant_summary: ForgeQueryGraphCompositionDomainInvariantSummary,
    ) -> Option<ForgeQueryGraphCompositionDomainInvariantDenial> {
        if !self.verdict.is_blocking() {
            return None;
        }
        if !matches!(
            self.kind,
            ForgeQueryGraphObligationKind::BlockingInvariant
                | ForgeQueryGraphObligationKind::SchemaContractValidator
        ) {
            return None;
        }
        let context = self.verdict.context()?;
        Some(
            ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
                self.rule_identity.domain_invariant_family(),
                context.to_string(),
                domain_invariant_summary,
            ),
        )
    }
}

impl ForgeQueryGraphObligationDispatchPlanDraft {
    fn new(kind: ForgeQueryGraphObligationKind, rule_name: impl Into<String>) -> Self {
        Self {
            kind,
            rule_identity: ForgeQueryGraphObligationRuleIdentity::new(
                "graph-obligation",
                rule_name,
                "v1",
            ),
            execution_budget:
                ForgeQueryGraphObligationExecutionBudget::selection_only_deferred_execution(),
        }
    }

    pub fn with_rule_identity(
        mut self,
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
    ) -> Self {
        self.rule_identity = Ok(rule_identity);
        self
    }

    pub fn with_execution_budget(
        mut self,
        execution_budget: ForgeQueryGraphObligationExecutionBudget,
    ) -> Self {
        self.execution_budget = execution_budget;
        self
    }

    pub fn verdict(
        self,
        verdict: ForgeQueryGraphObligationVerdict,
    ) -> Result<ForgeQueryGraphObligationDispatchPlan, ForgeQueryGraphObligationDispatchError> {
        let rule_identity = self.rule_identity?;
        let plan_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationDispatchPlan)
                .field_shape(ForgeQueryEvidenceTag::new("kind"), self.kind.as_str())
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("rule"),
                    rule_identity.identity_evidence_digest(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("verdict"), verdict.as_str())
                .optional_value(ForgeQueryEvidenceTag::new("context"), verdict.context())
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("execution_budget"),
                    self.execution_budget.budget_evidence_digest(),
                )
                .seal();
        Ok(ForgeQueryGraphObligationDispatchPlan {
            kind: self.kind,
            rule_identity,
            execution_budget: self.execution_budget,
            verdict,
            plan_digest,
        })
    }
}
