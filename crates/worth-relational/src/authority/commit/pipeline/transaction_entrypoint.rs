use super::authority_context::AuthoritativeCommitContext;
use super::bulk_mutation_telemetry::summarize_bulk_mutation_telemetry;
use super::execution::execute_authoritative_commit;
use super::rejection::{attach_rejection, elapsed_micros};
use crate::authority::commit::phases::prepare::prepare_working_state_scope;
use crate::transactions::data::{
    CommitLog, CommitPhase, CommitPhaseTiming, CommitResult, TransactionCommitError,
};
use crate::transactions::RelationalTransaction;
use std::time::Instant;

impl<'a> RelationalTransaction<'a> {
    /// Executes the serialized truth-commit pipeline.
    ///
    /// The phases are intentionally explicit:
    /// 1. build a deterministic merged plan over the current immutable committed state
    /// 2. run commit-boundary invariants before any authoritative mutation
    /// 3. apply the authoritative plan into detached working state
    /// 4. run mutation-sensitive and publication invariants
    /// 5. assemble the canonical publication bundle and durable envelope
    /// 6. publish history/version visibility atomically into the runtime on success
    ///
    /// Any failure before publication discards the touched-partition overlay without making the
    /// commit visible.
    pub fn commit(mut self) -> Result<CommitResult, TransactionCommitError> {
        let draft_started = Instant::now();
        let mut draft_preparation_log = CommitLog::new();
        draft_preparation_log.begin_phase(CommitPhase::DraftPreparation);
        let prepared = prepare_working_state_scope(&mut self).map_err(|error| {
            attach_rejection(
                &mut draft_preparation_log,
                CommitPhase::DraftPreparation,
                error,
            )
        })?;
        let bulk_mutation_telemetry =
            summarize_bulk_mutation_telemetry(&prepared.merged_plan, self.batches.len());
        execute_authoritative_commit(
            self.runtime,
            AuthoritativeCommitContext::from_mutation(
                self.transaction_id,
                self.options,
                CommitPhaseTiming {
                    draft_preparation_micros: elapsed_micros(draft_started),
                    draft_merge_plan_micros: prepared.phase_timing.draft_merge_plan_micros,
                    draft_structural_summary_micros: prepared
                        .phase_timing
                        .draft_structural_summary_micros,
                    draft_working_state_clone_micros: prepared
                        .phase_timing
                        .draft_working_state_clone_micros,
                    ..CommitPhaseTiming::default()
                },
                prepared,
                bulk_mutation_telemetry,
            ),
        )
    }
}
