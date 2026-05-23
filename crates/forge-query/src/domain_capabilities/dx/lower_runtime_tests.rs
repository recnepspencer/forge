use super::{forge_query_domain, ForgeQueryDomainCapabilityOutcomeKind};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    materialize_lower_runtime_support_traceability_artifact,
    materialize_query_causal_inspection_artifact, materialize_query_causal_inspection_review,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryExplanationContributionAuthoring, ForgeQueryLowerRuntimeExplanationRequest,
    ForgeQuerySupportContributionAuthoring,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    anchor_causal_observation, causal_inspection_target, resolve_causal_evidence_references,
    CausalEvidenceFamily, CausalEvidenceReferenceResolution, CausalInspectionMaterializationPolicy,
    CausalInspectionReason, CausalInspectionRedactionPolicy, ForgeQueryReadExecutionEngine,
    ForgeQueryReadReceipt,
};

#[test]
fn common_lower_runtime_support_lane_matches_proof_lane_materialization() {
    let envelope = lower_runtime_envelope("boundary-support");

    let common = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&envelope)
        .supports_boundary_traceability("routing.signal_invalidation")
        .because("lower-runtime routing narrows the supported authority seam")
        .materialize()
        .expect("lower-runtime support lane should materialize");

    let proof_requested = ForgeQuerySupportContributionAuthoring::narrowed_support(
        "worth.spatial.routing.signal_invalidation",
        "lower-runtime routing narrows the supported authority seam",
    )
    .for_lower_runtime_boundary_envelope(&envelope);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_lower_runtime_support_traceability_artifact(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn common_lower_runtime_explanation_review_matches_proof_lane_review() {
    let envelope = lower_runtime_envelope("boundary-explanation-review");
    let (reference_set, inspection_target) = replay_gap_inputs();

    let common = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&envelope)
        .explains_store_backed_replay_gap(
            "replay.store_gap",
            ForgeQueryLowerRuntimeExplanationRequest::explains_store_backed_replay_gap(
                reference_set.clone(),
                inspection_target.clone(),
                vec![CausalEvidenceFamily::QueryInspection],
                CausalInspectionRedactionPolicy::PreserveDetail,
                CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
            ),
        )
        .because("store-backed replay still lacks the retained lower-runtime evidence lane")
        .review()
        .expect("lower-runtime explanation review should materialize");

    let proof_requested =
        ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
            "worth.spatial.replay.store_gap",
            "store-backed replay still lacks the retained lower-runtime evidence lane",
            reference_set,
            inspection_target,
            vec![CausalEvidenceFamily::QueryInspection],
            CausalInspectionRedactionPolicy::PreserveDetail,
            CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
        )
        .for_lower_runtime_boundary_envelope(&envelope);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_query_causal_inspection_review(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn common_lower_runtime_explanation_artifact_matches_proof_lane_materialization() {
    let envelope = lower_runtime_envelope("boundary-explanation-artifact");
    let (reference_set, inspection_target) = replay_gap_inputs();

    let common = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&envelope)
        .explains_store_backed_replay_gap(
            "replay.store_gap",
            ForgeQueryLowerRuntimeExplanationRequest::explains_store_backed_replay_gap(
                reference_set.clone(),
                inspection_target.clone(),
                vec![CausalEvidenceFamily::QueryInspection],
                CausalInspectionRedactionPolicy::PreserveDetail,
                CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
            ),
        )
        .because("store-backed replay still lacks the retained lower-runtime evidence lane")
        .materialize_artifact()
        .expect("lower-runtime explanation artifact should materialize");

    let proof_requested =
        ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
            "worth.spatial.replay.store_gap",
            "store-backed replay still lacks the retained lower-runtime evidence lane",
            reference_set,
            inspection_target,
            vec![CausalEvidenceFamily::QueryInspection],
            CausalInspectionRedactionPolicy::PreserveDetail,
            CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
        )
        .for_lower_runtime_boundary_envelope(&envelope);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_query_causal_inspection_artifact(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn checked_lower_runtime_review_lane_preserves_denied_metadata() {
    let envelope = lower_runtime_envelope("boundary-explanation-denial");
    let (reference_set, inspection_target) = replay_gap_inputs();

    let checked = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&envelope)
        .explains_store_backed_replay_gap(
            "replay.store_gap",
            ForgeQueryLowerRuntimeExplanationRequest::explains_store_backed_replay_gap(
                reference_set,
                inspection_target,
                vec![CausalEvidenceFamily::QueryInspection],
                CausalInspectionRedactionPolicy::PreserveDetail,
                CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
            ),
        )
        .because("")
        .try_review();

    assert_eq!(
        checked.kind(),
        ForgeQueryDomainCapabilityOutcomeKind::Denied
    );
    assert_eq!(checked.category(), "explanation-inspection");
    assert_eq!(checked.semantic_posture(), "explains-ambiguity");
    assert_eq!(
        checked.target_kind(),
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::LowerRuntimeBoundaryEnvelope
    );
    assert!(checked.denial().is_some());
}

fn success<T>(
    outcome: crate::domain_capabilities::ForgeQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => panic!("expected success"),
    }
}

fn lower_runtime_envelope(target_digest: &str) -> ForgeQueryLowerRuntimeBoundaryEnvelope {
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

fn replay_gap_inputs() -> (
    crate::runtime::CausalEvidenceReferenceSet,
    crate::runtime::CausalInspectionTarget,
) {
    let read_receipt = ForgeQueryReadReceipt::test_only(
        "read-graph:domain-capability",
        "query:domain-capability",
        "basis:domain-capability",
        "result:domain-capability",
        ForgeQueryReadExecutionEngine::QueryRuntimeHistorical,
    );
    let observation = crate::runtime::QueryObservationReceipt::from_read_receipt(&read_receipt);
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
        observation.observation_target_digest(),
        observation.result_shape_context_digest(),
    )
    .expect("observation-derived target should be valid");

    (reference_set, target)
}
