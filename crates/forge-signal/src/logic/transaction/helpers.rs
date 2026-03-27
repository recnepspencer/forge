use crate::data::checkpoint::CheckpointBarrier;
use crate::data::error::SignalError;
use crate::data::evaluator::CheckpointEvaluator;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::DedupedNodeBatch;
use crate::logic::planner::ExecutionReport;

use super::runtime::SignalTransaction;

pub fn flush_checkpoint_in_txn<'a, D, I, E, Ctx, T, Ev>(
    txn: &mut SignalTransaction<'a, D, I, E, Ctx, T>,
    barrier: CheckpointBarrier,
    evaluator: &mut Ev,
    ctx: &mut Ev::Context,
) -> Result<usize, SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
    Ev: CheckpointEvaluator<Domain = D, Impact = I>,
{
    txn.flush_checkpoint(barrier, evaluator, ctx)
}

pub fn emit_event_in_txn<'a, D, I, E, Ctx, T>(
    txn: &mut SignalTransaction<'a, D, I, E, Ctx, T>,
    event: E,
) where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    txn.emit_event(event);
}

pub(super) fn collect_dirty_targets(graph: &SignalGraph) -> Vec<NodeId> {
    DedupedNodeBatch::canonicalize_unordered(graph.live_node_ids().into_iter().filter_map(|node| {
        let Ok(state) = graph.get_state(node) else {
            return None;
        };
        (!matches!(state, crate::data::node::NodeState::Clean)).then_some(node)
    }))
    .into_vec()
}

pub(super) fn empty_execution_report() -> ExecutionReport {
    ExecutionReport {
        plan_summary: crate::logic::planner::PlanSummary::default(),
        stage_count: 0,
        task_count: 0,
        tasks_executed: 0,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 0,
        tasks_with_suppressed_propagation: 0,
        execution_snapshots_built: 0,
        prepared_evaluations_produced: 0,
        prepared_evaluations_applied: 0,
        dependency_capture_updates: 0,
        execution_snapshot_nanos: 0,
        stage_precompute_nanos: 0,
        stage_apply_nanos: 0,
        semantic_finalize_nanos: 0,
        semantic_segment_count: 0,
        stages: Vec::new(),
    }
}
