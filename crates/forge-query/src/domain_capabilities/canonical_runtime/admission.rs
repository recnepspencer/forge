use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::ForgeQueryAdmissionContributionPosture;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalAdmissionArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyAdmissionContribution,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdmissionSupportTraceabilityReport,
    ForgeQueryIntentAdmissionSupportTraceabilityRow, ForgeQueryIntentAdvisoryDecision,
    ForgeQueryIntentViolationDecision,
};

pub fn materialize_canonical_admission_artifact<T>(
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<T>,
) -> ForgeQueryCanonicalAdmissionArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

pub fn materialize_runtime_admission_decision(
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionDecision> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let target = domain_contribution.target();
    let Some((family, entrypoint, request_digest, eligibility_digest, _decision_digest)) =
        target.semantics().admitted_intent_plan()
    else {
        unreachable!("admitted-plan bound target should preserve admitted-plan semantics")
    };

    match payload.posture() {
        ForgeQueryAdmissionContributionPosture::Advisory => TransitionOutcome::Success(
            ForgeQueryIntentAdmissionDecision::Advisory(ForgeQueryIntentAdvisoryDecision::new(
                family,
                entrypoint,
                payload.decision_stage(),
                payload.detail(),
                request_digest,
                eligibility_digest,
            )),
        ),
        ForgeQueryAdmissionContributionPosture::Violation => TransitionOutcome::Success(
            ForgeQueryIntentAdmissionDecision::Violation(ForgeQueryIntentViolationDecision::new(
                family,
                entrypoint,
                payload.decision_stage(),
                payload.detail(),
                request_digest,
                eligibility_digest,
            )),
        ),
        ForgeQueryAdmissionContributionPosture::SupportOnly => {
            TransitionOutcome::Denied(unsupported_decision_posture_denial(
                payload.posture(),
                domain_contribution.request_identity().clone(),
            ))
        }
    }
}

pub fn materialize_runtime_admission_support_traceability_report(
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityReport>
{
    match materialize_runtime_admission_support_traceability_row(contribution) {
        TransitionOutcome::Success(row) => TransitionOutcome::Success(
            ForgeQueryIntentAdmissionSupportTraceabilityReport::from_rows(vec![row]),
        ),
        TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    }
}

pub fn materialize_runtime_admission_support_traceability_row(
    contribution: ForgeQueryMaterializationReadyAdmissionContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityRow> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some((family, entrypoint, request_digest, eligibility_digest, decision_digest)) =
        domain_contribution
            .target()
            .semantics()
            .admitted_intent_plan()
    else {
        unreachable!("admitted-plan bound target should preserve admitted-plan semantics")
    };

    if payload.posture() != ForgeQueryAdmissionContributionPosture::SupportOnly {
        return TransitionOutcome::Denied(unsupported_support_posture_denial(
            payload.posture(),
            domain_contribution.request_identity().clone(),
        ));
    }

    TransitionOutcome::Success(
        ForgeQueryIntentAdmissionSupportTraceabilityRow::new_domain_scoped(
            "admission_local_support",
            family.as_str(),
            entrypoint.as_str(),
            format!("{}:{}", payload.semantic_code(), payload.detail()),
            Some(domain_contribution.target().binding_digest().to_string()),
            Some(request_digest.to_string()),
            Some(eligibility_digest.to_string()),
            Some(decision_digest.to_string()),
        ),
    )
}

fn unsupported_decision_posture_denial(
    posture: ForgeQueryAdmissionContributionPosture,
    request_identity: crate::evidence_identity::ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "admission",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "admission runtime decision materialization only supports advisory and violation postures; got `{}`",
            posture.as_str()
        ),
    )
}

fn unsupported_support_posture_denial(
    posture: ForgeQueryAdmissionContributionPosture,
    request_identity: crate::evidence_identity::ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "admission",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_identity,
        format!(
            "admission support traceability materialization only supports support-only posture; got `{}`",
            posture.as_str()
        ),
    )
}
