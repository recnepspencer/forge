use serde_json::json;

use crate::capabilities::{DiagnosticsSink, HistorySource, RuntimeConfigSource};
use crate::diagnostics::data::DiagnosticsScope;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::transactions::data::TransactionCommitError;
use crate::transactions::logic::RelationalTransaction;

pub(crate) struct ResolvedCommitHistory {
    pub(crate) commit_id: CommitId,
    pub(crate) branch_id: BranchId,
    pub(crate) previous_branch_head_version: Option<VersionId>,
    pub(crate) commit_reference: CommitReference,
    pub(crate) merge_base_commits: Vec<CommitId>,
}

pub(crate) fn resolve_commit_history(
    transaction: &mut RelationalTransaction<'_>,
    version_id: VersionId,
) -> Result<ResolvedCommitHistory, TransactionCommitError> {
    let commit_id = transaction.runtime.next_commit_id();
    let branch_id = transaction
        .options
        .target_branch
        .clone()
        .unwrap_or_else(|| transaction.runtime.runtime_config().main_branch.clone());
    let previous_branch_head_version = transaction
        .runtime
        .branch_head_ref(&branch_id)
        .map(|head| head.version_id);
    let (parents, merge_base_commits) = match transaction.resolve_parent_commits(&branch_id) {
        Ok(result) => result,
        Err(conflict) => {
            transaction.runtime.emit_diagnostic_entry(
                DiagnosticsScope::History,
                conflict.code,
                conflict.detail.clone(),
                json!({
                    "branch_id": branch_id.0,
                    "merge_parent_branches": transaction
                        .options
                        .merge_parent_branches
                        .iter()
                        .map(|branch| branch.0.clone())
                        .collect::<Vec<_>>(),
                }),
            );
            return Err(TransactionCommitError::Conflict(conflict));
        }
    };
    let commit_reference = CommitReference {
        commit_id,
        version_id,
        branch_id: branch_id.clone(),
        parents,
    };
    Ok(ResolvedCommitHistory {
        commit_id,
        branch_id,
        previous_branch_head_version,
        commit_reference,
        merge_base_commits,
    })
}
