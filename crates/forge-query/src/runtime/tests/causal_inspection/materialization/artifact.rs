use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenialKind,
    BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner, BridgeCausalEvidenceReferenceIdentity,
    BridgeCausalInspectionAdmissionSummary, BridgeIdentityEvidence, TruthCommitIdentity,
};

use super::super::super::super::*;
use super::support::*;

#[test]
fn admitted_query_causal_artifact_materializes_sealed_bridge_envelope() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-materialization",
        ))
        .unwrap();
    let reference_set = changed_reference_set(routed.route_identity());
    let flow = admit_causal_inspection(request_for(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
    ));
    let CausalInspectionProofFlow::Admitted(admitted) = flow else {
        panic!("reference-only cross-runtime inspection should admit");
    };
    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.admitted_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.subject().anchor_digest(),
        ),
    )
    .expect("query admission summary should be valid");
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
        .expect("bridge envelope should assemble");

    let artifact = materialize_admitted_causal_inspection(
        &admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("admitted materialization should consume bridge envelope");
    let QueryCausalInspectionArtifact::Admitted(artifact) = artifact else {
        panic!("expected admitted query causal artifact");
    };

    assert_eq!(
        artifact.query_observation_digest(),
        admitted
            .subject()
            .query_observation_bridge_evidence_identity()
            .as_str()
    );
    assert_eq!(
        artifact.result_shape_context_digest(),
        admitted.subject().result_shape_context_digest()
    );
    assert_eq!(
        artifact.bridge_envelope_digest(),
        envelope.envelope_digest()
    );
    assert_eq!(
        artifact.readmission_proof().query_admission_digest(),
        admitted.admitted_inspection_digest()
    );
    assert_eq!(
        artifact.readmission_proof().anchor_digest(),
        admitted.subject().anchor_digest()
    );
    assert_eq!(
        artifact.readmission_proof().bridge_envelope_digest(),
        envelope.envelope_digest()
    );
    assert_eq!(
        artifact.bridge_readmission_proof_digest(),
        artifact.readmission_proof().readmission_proof_digest()
    );
    assert_eq!(
        artifact.receipt().bridge_receipt_digest(),
        Some(envelope.receipt().receipt_digest())
    );
    assert_eq!(artifact.evidence_references().len(), 2);
    assert!(artifact
        .evidence_references()
        .iter()
        .any(|reference| reference.retained_record_digest().is_some()));
    assert_eq!(artifact.performance().bridge_envelope_assembly_count(), 1);
    assert_eq!(artifact.performance().artifact_serialization_count(), 1);
    assert_eq!(artifact.performance().bridge_unindexed_scan_count(), 0);
    assert_eq!(
        artifact.boundary_categories().len(),
        6,
        "artifact should carry all boundary envelope categories"
    );
}

#[test]
fn advisory_query_causal_artifact_redacts_detail_without_changing_bridge_identity() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-advisory",
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
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            advisory.advisory_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            advisory.subject().anchor_digest(),
        ),
    )
    .expect("query advisory summary should be valid");
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
        .expect("bridge envelope should assemble");

    let full_artifact = materialize_advisory_causal_inspection(
        &advisory,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("full advisory materialization should consume bridge envelope");
    let redacted_artifact = materialize_advisory_causal_inspection(
        &advisory,
        &envelope,
        CausalInspectionRedactionPolicy::DigestOnly,
        CausalInspectionMaterializationPolicy::DigestReferenceOnly,
    )
    .expect("redacted advisory materialization should consume bridge envelope");

    assert_eq!(
        full_artifact.causal_identity_digest(),
        redacted_artifact.causal_identity_digest(),
        "redaction must not change query causal identity"
    );
    assert_ne!(
        full_artifact.artifact_digest(),
        redacted_artifact.artifact_digest(),
        "redaction should change materialized artifact detail"
    );

    let QueryCausalInspectionArtifact::Advisory(artifact) = redacted_artifact else {
        panic!("expected advisory query causal artifact");
    };

    assert_eq!(
        artifact.query_observation_digest(),
        advisory
            .subject()
            .query_observation_bridge_evidence_identity()
            .as_str()
    );
    assert_eq!(
        artifact.result_shape_context_digest(),
        advisory.subject().result_shape_context_digest()
    );
    assert_eq!(
        artifact.bridge_envelope_digest(),
        envelope.envelope_digest()
    );
    assert_eq!(
        artifact.readmission_proof().query_admission_digest(),
        advisory.advisory_inspection_digest()
    );
    assert_eq!(
        artifact.readmission_proof().anchor_digest(),
        advisory.subject().anchor_digest()
    );
    assert_eq!(
        artifact.bridge_readmission_proof_digest(),
        artifact.readmission_proof().readmission_proof_digest()
    );
    assert!(artifact
        .evidence_references()
        .iter()
        .all(|reference| reference.detail_redacted()));
    assert!(artifact
        .evidence_references()
        .iter()
        .all(|reference| reference.retained_record_digest().is_none()));
    assert_eq!(
        artifact.performance().redaction_count(),
        artifact.evidence_references().len()
    );
}

#[test]
fn denied_query_causal_artifact_carries_boundary_context_without_bridge_envelope() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-denied",
        ))
        .unwrap();
    let reference_set = changed_reference_set(routed.route_identity());
    let receipt = reference_set.anchor().observation_receipt();
    let target = causal_inspection_target(
        receipt.observation_target().clone(),
        receipt.result_shape_context().clone(),
    )
    .expect("target should match receipt");
    let request = request_causal_inspection(
        reference_set,
        target,
        CausalInspectionExplanationFamily::DurableCausalArchive,
        CausalInspectionRichness::ReferenceOnly,
        &[CausalEvidenceFamily::BridgeRoute],
    )
    .expect("causal inspection request should reach admission boundary");
    let flow = admit_causal_inspection(request);
    let CausalInspectionProofFlow::Denied(denied) = flow else {
        panic!("durable causal archive should deny for phase-5 materialization");
    };

    let artifact = materialize_denied_causal_inspection(
        &denied,
        None,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    );
    let QueryCausalInspectionArtifact::Denied(artifact) = artifact else {
        panic!("expected denied query causal artifact");
    };

    assert_eq!(
        artifact.query_observation_digest(),
        denied
            .subject()
            .query_observation_bridge_evidence_identity()
            .as_str()
    );
    assert_eq!(
        artifact.result_shape_context_digest(),
        denied.subject().result_shape_context_digest()
    );
    assert_eq!(
        artifact.boundary_categories().len(),
        6,
        "denied artifact should remain a complete boundary envelope"
    );
    assert_eq!(artifact.performance().bridge_envelope_assembly_count(), 0);
    assert_eq!(artifact.performance().materialization_count(), 1);
    assert_eq!(artifact.performance().artifact_serialization_count(), 1);
    assert!(!artifact.causal_identity_digest().is_empty());
}

#[test]
fn denied_query_causal_artifact_carries_bridge_denial_posture_and_counters() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-bridge-denied",
        ))
        .unwrap();
    let reference_set = changed_reference_set(routed.route_identity());
    let receipt = reference_set.anchor().observation_receipt();
    let target = causal_inspection_target(
        receipt.observation_target().clone(),
        receipt.result_shape_context().clone(),
    )
    .expect("target should match receipt");
    let request = request_causal_inspection(
        reference_set,
        target,
        CausalInspectionExplanationFamily::DurableCausalArchive,
        CausalInspectionRichness::ReferenceOnly,
        &[CausalEvidenceFamily::BridgeRoute],
    )
    .expect("causal inspection request should reach admission boundary");
    let CausalInspectionProofFlow::Denied(denied) = admit_causal_inspection(request) else {
        panic!("durable causal archive should deny before materialization");
    };
    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            denied.denied_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            denied.subject().anchor_digest(),
        ),
    )
    .expect("query denial summary should be syntactically valid for bridge denial fixture");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        denied
                            .subject()
                            .query_observation_bridge_evidence_identity(),
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalInvalidation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence("signal-invalidation:missing-route-fixture"),
                    ),
                )
                .expect("signal invalidation reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid enough to reach assembly denial");
    let bridge_denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect_err("bridge envelope must require route evidence");

    assert_eq!(
        bridge_denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRequiredBridgeRouteEvidence
    );

    let artifact = materialize_denied_causal_inspection(
        &denied,
        Some(&bridge_denial),
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    );
    let QueryCausalInspectionArtifact::Denied(artifact) = artifact else {
        panic!("expected denied query causal artifact");
    };

    assert_eq!(
        artifact.bridge_denial_digest(),
        Some(bridge_denial.failure_digest())
    );
    assert_eq!(
        artifact.bridge_denial_kind(),
        Some("missing_required_bridge_route_evidence")
    );
    assert_eq!(artifact.bridge_denial_family(), Some("bridge_route"));
    assert_eq!(artifact.performance().bridge_envelope_assembly_count(), 1);
    assert_eq!(artifact.performance().bridge_binding_count(), 2);
    assert_eq!(artifact.performance().bridge_lookup_count(), 0);
    assert_eq!(artifact.performance().materialized_detail_count(), 2);
}

#[test]
fn admitted_materialization_rejects_wrong_bridge_summary_kind() {
    let runtime = bridge_runtime();
    let routed = runtime
        .route(TruthCommitIdentity::from_bridge_harness_label(
            "commit-query-summary-mismatch",
        ))
        .unwrap();
    let reference_set = changed_reference_set(routed.route_identity());
    let flow = admit_causal_inspection(request_for(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
    ));
    let CausalInspectionProofFlow::Admitted(admitted) = flow else {
        panic!("reference-only cross-runtime inspection should admit");
    };
    let wrong_summary = BridgeCausalInspectionAdmissionSummary::advisory(
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.admitted_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.subject().anchor_digest(),
        ),
    )
    .expect("mismatched bridge summary should still be structurally valid");
    let bridge_request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        wrong_summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        admitted
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
        .expect("bridge envelope should assemble");

    let error = materialize_admitted_causal_inspection(
        &admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        CausalInspectionMaterializationErrorKind::AdmissionSummaryKindMismatch
    );
}

fn bridge_evidence(value: impl AsRef<str>) -> BridgeIdentityEvidence {
    BridgeIdentityEvidence::from_external_authority(value)
}
