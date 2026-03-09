use serde::{Deserialize, Serialize};

/// Lightweight runtime telemetry for signal orchestration internals.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeTelemetry {
    /// Count of evaluation entrypoint calls.
    pub evaluation_calls: u64,
    /// Total nanoseconds spent in evaluation entrypoint calls.
    pub evaluation_nanos: u128,
    /// Count of nodes visited in recompute stage.
    pub nodes_evaluated: u64,
    /// Count of nodes that executed `compute`.
    pub nodes_recomputed: u64,
    /// Count of maybe-stale nodes reverted clean by comparator checks.
    pub skipped_by_comparator: u64,
    /// Count of downstream nodes reverted clean after output identity matched.
    pub suppressed_downstream_propagations: u64,
    /// Count of recomputations whose output identity remained unchanged.
    pub output_identity_unchanged_count: u64,
    /// Count of memoization cache hits.
    pub memoization_hits: u64,
    /// Count of memoization cache misses.
    pub memoization_misses: u64,
    /// Count of evaluations that reported changed partitions or regions.
    pub partition_aware_recomputations: u64,
    /// Count of keyed computation evaluations.
    pub keyed_evaluation_count: u64,
    /// Count of direct invalidation checks involving partition-scoped edges.
    pub partition_scoped_invalidation_checks: u64,
    /// Count of direct subscribers dirtied by matching whole-partition scopes.
    pub partition_match_dirty_count: u64,
    /// Count of direct subscribers dirtied by matching partition+detail scopes.
    pub detail_match_dirty_count: u64,
    /// Count of scoped nodes reverted clean because no subscribed partition was touched.
    pub partition_scope_revert_clean_count: u64,
    /// Event-bus flush count.
    pub event_flushes: u64,
    /// Total nanoseconds spent inside event-bus flush.
    pub event_flush_nanos: u128,
    /// Checkpoint-runtime flush count.
    pub checkpoint_flushes: u64,
    /// Total nanoseconds spent inside checkpoint flush.
    pub checkpoint_flush_nanos: u128,
    /// Rollback invocation count.
    pub rollback_count: u64,
    /// Transaction begin call count.
    pub transaction_begin_count: u64,
    /// Transaction commit success count.
    pub transaction_commit_count: u64,
    /// Transaction rollback count.
    pub transaction_rollback_count: u64,
    /// Transaction poison count.
    pub transaction_poison_count: u64,
    /// Number of staged node patches applied across commits.
    pub staged_node_patch_count: u64,
    /// Maximum number of touched node patches observed in one transaction.
    pub max_touched_nodes_in_txn: u64,
    /// Count of evaluation skips/deferments caused by node conditions.
    pub condition_skip_count: u64,
    /// Count of `OnDemand` nodes deferred under default evaluation mode.
    pub ondemand_deferred_count: u64,
    /// Count of `Debounce` nodes deferred because the quiet-period gate was not ready.
    pub debounce_deferred_count: u64,
    /// Count of rejected scratch re-entry attempts.
    pub scratch_reentry_error_count: u64,
    /// Count of nodes visited during invalidation passes.
    pub invalidation_nodes_visited: u64,
    /// Peak size reached by the evaluation task stack.
    pub evaluation_stack_peak: u64,
    /// Count of GC epochs executed.
    pub gc_epoch_count: u64,
    /// Total nanoseconds spent inside GC epochs.
    pub gc_epoch_nanos: u128,
    /// Count of execution plans built.
    pub plans_built: u64,
    /// Count of execution stages built.
    pub stages_built: u64,
    /// Count of tasks scheduled across built plans.
    pub tasks_scheduled: u64,
    /// Count of tasks pruned before execution.
    pub tasks_pruned_before_execution: u64,
    /// Count of maybe-stale validation tasks scheduled.
    pub maybe_stale_validation_tasks: u64,
    /// Count of stage executions performed.
    pub stage_execution_count: u64,
    /// Total nanoseconds spent executing stages.
    pub stage_execution_nanos: u128,
    /// Count of parallel stage dispatch attempts.
    pub parallel_stage_dispatch_count: u64,
    /// Maximum number of tasks scheduled in one stage.
    pub max_tasks_in_stage: u64,
    /// Count of serial executor uses.
    pub serial_executor_usage_count: u64,
    /// Count of parallel executor uses.
    pub parallel_executor_usage_count: u64,
    /// Count of lightweight execution snapshots built.
    pub execution_snapshots_built: u64,
    /// Count of prepared evaluations produced by precompute.
    pub prepared_evaluations_produced: u64,
    /// Count of prepared evaluations applied during serial commit.
    pub prepared_evaluations_applied: u64,
    /// Count of dependency-edge updates caused by prepared dependency capture.
    pub dependency_capture_updates: u64,
    /// Count of tasks precomputed serially.
    pub serial_precompute_task_count: u64,
    /// Count of tasks precomputed in parallel.
    pub parallel_precompute_task_count: u64,
    /// Total nanoseconds spent building execution snapshots.
    pub execution_snapshot_nanos: u128,
    /// Total nanoseconds spent in stage precompute work.
    pub stage_precompute_nanos: u128,
    /// Total nanoseconds spent in stage apply work.
    pub stage_apply_nanos: u128,
}
