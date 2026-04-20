use crate::backend::records::StoreState;
use crate::failure::StoreError;
use crate::live_query::continuation::ContinuationStrategy;

use super::selection::select_commit_ids_for_batch;
use super::strategies::{execute_admitted_batch, execute_broadened_batch};
use crate::live_query::continuation::batch_surface::{
    CaughtUpContinuationBatch, ContinuationBatchId, ContinuationBatchResult,
    ControlLaneBatchReceipt,
};
use crate::live_query::continuation::plan::CursorContinuationPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContinuationExecutionEffect {
    Batch,
    Broadening,
    ControlLaneFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContinuationExecutionMetrics {
    pub support_rows_read: u64,
    pub narrowed_item_count: u64,
    pub broadened_item_count: u64,
    pub step_count: u64,
}

#[derive(Debug)]
pub(crate) struct ExecutedContinuationBatch {
    result: ContinuationBatchResult,
    metrics: ContinuationExecutionMetrics,
    effects: Vec<ContinuationExecutionEffect>,
}

impl ExecutedContinuationBatch {
    pub(crate) fn new(
        result: ContinuationBatchResult,
        metrics: ContinuationExecutionMetrics,
        effects: Vec<ContinuationExecutionEffect>,
    ) -> Self {
        Self {
            result,
            metrics,
            effects,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ContinuationBatchResult,
        ContinuationExecutionMetrics,
        Vec<ContinuationExecutionEffect>,
    ) {
        (self.result, self.metrics, self.effects)
    }
}

pub(crate) fn verify_cursor_continuation_budget(
    state: &StoreState,
    plan: &CursorContinuationPlan,
) -> Result<(), StoreError> {
    let _ = select_commit_ids_for_batch(state, plan)?;
    Ok(())
}

pub(crate) fn execute_cursor_continuation(
    state: &StoreState,
    plan: &CursorContinuationPlan,
) -> Result<ExecutedContinuationBatch, StoreError> {
    let witness = plan.witness();
    let basis = witness.stable_basis();
    let resume_plan = witness.resume_plan();
    let latest_checkpoint = resume_plan.latest_checkpoint();
    let identity = resume_plan.identity();
    let (frontier_commit_id, commit_ids) = select_commit_ids_for_batch(state, plan)?;

    if commit_ids.is_empty() {
        return Ok(ExecutedContinuationBatch::new(
            ContinuationBatchResult::CaughtUp(CaughtUpContinuationBatch::new(
                basis.stable_basis_id().clone(),
                basis.branch_id().clone(),
                frontier_commit_id,
                basis.read_scope().clone(),
                plan.strategy(),
            )),
            ContinuationExecutionMetrics {
                support_rows_read: 0,
                narrowed_item_count: 0,
                broadened_item_count: 0,
                step_count: 0,
            },
            Vec::new(),
        ));
    }

    let covered_commit_range = (
        *commit_ids.first().expect("non-empty continuation batch"),
        *commit_ids.last().expect("non-empty continuation batch"),
    );
    let batch_id = ContinuationBatchId::from_parts(
        basis,
        &identity.cursor_id,
        &identity.subscriber_id,
        covered_commit_range,
        basis.read_scope(),
        1,
    );
    let covered_commit_count = commit_ids.len() as u64;
    let support_rows_read = covered_commit_count;
    let scope_lookup_count = 1;

    match plan.strategy() {
        ContinuationStrategy::AdmittedLayoutNarrow => execute_admitted_batch(
            basis,
            identity,
            batch_id,
            commit_ids,
            covered_commit_range,
            latest_checkpoint.basis_commit_id,
            covered_commit_count,
            support_rows_read,
            scope_lookup_count,
        ),
        ContinuationStrategy::ExplicitBroadened => execute_broadened_batch(
            basis,
            batch_id,
            commit_ids,
            covered_commit_range,
            latest_checkpoint.basis_commit_id,
            covered_commit_count,
            support_rows_read,
            scope_lookup_count,
        ),
        ContinuationStrategy::AuthorityReplayControlLane => Ok(ExecutedContinuationBatch::new(
            ContinuationBatchResult::ControlLane(ControlLaneBatchReceipt::new(
                batch_id,
                covered_commit_range,
                commit_ids,
                latest_checkpoint.basis_commit_id,
                covered_commit_range.1,
                basis.read_scope().clone(),
                1,
                covered_commit_count,
                covered_commit_count,
                support_rows_read,
                scope_lookup_count,
                "authority_replay",
            )),
            ContinuationExecutionMetrics {
                support_rows_read,
                narrowed_item_count: 0,
                broadened_item_count: covered_commit_count,
                step_count: covered_commit_count,
            },
            vec![
                ContinuationExecutionEffect::Batch,
                ContinuationExecutionEffect::Broadening,
                ContinuationExecutionEffect::ControlLaneFallback,
            ],
        )),
    }
}
