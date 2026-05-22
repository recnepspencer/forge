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
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentAdvisoryDecision,
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
            TransitionOutcome::Denied(unsupported_posture_denial(
                payload.semantic_code(),
                domain_contribution.request_digest(),
            ))
        }
    }
}

fn unsupported_posture_denial(
    semantic_code: &str,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "admission",
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
        request_digest,
        format!(
            "admission runtime decision materialization only supports advisory and violation postures; got `{semantic_code}`"
        ),
    )
}
