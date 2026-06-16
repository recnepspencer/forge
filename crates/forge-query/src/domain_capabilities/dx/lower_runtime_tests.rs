use super::{forge_query_domain, ForgeQueryDomainCapabilityOutcomeKind};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    materialize_graph_composition_domain_invariant_denial,
    materialize_lower_runtime_support_traceability_artifact,
    materialize_query_causal_inspection_artifact, materialize_query_causal_inspection_review,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryExplanationContributionAuthoring, ForgeQueryInvariantCapabilityContributionAuthoring,
    ForgeQueryLowerRuntimeExplanationRequest, ForgeQuerySupportContributionAuthoring,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeRouteSubjectIdentity,
    ForgeQueryLowerRuntimeSeamKey,
};
use crate::runtime::{
    anchor_causal_observation, causal_inspection_target, resolve_causal_evidence_references,
    CausalEvidenceFamily, CausalEvidenceReferenceResolution, CausalInspectionMaterializationPolicy,
    CausalInspectionReason, CausalInspectionRedactionPolicy,
    ForgeQueryGraphCompositionDomainInvariantDenial, ForgeQueryReadExecutionEngine,
    ForgeQueryReadReceipt, ForgeQueryWriteCommand,
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

#[test]
fn lower_runtime_source_support_binding_matches_envelope_binding() {
    let source = write_authority_boundary_source("boundary-source-support");
    let source_artifact = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_source(&source)
        .supports_boundary_traceability("routing.write_authority")
        .because("write authority receipt carries the lower-runtime boundary envelope")
        .materialize()
        .expect("source-bound support should materialize");
    let envelope_artifact = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(source.boundary_envelope())
        .supports_boundary_traceability("routing.write_authority")
        .because("write authority receipt carries the lower-runtime boundary envelope")
        .materialize()
        .expect("envelope-bound support should materialize");

    assert_eq!(source_artifact, envelope_artifact);
}

#[test]
fn lower_runtime_source_invariant_denial_matches_envelope_binding() {
    let source = write_authority_boundary_source("boundary-source-invariant");
    let source_denial = materialize_invariant_denial_from_source(&source);
    let envelope_denial = materialize_invariant_denial_from_envelope(source.boundary_envelope());

    assert_eq!(source_denial, envelope_denial);
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
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "domain-capabilities-dx-target",
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
        ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "domain-capabilities-dx-route",
            &detail_identity,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "domain-capabilities-dx-test",
            &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_value(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                format!("retained:{target_digest}"),
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

fn write_authority_boundary_source(
    target_digest: &str,
) -> crate::runtime::WriteAuthorityExecutionReceipt {
    use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

    let command = ForgeQueryWriteCommand::Delete {
        entity_identity: crate::memory_workspace::admit_authored_entity_label(target_digest),
    };
    let mutation_receipt =
        crate::memory_workspace::ForgeQueryMutationReceipt::from_authoritative_parts(
            crate::memory_workspace::ForgeQueryCommitIdentity::preview(
                crate::ForgeQueryEvidenceIdentity::compose(
                    crate::ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
                )
                .field_value(crate::ForgeQueryEvidenceTag::new("target"), target_digest)
                .seal(),
            ),
            crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(1, 1),
            ),
            Vec::new(),
        );
    crate::runtime::WriteAuthorityExecutionReceipt::from_command(&command, mutation_receipt)
}

fn materialize_invariant_denial_from_source<S>(
    source: &S,
) -> ForgeQueryGraphCompositionDomainInvariantDenial
where
    S: crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
{
    let requested = graph_invariant_denial_authoring().for_lower_runtime_boundary_source(source);
    materialize_invariant_denial(requested)
}

fn materialize_invariant_denial_from_envelope(
    envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
) -> ForgeQueryGraphCompositionDomainInvariantDenial {
    let requested =
        graph_invariant_denial_authoring().for_lower_runtime_boundary_envelope(envelope);
    materialize_invariant_denial(requested)
}

fn graph_invariant_denial_authoring() -> ForgeQueryInvariantCapabilityContributionAuthoring {
    ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
        "graph.non_manifold_edge_split",
        ["edges"],
        ["edge-1"],
        ["edge_split"],
        ["publication"],
        "program-digest",
        "breadth-digest",
        "counter-snapshot",
        "graph.non_manifold_edge_split",
        "edge split would violate graph composition invariant",
    )
}

fn materialize_invariant_denial(
    requested: crate::domain_capabilities::ForgeQueryRequestedInvariantCapabilityContribution<
        crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryGraphCompositionDomainInvariantDenial {
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let admitted = success(admit_eligible_domain_capability_contribution(eligible));
    let target = admitted.payload().target().clone();
    success(materialize_graph_composition_domain_invariant_denial(
        success(
            prepare_admitted_domain_capability_contribution_for_materialization(admitted, target),
        ),
    ))
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
        observation.observation_target().clone(),
        observation.result_shape_context().clone(),
    )
    .expect("observation-derived target should be valid");

    (reference_set, target)
}
