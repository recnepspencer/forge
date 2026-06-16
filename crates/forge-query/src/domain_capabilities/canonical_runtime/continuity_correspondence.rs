use forge_proof::TransitionOutcome;

use crate::correspondence::{
    resolve_correspondence_evidence, CorrespondenceEvaluationFailureClass,
    CorrespondenceEvidenceResolved,
};
use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryContinuityContributionPayload, ForgeQueryContinuityContributionPosture,
};
use crate::domain_capabilities::targets::ForgeQueryAdmittedPlanBoundContributionTarget;
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyContinuityContribution,
};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;

pub fn materialize_correspondence_evidence_resolved(
    contribution: ForgeQueryMaterializationReadyContinuityContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<CorrespondenceEvidenceResolved> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();

    if payload.posture() != ForgeQueryContinuityContributionPosture::CorrespondenceOnly {
        return TransitionOutcome::Denied(unsupported_posture_denial(
            payload,
            domain_contribution.request_identity().clone(),
        ));
    }

    let Some(correspondence_semantics) = payload.correspondence_semantics() else {
        return TransitionOutcome::Denied(missing_correspondence_semantics_denial(
            payload,
            domain_contribution.request_identity().clone(),
        ));
    };

    match resolve_correspondence_evidence(correspondence_semantics.to_request()) {
        Ok(resolved) => TransitionOutcome::Success(resolved),
        Err(error) => TransitionOutcome::Denied(correspondence_error_denial(
            payload,
            domain_contribution.request_identity().clone(),
            error.failure_class(),
        )),
    }
}

fn missing_correspondence_semantics_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "continuity correspondence materialization requires correspondence semantics for `{}`",
            payload.semantic_code()
        ),
    )
}

fn unsupported_posture_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    request_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "continuity correspondence materialization only supports `correspondence-only`; got `{}`",
            payload.posture().as_str()
        ),
    )
}

fn correspondence_error_denial(
    payload: &ForgeQueryContinuityContributionPayload,
    request_identity: ForgeQueryEvidenceIdentity,
    failure_class: CorrespondenceEvaluationFailureClass,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    let denial_kind = match failure_class {
        CorrespondenceEvaluationFailureClass::InvalidRequest => {
            ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics
        }
        CorrespondenceEvaluationFailureClass::UnsupportedTopology
        | CorrespondenceEvaluationFailureClass::UnsupportedStructuralFamily
        | CorrespondenceEvaluationFailureClass::UnsupportedMixedEvidence
        | CorrespondenceEvaluationFailureClass::BroadStructuralScanRequired
        | CorrespondenceEvaluationFailureClass::StructuralBreadthExceeded => {
            ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
        }
    };

    ForgeQueryDomainCapabilityProgressionDenial::new(
        denial_kind,
        "continuity-lineage",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "continuity correspondence materialization denied for `{}` with `{}`",
            payload.semantic_code(),
            failure_class_label(failure_class),
        ),
    )
}

fn failure_class_label(failure_class: CorrespondenceEvaluationFailureClass) -> &'static str {
    match failure_class {
        CorrespondenceEvaluationFailureClass::InvalidRequest => "invalid-request",
        CorrespondenceEvaluationFailureClass::UnsupportedTopology => "unsupported-topology",
        CorrespondenceEvaluationFailureClass::UnsupportedStructuralFamily => {
            "unsupported-structural-family"
        }
        CorrespondenceEvaluationFailureClass::UnsupportedMixedEvidence => {
            "unsupported-mixed-evidence"
        }
        CorrespondenceEvaluationFailureClass::BroadStructuralScanRequired => {
            "broad-structural-scan-required"
        }
        CorrespondenceEvaluationFailureClass::StructuralBreadthExceeded => {
            "structural-breadth-exceeded"
        }
    }
}
