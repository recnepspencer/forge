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
use crate::domain_capabilities::identity::{domain_capability_scope_encoder, seal};
use crate::evidence_identity::ForgeQueryEvidenceTag;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    admit_runtime_intent_request, ForgeQueryAdmittedIntentPlan, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentDeclaration,
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

pub(super) fn intent_declaration(label: &str) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        format!("domain-capability.{label}"),
        format!("forge.domain_capability.{label}"),
        "1",
        "forge.domain-capability.fixture",
        crate::runtime::ForgeQueryIntentInput::object([(
            "fixture",
            crate::runtime::ForgeQueryIntentInput::string(label),
        )]),
    )
}

pub(super) fn admitted_plan(label: &str) -> ForgeQueryAdmittedIntentPlan {
    let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
        intent_declaration(label),
    )
    .expect("domain-capability fixture intent request should build");
    match admit_runtime_intent_request(request) {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan,
        other => panic!("expected admitted domain-capability fixture plan, got {other:?}"),
    }
}

pub(super) fn admitted_plan_target_parts(
    plan_label: &str,
    request_digest: &str,
    eligibility_digest: &str,
    decision_digest: &str,
) -> ForgeQueryAdmittedPlanBoundContributionTarget {
    let fixture_label = seal(
        domain_capability_scope_encoder("forge_query_domain_capability_admitted_plan_fixture_v1")
            .field_shape(ForgeQueryEvidenceTag::new("plan_label"), plan_label)
            .field_shape(ForgeQueryEvidenceTag::new("request"), request_digest)
            .field_shape(
                ForgeQueryEvidenceTag::new("eligibility"),
                eligibility_digest,
            )
            .field_shape(ForgeQueryEvidenceTag::new("decision"), decision_digest),
    );
    ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(&admitted_plan(
        &fixture_label,
    ))
}

pub(super) fn declaration_target(label: &str) -> ForgeQueryDeclarationBoundContributionTarget {
    ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(&intent_declaration(label))
}

pub(super) fn admitted_plan_target(label: &str) -> ForgeQueryAdmittedPlanBoundContributionTarget {
    ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(&admitted_plan(label))
}

pub(super) fn lower_runtime_envelope(label: &str) -> ForgeQueryLowerRuntimeBoundaryEnvelope {
    let subject_identity =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "domain-capability-fixture-subject",
        )
        .field_value(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture"),
            label,
        )
        .seal();
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "signal-invalidation-routing",
        subject_identity,
    );
    let detail_identity = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_value(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture_detail"),
        label,
    )
    .seal();
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request,
        &detail_identity,
    );
    let route = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "domain-capability-fixture-route",
            &detail_identity,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "domain-capability-fixture",
            &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_value(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture_retained"),
                label,
            )
            .seal(),
        );
    let boundary =
        ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(&route, &retained_evidence);

    ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        &route,
        &boundary,
        &retained_evidence,
    )
}

pub(super) fn lower_runtime_target(
    label: &str,
) -> ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
        &lower_runtime_envelope(label),
    )
}
