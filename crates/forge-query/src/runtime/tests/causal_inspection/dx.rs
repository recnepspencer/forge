use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceFamily,
    BridgeCausalEvidenceReferenceIdentity, BridgeCausalInspectionAdmissionSummary,
    BridgeIdentityEvidence, TruthCommitIdentity,
};

use super::super::super::*;
use super::materialization::support::*;

#[test]
fn common_changed_observation_plans_and_materializes_admitted() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-causal-dx-changed",
        ))
        .unwrap();
    let plan = CausalInspection::for_observation(receipt_with_route(
        CausalObservationOutcome::Changed,
        routed.route_identity().evidence_identity(),
    ))
    .why_changed()
    .reference_only()
    .include_all_retained_evidence()
    .plan()
    .expect("changed observation should plan");

    assert_eq!(
        plan.support_posture(),
        CausalInspectionSupportPosture::Admitted
    );
    assert_eq!(plan.estimated_cost().bridge_envelope_assembly_count(), 1);

    let artifact = plan
        .materialize_with_bridge(&runtime)
        .expect("admitted plan should materialize");

    assert!(artifact.is_admitted());
    assert_eq!(
        artifact.primary_result(),
        CausalInspectionArtifactKind::Admitted
    );
    assert_eq!(artifact.evidence_reference_count(), 2);
    assert_eq!(
        artifact.authority_bindings().len(),
        artifact.evidence().len()
    );
    assert!(artifact
        .authority_bindings()
        .iter()
        .any(|binding| binding.family() == "query_observation"));
    assert!(artifact
        .authority_bindings()
        .iter()
        .any(|binding| binding.family() == "bridge_route"));
}

#[test]
fn support_discovery_names_supported_advisory_and_deferred_lanes() {
    let support = CausalInspection::support();
    let explanation = support.explain();

    assert_eq!(explanation.supported_row_count(), 1);
    assert_eq!(explanation.advisory_row_count(), 1);
    assert_eq!(explanation.deferred_row_count(), 2);
    assert!(support.rows().iter().any(|row| {
        row.explanation_family() == CausalInspectionExplanationFamily::CrossRuntimeCausalExplanation
            && row.default_richness() == CausalInspectionRichness::ReferenceOnly
            && row.posture() == CausalInspectionSupportRowPosture::Supported
    }));
    assert!(support
        .rows()
        .iter()
        .any(|row| row.note().contains("later-milestone debt")));
}

#[test]
fn common_suppressed_observation_uses_reason_helper() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-causal-dx-suppressed",
        ))
        .unwrap();
    let plan = CausalInspection::for_observation(receipt_with_route(
        CausalObservationOutcome::Suppressed,
        routed.route_identity().evidence_identity(),
    ))
    .why_suppressed()
    .reference_only()
    .plan()
    .expect("suppressed observation should plan through common path");

    let artifact = plan
        .materialize_with_bridge(&runtime)
        .expect("suppressed plan should materialize");

    assert!(artifact.is_admitted());
    assert!(artifact.denial_reason().is_none());
}

#[test]
fn temporal_async_reason_helpers_materialize_bridge_backed_explanations() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-causal-dx-temporal-async",
        ))
        .unwrap();
    let temporal_artifact = CausalInspection::for_observation(QueryObservationReceipt::fixture(
        CausalObservationOutcome::Changed,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "query-inspection:dx-temporal-wake",
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                routed.route_identity().evidence_identity(),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::SignalInvalidation,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "signal-invalidation:dx-temporal-wake",
                ),
            ),
        ],
    ))
    .why_temporal_wake()
    .reference_only()
    .plan()
    .expect("temporal wake helper should plan")
    .materialize_with_bridge(&runtime)
    .expect("temporal wake helper should materialize");
    let async_artifact = CausalInspection::for_observation(QueryObservationReceipt::fixture(
        CausalObservationOutcome::Changed,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "query-inspection:dx-async-completion",
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                routed.route_identity().evidence_identity(),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::SignalEvaluation,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "signal-evaluation:dx-async-completion",
                ),
            ),
        ],
    ))
    .why_async_completion()
    .reference_only()
    .plan()
    .expect("async completion helper should plan")
    .materialize_with_bridge(&runtime)
    .expect("async completion helper should materialize");

    assert_eq!(
        temporal_artifact.temporal_async_explanation().kind(),
        QueryCausalTemporalAsyncExplanationKind::TemporalWake
    );
    assert_eq!(
        async_artifact.temporal_async_explanation().kind(),
        QueryCausalTemporalAsyncExplanationKind::AsyncCompletion
    );
}

#[test]
fn materialized_detail_common_path_is_advisory_before_bridge_materialization() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-causal-dx-advisory",
        ))
        .unwrap();
    let plan = CausalInspection::for_observation(receipt_with_route(
        CausalObservationOutcome::Changed,
        routed.route_identity().evidence_identity(),
    ))
    .why_changed()
    .materialized_detail()
    .plan()
    .expect("materialized detail should plan as advisory");

    assert_eq!(
        plan.support_posture(),
        CausalInspectionSupportPosture::Advisory
    );
    assert_eq!(
        plan.explain().reason(),
        "materialized_detail_deferred_until_bridge_envelope"
    );

    let artifact = plan
        .materialize_with_bridge(&runtime)
        .expect("advisory plan should materialize with advisory bridge summary");

    assert!(artifact.is_advisory());
    assert_eq!(
        artifact.advisory_reason(),
        Some("materialized_detail_deferred_until_bridge_envelope")
    );
    assert_eq!(artifact.warnings().len(), 1);
}

#[test]
fn unsupported_durable_family_denies_without_bridge_assembly() {
    let runtime = bridge_runtime();
    let plan = CausalInspection::for_observation(QueryObservationReceipt::fixture(
        CausalObservationOutcome::Changed,
        vec![CausalObservationEvidenceIdentity::new(
            CausalEvidenceFamily::QueryInspection,
            crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                "query-inspection:durable-denied",
            ),
        )],
    ))
    .why_changed()
    .durable_archive()
    .plan()
    .expect("durable causal archive should reach Query denial");

    assert_eq!(
        plan.support_posture(),
        CausalInspectionSupportPosture::Denied
    );
    assert_eq!(plan.estimated_cost().bridge_envelope_assembly_count(), 0);
    assert_eq!(plan.estimated_cost().anchor_derivation_count(), 1);
    assert_eq!(
        plan.estimated_cost().evidence_reference_resolution_count(),
        1
    );
    assert_eq!(plan.estimated_cost().admission_count(), 1);
    assert_eq!(
        plan.redaction_policy(),
        CausalInspectionRedactionPolicy::PreserveDetail
    );
    assert_eq!(
        plan.materialization_policy(),
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact
    );
    assert_eq!(
        plan.explanation_family(),
        CausalInspectionExplanationFamily::DurableCausalArchive
    );
    assert_eq!(
        plan.requested_richness(),
        CausalInspectionRichness::ReferenceOnly
    );

    let artifact = plan
        .materialize_with_bridge(&runtime)
        .expect("Query denial materializes without bridge assembly");

    assert!(artifact.is_denied());
    assert_eq!(
        artifact.denial_reason(),
        Some("unsupported_explanation_family")
    );
    assert_eq!(artifact.performance().bridge_envelope_assembly_count(), 0);
    assert!(artifact.bridge_envelope_digest().is_none());
}

#[test]
fn bridge_envelope_denial_materializes_denied_artifact_with_bridge_fields() {
    let runtime = bridge_runtime();
    let plan = CausalInspection::for_observation(receipt_with_route(
        CausalObservationOutcome::Changed,
        BridgeIdentityEvidence::from_external_authority("route:causal-dx-missing"),
    ))
    .why_changed()
    .reference_only()
    .plan()
    .expect("missing retained route still admits at Query boundary");

    let artifact = plan
        .materialize_with_bridge(&runtime)
        .expect("bridge denial should become a denied Query artifact");

    assert!(artifact.is_denied());
    assert_eq!(artifact.denial_reason(), Some("bridge_envelope_denial"));
    assert!(artifact.decision_trace().bridge_denial_digest().is_some());
    assert_eq!(artifact.performance().bridge_envelope_assembly_count(), 1);
}

#[test]
fn common_path_preserves_core_digests_from_explicit_pipeline() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-causal-dx-parity",
        ))
        .unwrap();
    let receipt = receipt_with_route(
        CausalObservationOutcome::Changed,
        routed.route_identity().evidence_identity(),
    );
    let plan = CausalInspection::for_observation(receipt.clone())
        .why_changed()
        .reference_only()
        .evidence_families([
            CausalEvidenceFamily::QueryInspection,
            CausalEvidenceFamily::BridgeRoute,
        ])
        .plan()
        .expect("common path should plan");

    let anchor = anchor_causal_observation(receipt, CausalInspectionReason::ChangedResult)
        .expect("explicit anchor should work");
    let CausalEvidenceReferenceResolution::Resolved { reference_set, .. } =
        resolve_causal_evidence_references(
            anchor,
            &[
                CausalEvidenceFamily::QueryInspection,
                CausalEvidenceFamily::BridgeRoute,
            ],
        )
    else {
        panic!("explicit references should resolve");
    };
    let explicit_request = request_for_families(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
        &[
            CausalEvidenceFamily::QueryInspection,
            CausalEvidenceFamily::BridgeRoute,
        ],
    );
    let explicit_flow = admit_causal_inspection(explicit_request);
    let CausalInspectionProofFlow::Admitted(explicit_admitted) = explicit_flow else {
        panic!("explicit flow should admit");
    };

    assert_eq!(
        plan.anchor_digest(),
        explicit_admitted.subject().anchor_digest()
    );
    assert_eq!(
        plan.reference_set_digest(),
        explicit_admitted.subject().reference_set_digest()
    );
    assert_eq!(
        plan.request_digest(),
        explicit_admitted.subject().request_digest()
    );
    assert_eq!(
        plan.admission_digest(),
        explicit_admitted.admitted_inspection_digest()
    );

    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            explicit_admitted.admitted_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            explicit_admitted.subject().anchor_digest(),
        ),
    )
    .expect("summary should be valid");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        explicit_admitted
                            .subject()
                            .query_observation_bridge_evidence_identity(),
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().evidence_identity(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid");
    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("explicit bridge envelope should assemble");
    let explicit_artifact = materialize_admitted_causal_inspection(
        &explicit_admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("explicit materialization should work");
    let common_artifact = plan
        .materialize_with_bridge(&runtime)
        .expect("common materialization should work");

    assert_eq!(
        common_artifact.bridge_envelope_digest(),
        explicit_artifact.bridge_envelope_digest()
    );
    assert_eq!(
        common_artifact.artifact_digest(),
        explicit_artifact.artifact_digest()
    );
    assert_eq!(
        common_artifact.receipt().receipt_digest(),
        explicit_artifact.receipt().receipt_digest()
    );
}

fn receipt_with_route(
    outcome: CausalObservationOutcome,
    route_identity: BridgeIdentityEvidence,
) -> QueryObservationReceipt {
    QueryObservationReceipt::fixture(
        outcome,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(format!(
                    "query-inspection:{}",
                    outcome.as_str()
                )),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                route_identity,
            ),
        ],
    )
}
