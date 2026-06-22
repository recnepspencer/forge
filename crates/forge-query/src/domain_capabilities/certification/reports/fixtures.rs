use forge_foundational::facade::{AspectKey, FieldKey};
use forge_proof::TransitionOutcome;
use forge_relational::facade::runtime::InvariantCatalog;
use forge_runtime_bridge::facade::BridgePreviewSessionIdentity;

use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryLowerRuntimeExplanationRequest, ForgeQueryProjectionContractRequest,
    ForgeQueryRequestedAdmissionContribution, ForgeQueryRequestedAftermathContribution,
    ForgeQueryRequestedContinuityContribution, ForgeQueryRequestedDomainCapabilityContribution,
    ForgeQueryRequestedExplanationContribution, ForgeQueryRequestedInvariantCapabilityContribution,
    ForgeQueryRequestedSupportContribution, ForgeQueryRequestedWorkflowContribution,
    ForgeQuerySupportContributionAuthoring,
};
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
};
use crate::runtime::{
    anchor_causal_observation, causal_inspection_target, resolve_causal_evidence_references,
    CausalEvidenceFamily, CausalEvidenceReferenceResolution, CausalEvidenceReferenceSet,
    CausalInspectionMaterializationPolicy, CausalInspectionReason, CausalInspectionRedactionPolicy,
    CausalInspectionTarget, ForgeQueryAdmittedIntentPlan,
    ForgeQueryGraphCompositionCapabilityClass, ForgeQueryIntentDeclaration, ForgeQueryIntentInput,
    QueryObservationReceipt,
};

pub(super) fn intent_declaration(name: &str) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        name,
        "spatial.commit",
        "1",
        "geometry.patch",
        ForgeQueryIntentInput::object([("edge", ForgeQueryIntentInput::string("e-1"))]),
    )
}

pub(super) fn admitted_basis_observation_plan() -> ForgeQueryAdmittedIntentPlan {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(
            crate::basis_lifecycle::RawBasisIntent::CurrentHead,
        )
        .expect("basis-observation request should build");

    match crate::intent_admission::admit_runtime_intent_request(request) {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan,
        other => panic!("expected admitted basis-observation plan, got {other:?}"),
    }
}

pub(super) fn admitted_projection_consumption_plan() -> ForgeQueryAdmittedIntentPlan {
    let declaration = crate::projection_consumption::declare_projection_consumption(
        admitted_projection_source(),
        admitted_projection_binding(),
        ProjectMaterializedFacts::declare().display_field_path(
            crate::projection_consumption::projection_fact_field_path_from_segments([
                "field", "visible",
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

pub(super) fn projection_contract_request() -> ForgeQueryProjectionContractRequest {
    ForgeQueryProjectionContractRequest::new(
        admitted_projection_source(),
        admitted_projection_binding(),
        ProjectMaterializedFacts::declare().display_field_path(
            crate::projection_consumption::projection_fact_field_path_from_segments([
                "field", "visible",
            ]),
        ),
    )
}

pub(super) fn projection_contract_parts() -> (
    ProjectionConsumptionSource,
    ProjectionConsumptionBindingContext,
    ProjectMaterializedFacts,
) {
    projection_contract_request().into_parts()
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
            "certification-report-target",
        )
        .field_value(ForgeQueryEvidenceTag::new("test_target"), target_digest)
        .seal(),
    );
    let detail_identity =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(ForgeQueryEvidenceTag::new("test_detail"), "detail")
            .seal();
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request,
        &detail_identity,
    );
    let route = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "certification-report-route",
            &detail_identity,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "domain-capability-certification-report",
            &ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_value(
                ForgeQueryEvidenceTag::new("certification_report_target"),
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

pub(super) fn store_backed_replay_gap_request() -> ForgeQueryLowerRuntimeExplanationRequest {
    let (reference_set, target) = replay_gap_inputs();
    ForgeQueryLowerRuntimeExplanationRequest::explains_store_backed_replay_gap(
        reference_set,
        target,
        vec![CausalEvidenceFamily::QueryInspection],
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
}

pub(super) fn store_backed_replay_gap_parts() -> (CausalEvidenceReferenceSet, CausalInspectionTarget)
{
    replay_gap_inputs()
}

fn replay_gap_inputs() -> (CausalEvidenceReferenceSet, CausalInspectionTarget) {
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
        panic!("query-inspection-only replay evidence should resolve");
    };
    let target = causal_inspection_target(
        observation.observation_target().clone(),
        observation.result_shape_context().clone(),
    )
    .expect("observation-derived target should be valid");

    (reference_set, target)
}

fn admitted_projection_source() -> ProjectionConsumptionSource {
    crate::projection_consumption::intent_admission_admitted_projection_declaration()
        .source()
        .clone()
}

fn admitted_projection_binding() -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::intent_admission_certification_binding(
        "shape-digest",
        "query-digest",
        "shape-digest",
        "query-read:certification-admitted",
        "narrowed-shape-digest",
        "policy-digest",
        "tenant-schema-digest",
        vec![field_path("field.visible")],
    )
}

fn field_path(path: &str) -> crate::authorized_projection::AuthorizedProjectionFieldPath {
    let Some((aspect, field)) = path.split_once('.') else {
        panic!("domain capability fixture field path should include an aspect and field");
    };
    crate::authorized_projection::AuthorizedProjectionFieldPath::from_native_keys(
        AspectKey::new(aspect.to_string()).expect("fixture aspect key"),
        FieldKey::new(field.to_string()).expect("fixture field key"),
    )
}

pub(super) fn support_traceability_requested(
    declaration: &ForgeQueryIntentDeclaration,
) -> ForgeQueryRequestedSupportContribution<
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
> {
    ForgeQuerySupportContributionAuthoring::declaration_traceability(
        "worth.spatial.traceability.edge_split",
        "declaration-scoped support remains declaration scoped",
    )
    .for_intent_declaration(declaration)
}

pub(super) fn plain_support_requested(
    declaration: &ForgeQueryIntentDeclaration,
) -> ForgeQueryRequestedSupportContribution<
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
> {
    ForgeQuerySupportContributionAuthoring::declaration_support(
        "worth.spatial.traceability.edge_split",
        "declaration-scoped support remains declaration scoped",
    )
    .for_intent_declaration(declaration)
}

pub(super) fn plan_support_requested(
    plan: &ForgeQueryAdmittedIntentPlan,
) -> ForgeQueryRequestedSupportContribution<
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ForgeQuerySupportContributionAuthoring::declaration_support(
        "worth.spatial.support.runtime_floor",
        "runtime floor remains explicitly supported",
    )
    .for_admitted_intent_plan(plan)
}

pub(super) fn admission_requested(
    plan: &ForgeQueryAdmittedIntentPlan,
) -> ForgeQueryRequestedAdmissionContribution<
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    crate::domain_capabilities::ForgeQueryAdmissionContributionAuthoring::advisory(
        "worth.spatial.admission.routing_gap",
        "runtime routing still needs clarification",
    )
    .for_admitted_intent_plan(plan)
}

pub(super) fn workflow_requested(
    declaration: &ForgeQueryIntentDeclaration,
) -> ForgeQueryRequestedWorkflowContribution<
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
> {
    crate::domain_capabilities::ForgeQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(
        "worth.spatial.workflow.preview_mutation",
        "preview mutation planning should preserve canonical workflow semantics",
        BridgePreviewSessionIdentity::from_stable_name("preview-session:certification"),
    )
    .for_intent_declaration(declaration)
}

pub(super) fn continuity_requested(
    plan: &ForgeQueryAdmittedIntentPlan,
) -> ForgeQueryRequestedContinuityContribution<
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    crate::domain_capabilities::ForgeQueryContinuityContributionAuthoring::preserved_rebind(
        "edge:before",
        "edge:after",
        "worth.spatial.continuity.edge_split",
        "edge split preserves one authoritative successor",
    )
    .for_admitted_intent_plan(plan)
}

pub(super) fn aftermath_requested(
    plan: &ForgeQueryAdmittedIntentPlan,
) -> ForgeQueryRequestedAftermathContribution<
    crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    let (source, binding, facts) = projection_contract_parts();
    crate::domain_capabilities::ForgeQueryAftermathContributionAuthoring::consumes_projection_contract(
        "worth.spatial.aftermath.projection_contract",
        "projection aftermath should bind a stable contract",
        source,
        binding,
        facts,
    )
    .for_admitted_intent_plan(plan)
}

pub(super) fn explanation_requested(
    lower_runtime: &ForgeQueryLowerRuntimeBoundaryEnvelope,
) -> ForgeQueryRequestedExplanationContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    let (reference_set, target) = store_backed_replay_gap_parts();
    crate::domain_capabilities::ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
        "worth.spatial.explanation.store_backed_replay",
        "store-backed replay should preserve denied explanation identity",
        reference_set,
        target,
        vec![CausalEvidenceFamily::QueryInspection],
        crate::runtime::CausalInspectionRedactionPolicy::PreserveDetail,
        crate::runtime::CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .for_lower_runtime_boundary_envelope(lower_runtime)
}

pub(super) fn invariant_requested(
    declaration: &ForgeQueryIntentDeclaration,
) -> ForgeQueryRequestedInvariantCapabilityContribution<
    crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget,
> {
    crate::domain_capabilities::ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
        InvariantCatalog::default(),
        "worth.spatial.invariant.edge_split",
        "geometry kernel must reject invalid edge splits",
    )
    .for_intent_declaration(declaration)
}

pub(super) fn capability_requested(
    lower_runtime: &ForgeQueryLowerRuntimeBoundaryEnvelope,
) -> ForgeQueryRequestedInvariantCapabilityContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    crate::domain_capabilities::ForgeQueryInvariantCapabilityContributionAuthoring::graph_capability_gap(
        "face-split-target-combination",
        ForgeQueryGraphCompositionCapabilityClass::TargetCombination,
        "worth.spatial.capability.face_split",
        "face split still depends on graph-composition capability support",
    )
    .for_lower_runtime_boundary_envelope(lower_runtime)
}

pub(super) fn invariant_denial_requested(
    lower_runtime: &ForgeQueryLowerRuntimeBoundaryEnvelope,
) -> ForgeQueryRequestedInvariantCapabilityContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    crate::domain_capabilities::ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
        "non_manifold_edge_split",
        vec!["edges".to_string()],
        vec!["edge_symbol".to_string()],
        vec!["face-edge".to_string()],
        vec!["split-existing-target".to_string()],
        "program-digest:domain-capability",
        "breadth-digest:domain-capability",
        "counter-snapshot:domain-capability",
        "worth.spatial.invariant.non_manifold",
        "non-manifold edge split must deny graph composition",
    )
    .for_lower_runtime_boundary_envelope(lower_runtime)
}

pub(super) fn admitted_ready<P, T>(
    requested: ForgeQueryRequestedDomainCapabilityContribution<P, T>,
) -> crate::domain_capabilities::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: crate::domain_capabilities::payloads::ForgeQueryDomainCapabilityPayload,
    T: crate::domain_capabilities::targets::ForgeQueryDomainCapabilityTargetBinding,
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
