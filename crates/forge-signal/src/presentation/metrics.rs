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
    pub suppressed_downstream_propagations: u64,
    pub output_identity_unchanged_count: u64,
    pub memoization_hits: u64,
    pub memoization_misses: u64,
    pub partition_aware_recomputations: u64,
    pub keyed_evaluation_count: u64,
    pub partition_scoped_invalidation_checks: u64,
    pub partition_match_dirty_count: u64,
    pub detail_match_dirty_count: u64,
    pub partition_scope_revert_clean_count: u64,
    pub partition_interner_size: usize,
    pub gc_epoch_count: u64,
    pub gc_epoch_nanos: u128,
}

impl GraphMetrics {
    pub fn from_runtime_telemetry(telemetry: &RuntimeTelemetry, partition_interner_size: usize) -> Self {
        Self {
            evaluation_calls: telemetry.evaluation_calls,
            nodes_evaluated: telemetry.nodes_evaluated,
            nodes_recomputed: telemetry.nodes_recomputed,
            invalidation_nodes_visited: telemetry.invalidation_nodes_visited,
            condition_skip_count: telemetry.condition_skip_count,
            skipped_by_comparator: telemetry.skipped_by_comparator,
            suppressed_downstream_propagations: telemetry.suppressed_downstream_propagations,
            output_identity_unchanged_count: telemetry.output_identity_unchanged_count,
            memoization_hits: telemetry.memoization_hits,
            memoization_misses: telemetry.memoization_misses,
            partition_aware_recomputations: telemetry.partition_aware_recomputations,
            keyed_evaluation_count: telemetry.keyed_evaluation_count,
            partition_scoped_invalidation_checks: telemetry.partition_scoped_invalidation_checks,
            partition_match_dirty_count: telemetry.partition_match_dirty_count,
            detail_match_dirty_count: telemetry.detail_match_dirty_count,
            partition_scope_revert_clean_count: telemetry.partition_scope_revert_clean_count,
            partition_interner_size,
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
    pub keyed_evaluation_count: u64,
    pub memoization_hits: u64,
    pub memoization_misses: u64,
    pub suppressed_downstream_propagations: u64,
    pub partition_scoped_invalidation_checks: u64,
    pub partition_match_dirty_count: u64,
    pub detail_match_dirty_count: u64,
    pub partition_scope_revert_clean_count: u64,
}
