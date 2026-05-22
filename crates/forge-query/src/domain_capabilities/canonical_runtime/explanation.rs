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
    admit_causal_inspection, materialize_admitted_causal_inspection,
    materialize_advisory_causal_inspection, materialize_denied_causal_inspection,
    request_causal_inspection, CausalInspectionProofFlow, QueryCausalInspectionArtifact,
};

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
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload.semantic_code(),
            domain_contribution.request_digest(),
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
                domain_contribution.request_digest(),
                format!(
                    "causal inspection request denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error.kind()
                ),
            ));
        }
    };

    match admit_causal_inspection(request) {
        CausalInspectionProofFlow::Admitted(inspection) => {
            let Some(envelope) = runtime_semantics.bridge_envelope() else {
                return TransitionOutcome::Denied(missing_bridge_envelope_denial(
                    payload.semantic_code(),
                    domain_contribution.request_digest(),
                ));
            };
            match materialize_admitted_causal_inspection(
                &inspection,
                envelope,
                runtime_semantics.redaction_policy(),
                runtime_semantics.materialization_policy(),
            ) {
                Ok(artifact) => TransitionOutcome::Success(artifact),
                Err(error) => {
                    TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                        "explanation-inspection",
                        domain_contribution.target().kind(),
                        domain_contribution.request_digest(),
                        format!(
                            "admitted causal inspection materialization denied for `{}` with `{:?}`",
                            payload.semantic_code(),
                            error.kind()
                        ),
                    ))
                }
            }
        }
        CausalInspectionProofFlow::Advisory(inspection) => {
            let Some(envelope) = runtime_semantics.bridge_envelope() else {
                return TransitionOutcome::Denied(missing_bridge_envelope_denial(
                    payload.semantic_code(),
                    domain_contribution.request_digest(),
                ));
            };
            match materialize_advisory_causal_inspection(
                &inspection,
                envelope,
                runtime_semantics.redaction_policy(),
                runtime_semantics.materialization_policy(),
            ) {
                Ok(artifact) => TransitionOutcome::Success(artifact),
                Err(error) => {
                    TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                        "explanation-inspection",
                        domain_contribution.target().kind(),
                        domain_contribution.request_digest(),
                        format!(
                            "advisory causal inspection materialization denied for `{}` with `{:?}`",
                            payload.semantic_code(),
                            error.kind()
                        ),
                    ))
                }
            }
        }
        CausalInspectionProofFlow::Denied(inspection) => {
            TransitionOutcome::Success(materialize_denied_causal_inspection(
                &inspection,
                None,
                runtime_semantics.redaction_policy(),
                runtime_semantics.materialization_policy(),
            ))
        }
    }
}

fn missing_runtime_semantics_denial(
    semantic_code: &str,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "explanation-inspection",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
        request_digest,
        format!(
            "causal inspection materialization requires runtime explanation semantics for `{semantic_code}`"
        ),
    )
}

fn missing_bridge_envelope_denial(
    semantic_code: &str,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "explanation-inspection",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope,
        request_digest,
        format!(
            "bridge-backed causal inspection materialization requires a bridge envelope for `{semantic_code}`"
        ),
    )
}
