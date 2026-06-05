use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};
use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
};

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    ForgeQueryAftermathContributionPayload, ForgeQueryAftermathContributionPosture,
    ForgeQueryAftermathRuntimeSemantics,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedAftermathContribution;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAftermathContributionAuthoring {
    payload: ForgeQueryAftermathContributionPayload,
}

impl ForgeQueryAftermathContributionAuthoring {
    pub fn establishes_fact(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryAftermathContributionPosture::EstablishesFact,
            semantic_code,
            detail,
        )
    }

    pub fn consumes_fact(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryAftermathContributionPosture::ConsumesFact,
            semantic_code,
            detail,
        )
    }

    pub fn declares_residue(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryAftermathContributionPosture::DeclaresResidue,
            semantic_code,
            detail,
        )
    }

    pub fn establishes_projection_contract(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        source: ProjectionConsumptionSource,
        binding: ProjectionConsumptionBindingContext,
        requested_facts: ProjectMaterializedFacts,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryAftermathContributionPosture::EstablishesFact,
            semantic_code,
            detail,
            ForgeQueryAftermathRuntimeSemantics::new(source, binding, requested_facts),
        )
    }

    pub fn consumes_projection_contract(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        source: ProjectionConsumptionSource,
        binding: ProjectionConsumptionBindingContext,
        requested_facts: ProjectMaterializedFacts,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryAftermathContributionPosture::ConsumesFact,
            semantic_code,
            detail,
            ForgeQueryAftermathRuntimeSemantics::new(source, binding, requested_facts),
        )
    }

    pub fn declares_projection_residue(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        source: ProjectionConsumptionSource,
        binding: ProjectionConsumptionBindingContext,
        requested_facts: ProjectMaterializedFacts,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryAftermathContributionPosture::DeclaresResidue,
            semantic_code,
            detail,
            ForgeQueryAftermathRuntimeSemantics::new(source, binding, requested_facts),
        )
    }

    pub fn for_admitted_intent_plan(
        self,
        plan: &ForgeQueryAdmittedIntentPlan,
    ) -> ForgeQueryRequestedAftermathContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        self.bind_to_admitted_plan_target(
            ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn for_lower_runtime_boundary_envelope(
        self,
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> ForgeQueryRequestedAftermathContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    > {
        self.bind_to_lower_runtime_boundary_target(
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
                envelope,
            ),
        )
    }

    pub fn for_lower_runtime_boundary_source<S>(
        self,
        source: &S,
    ) -> ForgeQueryRequestedAftermathContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >
    where
        S: ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.for_lower_runtime_boundary_envelope(source.lower_runtime_boundary_envelope())
    }

    pub fn bind_to_admitted_plan_target(
        self,
        target: ForgeQueryAdmittedPlanBoundContributionTarget,
    ) -> ForgeQueryRequestedAftermathContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_lower_runtime_boundary_target(
        self,
        target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ) -> ForgeQueryRequestedAftermathContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    > {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: ForgeQueryAftermathContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: ForgeQueryAftermathContributionPayload::new(posture, semantic_code, detail),
        }
    }

    fn with_runtime_semantics(
        posture: ForgeQueryAftermathContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: ForgeQueryAftermathRuntimeSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryAftermathContributionPayload::with_runtime_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
            ),
        }
    }
}
