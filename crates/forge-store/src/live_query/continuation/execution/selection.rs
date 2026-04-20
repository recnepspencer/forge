use crate::backend::records::StoreState;
use crate::failure::{StoreError, StoreErrorKind};
use forge_relational::facade::history::CommitId;

use crate::live_query::continuation::budget::continuation_batch_limit;
use crate::live_query::continuation::plan::CursorContinuationPlan;

fn commit_materialized_bytes(state: &StoreState, commit_id: CommitId) -> Result<u64, StoreError> {
    let record = state.commit_record(commit_id).ok_or_else(|| {
        StoreError::new(
            StoreErrorKind::CommitNotFound,
            format!("continuation batch budget could not find commit {}", commit_id.0),
        )
    })?;
    serde_json::to_vec(&record.envelope)
        .map(|bytes| bytes.len() as u64)
        .map_err(|error| {
            StoreError::new(
                StoreErrorKind::Serialization,
                format!(
                    "continuation batch budget failed to measure commit {} payload bytes: {error}",
                    commit_id.0
                ),
            )
        })
}

pub(crate) fn select_commit_ids_for_batch(
    state: &StoreState,
    plan: &CursorContinuationPlan,
) -> Result<(CommitId, Vec<CommitId>), StoreError> {
    let witness = plan.witness();
    let basis = witness.stable_basis();
    let resume_plan = witness.resume_plan();
    let latest_checkpoint = resume_plan.latest_checkpoint();
    let batch_limit = continuation_batch_limit(plan.batch_budget())?;
    let max_materialized_bytes = plan.batch_budget().max_materialized_bytes().get();

    let frontier_commit_id = state
        .branch_head_record(basis.branch_id())
        .and_then(|record| record.head_commit_id)
        .unwrap_or(latest_checkpoint.basis_commit_id);
    let latest_sequence = state
        .commit_record(latest_checkpoint.basis_commit_id)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::ContinuationCursorIncompatibility,
                format!(
                    "durable continuation checkpoint basis commit {} is missing",
                    latest_checkpoint.basis_commit_id.0
                ),
            )
        })?
        .commit_sequence;
    let frontier_sequence = state
        .commit_record(frontier_commit_id)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::ContinuationCursorIncompatibility,
                format!(
                    "durable continuation frontier commit {} is missing",
                    frontier_commit_id.0
                ),
            )
        })?
        .commit_sequence;

    let mut selected_commit_ids = Vec::new();
    let mut materialized_bytes = 0_u64;
    for (_, commit_id) in state
        .branch_commit_sequences(basis.branch_id())
        .into_iter()
        .filter(|(sequence, _)| *sequence > latest_sequence && *sequence <= frontier_sequence)
    {
        if selected_commit_ids.len() >= batch_limit {
            break;
        }
        let commit_bytes = commit_materialized_bytes(state, commit_id)?;
        if commit_bytes > max_materialized_bytes && selected_commit_ids.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::ContinuationBudgetExceeded,
                format!(
                    "continuation batch budget cannot admit commit {} within max_materialized_bytes={}",
                    commit_id.0, max_materialized_bytes
                ),
            ));
        }
        if materialized_bytes + commit_bytes > max_materialized_bytes {
            break;
        }
        materialized_bytes += commit_bytes;
        selected_commit_ids.push(commit_id);
    }

    Ok((frontier_commit_id, selected_commit_ids))
}
