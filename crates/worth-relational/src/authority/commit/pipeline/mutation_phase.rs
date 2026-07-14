use super::rejection::{attach_rejection, elapsed_micros};
use crate::authority::commit::phases::mutation::{
    run_authoritative_mutation_for_runtime, MutationPhaseOutput,
};
use crate::history::data::BranchId;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitLog, CommitPhase, CommitPhaseTiming, MergedCommitPlan, TransactionCommitError,
    TransactionId,
};

pub(super) fn run_authoritative_mutation_phase(
    runtime: &mut RelationalRuntime,
    commit_log: &mut CommitLog,
    phase_timing: &mut CommitPhaseTiming,
    transaction_id: TransactionId,
    working_state: &mut crate::storage::overlay::WorkingState,
    merged_plan: &MergedCommitPlan,
    target_branch: Option<&BranchId>,
) -> Result<MutationPhaseOutput, TransactionCommitError> {
    commit_log.begin_phase(CommitPhase::AuthoritativeMutation);
    let phase_started = std::time::Instant::now();
    let mutation = run_authoritative_mutation_for_runtime(
        runtime,
        transaction_id,
        working_state,
        merged_plan,
        target_branch,
    )
    .map_err(|error| attach_rejection(commit_log, CommitPhase::AuthoritativeMutation, error))?;
    commit_log.record_invariant_outcomes(&mutation.invariant_results);
    commit_log.complete_phase(CommitPhase::AuthoritativeMutation);
    phase_timing.authoritative_mutation_micros = elapsed_micros(phase_started);
    Ok(mutation)
}
