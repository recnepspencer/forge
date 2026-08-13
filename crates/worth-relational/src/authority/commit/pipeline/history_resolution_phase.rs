use super::rejection::{attach_rejection, elapsed_micros};
use crate::authority::commit::phases::history::{
    resolve_commit_history, resolve_commit_history_for_merge, ResolvedCommitHistory,
};
use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitLog, CommitPhase, CommitPhaseTiming, MergeCommitMutationPlan, TransactionCommitError,
    TransactionId, TransactionOptions,
};
use crate::transactions::RelationalTransaction;

pub(super) fn resolve_authoritative_history_phase(
    runtime: &mut RelationalRuntime,
    commit_log: &mut CommitLog,
    phase_timing: &mut CommitPhaseTiming,
    transaction_id: TransactionId,
    options: &TransactionOptions,
    version_id: VersionId,
    merge_history_plan: Option<&MergeCommitMutationPlan>,
) -> Result<ResolvedCommitHistory, TransactionCommitError> {
    commit_log.begin_phase(CommitPhase::HistoryResolution);
    let phase_started = std::time::Instant::now();
    let history = match merge_history_plan {
        None => {
            let mut transaction = RelationalTransaction {
                runtime,
                transaction_id,
                options: options.clone(),
                batches: Vec::new(),
                savepoints: Vec::new(),
                last_merged_plan: None,
            };
            resolve_commit_history(&mut transaction, version_id)
        }
        Some(plan) => resolve_commit_history_for_merge(runtime, options, plan, version_id),
    }
    .map_err(|error| attach_rejection(commit_log, CommitPhase::HistoryResolution, error))?;
    let history_summary = history.summary();
    commit_log.record_history_resolution(&history_summary);
    commit_log.complete_phase(CommitPhase::HistoryResolution);
    phase_timing.history_resolution_micros = elapsed_micros(phase_started);
    Ok(history)
}
