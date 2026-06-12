use crate::diagnostics::history::{
    BridgeHistoricalEvaluationCounters, BridgeHistoricalEvaluationFailureClass,
    BridgeHistoricalEvaluationFailureIdentity, BridgeHistoricalEvaluationFailureRecord,
};
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::identity::BridgeIdentityEvidence;
use crate::routing::{
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeCanonicalBulkPlanRecord,
    BridgePreparationMode, BridgeWorkloadIdentity,
};
use crate::stream::{
    ConsumerCheckpointToken, StreamCheckpointFrontierKind, StreamProtocolCounters,
};

use super::super::digest_basis::{
    compose_retained_causal_mapping_evidence_identity,
    compose_retained_causal_mapping_evidence_identity_for_basis,
    retained_mapping_identity_digest_part, retained_mapping_shape_part,
    retained_mapping_value_part, RetainedCausalMappingDigestArtifact,
    RetainedCausalMappingDigestBasis,
};

pub(crate) fn bulk_planning_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .bulk_record_for_workload_identity(&BridgeWorkloadIdentity::new(reference_identity))
        .map(|record| bulk_planning_digest(&record))
}

pub(crate) fn historical_evaluation_failure_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .historical_failure_for_identity(&BridgeHistoricalEvaluationFailureIdentity::new(
            reference_identity,
        ))
        .map(|record| historical_evaluation_failure_digest(&record))
}

pub(crate) fn stream_checkpoint_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .stream_checkpoint_for_identity(reference_identity)
        .map(|record| stream_checkpoint_digest(&record))
}

pub(crate) fn bulk_planning_digest(
    record: &BridgeCanonicalBulkPlanRecord,
) -> BridgeIdentityEvidence {
    let planning_failure_count = record.planning_failure_count().to_string();
    let planning_failures_digest = bulk_planning_failures_digest(record.planning_failures());
    let counters_digest = bulk_planning_counters_digest(record.counters());
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::BulkPlanningRecord,
        &[
            retained_mapping_identity_digest_part(record.workload_identity().as_str()),
            retained_mapping_shape_part(record.schema_version()),
            retained_mapping_identity_digest_part(record.canonical_request_digest()),
            retained_mapping_identity_digest_part(record.normalized_summary_digest()),
            retained_mapping_identity_digest_part(record.canonical_planning_identity().as_str()),
            retained_mapping_identity_digest_part(record.admission_profile_identity().as_str()),
            retained_mapping_identity_digest_part(record.packet_set_digest()),
            retained_mapping_identity_digest_part(record.execution_plan_digest()),
            retained_mapping_identity_digest_part(record.reduced_artifact_digest()),
            retained_mapping_shape_part(preparation_mode_label(record.selected_mode())),
            retained_mapping_identity_digest_part(record.decision_log_digest()),
            retained_mapping_identity_digest_part(counters_digest.as_str()),
            retained_mapping_value_part(planning_failure_count.as_str()),
            retained_mapping_identity_digest_part(planning_failures_digest.as_str()),
        ],
    )
}

fn preparation_mode_label(value: BridgePreparationMode) -> &'static str {
    match value {
        BridgePreparationMode::Serial => "serial",
        BridgePreparationMode::ParallelPreparation => "parallel-preparation",
    }
}

pub(crate) fn historical_evaluation_failure_digest(
    record: &BridgeHistoricalEvaluationFailureRecord,
) -> BridgeIdentityEvidence {
    let commit_identity = record
        .commit_identity()
        .map(|identity| identity.as_str())
        .unwrap_or("none");
    let snapshot_identity = record
        .snapshot_identity()
        .map(|identity| identity.as_str())
        .unwrap_or("none");
    let counters_digest = historical_evaluation_counters_digest(record.counters());
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::HistoricalEvaluationFailureRecord,
        &[
            retained_mapping_identity_digest_part(record.failure_identity().as_str()),
            retained_mapping_identity_digest_part(record.declaration_identity().as_str()),
            retained_mapping_identity_digest_part(record.selector_identity().as_str()),
            retained_mapping_identity_digest_part(record.branch_identity().as_str()),
            retained_mapping_identity_digest_part(commit_identity),
            retained_mapping_identity_digest_part(snapshot_identity),
            retained_mapping_shape_part(historical_failure_class_label(record.failure_class())),
            retained_mapping_value_part(record.detail()),
            retained_mapping_identity_digest_part(counters_digest.as_str()),
        ],
    )
}

fn historical_failure_class_label(value: BridgeHistoricalEvaluationFailureClass) -> &'static str {
    match value {
        BridgeHistoricalEvaluationFailureClass::UnsupportedTruthViewSelector => {
            "unsupported-truth-view-selector"
        }
        BridgeHistoricalEvaluationFailureClass::TruthViewUnavailable => "truth-view-unavailable",
        BridgeHistoricalEvaluationFailureClass::RejectedBranchMismatch => {
            "rejected-branch-mismatch"
        }
        BridgeHistoricalEvaluationFailureClass::RejectedSnapshotMismatch => {
            "rejected-snapshot-mismatch"
        }
        BridgeHistoricalEvaluationFailureClass::RejectedHistoricalResolutionFailure => {
            "rejected-historical-resolution-failure"
        }
        BridgeHistoricalEvaluationFailureClass::HistoricalReplayMismatch => {
            "historical-replay-mismatch"
        }
        BridgeHistoricalEvaluationFailureClass::UnresolvedTruthViewPolicyConflict => {
            "unresolved-truth-view-policy-conflict"
        }
    }
}

pub(crate) fn stream_checkpoint_digest(record: &ConsumerCheckpointToken) -> BridgeIdentityEvidence {
    let checkpoint_member_count = record.checkpoint_member_count().to_string();
    let counters_digest = stream_protocol_counters_digest(record.counters());
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::StreamCheckpointRecord,
        &[
            retained_mapping_identity_digest_part(record.checkpoint_token_identity()),
            retained_mapping_identity_digest_part(record.consumer_contract_identity().as_str()),
            retained_mapping_identity_digest_part(record.stream_protocol_identity().as_str()),
            retained_mapping_shape_part(checkpoint_frontier_kind_label(
                record.checkpoint_frontier_kind(),
            )),
            retained_mapping_value_part(record.contiguous_acknowledged_through_position()),
            retained_mapping_identity_digest_part(
                record.contiguous_acknowledged_through_member_identity(),
            ),
            retained_mapping_identity_digest_part(record.acknowledged_member_set_digest()),
            retained_mapping_value_part(checkpoint_member_count.as_str()),
            retained_mapping_identity_digest_part(record.source_retention_anchor()),
            retained_mapping_shape_part(record.protocol_semantics_version()),
            retained_mapping_identity_digest_part(counters_digest.as_str()),
        ],
    )
}

fn historical_evaluation_counters_digest(
    counters: &BridgeHistoricalEvaluationCounters,
) -> BridgeIdentityEvidence {
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
    let counter_basis = RetainedCausalMappingDigestBasis::from_counter_values(counter_parts);
    compose_retained_causal_mapping_evidence_identity_for_basis(
        RetainedCausalMappingDigestArtifact::HistoricalEvaluationCounters,
        &counter_basis,
    )
}

fn bulk_planning_counters_digest(counters: &BridgeBulkPlanningCounters) -> BridgeIdentityEvidence {
    let counter_parts = [
        counters.bulk_workload_count().to_string(),
        counters.bulk_routed_item_count().to_string(),
        counters.bulk_normalized_workload_width().to_string(),
        counters.bulk_packet_count().to_string(),
        counters.bulk_packet_entry_count().to_string(),
        counters.bulk_reduction_input_count().to_string(),
        counters.bulk_reduction_output_count().to_string(),
        counters.bulk_widening_count().to_string(),
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
        counters.bulk_parallel_serial_reduction_count().to_string(),
    ];
    let counter_basis = RetainedCausalMappingDigestBasis::from_counter_values(counter_parts);
    compose_retained_causal_mapping_evidence_identity_for_basis(
        RetainedCausalMappingDigestArtifact::BulkPlanningCounters,
        &counter_basis,
    )
}

fn bulk_planning_failures_digest(failures: &[BridgeBulkPlanningFailure]) -> BridgeIdentityEvidence {
    let failure_basis =
        RetainedCausalMappingDigestBasis::from_bulk_planning_failure_records(failures);
    compose_retained_causal_mapping_evidence_identity_for_basis(
        RetainedCausalMappingDigestArtifact::BulkPlanningFailures,
        &failure_basis,
    )
}

fn stream_protocol_counters_digest(counters: &StreamProtocolCounters) -> BridgeIdentityEvidence {
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
    let counter_basis = RetainedCausalMappingDigestBasis::from_counter_values(counter_parts);
    compose_retained_causal_mapping_evidence_identity_for_basis(
        RetainedCausalMappingDigestArtifact::StreamProtocolCounters,
        &counter_basis,
    )
}

fn checkpoint_frontier_kind_label(value: StreamCheckpointFrontierKind) -> &'static str {
    match value {
        StreamCheckpointFrontierKind::ContiguousFrontier => "contiguous-frontier",
        StreamCheckpointFrontierKind::ContiguousFrontierWithObservedDuplicates => {
            "contiguous-frontier-with-observed-duplicates"
        }
    }
}
