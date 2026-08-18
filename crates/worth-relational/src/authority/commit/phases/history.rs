use crate::branch::RelationalLegacyBranchBinding;
use crate::capabilities::DiagnosticArtifactSink;
use crate::diagnostics::data::{
    DiagnosticsScope, RelationalDiagnosticFields, RelationalDiagnosticValue,
};
use crate::history::data::{BranchId, CommitId, RelationalCommitReceipt};
use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitHistorySummary, MergeCommitMutationPlan, TransactionCommitError, TransactionOptions,
};
use crate::transactions::RelationalTransaction;

pub(crate) struct ResolvedCommitHistory {
    pub(crate) commit_id: CommitId,
    pub(crate) branch_id: BranchId,
    pub(crate) previous_branch_head_version: Option<VersionId>,
    pub(crate) commit_reference: RelationalCommitReceipt,
    pub(crate) merge_base_commits: Vec<CommitId>,
    pub(crate) requested_merge_parent_count: usize,
    pub(crate) effective_merge_parent_count: usize,
    pub(crate) branch_binding: RelationalLegacyBranchBinding,
}

impl ResolvedCommitHistory {
    pub(crate) fn summary(&self) -> CommitHistorySummary {
        CommitHistorySummary {
            target_branch: self.branch_binding.identity().branch_id().0.clone(),
            requested_merge_parent_count: self.requested_merge_parent_count,
            effective_merge_parent_count: self.effective_merge_parent_count,
            parent_count: self.commit_reference.parents.len(),
            merge_base_count: self.merge_base_commits.len(),
            had_previous_branch_head: self.previous_branch_head_version.is_some(),
        }
    }
}

pub(crate) fn resolve_commit_history(
    transaction: &mut RelationalTransaction<'_>,
    version_id: VersionId,
) -> Result<ResolvedCommitHistory, TransactionCommitError> {
    let commit_id = transaction.runtime.history().next_commit_id();
    let branch_binding = transaction.options.branch_binding().clone();
    let branch_id = branch_binding.identity().branch_id().clone();
    if !transaction
        .runtime
        .legacy_branch_binding_is_current(&branch_binding)
    {
        return Err(TransactionCommitError::conflict(
            crate::transactions::data::CommitConflict::new(
                crate::transactions::data::ConflictClass::StaleValidationBasis {
                    detail: "owner-issued branch binding is foreign or no longer current"
                        .to_owned(),
                },
            ),
        ));
    }
    let previous_branch_head_version = transaction
        .runtime
        .legacy_branch_binding_commit(&branch_binding)
        .map(|head| head.version_id());
    let requested_merge_parent_count = transaction.options.merge_parent_bindings().len();
    let (parents, merge_base_commits) = match transaction.resolve_parent_commits(&branch_id) {
        Ok(result) => result,
        Err(conflict) => {
            transaction.runtime.emit_failure_diagnostic(
                DiagnosticsScope::History,
                conflict.code,
                conflict.detail.clone(),
                merge_parent_resolution_failure_fields(
                    &branch_id,
                    &transaction.options.merge_parent_branch_ids(),
                ),
            );
            return Err(TransactionCommitError::conflict(conflict));
        }
    };
    let effective_merge_parent_count = parents
        .len()
        .saturating_sub(usize::from(previous_branch_head_version.is_some()));
    let commit_reference = RelationalCommitReceipt {
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
        requested_merge_parent_count,
        effective_merge_parent_count,
        branch_binding,
    })
}

pub(crate) fn resolve_commit_history_for_merge(
    runtime: &mut RelationalRuntime,
    options: &TransactionOptions,
    merge_plan: &MergeCommitMutationPlan,
    version_id: VersionId,
) -> Result<ResolvedCommitHistory, TransactionCommitError> {
    resolve_commit_history_for_runtime(
        runtime,
        options,
        Some(merge_plan),
        version_id,
        |_branch_id| {
            Ok((
                merge_plan.parent_commits.clone_inner(),
                merge_plan.merge_base_commits.iter().copied().collect(),
            ))
        },
    )
}

fn resolve_commit_history_for_runtime<F>(
    runtime: &mut RelationalRuntime,
    options: &TransactionOptions,
    merge_plan: Option<&MergeCommitMutationPlan>,
    version_id: VersionId,
    resolve_parents: F,
) -> Result<ResolvedCommitHistory, TransactionCommitError>
where
    F: FnOnce(
        &BranchId,
    )
        -> Result<(Vec<CommitId>, Vec<CommitId>), crate::transactions::data::CommitConflict>,
{
    let commit_id = runtime.history().next_commit_id();
    let branch_id = merge_plan
        .map(|plan| plan.target_branch.clone())
        .unwrap_or_else(|| options.target_branch().clone());
    let requested_merge_parent_count = merge_plan
        .map(|plan| plan.requested_merge_parent_count)
        .unwrap_or(options.merge_parent_bindings().len());
    let merge_parent_branches = merge_plan
        .map(|plan| plan.merge_parent_branches.as_ref().to_vec())
        .unwrap_or_else(|| options.merge_parent_branch_ids());
    let branch_binding = options.branch_binding().clone();
    if branch_binding.identity().branch_id() != &branch_id {
        return Err(TransactionCommitError::conflict(
            crate::transactions::data::CommitConflict::new(
                crate::transactions::data::ConflictClass::InvalidMergeParent {
                    detail: format!(
                        "owner-issued branch binding does not match target branch: {}",
                        branch_id.0
                    ),
                },
            ),
        ));
    }
    if !runtime.legacy_branch_binding_is_current(&branch_binding) {
        return Err(TransactionCommitError::conflict(
            crate::transactions::data::CommitConflict::new(
                crate::transactions::data::ConflictClass::StaleValidationBasis {
                    detail: "owner-issued branch binding is foreign or no longer current"
                        .to_owned(),
                },
            ),
        ));
    }
    let previous_branch_head_version = runtime
        .legacy_branch_binding_commit(&branch_binding)
        .map(|head| head.version_id());
    let (parents, merge_base_commits) = match resolve_parents(&branch_id) {
        Ok(result) => result,
        Err(conflict) => {
            runtime.emit_failure_diagnostic(
                DiagnosticsScope::History,
                conflict.code,
                conflict.detail.clone(),
                merge_parent_resolution_failure_fields(&branch_id, &merge_parent_branches),
            );
            return Err(TransactionCommitError::conflict(conflict));
        }
    };
    let effective_merge_parent_count = parents
        .len()
        .saturating_sub(usize::from(previous_branch_head_version.is_some()));
    let commit_reference = RelationalCommitReceipt {
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
        requested_merge_parent_count,
        effective_merge_parent_count,
        branch_binding,
    })
}

fn merge_parent_resolution_failure_fields(
    branch_id: &BranchId,
    merge_parent_branches: &[BranchId],
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "branch_id",
            RelationalDiagnosticValue::string(branch_id.0.clone()),
        ),
        (
            "merge_parent_branches",
            RelationalDiagnosticValue::array(
                merge_parent_branches
                    .iter()
                    .map(|branch| RelationalDiagnosticValue::string(branch.0.clone())),
            ),
        ),
    ])
    .into()
}
