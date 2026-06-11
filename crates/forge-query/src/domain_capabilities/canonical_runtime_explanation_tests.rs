use forge_proof::TransitionOutcome;

use super::targets::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget;
use super::test_support::{lower_runtime_target, ready, success};
use super::{
    materialize_query_causal_inspection_artifact, materialize_query_causal_inspection_review,
    ForgeQueryExplanationContributionAuthoring,
};
use crate::runtime::{
    anchor_causal_observation, causal_inspection_target, resolve_causal_evidence_references,
    CausalEvidenceFamily, CausalEvidenceReferenceResolution, CausalInspectionReason,
    CausalInspectionSupportPosture, ForgeQueryReadExecutionEngine, ForgeQueryReadReceipt,
};

#[test]
fn explanation_runtime_materializer_builds_denied_causal_artifact() {
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

    let artifact = success(materialize_query_causal_inspection_artifact(
        ready_explanation(
            ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
                "explanation.store_backed_replay",
                "store-backed replay should deny without the required lower-runtime evidence",
                reference_set,
                target,
                vec![CausalEvidenceFamily::QueryInspection],
                crate::runtime::CausalInspectionRedactionPolicy::PreserveDetail,
                crate::runtime::CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
            ),
        ),
    ));

    assert_eq!(
        artifact.kind(),
        crate::runtime::CausalInspectionArtifactKind::Denied
    );
    assert!(artifact.is_denied());
    assert!(artifact.bridge_envelope_digest().is_none());
    assert!(!artifact.artifact_digest().is_empty());
}

#[test]
fn explanation_runtime_materializer_denies_missing_semantics() {
    let outcome = materialize_query_causal_inspection_artifact(ready_explanation(
        ForgeQueryExplanationContributionAuthoring::requires_context(
            "explanation.support.only",
            "support-only explanation cannot mint a causal inspection artifact",
        ),
    ));

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
}

#[test]
fn explanation_runtime_review_preserves_denied_plan_identity() {
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
    let review_contribution = ready_explanation(
        ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
            "explanation.store_backed_replay",
            "store-backed replay should deny without the required lower-runtime evidence",
            reference_set.clone(),
            target.clone(),
            vec![CausalEvidenceFamily::QueryInspection],
            crate::runtime::CausalInspectionRedactionPolicy::PreserveDetail,
            crate::runtime::CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
        ),
    );
    let artifact_contribution = ready_explanation(
        ForgeQueryExplanationContributionAuthoring::store_backed_replay_gap_explanation(
            "explanation.store_backed_replay",
            "store-backed replay should deny without the required lower-runtime evidence",
            reference_set,
            target,
            vec![CausalEvidenceFamily::QueryInspection],
            crate::runtime::CausalInspectionRedactionPolicy::PreserveDetail,
            crate::runtime::CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
        ),
    );

    let review = success(materialize_query_causal_inspection_review(
        review_contribution,
    ));
    let artifact = success(materialize_query_causal_inspection_artifact(
        artifact_contribution,
    ));

    assert_eq!(
        review.plan().support_posture(),
        CausalInspectionSupportPosture::Denied
    );
    assert_eq!(review.semantic_code(), "explanation.store_backed_replay");
    match artifact {
        crate::runtime::QueryCausalInspectionArtifact::Denied(denied) => {
            assert_eq!(
                denied.query_denial_digest(),
                review.plan().admission_digest()
            );
        }
        other => panic!("expected denied causal inspection artifact, got {other:?}"),
    }
}

fn ready_explanation(
    authoring: ForgeQueryExplanationContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyExplanationContribution<
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    ready(
        authoring
            .bind_to_lower_runtime_boundary_target(lower_runtime_target("boundary-explanation")),
    )
}
