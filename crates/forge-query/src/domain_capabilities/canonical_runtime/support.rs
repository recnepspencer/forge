use forge_proof::TransitionOutcome;

use crate::domain_capabilities::payloads::ForgeQuerySupportContributionPosture;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalSupportTraceabilityArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadySupportContribution,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionSupportTraceabilityReport,
    ForgeQueryIntentAdmissionSupportTraceabilityRow,
};

pub fn materialize_canonical_support_traceability_artifact<T>(
    contribution: ForgeQueryMaterializationReadySupportContribution<T>,
) -> ForgeQueryCanonicalSupportTraceabilityArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

pub fn materialize_intent_admission_support_traceability_report(
    contribution: ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityReport>
{
    match support_traceability_row(&contribution) {
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

pub fn materialize_intent_admission_support_traceability_row(
    contribution: ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityRow> {
    support_traceability_row(&contribution)
}

fn support_traceability_row(
    contribution: &ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityRow> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some((family, entrypoint, ..)) = domain_contribution
        .target()
        .semantics()
        .admitted_intent_plan()
    else {
        unreachable!("admitted-plan target should preserve admitted-plan semantics");
    };
    let Some((_, _, request_digest, eligibility_digest, decision_digest)) = domain_contribution
        .target()
        .semantics()
        .admitted_intent_plan()
    else {
        unreachable!("admitted-plan target should preserve admitted-plan semantics");
    };
    TransitionOutcome::Success(
        ForgeQueryIntentAdmissionSupportTraceabilityRow::new_domain_scoped(
            support_lane(payload.posture()),
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

fn support_lane(posture: ForgeQuerySupportContributionPosture) -> &'static str {
    match posture {
        ForgeQuerySupportContributionPosture::DeclarationSupport => "domain_support",
        ForgeQuerySupportContributionPosture::DeclarationTraceability => "domain_traceability",
        ForgeQuerySupportContributionPosture::NarrowedSupport => "domain_narrowed_support",
    }
}
