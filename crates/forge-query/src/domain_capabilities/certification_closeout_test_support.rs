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
        crate::runtime::ForgeQueryIntentInput::object([(
            "edge",
            crate::runtime::ForgeQueryIntentInput::string("e-1"),
        )]),
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
        crate::projection_consumption::ProjectMaterializedFacts::declare().display_field_path(
            crate::projection_consumption::projection_fact_field_path_from_segments([
                forge_foundational::facade::FieldKey::new("field")
                    .expect("projection fact field segment should admit"),
                forge_foundational::facade::FieldKey::new("visible")
                    .expect("projection fact field segment should admit"),
            ]),
        ),
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
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "domain-capabilities-closeout-target",
        )
        .field_value(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("test_target"),
            target_digest,
        )
        .seal(),
    );
    let detail_identity = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_value(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("test_detail"),
        "detail",
    )
    .seal();
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request,
        &detail_identity,
    );
    let route = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "domain-capabilities-closeout-route",
            &detail_identity,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "domain-capabilities-closeout-test",
            &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_value(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained_target"),
                target_digest,
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
        crate::projection_consumption::test_authorized_field_paths(&["field.visible"]),
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
