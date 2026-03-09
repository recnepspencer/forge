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
}
