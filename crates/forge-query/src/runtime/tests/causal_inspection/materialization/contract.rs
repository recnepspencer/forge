use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenialKind,
    BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner, BridgeCausalEvidenceReferenceIdentity,
    BridgeCausalInspectionAdmissionSummary, TruthCommitIdentity,
};

use super::super::super::super::*;
use super::support::*;

#[test]
fn admitted_replay_materialization_rejects_missing_requested_replay_posture() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::new(
            "commit-query-replay-posture-missing",
        ))
        .unwrap();
    let signal_replay_cursor = "signal-replay-cursor:missing-posture";
    let flow = admitted_replay_flow_requesting_signal_cursor(
        routed.route_identity(),
        signal_replay_cursor,
        CausalInspectionRichness::ReferenceOnly,
    );
    let CausalInspectionProofFlow::Admitted(admitted) = flow else {
        panic!("replay inspection should admit before bridge envelope materialization");
    };
    let envelope = bridge_route_only_envelope_for_admitted_replay(&runtime, &admitted, &routed);

    let error = materialize_admitted_causal_inspection(
        &admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        CausalInspectionMaterializationErrorKind::ReplayPostureUnsupported
    );
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn bridge_request_rejects_missing_query_observation_binding_before_materialization() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::new("commit-query-observation-missing"))
        .unwrap();
    let reference_set = changed_reference_set(routed.route_identity());
    let flow = admit_causal_inspection(request_for(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
    ));
    let CausalInspectionProofFlow::Admitted(admitted) = flow else {
        panic!("reference-only inspection should admit");
    };
    let bridge_request_denial = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary_for_admitted(&admitted),
        vec![bridge_reference(
            BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.route_identity().as_str(),
            )
            .expect("route evidence reference identity should be valid"),
        )],
    )
    .expect_err("bridge request should reject a missing query observation anchor");

    assert_eq!(
        bridge_request_denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingQueryObservationAnchor
    );
}

#[test]
fn advisory_materialization_rejects_mismatched_query_observation_binding() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::new(
            "commit-query-observation-mismatch",
        ))
        .unwrap();
    let reference_set = changed_reference_set(routed.route_identity());
    let flow = admit_causal_inspection(request_for(
        reference_set,
        CausalInspectionRichness::MaterializedDetail,
    ));
    let CausalInspectionProofFlow::Advisory(advisory) = flow else {
        panic!("materialized detail should narrow to advisory");
    };
    let summary = BridgeCausalInspectionAdmissionSummary::advisory(
        advisory.advisory_inspection_digest(),
        advisory.subject().anchor_digest(),
    )
    .expect("query advisory summary should be valid");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    "query-observation:wrong-inspection",
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().as_str(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid");
    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("bridge envelope should assemble");

    let error = materialize_advisory_causal_inspection(
        &advisory,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        CausalInspectionMaterializationErrorKind::QueryObservationBindingMismatch
    );
}

#[test]
fn bridge_request_rejects_multiple_query_observation_bindings_before_materialization() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::new(
            "commit-query-observation-overclaim",
        ))
        .unwrap();
    let reference_set = changed_reference_set(routed.route_identity());
    let flow = admit_causal_inspection(request_for(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
    ));
    let CausalInspectionProofFlow::Admitted(admitted) = flow else {
        panic!("reference-only inspection should admit");
    };
    let bridge_request_denial = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary_for_admitted(&admitted),
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    admitted.subject().query_observation_digest(),
                )
                .expect("query observation reference identity should be valid"),
            ),
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    "query-observation:unrelated-overclaim",
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().as_str(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
        ],
    )
    .expect_err("bridge request should reject multiple query observation anchors");

    assert_eq!(
        bridge_request_denial.kind(),
        BridgeCausalEnvelopeDenialKind::QueryObservationAnchorOverclaim
    );
}

#[test]
fn advisory_replay_materialization_rejects_missing_requested_replay_posture() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::new(
            "commit-query-advisory-replay-posture-missing",
        ))
        .unwrap();
    let signal_replay_cursor = "signal-replay-cursor:advisory-missing-posture";
    let flow = admitted_replay_flow_requesting_signal_cursor(
        routed.route_identity(),
        signal_replay_cursor,
        CausalInspectionRichness::MaterializedDetail,
    );
    let CausalInspectionProofFlow::Advisory(advisory) = flow else {
        panic!("materialized replay inspection should narrow to advisory");
    };
    let envelope = bridge_route_only_envelope_for_advisory_replay(&runtime, &advisory, &routed);

    let error = materialize_advisory_causal_inspection(
        &advisory,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        CausalInspectionMaterializationErrorKind::ReplayPostureUnsupported
    );
}

#[test]
fn admitted_replay_materialization_accepts_signal_owned_replay_cursor_posture() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::new(
            "commit-query-replay-posture-bound",
        ))
        .unwrap();
    let signal_replay_cursor = "signal-replay-cursor:bound-posture";
    let flow = admitted_replay_flow_requesting_signal_cursor(
        routed.route_identity(),
        signal_replay_cursor,
        CausalInspectionRichness::ReferenceOnly,
    );
    let CausalInspectionProofFlow::Admitted(admitted) = flow else {
        panic!("replay inspection should admit before bridge envelope materialization");
    };
    let envelope = signal_replay_cursor_envelope_for_admitted_replay(
        &runtime,
        &admitted,
        &routed,
        signal_replay_cursor,
    );

    let artifact = materialize_admitted_causal_inspection(
        &admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("signal replay cursor should satisfy replay posture materialization");
    let QueryCausalInspectionArtifact::Admitted(artifact) = artifact else {
        panic!("expected admitted query causal artifact");
    };

    assert!(artifact.evidence_references().iter().any(|reference| {
        reference.owner() == BridgeCausalEvidenceOwner::Signal.as_str()
            && reference.family() == BridgeCausalEvidenceFamily::SignalReplayCursor.as_str()
            && reference.reference_identity() == signal_replay_cursor
    }));
    assert_eq!(artifact.performance().bridge_unindexed_scan_count(), 0);
}

fn admitted_replay_flow_requesting_signal_cursor(
    route_identity: &forge_runtime_bridge::facade::BridgeRouteIdentity,
    signal_replay_cursor: &str,
    richness: CausalInspectionRichness,
) -> CausalInspectionProofFlow {
    let reference_set =
        replay_reference_set_with_signal_cursor(route_identity, signal_replay_cursor);
    admit_causal_inspection(request_for_families(
        reference_set,
        richness,
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalReplayCursor,
        ],
    ))
}

fn bridge_route_only_envelope_for_admitted_replay(
    runtime: &forge_runtime_bridge::facade::RuntimeBridge,
    admitted: &AdmittedCausalInspection,
    routed: &forge_runtime_bridge::facade::BridgeRoute,
) -> forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope {
    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        admitted.admitted_inspection_digest(),
        admitted.subject().anchor_digest(),
    )
    .expect("query admission summary should be valid");
    bridge_route_only_envelope(
        runtime,
        summary,
        admitted.subject().query_observation_digest(),
        routed,
    )
}

fn bridge_route_only_envelope_for_advisory_replay(
    runtime: &forge_runtime_bridge::facade::RuntimeBridge,
    advisory: &AdvisoryCausalInspection,
    routed: &forge_runtime_bridge::facade::BridgeRoute,
) -> forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope {
    let summary = BridgeCausalInspectionAdmissionSummary::advisory(
        advisory.advisory_inspection_digest(),
        advisory.subject().anchor_digest(),
    )
    .expect("query advisory summary should be valid");
    bridge_route_only_envelope(
        runtime,
        summary,
        advisory.subject().query_observation_digest(),
        routed,
    )
}

fn signal_replay_cursor_envelope_for_admitted_replay(
    runtime: &forge_runtime_bridge::facade::RuntimeBridge,
    admitted: &AdmittedCausalInspection,
    routed: &forge_runtime_bridge::facade::BridgeRoute,
    signal_replay_cursor: &str,
) -> forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope {
    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        admitted.admitted_inspection_digest(),
        admitted.subject().anchor_digest(),
    )
    .expect("query admission summary should be valid");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    admitted.subject().query_observation_digest(),
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().as_str(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalReplayCursor,
                    signal_replay_cursor,
                )
                .expect("signal replay cursor reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid");
    runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("bridge envelope should assemble")
}

fn bridge_route_only_envelope(
    runtime: &forge_runtime_bridge::facade::RuntimeBridge,
    summary: BridgeCausalInspectionAdmissionSummary,
    query_observation_digest: &str,
    routed: &forge_runtime_bridge::facade::BridgeRoute,
) -> forge_runtime_bridge::facade::BridgeCausalExplanationEnvelope {
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(query_observation_digest)
                    .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().as_str(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid");
    runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("bridge envelope should assemble")
}

fn summary_for_admitted(
    admitted: &AdmittedCausalInspection,
) -> BridgeCausalInspectionAdmissionSummary {
    BridgeCausalInspectionAdmissionSummary::admitted(
        admitted.admitted_inspection_digest(),
        admitted.subject().anchor_digest(),
    )
    .expect("query admission summary should be valid")
}
