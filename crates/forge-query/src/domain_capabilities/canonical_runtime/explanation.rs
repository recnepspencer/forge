use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::targets::{
    ForgeQueryDomainCapabilityTargetBinding, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalExplanationArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyExplanationContribution,
};
use crate::runtime::{
    materialize_admitted_causal_inspection, materialize_advisory_causal_inspection,
    materialize_denied_causal_inspection, request_causal_inspection, CausalInspectionPlan,
    CausalInspectionProofFlow, QueryCausalInspectionArtifact,
};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExplanationInspectionReview {
    semantic_code: String,
    request_identity: ForgeQueryEvidenceIdentity,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    bridge_envelope: Option<forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope>,
    plan: CausalInspectionPlan,
}

impl ForgeQueryExplanationInspectionReview {
    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_for_reporting(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn target_kind(&self) -> crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind {
        self.target_kind
    }

    pub fn bridge_envelope(
        &self,
    ) -> Option<&forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope> {
        self.bridge_envelope.as_ref()
    }

    pub fn plan(&self) -> &CausalInspectionPlan {
        &self.plan
    }
}

pub fn materialize_canonical_explanation_artifact<T>(
    contribution: ForgeQueryMaterializationReadyExplanationContribution<T>,
) -> ForgeQueryCanonicalExplanationArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

pub fn materialize_query_causal_inspection_artifact(
    contribution: ForgeQueryMaterializationReadyExplanationContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<QueryCausalInspectionArtifact> {
    let review = match materialize_query_causal_inspection_review(contribution) {
        TransitionOutcome::Success(review) => review,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => return TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => {
            return TransitionOutcome::RebindRequired(rebind);
        }
        TransitionOutcome::Failed(failure) => return TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    };
    let domain_plan = review.plan();
    let domain_contribution_semantic_code = review.semantic_code().to_string();
    let request_identity = review.request_identity().clone();

    let domain_target_kind = review.target_kind();
    let materialization_policy = domain_plan.materialization_policy();
    let redaction_policy = domain_plan.redaction_policy();
    match domain_plan.admission() {
        CausalInspectionProofFlow::Admitted(inspection) => {
            let Some(envelope) = review.bridge_envelope() else {
                return TransitionOutcome::Denied(missing_bridge_envelope_denial(
                    &domain_contribution_semantic_code,
                    request_identity.clone(),
                ));
            };
            match materialize_admitted_causal_inspection(
                inspection,
                envelope,
                redaction_policy,
                materialization_policy,
            ) {
                Ok(artifact) => TransitionOutcome::Success(artifact),
                Err(error) => TransitionOutcome::Denied(
                    ForgeQueryDomainCapabilityProgressionDenial::new(
                        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                        "explanation-inspection",
                        domain_target_kind,
                        request_identity.clone(),
                        format!(
                            "admitted causal inspection materialization denied for `{}` with `{:?}`",
                            domain_contribution_semantic_code,
                            error.kind()
                        ),
                    ),
                ),
            }
        }
        CausalInspectionProofFlow::Advisory(inspection) => {
            let Some(envelope) = review.bridge_envelope() else {
                return TransitionOutcome::Denied(missing_bridge_envelope_denial(
                    &domain_contribution_semantic_code,
                    request_identity.clone(),
                ));
            };
            match materialize_advisory_causal_inspection(
                inspection,
                envelope,
                redaction_policy,
                materialization_policy,
            ) {
                Ok(artifact) => TransitionOutcome::Success(artifact),
                Err(error) => TransitionOutcome::Denied(
                    ForgeQueryDomainCapabilityProgressionDenial::new(
                        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                        "explanation-inspection",
                        domain_target_kind,
                        request_identity.clone(),
                        format!(
                            "advisory causal inspection materialization denied for `{}` with `{:?}`",
                            domain_contribution_semantic_code,
                            error.kind()
                        ),
                    ),
                ),
            }
        }
        CausalInspectionProofFlow::Denied(inspection) => {
            TransitionOutcome::Success(materialize_denied_causal_inspection(
                inspection,
                None,
                redaction_policy,
                materialization_policy,
            ))
        }
    }
}

pub fn materialize_query_causal_inspection_review(
    contribution: ForgeQueryMaterializationReadyExplanationContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryExplanationInspectionReview> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload.semantic_code(),
            domain_contribution.request_identity().clone(),
        ));
    };

    let request = match request_causal_inspection(
        runtime_semantics.reference_set().clone(),
        runtime_semantics.target().clone(),
        runtime_semantics.explanation_family(),
        runtime_semantics.requested_richness(),
        runtime_semantics.requested_evidence_families(),
    ) {
        Ok(request) => request,
        Err(error) => {
            return TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
                "explanation-inspection",
                domain_contribution.target().kind(),
                domain_contribution.request_identity().clone(),
                format!(
                    "causal inspection request denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error.kind()
                ),
            ));
        }
    };

    TransitionOutcome::Success(ForgeQueryExplanationInspectionReview {
        semantic_code: payload.semantic_code().to_string(),
        request_identity: domain_contribution.request_identity().clone(),
        target_kind: domain_contribution.target().kind(),
        bridge_envelope: runtime_semantics.bridge_envelope().cloned(),
        plan: CausalInspectionPlan::from_resolved_request(
            runtime_semantics.reference_set().clone(),
            request,
            runtime_semantics.redaction_policy(),
            runtime_semantics.materialization_policy(),
        ),
    })
}

fn missing_runtime_semantics_denial(
    semantic_code: &str,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "explanation-inspection",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
        request_identity,
        format!(
            "causal inspection materialization requires runtime explanation semantics for `{semantic_code}`"
        ),
    )
}

fn missing_bridge_envelope_denial(
    semantic_code: &str,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "explanation-inspection",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
        request_identity,
        format!(
            "bridge-backed causal inspection materialization requires a bridge envelope for `{semantic_code}`"
        ),
    )
}
