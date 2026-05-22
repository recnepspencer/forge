use forge_proof::TransitionOutcome;

use super::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use super::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryDomainCapabilityTargetBinding, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use super::{
    ForgeQueryDomainCapabilityPayload, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyDomainCapabilityContribution,
    ForgeQueryRequestedDomainCapabilityContribution,
};

pub(super) fn ready<P, T>(
    requested: ForgeQueryRequestedDomainCapabilityContribution<P, T>,
) -> ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let admitted = success(admit_eligible_domain_capability_contribution(eligible));
    let target = admitted.payload().target().clone();
    success(prepare_admitted_domain_capability_contribution_for_materialization(admitted, target))
}

pub(super) fn ready_payload<P, T>(
    target: T,
    payload: P,
) -> ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
    (P, T): super::proof_integration::AllowedContributionBinding<P, T>,
{
    ready(
        super::proof_integration::create_requested_domain_capability_contribution(target, payload),
    )
}

pub(super) fn success<T>(outcome: ForgeQueryDomainCapabilityTransitionOutcome<T>) -> T {
    match outcome {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => {
            panic!("expected success, got denial {:?}", denial.kind())
        }
        TransitionOutcome::Stale(stale) => {
            panic!("expected success, got stale {}", stale.category())
        }
        TransitionOutcome::RebindRequired(rebind) => {
            panic!(
                "expected success, got rebind-required {}",
                rebind.category()
            )
        }
        TransitionOutcome::Failed(failure) => {
            panic!("expected success, got failure {}", failure.message())
        }
        TransitionOutcome::Deferred(never) => match never {},
    }
}

pub(super) fn declaration_target(
    target_digest: &str,
) -> ForgeQueryDeclarationBoundContributionTarget {
    ForgeQueryDeclarationBoundContributionTarget::from_digest(target_digest)
}

pub(super) fn admitted_plan_target(
    target_digest: &str,
) -> ForgeQueryAdmittedPlanBoundContributionTarget {
    ForgeQueryAdmittedPlanBoundContributionTarget::from_digest(target_digest)
}

pub(super) fn admitted_plan_target_parts(
    target_digest: &str,
    request_digest: &str,
    eligibility_digest: &str,
    decision_digest: &str,
) -> ForgeQueryAdmittedPlanBoundContributionTarget {
    ForgeQueryAdmittedPlanBoundContributionTarget::from_digest_parts(
        target_digest,
        request_digest,
        eligibility_digest,
        decision_digest,
    )
}

pub(super) fn lower_runtime_target(
    target_digest: &str,
) -> ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::from_digest(target_digest)
}
