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
}
