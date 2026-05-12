use crate::diagnostics::history::{
    BridgeHistoricalEvaluationCounters, BridgeHistoricalEvaluationFailureRecord,
};
use crate::routing::{
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeCanonicalBulkPlanRecord,
};
use crate::stream::{
    ConsumerCheckpointToken, StreamCheckpointFrontierKind, StreamProtocolCounters,
};

pub(crate) fn bulk_planning_digest(record: &BridgeCanonicalBulkPlanRecord) -> String {
    let selected_mode = format!("{:?}", record.selected_mode());
    let planning_failure_count = record.planning_failure_count().to_string();
    let planning_failures_digest = bulk_planning_failures_digest(record.planning_failures());
    let counters_digest = bulk_planning_counters_digest(record.counters());
    digest(
        "bridge-causal-retained-bulk-planning-record",
        &[
            record.workload_identity().as_str(),
            record.schema_version(),
            record.canonical_request_digest(),
            record.normalized_summary_digest(),
            record.canonical_planning_identity().as_str(),
            record.admission_profile_identity().as_str(),
            record.packet_set_digest(),
            record.execution_plan_digest(),
            record.reduced_artifact_digest(),
            selected_mode.as_str(),
            record.decision_log_digest(),
            counters_digest.as_str(),
            planning_failure_count.as_str(),
            planning_failures_digest.as_str(),
        ],
    )
}

pub(crate) fn historical_evaluation_failure_digest(
    record: &BridgeHistoricalEvaluationFailureRecord,
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

pub(crate) fn stream_checkpoint_digest(record: &ConsumerCheckpointToken) -> String {
    let checkpoint_member_count = record.checkpoint_member_count().to_string();
    let counters_digest = stream_protocol_counters_digest(record.counters());
    digest(
        "bridge-causal-retained-stream-checkpoint-record",
        &[
            record.checkpoint_token_identity(),
            record.consumer_contract_identity().as_str(),
            record.stream_protocol_identity().as_str(),
            checkpoint_frontier_kind_label(record.checkpoint_frontier_kind()),
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

fn historical_evaluation_counters_digest(counters: &BridgeHistoricalEvaluationCounters) -> String {
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

fn bulk_planning_counters_digest(counters: &BridgeBulkPlanningCounters) -> String {
    let counter_parts = [
        counters.bulk_workload_count().to_string(),
        counters.bulk_routed_item_count().to_string(),
        counters.bulk_normalized_workload_width().to_string(),
        counters.bulk_packet_count().to_string(),
        counters.bulk_packet_entry_count().to_string(),
        counters.bulk_reduction_input_count().to_string(),
        counters.bulk_reduction_output_count().to_string(),
        counters.bulk_fallback_count().to_string(),
        counters.bulk_packet_queue_depth_peak().to_string(),
        counters.bulk_reducer_input_buffer_peak().to_string(),
        counters.bulk_replay_mismatch_count().to_string(),
        counters.bulk_unsupported_path_count().to_string(),
        counters.bulk_serial_required_count().to_string(),
        counters.bulk_parallel_legal_count().to_string(),
        counters.bulk_parallel_profitable_count().to_string(),
        counters
            .bulk_parallel_preparation_admitted_count()
            .to_string(),
        counters
            .bulk_parallel_preparation_rejected_count()
            .to_string(),
        counters
            .bulk_parallel_fallback_to_serial_count()
            .to_string(),
    ];
    let counter_refs: Vec<&str> = counter_parts.iter().map(String::as_str).collect();
    digest("bridge-bulk-planning-counters", &counter_refs)
}

fn bulk_planning_failures_digest(failures: &[BridgeBulkPlanningFailure]) -> String {
    let failure_digests: Vec<&str> = failures.iter().map(|failure| failure.digest()).collect();
    digest("bridge-bulk-planning-failures", &failure_digests)
}

fn stream_protocol_counters_digest(counters: &StreamProtocolCounters) -> String {
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

fn checkpoint_frontier_kind_label(value: StreamCheckpointFrontierKind) -> &'static str {
    match value {
        StreamCheckpointFrontierKind::ContiguousFrontier => "contiguous-frontier",
        StreamCheckpointFrontierKind::ContiguousFrontierWithObservedDuplicates => {
            "contiguous-frontier-with-observed-duplicates"
        }
    }
}

fn digest(label: &str, parts: &[&str]) -> String {
    use sha2::{Digest, Sha256};

    let mut canonical = String::from(label);
    for part in parts {
        canonical.push('|');
        canonical.push_str(part);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    format!("{label}:sha256:{digest:x}")
}
