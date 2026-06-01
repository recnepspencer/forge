use super::retained_mapping_support::{
    binding_for, bridge_reference, digest, query_observation_reference,
};
use super::{
    registered_source, runtime, runtime_with_source_adapter, BridgeRuntimePolicy,
    BridgeSourceCapability, BridgeTruthViewSelector, RejectingSourceAdapter, SnapshotReadPacket,
    TruthBranchIdentity, TruthCommitIdentity,
};
use crate::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenialKind, BridgeCausalEvidenceFamily,
};

#[test]
fn causal_envelope_maps_source_failure_by_exact_failure_identity() {
    let runtime =
        runtime_with_source_adapter(BridgeRuntimePolicy::default(), RejectingSourceAdapter);
    let routed = runtime
        .route("commit-causal-source-failure")
        .expect("route should succeed");
    let contract = runtime
        .admit_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .expect("source should admit");
    assert!(
        runtime
            .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
            .is_err(),
        "rejecting adapter should produce a retained source failure"
    );
    let failure = runtime
        .diagnostics()
        .last_source_failure_record()
        .expect("source failure should be retained");

    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:source-failure",
            "causal-anchor:source-failure",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:source-failure"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeSourceFailure,
                failure.failure_identity().as_str(),
            ),
        ],
    )
    .expect("request should be valid");
    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect("source failure should bind by exact identity");
    let failure_class = format!("{:?}", failure.failure_class());
    let delivery_error_kind = format!("{:?}", failure.delivery_error_kind());

    assert_eq!(envelope.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(envelope.counters().retained_bridge_binding_count(), 2);
    assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
    assert_eq!(
        binding_for(
            envelope.bindings(),
            BridgeCausalEvidenceFamily::BridgeSourceFailure,
            failure.failure_identity().as_str()
        )
        .retained_record_digest(),
        Some(
            digest(
                "bridge-causal-retained-source-failure-record",
                &[
                    failure.failure_identity().as_str(),
                    failure.declaration_identity().as_str(),
                    failure.selector_identity(),
                    failure.source_capability_digest(),
                    failure_class.as_str(),
                    delivery_error_kind.as_str(),
                    failure.digest(),
                ],
            )
            .as_str()
        )
    );
}

#[test]
fn causal_envelope_denies_missing_retained_expansion_record_without_scan_fallback() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route("commit-causal-missing-retained-expansion")
        .expect("route should succeed");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing-retained-expansion",
            "causal-anchor:missing-retained-expansion",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:missing-retained-expansion"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeStructuralBranchComparison,
                "missing-branch-comparison-record",
            ),
        ],
    )
    .expect("request should be valid");

    let denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect_err("missing retained record should deny");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRetainedBridgeRecord
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::BridgeStructuralBranchComparison
    );
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(denial.counters().retained_bridge_binding_count(), 1);
    assert_eq!(denial.counters().missing_bridge_record_count(), 1);
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_source_materialization_lookup_cost_ignores_unrelated_records() {
    let mut envelope_identities = Vec::new();

    for unrelated_records in [0, 3, 8] {
        let runtime = runtime(BridgeRuntimePolicy::default());
        let contract = runtime
            .admit_source(registered_source(
                "source:analysis-history",
                BridgeTruthViewSelector::historical_commit(
                    TruthBranchIdentity::new("analysis"),
                    TruthCommitIdentity::new("commit-a"),
                ),
                vec![
                    BridgeSourceCapability::SnapshotRead,
                    BridgeSourceCapability::HistoricalRead,
                    BridgeSourceCapability::BranchRead,
                    BridgeSourceCapability::ReplayCompatibleRead,
                ],
            ))
            .expect("source should admit");
        for index in 0..unrelated_records {
            let packet =
                SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                    format!("entity-noise-{index}"),
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid snapshot aspect key"),
                )]);
            let observation = runtime
                .materialize_source_packet(&contract, packet)
                .expect("noise source should materialize");
            runtime
                .canonicalize_source_materialization_record(&contract, &observation)
                .expect("noise source should canonicalize");
        }
        let routed = runtime
            .route("commit-causal-source-scale")
            .expect("route should succeed");
        let target_observation = runtime
            .materialize_source_packet(&contract, SnapshotReadPacket::new(vec![]))
            .expect("target source should materialize");
        let target_record = runtime
            .canonicalize_source_materialization_record(&contract, &target_observation)
            .expect("target source should canonicalize");
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                "query-admission:source-scale",
                "causal-anchor:source-scale",
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference("query-observation:source-scale"),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.result().result_summary().route_identity().as_str(),
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeSourceMaterialization,
                    target_record.record_identity().as_str(),
                ),
            ],
        )
        .expect("request should be valid");

        let envelope = runtime
            .diagnostics()
            .assemble_causal_explanation_envelope(request)
            .expect("target source materialization should bind");

        assert_eq!(
            runtime.diagnostics().source_materialization_records().len(),
            unrelated_records + 1
        );
        assert_eq!(envelope.counters().bridge_retained_lookup_count(), 2);
        assert_eq!(envelope.counters().retained_bridge_binding_count(), 2);
        assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
        envelope_identities.push(envelope.identity().identity_digest().to_string());
    }

    assert_eq!(envelope_identities[0], envelope_identities[1]);
    assert_eq!(envelope_identities[1], envelope_identities[2]);
}
