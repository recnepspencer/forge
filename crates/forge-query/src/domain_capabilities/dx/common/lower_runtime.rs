use crate::domain_capabilities::authoring::{
    ForgeQueryExplanationContributionAuthoring, ForgeQuerySupportContributionAuthoring,
};
use crate::domain_capabilities::canonical_runtime::{
    materialize_lower_runtime_support_traceability_artifact,
    materialize_query_causal_inspection_artifact, materialize_query_causal_inspection_review,
    ForgeQueryExplanationInspectionReview,
    ForgeQueryLowerRuntimeBoundarySupportTraceabilityArtifact,
};
use crate::domain_capabilities::dx::checked::{
    ForgeQueryCheckedDomainCapabilityOutcome, ForgeQueryDomainCapabilityMaterializationError,
};
use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryExplanationContributionPosture, ForgeQuerySupportContributionPosture,
};
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityTargetKind, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::runtime::QueryCausalInspectionArtifact;

use super::lower_runtime_explanation_request::{
    ForgeQueryLowerRuntimeExplanationRequest, ForgeQueryLowerRuntimeExplanationRequestKind,
};
use super::shared::{materialize_common_lane, qualify_semantic_code};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeDomainContributionSurface {
    pub(crate) domain: String,
    pub(crate) target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
}

impl ForgeQueryLowerRuntimeDomainContributionSurface {
    pub fn supports_boundary_traceability(
        self,
        semantic_code: impl Into<String>,
    ) -> ForgeQueryLowerRuntimeSupportDraft {
        ForgeQueryLowerRuntimeSupportDraft {
            domain: self.domain,
            target: self.target,
            semantic_code: semantic_code.into(),
        }
    }

    pub fn requires_cross_runtime_context(
        self,
        semantic_code: impl Into<String>,
        request: ForgeQueryLowerRuntimeExplanationRequest,
    ) -> ForgeQueryLowerRuntimeExplanationDraft {
        ForgeQueryLowerRuntimeExplanationDraft::new(
            self.domain,
            self.target,
            semantic_code,
            request.kind(),
        )
    }

    pub fn explains_cross_runtime_fallback(
        self,
        semantic_code: impl Into<String>,
        request: ForgeQueryLowerRuntimeExplanationRequest,
    ) -> ForgeQueryLowerRuntimeExplanationDraft {
        ForgeQueryLowerRuntimeExplanationDraft::new(
            self.domain,
            self.target,
            semantic_code,
            request.kind(),
        )
    }

    pub fn explains_store_backed_replay_gap(
        self,
        semantic_code: impl Into<String>,
        request: ForgeQueryLowerRuntimeExplanationRequest,
    ) -> ForgeQueryLowerRuntimeExplanationDraft {
        ForgeQueryLowerRuntimeExplanationDraft::new(
            self.domain,
            self.target,
            semantic_code,
            request.kind(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeSupportDraft {
    domain: String,
    target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    semantic_code: String,
}

impl ForgeQueryLowerRuntimeSupportDraft {
    pub fn because(self, detail: impl Into<String>) -> ForgeQueryLowerRuntimeSupportContribution {
        ForgeQueryLowerRuntimeSupportContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeSupportContribution {
    domain: String,
    target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    semantic_code: String,
    detail: String,
}

impl ForgeQueryLowerRuntimeSupportContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<
        ForgeQueryLowerRuntimeBoundarySupportTraceabilityArtifact,
    > {
        let target = self.target.clone();
        let requested = ForgeQuerySupportContributionAuthoring::narrowed_support(
            qualify_semantic_code(&self.domain, &self.semantic_code),
            self.detail,
        )
        .bind_to_lower_runtime_boundary_target(self.target);

        materialize_common_lane(
            "support-traceability",
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
            ForgeQuerySupportContributionPosture::NarrowedSupport.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize_lower_runtime_support_traceability_artifact,
        )
    }

    pub fn materialize(
        self,
    ) -> Result<
        ForgeQueryLowerRuntimeBoundarySupportTraceabilityArtifact,
        ForgeQueryDomainCapabilityMaterializationError,
    > {
        self.try_materialize().into_result()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeExplanationDraft {
    domain: String,
    target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    semantic_code: String,
    kind: ForgeQueryLowerRuntimeExplanationRequestKind,
}

impl ForgeQueryLowerRuntimeExplanationDraft {
    fn new(
        domain: String,
        target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        semantic_code: impl Into<String>,
        kind: ForgeQueryLowerRuntimeExplanationRequestKind,
    ) -> Self {
        Self {
            domain,
            target,
            semantic_code: semantic_code.into(),
            kind,
        }
    }

    pub fn because(
        self,
        detail: impl Into<String>,
    ) -> ForgeQueryLowerRuntimeExplanationContribution {
        ForgeQueryLowerRuntimeExplanationContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            detail: detail.into(),
            kind: self.kind,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeExplanationContribution {
    domain: String,
    target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    semantic_code: String,
    detail: String,
    kind: ForgeQueryLowerRuntimeExplanationRequestKind,
}

impl ForgeQueryLowerRuntimeExplanationContribution {
    pub fn try_review(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ForgeQueryExplanationInspectionReview> {
        let posture = self.posture();
        let target = self.target.clone();
        let requested = self.into_requested();

        materialize_common_lane(
            "explanation-inspection",
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
            posture.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize_query_causal_inspection_review,
        )
    }

    pub fn review(
        self,
    ) -> Result<ForgeQueryExplanationInspectionReview, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_review().into_result()
    }

    pub fn try_materialize_artifact(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<QueryCausalInspectionArtifact> {
        let posture = self.posture();
        let target = self.target.clone();
        let requested = self.into_requested();

        materialize_common_lane(
            "explanation-inspection",
            ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
            posture.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize_query_causal_inspection_artifact,
        )
    }

    pub fn materialize_artifact(
        self,
    ) -> Result<QueryCausalInspectionArtifact, ForgeQueryDomainCapabilityMaterializationError> {
        self.try_materialize_artifact().into_result()
    }

    fn posture(&self) -> ForgeQueryExplanationContributionPosture {
        match self.kind {
            ForgeQueryLowerRuntimeExplanationRequestKind::CrossRuntimeContext { .. } => {
                ForgeQueryExplanationContributionPosture::RequiresContext
            }
            ForgeQueryLowerRuntimeExplanationRequestKind::CrossRuntimeFallback { .. } => {
                ForgeQueryExplanationContributionPosture::ExplainsFallback
            }
            ForgeQueryLowerRuntimeExplanationRequestKind::StoreBackedReplayGap { .. } => {
                ForgeQueryExplanationContributionPosture::ExplainsAmbiguity
            }
        }
    }

    fn into_requested(
        self,
    ) -> crate::domain_capabilities::ForgeQueryRequestedExplanationContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    > {
        let semantic_code = qualify_semantic_code(&self.domain, &self.semantic_code);
        match self.kind {
            ForgeQueryLowerRuntimeExplanationRequestKind::CrossRuntimeContext {
                reference_set,
                target,
                requested_evidence_families,
                bridge_envelope,
                redaction_policy,
                materialization_policy,
            } => ForgeQueryExplanationContributionAuthoring::cross_runtime_causal_context(
                semantic_code,
                self.detail,
                reference_set,
                target,
                requested_evidence_families,
                bridge_envelope,
                redaction_policy,
                materialization_policy,
            )
            .bind_to_lower_runtime_boundary_target(self.target),
            ForgeQueryLowerRuntimeExplanationRequestKind::CrossRuntimeFallback {
                reference_set,
                target,
                requested_evidence_families,
                bridge_envelope,
                redaction_policy,
                materialization_policy,
            } => ForgeQueryExplanationContributionAuthoring::cross_runtime_fallback_explanation(
                semantic_code,
                self.detail,
                reference_set,
                target,
                requested_evidence_families,
                bridge_envelope,
                redaction_policy,
                materialization_policy,
            )
            .bind_to_lower_runtime_boundary_target(self.target),
            ForgeQueryLowerRuntimeExplanationRequestKind::StoreBackedReplayGap {
                reference_set,
                target,
                requested_evidence_families,
                redaction_policy,
                materialization_policy,
            } => ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
                semantic_code,
                self.detail,
                reference_set,
                target,
                requested_evidence_families,
                redaction_policy,
                materialization_policy,
            )
            .bind_to_lower_runtime_boundary_target(self.target),
        }
    }
}
