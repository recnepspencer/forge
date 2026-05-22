use crate::runtime::{ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration};

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    ForgeQueryAdmissionContributionPayload, ForgeQueryAdmissionContributionPosture,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedAdmissionContribution;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmissionContributionAuthoring {
    payload: ForgeQueryAdmissionContributionPayload,
}

impl ForgeQueryAdmissionContributionAuthoring {
    pub fn advisory(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryAdmissionContributionPosture::Advisory,
            semantic_code,
            detail,
        )
    }

    pub fn advisory_at_stage(
        decision_stage: &'static str,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_decision_stage(
            ForgeQueryAdmissionContributionPosture::Advisory,
            semantic_code,
            detail,
            decision_stage,
        )
    }

    pub fn violation(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryAdmissionContributionPosture::Violation,
            semantic_code,
            detail,
        )
    }

    pub fn violation_at_stage(
        decision_stage: &'static str,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::with_decision_stage(
            ForgeQueryAdmissionContributionPosture::Violation,
            semantic_code,
            detail,
            decision_stage,
        )
    }

    pub fn support_only(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryAdmissionContributionPosture::SupportOnly,
            semantic_code,
            detail,
        )
    }

    pub fn for_intent_declaration(
        self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> ForgeQueryRequestedAdmissionContribution<ForgeQueryDeclarationBoundContributionTarget>
    {
        self.bind_to_declaration_target(
            ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        )
    }

    pub fn for_admitted_intent_plan(
        self,
        plan: &ForgeQueryAdmittedIntentPlan,
    ) -> ForgeQueryRequestedAdmissionContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        self.bind_to_admitted_plan_target(
            ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn bind_to_declaration_target(
        self,
        target: ForgeQueryDeclarationBoundContributionTarget,
    ) -> ForgeQueryRequestedAdmissionContribution<ForgeQueryDeclarationBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_admitted_plan_target(
        self,
        target: ForgeQueryAdmittedPlanBoundContributionTarget,
    ) -> ForgeQueryRequestedAdmissionContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: ForgeQueryAdmissionContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: ForgeQueryAdmissionContributionPayload::new(posture, semantic_code, detail),
        }
    }

    fn with_decision_stage(
        posture: ForgeQueryAdmissionContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        decision_stage: &'static str,
    ) -> Self {
        Self {
            payload: ForgeQueryAdmissionContributionPayload::with_decision_stage(
                posture,
                semantic_code,
                detail,
                decision_stage,
            ),
        }
    }
}
