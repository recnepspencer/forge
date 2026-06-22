use std::collections::BTreeMap;

use super::super::certification_bundle::StreamHarnessCertificationBundle;
use super::super::{pressure_report, StreamHarnessExecution};
use crate::facade::{
    CanonicalStreamReplayRecord, ConsumerCheckpointToken, PlannedChangeStreamWindow,
    StreamReplayAuditResult, StreamWindowDeliveryResult,
};
use crate::stream::BackpressureDecisionRecord;
use serde_json::json;

pub(in crate::harness::adapter::adapter_impl) fn summary_json(
    execution: &StreamHarnessExecution,
) -> serde_json::Value {
    match execution {
        StreamHarnessExecution::Routing {
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
            "checkpoint_digest": checkpoint.checkpoint_token_identity_for_reporting(),
            "replay_digest": replay_record.digest(),
            "first_stream_member_identity": window
                .members()
                .first()
                .map(|member| member.stream_member_identity()),
            "last_stream_member_identity": window
                .members()
                .last()
                .map(|member| member.stream_member_identity()),
            "checkpoint_token_identity": checkpoint.checkpoint_token_identity_for_reporting(),
            "replay_record_identity": replay_record.replay_record_identity().as_str(),
            "delivered_route_count": result.summary().delivered_route_count(),
            "delivered_target_count": result.summary().delivered_target_count(),
            "pressure_report": pressure_report_json(&BackpressureDecisionRecord::classify(window)),
            "counter_snapshot": counter_snapshot_json(result.summary().counters()),
        }),
        StreamHarnessExecution::ReplayAudit { window, result } => json!({
            "consumer_shape": "replay-audit-consumer",
            "stream_window_identity": window.stream_window_identity().as_str(),
            "stream_member_count": window.counters().stream_member_count(),
            "stream_digest": result.summary().stream_digest(),
            "window_digest": result.summary().window_digest(),
            "consumer_contract_digest": result.summary().consumer_contract_digest(),
            "diagnostics_digest": result.summary().diagnostics_digest(),
            "checkpoint_digest": result.checkpoint().checkpoint_token_identity_for_reporting(),
            "replay_digest": result.replay_record().digest(),
            "first_stream_member_identity": window
                .members()
                .first()
                .map(|member| member.stream_member_identity()),
            "last_stream_member_identity": window
                .members()
                .last()
                .map(|member| member.stream_member_identity()),
            "checkpoint_token_identity": result.checkpoint().checkpoint_token_identity_for_reporting(),
            "replay_record_identity": result.replay_record().replay_record_identity().as_str(),
            "audited_member_count": result.summary().audited_member_count(),
            "pressure_report": pressure_report_json(&BackpressureDecisionRecord::classify(window)),
            "counter_snapshot": counter_snapshot_json(result.summary().counters()),
        }),
    }
}

pub(in crate::harness::adapter::adapter_impl) fn extensions_json(
    execution: &StreamHarnessExecution,
    runtime_bridge: &crate::facade::RuntimeBridge,
) -> BTreeMap<String, serde_json::Value> {
    match execution {
        StreamHarnessExecution::Routing {
            window,
            result,
            checkpoint,
            replay_record,
        } => BTreeMap::from([
            (
                "bridge_stream_certification_bundle".to_string(),
                certification_bundle_json(&StreamHarnessCertificationBundle::routing(
                    result,
                    checkpoint,
                    replay_record,
                    pressure_report(window),
                )),
            ),
            ("bridge_stream_window".to_string(), window_json(window)),
            (
                "bridge_stream_delivery".to_string(),
                routing_delivery_json(result, &pressure_report(window)),
            ),
            stream_checkpoint_extension(runtime_bridge, checkpoint),
            stream_replay_extension(runtime_bridge, replay_record),
        ]),
        StreamHarnessExecution::ReplayAudit { window, result } => BTreeMap::from([
            (
                "bridge_stream_certification_bundle".to_string(),
                certification_bundle_json(&StreamHarnessCertificationBundle::replay_audit(
                    result,
                    pressure_report(window),
                )),
            ),
            ("bridge_stream_window".to_string(), window_json(window)),
            (
                "bridge_stream_delivery".to_string(),
                replay_audit_delivery_json(result, &pressure_report(window)),
            ),
            stream_checkpoint_extension(runtime_bridge, result.checkpoint()),
            stream_replay_extension(runtime_bridge, result.replay_record()),
        ]),
    }
}

pub(in crate::harness::adapter::adapter_impl) fn window_json(
    window: &PlannedChangeStreamWindow,
) -> serde_json::Value {
    json!({
        "stream_window_identity": window.stream_window_identity().as_str(),
        "consumer_contract_identity": window.consumer_contract_identity().as_str(),
        "member_set_digest": window.member_set_digest(),
        "member_count": window.members().len(),
        "coalescing_family": format!("{:?}", window.coalescing_family()),
        "window_digest": window.digest(),
        "diagnostics_policy_class": format!("{:?}", window.diagnostics_policy_class()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn routing_delivery_json(
    result: &StreamWindowDeliveryResult,
    pressure: &BackpressureDecisionRecord,
) -> serde_json::Value {
    json!({
        "delivered_member_count": result.summary().delivered_member_count(),
        "delivered_route_count": result.summary().delivered_route_count(),
        "delivered_target_count": result.summary().delivered_target_count(),
        "stream_digest": result.summary().stream_digest(),
        "window_digest": result.summary().window_digest(),
        "consumer_contract_digest": result.summary().consumer_contract_digest(),
        "diagnostics_digest": result.summary().diagnostics_digest(),
        "pressure_report": pressure_report_json(pressure),
        "counter_snapshot": counter_snapshot_json(result.summary().counters()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn replay_audit_delivery_json(
    result: &StreamReplayAuditResult,
    pressure: &BackpressureDecisionRecord,
) -> serde_json::Value {
    json!({
        "audited_member_count": result.summary().audited_member_count(),
        "stream_digest": result.summary().stream_digest(),
        "window_digest": result.summary().window_digest(),
        "consumer_contract_digest": result.summary().consumer_contract_digest(),
        "diagnostics_digest": result.summary().diagnostics_digest(),
        "pressure_report": pressure_report_json(pressure),
        "counter_snapshot": counter_snapshot_json(result.summary().counters()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn certification_bundle_json(
    bundle: &StreamHarnessCertificationBundle,
) -> serde_json::Value {
    json!({
        "stream_digest": bundle.stream_digest(),
        "window_digest": bundle.window_digest(),
        "checkpoint_digest": bundle.checkpoint_digest(),
        "consumer_contract_digest": bundle.consumer_contract_digest(),
        "diagnostics_digest": bundle.diagnostics_digest(),
        "replay_digest": bundle.replay_digest(),
        "routing_digest": bundle.routing_digest(),
        "failure_digest": bundle.failure_digest(),
        "pressure_report": pressure_report_json(bundle.pressure_report()),
        "counter_snapshot": counter_snapshot_json(bundle.counters()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn stream_checkpoint_extension(
    runtime_bridge: &crate::facade::RuntimeBridge,
    checkpoint: &ConsumerCheckpointToken,
) -> (String, serde_json::Value) {
    let explanation = runtime_bridge
        .diagnostics()
        .explain_stream_checkpoint(checkpoint);
    (
        "bridge_stream_checkpoint".to_string(),
        json!({
            "checkpoint_token_identity": checkpoint.checkpoint_token_identity_for_reporting(),
            "checkpoint_digest": checkpoint.checkpoint_token_identity_for_reporting(),
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

pub(in crate::harness::adapter::adapter_impl) fn stream_replay_extension(
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
            "checkpoint_token_identity": replay_record.checkpoint_token_identity_for_reporting(),
            "replay_basis_digest": replay_record.replay_basis_digest(),
            "counter_snapshot": counter_snapshot_json(replay_record.counters()),
            "explanation": json!({
                "protocol_semantics_version": explanation.protocol_semantics_version(),
            }),
        }),
    )
}

pub(in crate::harness::adapter::adapter_impl) fn pressure_report_json(
    pressure: &BackpressureDecisionRecord,
) -> serde_json::Value {
    json!({
        "backpressure_decision_identity": pressure.backpressure_decision_identity(),
        "pressure_class": pressure.pressure_class(),
        "pressure_reason_family": pressure.pressure_reason_family(),
        "counter_snapshot": counter_snapshot_json(pressure.counters()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn counter_snapshot_json(
    counters: &crate::facade::StreamProtocolCounters,
) -> serde_json::Value {
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
