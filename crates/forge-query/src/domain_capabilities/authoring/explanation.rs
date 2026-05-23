use forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope;

use crate::runtime::{
    CausalEvidenceFamily, CausalEvidenceReferenceSet, CausalInspectionExplanationFamily,
    CausalInspectionMaterializationPolicy, CausalInspectionRedactionPolicy,
    CausalInspectionRichness, CausalInspectionTarget, ForgeQueryAdmittedIntentPlan,
    ForgeQueryIntentDeclaration, ForgeQueryLowerRuntimeBoundaryEnvelope,
};

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    ForgeQueryExplanationContributionPayload, ForgeQueryExplanationContributionPosture,
    ForgeQueryExplanationRuntimeSemantics,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedExplanationContribution;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExplanationContributionAuthoring {
    payload: ForgeQueryExplanationContributionPayload,
}

impl ForgeQueryExplanationContributionAuthoring {
    pub fn requires_context(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryExplanationContributionPosture::RequiresContext,
            semantic_code,
            detail,
        )
    }

    pub fn explains_fallback(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryExplanationContributionPosture::ExplainsFallback,
            semantic_code,
            detail,
        )
    }

    pub fn explains_ambiguity(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryExplanationContributionPosture::ExplainsAmbiguity,
            semantic_code,
            detail,
        )
    }

    pub fn cross_runtime_causal_context(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: BridgeCausalExplanationEnvelope,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryExplanationContributionPosture::RequiresContext,
            semantic_code,
            detail,
            ForgeQueryExplanationRuntimeSemantics::new(
                reference_set,
                target,
                CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation,
                CausalInspectionRichness::ReferenceOnly,
                requested_evidence_families,
                Some(bridge_envelope),
                redaction_policy,
                materialization_policy,
            ),
        )
    }

    pub fn cross_runtime_fallback_explanation(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        bridge_envelope: BridgeCausalExplanationEnvelope,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryExplanationContributionPosture::ExplainsFallback,
            semantic_code,
            detail,
            ForgeQueryExplanationRuntimeSemantics::new(
                reference_set,
                target,
                CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation,
                CausalInspectionRichness::MaterializedDetail,
                requested_evidence_families,
                Some(bridge_envelope),
                redaction_policy,
                materialization_policy,
            ),
        )
    }

    pub fn store_backed_replay_gap_explanation(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        reference_set: CausalEvidenceReferenceSet,
        target: CausalInspectionTarget,
        requested_evidence_families: Vec<CausalEvidenceFamily>,
        redaction_policy: CausalInspectionRedactionPolicy,
        materialization_policy: CausalInspectionMaterializationPolicy,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryExplanationContributionPosture::ExplainsAmbiguity,
            semantic_code,
            detail,
            ForgeQueryExplanationRuntimeSemantics::new(
                reference_set,
                target,
                CausalInspectionExplanationFamily::StoreBackedReplayReconstruction,
                CausalInspectionRichness::ReferenceOnly,
                requested_evidence_families,
                None,
                redaction_policy,
                materialization_policy,
            ),
        )
    }

    pub fn for_intent_declaration(
        self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> ForgeQueryRequestedExplanationContribution<ForgeQueryDeclarationBoundContributionTarget>
    {
        self.bind_to_declaration_target(
            ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        )
    }

    pub fn for_admitted_intent_plan(
        self,
        plan: &ForgeQueryAdmittedIntentPlan,
    ) -> ForgeQueryRequestedExplanationContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        self.bind_to_admitted_plan_target(
            ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn for_lower_runtime_boundary_envelope(
        self,
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> ForgeQueryRequestedExplanationContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    > {
        self.bind_to_lower_runtime_boundary_target(
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
                envelope,
            ),
        )
    }

    pub fn bind_to_declaration_target(
        self,
        target: ForgeQueryDeclarationBoundContributionTarget,
    ) -> ForgeQueryRequestedExplanationContribution<ForgeQueryDeclarationBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_admitted_plan_target(
        self,
        target: ForgeQueryAdmittedPlanBoundContributionTarget,
    ) -> ForgeQueryRequestedExplanationContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_lower_runtime_boundary_target(
        self,
        target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ) -> ForgeQueryRequestedExplanationContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    > {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: ForgeQueryExplanationContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: ForgeQueryExplanationContributionPayload::new(posture, semantic_code, detail),
        }
    }

    fn with_runtime_semantics(
        posture: ForgeQueryExplanationContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: ForgeQueryExplanationRuntimeSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryExplanationContributionPayload::with_runtime_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
            ),
        }
    }
}
