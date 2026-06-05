use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
};

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    ForgeQuerySupportContributionPayload, ForgeQuerySupportContributionPosture,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedSupportContribution;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportContributionAuthoring {
    payload: ForgeQuerySupportContributionPayload,
}

impl ForgeQuerySupportContributionAuthoring {
    pub fn declaration_support(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQuerySupportContributionPosture::DeclarationSupport,
            semantic_code,
            detail,
        )
    }

    pub fn declaration_traceability(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQuerySupportContributionPosture::DeclarationTraceability,
            semantic_code,
            detail,
        )
    }

    pub fn narrowed_support(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQuerySupportContributionPosture::NarrowedSupport,
            semantic_code,
            detail,
        )
    }

    pub fn for_intent_declaration(
        self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> ForgeQueryRequestedSupportContribution<ForgeQueryDeclarationBoundContributionTarget> {
        self.bind_to_declaration_target(
            ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        )
    }

    pub fn for_admitted_intent_plan(
        self,
        plan: &ForgeQueryAdmittedIntentPlan,
    ) -> ForgeQueryRequestedSupportContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
        self.bind_to_admitted_plan_target(
            ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn for_lower_runtime_boundary_envelope(
        self,
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> ForgeQueryRequestedSupportContribution<ForgeQueryLowerRuntimeBoundaryBoundContributionTarget>
    {
        self.bind_to_lower_runtime_boundary_target(
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
                envelope,
            ),
        )
    }

    pub fn for_lower_runtime_boundary_source<S>(
        self,
        source: &S,
    ) -> ForgeQueryRequestedSupportContribution<ForgeQueryLowerRuntimeBoundaryBoundContributionTarget>
    where
        S: ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.for_lower_runtime_boundary_envelope(source.lower_runtime_boundary_envelope())
    }

    pub fn bind_to_declaration_target(
        self,
        target: ForgeQueryDeclarationBoundContributionTarget,
    ) -> ForgeQueryRequestedSupportContribution<ForgeQueryDeclarationBoundContributionTarget> {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_admitted_plan_target(
        self,
        target: ForgeQueryAdmittedPlanBoundContributionTarget,
    ) -> ForgeQueryRequestedSupportContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_lower_runtime_boundary_target(
        self,
        target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ) -> ForgeQueryRequestedSupportContribution<ForgeQueryLowerRuntimeBoundaryBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: ForgeQuerySupportContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: ForgeQuerySupportContributionPayload::new(posture, semantic_code, detail),
        }
    }
}
