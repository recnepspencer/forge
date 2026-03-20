use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationTelemetry {
    pub evaluation_calls: u64,
    pub evaluation_nanos: u128,
    pub nodes_evaluated: u64,
    pub nodes_recomputed: u64,
    pub skipped_by_comparator: u64,
    pub suppressed_downstream_propagations: u64,
    pub output_identity_unchanged_count: u64,
    pub memoization_hits: u64,
    pub memoization_misses: u64,
    pub condition_skip_count: u64,
    pub ondemand_deferred_count: u64,
    pub debounce_deferred_count: u64,
    pub evaluation_stack_peak: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationTelemetry {
    pub batch_width: u64,
    pub dirty_delta_breadth: u64,
    pub partition_aware_recomputations: u64,
    pub keyed_evaluation_count: u64,
    pub partition_scoped_invalidation_checks: u64,
    pub partition_match_dirty_count: u64,
    pub detail_match_dirty_count: u64,
    pub partition_scope_revert_clean_count: u64,
    pub partition_interner_growth_delta: u64,
    pub invalidation_nodes_visited: u64,
    pub narrowed_frontier_width: u64,
    pub transitive_frontier_width: u64,
    pub subscriber_repair_breadth: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionTelemetry {
    pub transaction_begin_count: u64,
    pub transaction_commit_count: u64,
    pub transaction_rollback_count: u64,
    pub transaction_poison_count: u64,
    pub decision_log_event_count: u64,
    pub staged_node_patch_count: u64,
    pub max_touched_nodes_in_txn: u64,
    pub transaction_mark_dirty_candidate_visits: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannerTelemetry {
    pub plans_built: u64,
    pub stages_built: u64,
    pub tasks_scheduled: u64,
    pub tasks_pruned_before_execution: u64,
    pub maybe_stale_validation_tasks: u64,
    pub incremental_strategy_count: u64,
    pub rebuild_strategy_count: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionTelemetry {
    pub stage_execution_count: u64,
    pub stage_execution_nanos: u128,
    pub parallel_stage_dispatch_count: u64,
    pub max_tasks_in_stage: u64,
    pub serial_executor_usage_count: u64,
    pub parallel_executor_usage_count: u64,
    pub execution_snapshots_built: u64,
    pub execution_snapshot_nanos: u128,
    pub prepared_evaluations_produced: u64,
    pub prepared_evaluations_applied: u64,
    pub dependency_capture_updates: u64,
    pub rewiring_apply_count: u64,
    pub apply_group_width_total: u64,
    pub max_apply_group_width: u64,
    pub apply_group_disjoint_count: u64,
    pub serial_precompute_task_count: u64,
    pub parallel_precompute_task_count: u64,
    pub stage_precompute_nanos: u128,
    pub stage_apply_nanos: u128,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageTelemetry {
    pub gc_epoch_count: u64,
    pub gc_epoch_nanos: u128,
    pub graph_storage_compaction_count: u64,
    pub graph_storage_dependency_segments_rewritten: u64,
    pub graph_storage_subscriber_segments_rewritten: u64,
    pub graph_storage_snapshot_rewrites: u64,
    pub shared_snapshot_replacement_count: u64,
    pub version_only_snapshot_update_count: u64,
    pub snapshot_batch_size: u64,
    pub rolled_back_created_node_count: u64,
    pub subscriber_index_rebuild_count: u64,
    pub scratch_reentry_error_count: u64,
    pub hot_path_artifact_retention_count: u64,
    pub hot_path_artifact_reconstruction_count: u64,
    pub structural_delta_size: u64,
    pub patch_application_breadth: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointTelemetry {
    pub event_flushes: u64,
    pub event_flush_nanos: u128,
    pub checkpoint_flushes: u64,
    pub checkpoint_flush_nanos: u128,
    pub rollback_count: u64,
    pub snapshot_restore_count: u64,
    pub snapshot_restore_apply_active_policy_count: u64,
    pub snapshot_restore_shared_delta_node_count: u64,
    pub snapshot_restore_coarse_reason_count: u64,
    pub checkpoint_size: u64,
    pub journal_replay_span: u64,
}

/// Lightweight runtime telemetry for signal orchestration internals.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeTelemetry {
    pub evaluation: EvaluationTelemetry,
    pub invalidation: InvalidationTelemetry,
    pub transaction: TransactionTelemetry,
    pub planner: PlannerTelemetry,
    pub execution: ExecutionTelemetry,
    pub storage: StorageTelemetry,
    pub checkpoint: CheckpointTelemetry,
}
