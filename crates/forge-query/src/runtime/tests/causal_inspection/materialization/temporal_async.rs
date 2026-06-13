use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner,
    BridgeCausalEvidenceReferenceIdentity, BridgeCausalInspectionAdmissionSummary,
    BridgeIdentityEvidence, TruthCommitIdentity,
};

use super::super::super::super::*;
use super::support::*;

fn reference_set_for(
    outcome: CausalObservationOutcome,
    reason: CausalInspectionReason,
    evidence_identities: Vec<CausalObservationEvidenceIdentity>,
    requested_families: &[CausalEvidenceFamily],
) -> CausalEvidenceReferenceSet {
    let anchor = anchor_causal_observation(
        QueryObservationReceipt::fixture(outcome, evidence_identities),
        reason,
    )
    .expect("temporal/async fixture should anchor");
    let CausalEvidenceReferenceResolution::Resolved { reference_set, .. } =
        resolve_causal_evidence_references(anchor, requested_families)
    else {
        panic!("temporal/async fixture references should resolve");
    };
    reference_set
}

#[test]
fn admitted_temporal_wake_materialization_projects_query_owned_temporal_explanation() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-temporal-wake",
        ))
        .expect("temporal wake route should resolve");
    let reference_set = reference_set_for(
        CausalObservationOutcome::Changed,
        CausalInspectionReason::ChangedResult,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "query-inspection:temporal-wake",
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                routed.route_identity().evidence_identity(),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::SignalInvalidation,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "signal-invalidation:temporal-wake",
                ),
            ),
        ],
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalInvalidation,
        ],
    );
    let request = request_for_families(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalInvalidation,
        ],
    );
    let CausalInspectionProofFlow::Admitted(admitted) = admit_causal_inspection(request) else {
        panic!("reference-only temporal wake should admit");
    };
    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.admitted_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.subject().anchor_for_reporting(),
        ),
    )
    .expect("temporal wake admission summary should be valid");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        admitted
                            .subject()
                            .query_observation_bridge_evidence_identity(),
                    ),
                )
                .expect("query observation reference should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().evidence_identity(),
                )
                .expect("bridge route reference should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalInvalidation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence("signal-invalidation:temporal-wake"),
                    ),
                )
                .expect("signal invalidation reference should be valid"),
            ),
        ],
    )
    .expect("temporal wake bridge request should be valid");
    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("temporal wake envelope should assemble");

    let QueryCausalInspectionArtifact::Admitted(artifact) = materialize_admitted_causal_inspection(
        &admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("temporal wake materialization should succeed") else {
        panic!("expected admitted temporal wake artifact");
    };

    assert_eq!(
        artifact.temporal_async_explanation().kind(),
        QueryCausalTemporalAsyncExplanationKind::TemporalWake
    );
    assert!(artifact.temporal_async_explanation().offline_explainable());
}

#[test]
fn advisory_async_completion_materialization_projects_query_owned_async_explanation() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-async-completion",
        ))
        .expect("async completion route should resolve");
    let reference_set = reference_set_for(
        CausalObservationOutcome::Changed,
        CausalInspectionReason::ChangedResult,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "query-inspection:async-completion",
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                routed.route_identity().evidence_identity(),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::SignalEvaluation,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "signal-evaluation:async-completion",
                ),
            ),
        ],
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalEvaluation,
        ],
    );
    let request = request_for_families(
        reference_set,
        CausalInspectionRichness::MaterializedDetail,
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalEvaluation,
        ],
    );
    let CausalInspectionProofFlow::Advisory(advisory) = admit_causal_inspection(request) else {
        panic!("materialized async completion should narrow to advisory");
    };
    let summary = BridgeCausalInspectionAdmissionSummary::advisory(
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            advisory.advisory_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            advisory.subject().anchor_for_reporting(),
        ),
    )
    .expect("async completion advisory summary should be valid");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        advisory
                            .subject()
                            .query_observation_bridge_evidence_identity(),
                    ),
                )
                .expect("query observation reference should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().evidence_identity(),
                )
                .expect("bridge route reference should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalEvaluation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence("signal-evaluation:async-completion"),
                    ),
                )
                .expect("signal evaluation reference should be valid"),
            ),
        ],
    )
    .expect("async completion bridge request should be valid");
    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("async completion envelope should assemble");

    let QueryCausalInspectionArtifact::Advisory(artifact) = materialize_advisory_causal_inspection(
        &advisory,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("async completion advisory materialization should succeed") else {
        panic!("expected advisory async completion artifact");
    };

    assert_eq!(
        artifact.temporal_async_explanation().kind(),
        QueryCausalTemporalAsyncExplanationKind::AsyncCompletion
    );
    assert!(artifact.temporal_async_explanation().offline_explainable());
}

#[test]
fn admitted_mixed_cause_suppression_materialization_retains_suppression_identity() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-mixed-suppressed",
        ))
        .expect("mixed suppression route should resolve");
    let reference_set = reference_set_for(
        CausalObservationOutcome::Suppressed,
        CausalInspectionReason::SuppressedResult,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "query-inspection:mixed-suppressed",
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                routed.route_identity().evidence_identity(),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::SignalInvalidation,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "signal-invalidation:mixed-suppressed",
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::SignalEvaluation,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "signal-evaluation:mixed-suppressed",
                ),
            ),
        ],
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalInvalidation,
            CausalEvidenceFamily::SignalEvaluation,
        ],
    );
    let request = request_for_families(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalInvalidation,
            CausalEvidenceFamily::SignalEvaluation,
        ],
    );
    let CausalInspectionProofFlow::Admitted(admitted) = admit_causal_inspection(request) else {
        panic!("reference-only mixed suppression should admit");
    };
    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.admitted_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.subject().anchor_for_reporting(),
        ),
    )
    .expect("mixed suppression summary should be valid");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        admitted
                            .subject()
                            .query_observation_bridge_evidence_identity(),
                    ),
                )
                .expect("query observation reference should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().evidence_identity(),
                )
                .expect("bridge route reference should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalInvalidation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence("signal-invalidation:mixed-suppressed"),
                    ),
                )
                .expect("signal invalidation reference should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalEvaluation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence("signal-evaluation:mixed-suppressed"),
                    ),
                )
                .expect("signal evaluation reference should be valid"),
            ),
        ],
    )
    .expect("mixed suppression bridge request should be valid");
    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("mixed suppression envelope should assemble");

    let artifact = materialize_admitted_causal_inspection(
        &admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("mixed suppression materialization should succeed");

    assert_eq!(
        artifact.temporal_async_explanation().kind(),
        QueryCausalTemporalAsyncExplanationKind::MixedCauseSuppression
    );
}

fn bridge_evidence(value: impl AsRef<str>) -> BridgeIdentityEvidence {
    BridgeIdentityEvidence::from_external_authority(value)
}

#[test]
fn retained_temporal_evidence_projects_same_explanation_for_all_retained_and_explicit_requests() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-temporal-request-parity",
        ))
        .expect("temporal parity route should resolve");
    let receipt = QueryObservationReceipt::fixture(
        CausalObservationOutcome::Changed,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "query-inspection:temporal-request-parity",
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                routed.route_identity().evidence_identity(),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::SignalInvalidation,
                crate::runtime::tests::causal_inspection::causal_test_reference_digest(
                    "signal-invalidation:temporal-request-parity",
                ),
            ),
        ],
    );
    let all_retained_artifact = CausalInspection::for_observation(receipt.clone())
        .why_changed()
        .include_all_retained_evidence()
        .plan()
        .expect("all-retained temporal request should plan")
        .materialize_with_bridge(&runtime)
        .expect("all-retained temporal request should materialize");
    let explicit_artifact = CausalInspection::for_observation(receipt)
        .why_temporal_wake()
        .reference_only()
        .plan()
        .expect("explicit temporal request should plan")
        .materialize_with_bridge(&runtime)
        .expect("explicit temporal request should materialize");

    assert_eq!(
        all_retained_artifact.temporal_async_explanation().kind(),
        QueryCausalTemporalAsyncExplanationKind::TemporalWake
    );
    assert_eq!(
        all_retained_artifact.temporal_async_explanation().kind(),
        explicit_artifact.temporal_async_explanation().kind()
    );
}
