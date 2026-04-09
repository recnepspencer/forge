use super::*;
use crate::facade::{
    AdmittedConsumerContract, BridgeRouteRequest, CanonicalStreamReplayRecord,
    ChangeStreamDeclaration, ConsumerCheckpointToken, PlannedChangeStreamWindow,
    StreamCheckpointFrontierKind, StreamCoalescingFamily, StreamCoalescingIntent,
    StreamConsumerShape, StreamDeliveryIntent, StreamDiagnosticsPolicyClass,
    StreamReplayAuditResult, StreamReplayMode, StreamResumeMode, StreamWindowDeliveryResult,
    ValidatedStreamProtocol,
};
use crate::routing::canonicalization::digest_string;

pub(super) enum StreamHarnessTarget {
    RoutingWindow { commit_identities: Vec<String> },
    ReplayAuditWindow { commit_identities: Vec<String> },
}

pub(super) enum StreamHarnessExecution {
    Routing {
        window: PlannedChangeStreamWindow,
        result: StreamWindowDeliveryResult,
        checkpoint: ConsumerCheckpointToken,
        replay_record: CanonicalStreamReplayRecord,
    },
    ReplayAudit {
        window: PlannedChangeStreamWindow,
        result: StreamReplayAuditResult,
    },
}

impl StreamHarnessExecution {
    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::Routing {
                window,
                result,
                checkpoint,
                replay_record,
            } => json!({
                "consumer_shape": "routing-consumer",
                "stream_window_identity": window.stream_window_identity().as_str(),
                "stream_member_count": window.counters().stream_member_count(),
                "stream_digest": result.summary().stream_digest(),
                "window_digest": result.summary().window_digest(),
                "consumer_contract_digest": result.summary().consumer_contract_digest(),
                "diagnostics_digest": result.summary().diagnostics_digest(),
                "checkpoint_digest": checkpoint.checkpoint_token_identity(),
                "replay_digest": replay_record.digest(),
                "first_stream_member_identity": window
                    .members()
                    .first()
                    .map(|member| member.stream_member_identity()),
                "last_stream_member_identity": window
                    .members()
                    .last()
                    .map(|member| member.stream_member_identity()),
                "checkpoint_token_identity": checkpoint.checkpoint_token_identity(),
                "replay_record_identity": replay_record.replay_record_identity().as_str(),
                "delivered_route_count": result.summary().delivered_route_count(),
                "delivered_target_count": result.summary().delivered_target_count(),
                "pressure_report": pressure_report_json(window),
                "counter_snapshot": counter_snapshot_json(result.summary().counters()),
            }),
            Self::ReplayAudit { window, result } => json!({
                "consumer_shape": "replay-audit-consumer",
                "stream_window_identity": window.stream_window_identity().as_str(),
                "stream_member_count": window.counters().stream_member_count(),
                "stream_digest": result.summary().stream_digest(),
                "window_digest": result.summary().window_digest(),
                "consumer_contract_digest": result.summary().consumer_contract_digest(),
                "diagnostics_digest": result.summary().diagnostics_digest(),
                "checkpoint_digest": result.checkpoint().checkpoint_token_identity(),
                "replay_digest": result.replay_record().digest(),
                "first_stream_member_identity": window
                    .members()
                    .first()
                    .map(|member| member.stream_member_identity()),
                "last_stream_member_identity": window
                    .members()
                    .last()
                    .map(|member| member.stream_member_identity()),
                "checkpoint_token_identity": result.checkpoint().checkpoint_token_identity(),
                "replay_record_identity": result.replay_record().replay_record_identity().as_str(),
                "audited_member_count": result.summary().audited_member_count(),
                "pressure_report": pressure_report_json(window),
                "counter_snapshot": counter_snapshot_json(result.summary().counters()),
            }),
        }
    }

    pub(super) fn extensions_json(
        &self,
        runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::Routing {
                window,
                result,
                checkpoint,
                replay_record,
            } => BTreeMap::from([
                (
                    "bridge_stream_certification_bundle".to_string(),
                    certification_bundle_json(
                        result.summary().stream_digest(),
                        result.summary().window_digest(),
                        result.summary().consumer_contract_digest(),
                        result.summary().diagnostics_digest(),
                        checkpoint.checkpoint_token_identity(),
                        replay_record.digest(),
                        Some(routing_digest(result)),
                        result.summary().counters(),
                        None,
                        Some(pressure_report_json(window)),
                    ),
                ),
                (
                    "bridge_stream_window".to_string(),
                    json!({
                        "stream_window_identity": window.stream_window_identity().as_str(),
                        "consumer_contract_identity": window.consumer_contract_identity().as_str(),
                        "member_set_digest": window.member_set_digest(),
                        "member_count": window.members().len(),
                        "coalescing_family": format!("{:?}", window.coalescing_family()),
                        "window_digest": window.digest(),
                        "diagnostics_policy_class": format!("{:?}", window.diagnostics_policy_class()),
                    }),
                ),
                (
                    "bridge_stream_delivery".to_string(),
                    json!({
                        "delivered_member_count": result.summary().delivered_member_count(),
                        "delivered_route_count": result.summary().delivered_route_count(),
                        "delivered_target_count": result.summary().delivered_target_count(),
                        "stream_digest": result.summary().stream_digest(),
                        "window_digest": result.summary().window_digest(),
                        "consumer_contract_digest": result.summary().consumer_contract_digest(),
                        "diagnostics_digest": result.summary().diagnostics_digest(),
                        "pressure_report": pressure_report_json(window),
                        "counter_snapshot": counter_snapshot_json(result.summary().counters()),
                    }),
                ),
                stream_checkpoint_extension(runtime_bridge, checkpoint),
                stream_replay_extension(runtime_bridge, replay_record),
            ]),
            Self::ReplayAudit { window, result } => BTreeMap::from([
                (
                    "bridge_stream_certification_bundle".to_string(),
                    certification_bundle_json(
                        result.summary().stream_digest(),
                        result.summary().window_digest(),
                        result.summary().consumer_contract_digest(),
                        result.summary().diagnostics_digest(),
                        result.checkpoint().checkpoint_token_identity(),
                        result.replay_record().digest(),
                        None,
                        result.summary().counters(),
                        None,
                        Some(pressure_report_json(window)),
                    ),
                ),
                (
                    "bridge_stream_window".to_string(),
                    json!({
                        "stream_window_identity": window.stream_window_identity().as_str(),
                        "consumer_contract_identity": window.consumer_contract_identity().as_str(),
                        "member_set_digest": window.member_set_digest(),
                        "member_count": window.members().len(),
                        "coalescing_family": format!("{:?}", window.coalescing_family()),
                        "window_digest": window.digest(),
                        "diagnostics_policy_class": format!("{:?}", window.diagnostics_policy_class()),
                    }),
                ),
                (
                    "bridge_stream_delivery".to_string(),
                    json!({
                        "audited_member_count": result.summary().audited_member_count(),
                        "stream_digest": result.summary().stream_digest(),
                        "window_digest": result.summary().window_digest(),
                        "consumer_contract_digest": result.summary().consumer_contract_digest(),
                        "diagnostics_digest": result.summary().diagnostics_digest(),
                        "pressure_report": pressure_report_json(window),
                        "counter_snapshot": counter_snapshot_json(result.summary().counters()),
                    }),
                ),
                stream_checkpoint_extension(runtime_bridge, result.checkpoint()),
                stream_replay_extension(runtime_bridge, result.replay_record()),
            ]),
        }
    }
}

pub(super) fn parse_stream_harness_target(
    target: &str,
) -> Option<Result<StreamHarnessTarget, BridgeHarnessError>> {
    if let Some(rest) = target.strip_prefix("stream-routing:") {
        return Some(
            parse_commit_list(rest)
                .map(|commit_identities| StreamHarnessTarget::RoutingWindow { commit_identities }),
        );
    }
    if let Some(rest) = target.strip_prefix("stream-replay-audit:") {
        return Some(parse_commit_list(rest).map(|commit_identities| {
            StreamHarnessTarget::ReplayAuditWindow { commit_identities }
        }));
    }
    None
}

pub(super) fn execute_stream_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    target: StreamHarnessTarget,
) -> Result<StreamHarnessExecution, BridgeHarnessError> {
    match target {
        StreamHarnessTarget::RoutingWindow { commit_identities } => {
            let protocol = runtime_bridge
                .validate_change_stream_declaration(stream_declaration(
                    runtime_bridge,
                    StreamConsumerShape::RoutingConsumer,
                ))
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "bridge stream declaration validation failed: {error}"
                    ))
                })?;
            let contract = resolve_stream_contract(runtime_bridge, &protocol)?;
            let window = plan_stream_window(runtime_bridge, &contract, &commit_identities)?;
            let result = runtime_bridge
                .deliver_change_stream_window(&contract, &window)
                .map_err(|error| {
                    BridgeHarnessError::new(format!("bridge stream delivery failed: {error}"))
                })?;
            let checkpoint = runtime_bridge.publish_consumer_checkpoint(
                &contract,
                &window,
                StreamCheckpointFrontierKind::ContiguousFrontier,
            );
            let replay_record = runtime_bridge
                .canonicalize_stream_replay_record(&contract, &window, &checkpoint)
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "bridge stream replay record canonicalization failed: {error}"
                    ))
                })?;
            Ok(StreamHarnessExecution::Routing {
                window,
                result,
                checkpoint,
                replay_record,
            })
        }
        StreamHarnessTarget::ReplayAuditWindow { commit_identities } => {
            let protocol = runtime_bridge
                .validate_change_stream_declaration(stream_declaration(
                    runtime_bridge,
                    StreamConsumerShape::ReplayAuditConsumer,
                ))
                .map_err(|error| {
                    BridgeHarnessError::new(format!(
                        "bridge stream declaration validation failed: {error}"
                    ))
                })?;
            let contract = resolve_stream_contract(runtime_bridge, &protocol)?;
            let window = plan_stream_window(runtime_bridge, &contract, &commit_identities)?;
            let result = runtime_bridge
                .deliver_replay_audit_stream_window(&contract, &window)
                .map_err(|error| {
                    BridgeHarnessError::new(format!("bridge stream replay-audit failed: {error}"))
                })?;
            Ok(StreamHarnessExecution::ReplayAudit { window, result })
        }
    }
}

fn resolve_stream_contract(
    runtime_bridge: &crate::facade::RuntimeBridge,
    protocol: &ValidatedStreamProtocol,
) -> Result<AdmittedConsumerContract, BridgeHarnessError> {
    runtime_bridge
        .resolve_change_stream_consumer_contract(protocol)
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge stream contract resolution failed: {error}"))
        })
}

fn plan_stream_window(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &AdmittedConsumerContract,
    commit_identities: &[String],
) -> Result<PlannedChangeStreamWindow, BridgeHarnessError> {
    let mut envelopes = Vec::with_capacity(commit_identities.len());
    for commit_identity in commit_identities {
        let envelope = runtime_bridge
            .ingest_committed_patch(BridgeRouteRequest::for_commit(commit_identity.clone()))
            .map_err(|error| {
                BridgeHarnessError::new(format!(
                    "bridge stream ingestion failed for `{commit_identity}`: {error}"
                ))
            })?;
        envelopes.push(envelope);
    }
    runtime_bridge
        .plan_change_stream_window(contract, envelopes)
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge stream window planning failed: {error}"))
        })
}

fn stream_declaration(
    runtime_bridge: &crate::facade::RuntimeBridge,
    consumer_shape: StreamConsumerShape,
) -> ChangeStreamDeclaration {
    let (coalescing_intent, delivery_intent) = match consumer_shape {
        StreamConsumerShape::RoutingConsumer => (
            StreamCoalescingIntent::Prefer(StreamCoalescingFamily::RoutingWindowCoalescing),
            StreamDeliveryIntent::RouteInvalidations,
        ),
        StreamConsumerShape::ReplayAuditConsumer => (
            StreamCoalescingIntent::Prefer(StreamCoalescingFamily::ReplayAuditWindowCoalescing),
            StreamDeliveryIntent::ReplayAudit,
        ),
    };
    ChangeStreamDeclaration::new(
        consumer_shape,
        StreamResumeMode::FromCheckpointOnly,
        crate::facade::StreamCheckpointPublicationMode::PublishEveryWindow,
        coalescing_intent,
        StreamReplayMode::Enabled,
        delivery_intent,
        diagnostics_policy_class(runtime_bridge.policy().diagnostics_tier()),
    )
}

fn diagnostics_policy_class(
    tier: crate::facade::BridgeDiagnosticsTier,
) -> StreamDiagnosticsPolicyClass {
    match tier {
        crate::facade::BridgeDiagnosticsTier::Minimal => StreamDiagnosticsPolicyClass::Minimal,
        crate::facade::BridgeDiagnosticsTier::Standard => StreamDiagnosticsPolicyClass::Standard,
        crate::facade::BridgeDiagnosticsTier::Exhaustive => {
            StreamDiagnosticsPolicyClass::Exhaustive
        }
    }
}

fn parse_commit_list(rest: &str) -> Result<Vec<String>, BridgeHarnessError> {
    let commit_identities = rest
        .split(',')
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    if commit_identities.is_empty() {
        return Err(BridgeHarnessError::new(
            "stream harness targets require at least one commit identity",
        ));
    }
    Ok(commit_identities)
}

fn stream_checkpoint_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    checkpoint: &ConsumerCheckpointToken,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_stream_checkpoint(checkpoint);
    (
        "bridge_stream_checkpoint".to_string(),
        json!({
            "checkpoint_token_identity": checkpoint.checkpoint_token_identity(),
            "checkpoint_digest": checkpoint.checkpoint_token_identity(),
            "consumer_contract_identity": checkpoint.consumer_contract_identity().as_str(),
            "stream_protocol_identity": checkpoint.stream_protocol_identity().as_str(),
            "checkpoint_member_count": checkpoint.checkpoint_member_count(),
            "acknowledged_member_set_digest": checkpoint.acknowledged_member_set_digest(),
            "source_retention_anchor": checkpoint.source_retention_anchor(),
            "counter_snapshot": counter_snapshot_json(checkpoint.counters()),
            "explanation": json!({
                "checkpoint_frontier_kind": format!("{:?}", explanation.checkpoint_frontier_kind()),
                "contiguous_acknowledged_through_position": explanation.contiguous_acknowledged_through_position(),
            }),
        }),
    )
}

fn stream_replay_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    replay_record: &CanonicalStreamReplayRecord,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_stream_replay_record(replay_record);
    (
        "bridge_stream_replay_record".to_string(),
        json!({
            "replay_record_identity": replay_record.replay_record_identity().as_str(),
            "replay_digest": replay_record.digest(),
            "consumer_contract_identity": replay_record.consumer_contract_identity().as_str(),
            "stream_window_identity": replay_record.stream_window_identity().as_str(),
            "checkpoint_token_identity": replay_record.checkpoint_token_identity(),
            "replay_basis_digest": replay_record.replay_basis_digest(),
            "counter_snapshot": counter_snapshot_json(replay_record.counters()),
            "explanation": json!({
                "protocol_semantics_version": explanation.protocol_semantics_version(),
            }),
        }),
    )
}

fn counter_snapshot_json(counters: &crate::facade::StreamProtocolCounters) -> serde_json::Value {
    json!({
        "stream_member_count": counters.stream_member_count(),
        "stream_window_count": counters.stream_window_count(),
        "stream_window_member_count": counters.stream_window_member_count(),
        "stream_consumer_contract_count": counters.stream_consumer_contract_count(),
        "stream_checkpoint_count": counters.stream_checkpoint_count(),
        "stream_checkpoint_member_count": counters.stream_checkpoint_member_count(),
        "stream_resume_attempt_count": counters.stream_resume_attempt_count(),
        "stream_resume_rejection_count": counters.stream_resume_rejection_count(),
        "stream_replay_count": counters.stream_replay_count(),
        "stream_replay_mismatch_count": counters.stream_replay_mismatch_count(),
        "stream_coalesced_member_count": counters.stream_coalesced_member_count(),
        "stream_coalesced_window_count": counters.stream_coalesced_window_count(),
        "stream_duplicate_member_observation_count": counters.stream_duplicate_member_observation_count(),
        "stream_backpressure_signal_count": counters.stream_backpressure_signal_count(),
        "stream_consumer_saturated_count": counters.stream_consumer_saturated_count(),
        "stream_checkpoint_lag_count": counters.stream_checkpoint_lag_count(),
        "stream_protocol_mismatch_count": counters.stream_protocol_mismatch_count(),
    })
}

fn certification_bundle_json(
    stream_digest: &str,
    window_digest: &str,
    consumer_contract_digest: &str,
    diagnostics_digest: &str,
    checkpoint_digest: &str,
    replay_digest: &str,
    routing_digest: Option<String>,
    counters: &crate::facade::StreamProtocolCounters,
    failure_digest: Option<String>,
    pressure_report: Option<serde_json::Value>,
) -> serde_json::Value {
    json!({
        "stream_digest": stream_digest,
        "window_digest": window_digest,
        "checkpoint_digest": checkpoint_digest,
        "consumer_contract_digest": consumer_contract_digest,
        "diagnostics_digest": diagnostics_digest,
        "replay_digest": replay_digest,
        "routing_digest": routing_digest,
        "failure_digest": failure_digest,
        "pressure_report": pressure_report,
        "counter_snapshot": counter_snapshot_json(counters),
    })
}

fn routing_digest(result: &StreamWindowDeliveryResult) -> String {
    digest_string(
        "stream-routing-digest",
        &result
            .route_results()
            .iter()
            .map(|entry| entry.result_summary().route_identity().as_str())
            .collect::<Vec<_>>()
            .join("|"),
    )
    .to_string()
}

fn pressure_report_json(window: &PlannedChangeStreamWindow) -> serde_json::Value {
    let pressure = crate::stream::BackpressureDecisionRecord::classify(window);
    json!({
        "backpressure_decision_identity": pressure.backpressure_decision_identity(),
        "pressure_class": pressure.pressure_class(),
        "pressure_reason_family": pressure.pressure_reason_family(),
        "counter_snapshot": counter_snapshot_json(pressure.counters()),
    })
}
