mod accounting;
mod boundary;
mod checkpoint;
mod diagnostics;
mod rollback;

use super::super::transaction_types::{SignalTransaction, TransactionOutcome, TransactionResult};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn finalize_semantic_delta(
        &mut self,
        restore_baseline: bool,
        outcome: TransactionOutcome,
        touched_nodes: u32,
        commit_nanos: u128,
    ) -> TransactionResult {
        let outcome = self.restore_baseline_if_requested(restore_baseline, outcome);
        let mut captured = self.capture_finalization_boundary(outcome, touched_nodes, commit_nanos);
        self.finalize_result_accounting(&mut captured.result);
        self.publish_finalization_diagnostics(captured)
    }
}
