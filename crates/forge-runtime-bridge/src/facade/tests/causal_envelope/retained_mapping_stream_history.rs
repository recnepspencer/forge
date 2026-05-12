use super::retained_mapping_support::{
    binding_for, bridge_reference, digest, query_observation_reference,
};
use super::{
    canonical_envelope, registered_source, runtime, runtime_with_source_adapter,
    BridgeRuntimePolicy, BridgeSourceCapability, BridgeTruthViewSelector, RejectingSourceAdapter,
    SnapshotReadPacket, TruthBranchIdentity, TruthCommitIdentity,
};
use crate::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenialKind, BridgeCausalEvidenceFamily,
};

#[test]
fn causal_envelope_maps_historical_failure_and_stream_checkpoint_by_exact_identity() {
    let runtime =
        runtime_with_source_adapter(BridgeRuntimePolicy::default(), RejectingSourceAdapter);
    let routed = runtime
        .route("commit-causal-history-stream")
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
        "rejecting source adapter should record historical failure"
    );
    let historical_failure = runtime
        .diagnostics()
        .last_historical_evaluation_failure()
        .expect("historical failure should be retained");
    let stream_checkpoint = retain_stream_checkpoint(&runtime, "target");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:history-stream",
            "causal-anchor:history-stream",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:history-stream"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeHistoricalEvaluationFailure,
                historical_failure.failure_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeStreamCheckpoint,
                stream_checkpoint.checkpoint_token_identity(),
            ),
        ],
    )
    .expect("request should be valid");

    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect("historical failure and stream checkpoint should bind");

    assert_eq!(envelope.counters().bridge_retained_lookup_count(), 3);
    assert_eq!(envelope.counters().retained_bridge_binding_count(), 3);
    assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
    assert_eq!(
        binding_for(
            envelope.bindings(),
            BridgeCausalEvidenceFamily::BridgeHistoricalEvaluationFailure,
            historical_failure.failure_identity().as_str()
        )
        .retained_record_digest(),
        Some(historical_failure_digest(&historical_failure).as_str())
    );
    assert_eq!(
        binding_for(
            envelope.bindings(),
            BridgeCausalEvidenceFamily::BridgeStreamCheckpoint,
            stream_checkpoint.checkpoint_token_identity()
        )
        .retained_record_digest(),
        Some(stream_checkpoint_digest(&stream_checkpoint).as_str())
    );
}

#[test]
fn causal_envelope_denies_missing_stream_checkpoint_without_scan_fallback() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route("commit-causal-missing-stream-checkpoint")
        .expect("route should succeed");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing-stream-checkpoint",
            "causal-anchor:missing-stream-checkpoint",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:missing-stream-checkpoint"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeStreamCheckpoint,
                "missing-stream-checkpoint",
            ),
        ],
    )
    .expect("request should be valid");

    let denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect_err("missing stream checkpoint should deny");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRetainedBridgeRecord
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::BridgeStreamCheckpoint
    );
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(denial.counters().retained_bridge_binding_count(), 1);
    assert_eq!(denial.counters().missing_bridge_record_count(), 1);
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_stream_checkpoint_lookup_cost_ignores_unrelated_records() {
    let mut envelope_identities = Vec::new();

    for unrelated_records in [0, 3, 8] {
        let runtime = runtime(BridgeRuntimePolicy::default());
        for index in 0..unrelated_records {
            retain_stream_checkpoint(&runtime, &format!("noise-{index}"));
        }
        let routed = runtime
            .route("commit-causal-stream-checkpoint-scale")
            .expect("route should succeed");
        let target_checkpoint = retain_stream_checkpoint(&runtime, "target");
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                "query-admission:stream-checkpoint-scale",
                "causal-anchor:stream-checkpoint-scale",
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference("query-observation:stream-checkpoint-scale"),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.result().result_summary().route_identity().as_str(),
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeStreamCheckpoint,
                    target_checkpoint.checkpoint_token_identity(),
                ),
            ],
        )
        .expect("request should be valid");

        let envelope = runtime
            .diagnostics()
            .assemble_causal_explanation_envelope(request)
            .expect("target stream checkpoint should bind");

        assert_eq!(
            runtime.diagnostics().stream_checkpoints().len(),
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

fn retain_stream_checkpoint(
    runtime: &crate::facade::RuntimeBridge,
    suffix: &str,
) -> crate::stream::ConsumerCheckpointToken {
    let stream_declaration = crate::stream::ChangeStreamDeclaration::new(
        crate::stream::StreamConsumerShape::RoutingConsumer,
        crate::stream::StreamResumeMode::FromCheckpointOnly,
        crate::stream::StreamCheckpointPublicationMode::PublishEveryWindow,
        crate::stream::StreamCoalescingIntent::Prefer(
            crate::stream::StreamCoalescingFamily::RoutingWindowCoalescing,
        ),
        crate::stream::StreamReplayMode::Enabled,
        crate::stream::StreamDeliveryIntent::RouteInvalidations,
        crate::stream::StreamDiagnosticsPolicyClass::Standard,
    );
    let stream_protocol = runtime
        .validate_change_stream_declaration(stream_declaration)
        .expect("stream declaration should validate");
    let stream_contract = runtime
        .resolve_change_stream_consumer_contract(&stream_protocol)
        .expect("stream contract should resolve");
    let stream_window = runtime
        .plan_change_stream_window(
            &stream_contract,
            vec![canonical_envelope(
                "main",
                &format!("commit-{suffix}"),
                &format!("patch-{suffix}"),
                &format!("snapshot-{suffix}"),
            )],
        )
        .expect("stream window should plan");
    runtime.publish_consumer_checkpoint(
        &stream_contract,
        &stream_window,
        crate::stream::StreamCheckpointFrontierKind::ContiguousFrontier,
    )
}

fn historical_failure_digest(
    record: &crate::facade::BridgeHistoricalEvaluationFailureRecord,
) -> String {
    let failure_class = format!("{:?}", record.failure_class());
    let commit_identity = record
        .commit_identity()
        .map(|identity| identity.as_str())
        .unwrap_or("none");
    let snapshot_identity = record
        .snapshot_identity()
        .map(|identity| identity.as_str())
        .unwrap_or("none");
    let counters_digest = historical_evaluation_counters_digest(record.counters());
    digest(
        "bridge-causal-retained-historical-evaluation-failure-record",
        &[
            record.failure_identity().as_str(),
            record.declaration_identity().as_str(),
            record.selector_identity().as_str(),
            record.branch_identity().as_str(),
            commit_identity,
            snapshot_identity,
            failure_class.as_str(),
            record.detail(),
            counters_digest.as_str(),
        ],
    )
}

fn stream_checkpoint_digest(record: &crate::stream::ConsumerCheckpointToken) -> String {
    let checkpoint_member_count = record.checkpoint_member_count().to_string();
    let counters_digest = stream_protocol_counters_digest(record.counters());
    digest(
        "bridge-causal-retained-stream-checkpoint-record",
        &[
            record.checkpoint_token_identity(),
            record.consumer_contract_identity().as_str(),
            record.stream_protocol_identity().as_str(),
            "contiguous-frontier",
            record.contiguous_acknowledged_through_position(),
            record.contiguous_acknowledged_through_member_identity(),
            record.acknowledged_member_set_digest(),
            checkpoint_member_count.as_str(),
            record.source_retention_anchor(),
            record.protocol_semantics_version(),
            counters_digest.as_str(),
        ],
    )
}

fn historical_evaluation_counters_digest(
    counters: &crate::facade::BridgeHistoricalEvaluationCounters,
) -> String {
    let counter_parts = [
        counters.truth_view_selector_count().to_string(),
        counters.historical_truth_view_count().to_string(),
        counters.branch_truth_view_count().to_string(),
        counters.planned_truth_view_packet_count().to_string(),
        counters.resolved_truth_view_policy_count().to_string(),
        counters.materialized_truth_view_count().to_string(),
        counters.truth_view_unavailable_count().to_string(),
        counters.truth_view_branch_mismatch_count().to_string(),
        counters.truth_view_snapshot_mismatch_count().to_string(),
        counters.historical_replay_mismatch_count().to_string(),
        counters.branch_local_evaluation_count().to_string(),
        counters.truth_view_decision_log_count().to_string(),
        counters.selector_width().to_string(),
        counters.branch_width().to_string(),
        counters.direct_snapshot_materialization_count().to_string(),
        counters.commit_envelope_materialization_count().to_string(),
        counters.branch_head_materialization_count().to_string(),
    ];
    let counter_refs: Vec<&str> = counter_parts.iter().map(String::as_str).collect();
    digest("bridge-historical-evaluation-counters", &counter_refs)
}

fn stream_protocol_counters_digest(counters: &crate::stream::StreamProtocolCounters) -> String {
    let counter_parts = [
        counters.stream_member_count().to_string(),
        counters.stream_window_count().to_string(),
        counters.stream_window_member_count().to_string(),
        counters.stream_consumer_contract_count().to_string(),
        counters.stream_checkpoint_count().to_string(),
        counters.stream_checkpoint_member_count().to_string(),
        counters.stream_resume_attempt_count().to_string(),
        counters.stream_resume_rejection_count().to_string(),
        counters.stream_replay_count().to_string(),
        counters.stream_replay_mismatch_count().to_string(),
        counters.stream_coalesced_member_count().to_string(),
        counters.stream_coalesced_window_count().to_string(),
        counters
            .stream_duplicate_member_observation_count()
            .to_string(),
        counters.stream_backpressure_signal_count().to_string(),
        counters.stream_consumer_saturated_count().to_string(),
        counters.stream_checkpoint_lag_count().to_string(),
        counters.stream_protocol_mismatch_count().to_string(),
    ];
    let counter_refs: Vec<&str> = counter_parts.iter().map(String::as_str).collect();
    digest("bridge-stream-protocol-counters", &counter_refs)
}
