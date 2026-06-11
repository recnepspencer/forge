use forge_proof::TransitionOutcome;

use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedDomainCapabilityContribution;
use crate::domain_capabilities::targets::ForgeQueryDomainCapabilityTargetBinding;
use crate::domain_capabilities::ForgeQueryDomainCapabilityPayload;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    admit_runtime_intent_request, anchor_causal_observation, causal_inspection_target,
    resolve_causal_evidence_references, CausalEvidenceFamily, CausalEvidenceReferenceResolution,
    CausalInspectionReason, ForgeQueryAdmittedIntentPlan, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentDeclaration, QueryObservationReceipt,
};

pub(super) fn intent_declaration() -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        "domain-capability-certification",
        "spatial.commit",
        "1",
        "geometry.patch",
        serde_json::json!({"edge":"e-1"}),
    )
}

pub(super) fn admitted_basis_observation_plan() -> ForgeQueryAdmittedIntentPlan {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(
            crate::basis_lifecycle::RawBasisIntent::CurrentHead,
        )
        .expect("basis observation request should build");
    let ForgeQueryIntentAdmissionDecision::Admitted(plan) = admit_runtime_intent_request(request)
    else {
        panic!("basis observation lane should admit");
    };
    plan
}

pub(super) fn admitted_projection_consumption_plan() -> ForgeQueryAdmittedIntentPlan {
    let declaration = crate::projection_consumption::declare_projection_consumption(
        projection_source(),
        projection_binding(),
        crate::projection_consumption::ProjectMaterializedFacts::declare()
            .display_field("field.visible"),
    )
    .expect("projection declaration should build");
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::projection_consumption(
            declaration,
        )
        .expect("projection request should build");

    match crate::intent_admission::admit_runtime_intent_request(request) {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan,
        other => panic!("expected admitted projection-consumption plan, got {other:?}"),
    }
}

pub(super) fn lower_runtime_envelope(
    target_digest: &str,
) -> ForgeQueryLowerRuntimeBoundaryEnvelope {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "signal-invalidation-routing",
        target_digest,
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request, "detail");
    let route = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, target_digest);
    let boundary = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route,
        format!("retained:{target_digest}"),
    );

    ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        &route,
        &boundary,
        &format!("retained:{target_digest}"),
    )
}

pub(super) fn replay_gap_inputs() -> (
    crate::runtime::CausalEvidenceReferenceSet,
    crate::runtime::CausalInspectionTarget,
) {
    let observation =
        QueryObservationReceipt::certification_historical_replay_fixture("domain-capability");
    let anchor = anchor_causal_observation(
        observation.clone(),
        CausalInspectionReason::HistoricalReplayResult,
    )
    .expect("historical replay observation should anchor");
    let CausalEvidenceReferenceResolution::Resolved { reference_set, .. } =
        resolve_causal_evidence_references(anchor, &[CausalEvidenceFamily::QueryInspection])
    else {
        panic!("query-inspection replay evidence should resolve");
    };
    let target = causal_inspection_target(
        observation.observation_target().clone(),
        observation.result_shape_context().clone(),
    )
    .expect("observation-derived target should be valid");

    (reference_set, target)
}

pub(super) fn projection_source() -> crate::projection_consumption::ProjectionConsumptionSource {
    crate::projection_consumption::intent_admission_admitted_projection_declaration()
        .source()
        .clone()
}

pub(super) fn projection_binding(
) -> crate::projection_consumption::ProjectionConsumptionBindingContext {
    crate::projection_consumption::ProjectionConsumptionBindingContext::intent_admission_certification_binding(
        "shape-digest",
        "query-digest",
        "shape-digest",
        "query-read:certification-admitted",
        "narrowed-shape-digest",
        "policy-digest",
        "tenant-schema-digest",
        vec!["field.visible".to_string()],
    )
}

pub(super) fn admission_digest(
    decision: &crate::runtime::ForgeQueryIntentAdmissionDecision,
) -> &str {
    match decision {
        crate::runtime::ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan.decision_digest(),
        crate::runtime::ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            advisory.decision_digest()
        }
        crate::runtime::ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            violation.decision_digest()
        }
    }
}

pub(super) fn admitted_ready<P, T>(
    requested: ForgeQueryRequestedDomainCapabilityContribution<P, T>,
) -> crate::domain_capabilities::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let admitted = success(admit_eligible_domain_capability_contribution(eligible));
    let target = admitted.payload().target().clone();
    success(prepare_admitted_domain_capability_contribution_for_materialization(admitted, target))
}

pub(super) fn success<T>(
    outcome: crate::domain_capabilities::ForgeQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        TransitionOutcome::Success(value) => value,
        _ => panic!("expected success"),
    }
}
