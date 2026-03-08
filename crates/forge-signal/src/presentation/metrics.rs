use serde::{Deserialize, Serialize};

use crate::data::telemetry::RuntimeTelemetry;

/// Read-only summary of graph-local runtime telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GraphMetrics {
    pub evaluation_calls: u64,
    pub nodes_evaluated: u64,
    pub nodes_recomputed: u64,
    pub invalidation_nodes_visited: u64,
    pub condition_skip_count: u64,
    pub skipped_by_comparator: u64,
    pub gc_epoch_count: u64,
    pub gc_epoch_nanos: u128,
}

impl From<&RuntimeTelemetry> for GraphMetrics {
    fn from(telemetry: &RuntimeTelemetry) -> Self {
        Self {
            evaluation_calls: telemetry.evaluation_calls,
            nodes_evaluated: telemetry.nodes_evaluated,
            nodes_recomputed: telemetry.nodes_recomputed,
            invalidation_nodes_visited: telemetry.invalidation_nodes_visited,
            condition_skip_count: telemetry.condition_skip_count,
            skipped_by_comparator: telemetry.skipped_by_comparator,
            gc_epoch_count: telemetry.gc_epoch_count,
            gc_epoch_nanos: telemetry.gc_epoch_nanos,
        }
    }
}

/// Read-only summary of runtime-level orchestration telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeMetrics {
    pub transaction_begin_count: u64,
    pub transaction_commit_count: u64,
    pub transaction_rollback_count: u64,
    pub transaction_poison_count: u64,
    pub checkpoint_flushes: u64,
    pub checkpoint_flush_nanos: u128,
    pub event_flushes: u64,
    pub rollback_count: u64,
    pub staged_node_patch_count: u64,
    pub max_touched_nodes_in_txn: u64,
}
