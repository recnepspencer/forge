use worth_relational::facade::history::{BranchId, CommitId, CommitReference};
use worth_relational::facade::indexes::DerivedIndexBuildRequest;

use super::WorthQueryPrimaryGraphIntegrationHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPrimaryGraphIndexRefreshDenialKind {
    MissingCommittedMutation,
    IndexBuildRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPrimaryGraphIndexRefreshDenial {
    kind: WorthQueryPrimaryGraphIndexRefreshDenialKind,
    previous_commit_id: Option<CommitId>,
    committed_mutation_id: Option<CommitId>,
    committed_branch_id: Option<BranchId>,
    requested_index_count: usize,
    failed_index_count: usize,
}

impl WorthQueryPrimaryGraphIndexRefreshDenial {
    pub const fn kind(&self) -> WorthQueryPrimaryGraphIndexRefreshDenialKind {
        self.kind
    }

    pub const fn previous_commit_id(&self) -> Option<CommitId> {
        self.previous_commit_id
    }

    pub const fn committed_mutation_id(&self) -> Option<CommitId> {
        self.committed_mutation_id
    }

    pub fn committed_branch_id(&self) -> Option<&BranchId> {
        self.committed_branch_id.as_ref()
    }

    pub const fn requested_index_count(&self) -> usize {
        self.requested_index_count
    }

    pub const fn failed_index_count(&self) -> usize {
        self.failed_index_count
    }
}

impl std::fmt::Display for WorthQueryPrimaryGraphIndexRefreshDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "primary graph index refresh denied: {:?} (previous={:?}, committed={:?}, branch={:?}, requested={}, failed={})",
            self.kind,
            self.previous_commit_id,
            self.committed_mutation_id,
            self.committed_branch_id,
            self.requested_index_count,
            self.failed_index_count,
        )
    }
}

impl std::error::Error for WorthQueryPrimaryGraphIndexRefreshDenial {}

impl WorthQueryPrimaryGraphIntegrationHandle {
    /// Executes one ordinary mutation and synchronously refreshes every primary
    /// identity index when that mutation advances authoritative graph state.
    ///
    /// The outer result reports derived-index maintenance. The inner result is
    /// the caller's mutation outcome and is preserved when maintenance
    /// succeeds, including ordinary mutation rejection.
    #[doc(hidden)]
    pub fn execute_mutation_with_index_refresh<T, E>(
        &self,
        mutate: impl FnOnce(&mut worth_relational::facade::runtime::RelationalRuntime) -> Result<T, E>,
    ) -> Result<Result<T, E>, WorthQueryPrimaryGraphIndexRefreshDenial> {
        let mut runtime = self
            .runtime
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let branch = runtime.config().history.main_branch.clone();
        let previous = runtime.history().branch_head(&branch).cloned();
        let outcome = mutate(&mut runtime);
        let committed = runtime.history().branch_head(&branch).cloned();
        if previous == committed {
            return Ok(outcome);
        }
        let committed = committed.ok_or_else(|| {
            missing_committed_mutation(previous.as_ref(), self.primary_index_ids.len())
        })?;
        let build = runtime
            .index_authority()
            .build_for_commit(DerivedIndexBuildRequest {
                source_commit_id: committed.commit_id,
                branch_id: committed.branch_id.clone(),
                index_ids: self.primary_index_ids.to_vec(),
            });
        if !build.failed_indexes.is_empty()
            || build.generations.len() != self.primary_index_ids.len()
        {
            return Err(index_build_rejected(
                previous.as_ref(),
                &committed,
                self.primary_index_ids.len(),
                build.failed_indexes.len(),
            ));
        }
        Ok(outcome)
    }
}

fn missing_committed_mutation(
    previous: Option<&CommitReference>,
    requested_index_count: usize,
) -> WorthQueryPrimaryGraphIndexRefreshDenial {
    WorthQueryPrimaryGraphIndexRefreshDenial {
        kind: WorthQueryPrimaryGraphIndexRefreshDenialKind::MissingCommittedMutation,
        previous_commit_id: previous.map(|commit| commit.commit_id),
        committed_mutation_id: None,
        committed_branch_id: None,
        requested_index_count,
        failed_index_count: requested_index_count,
    }
}

fn index_build_rejected(
    previous: Option<&CommitReference>,
    committed: &CommitReference,
    requested_index_count: usize,
    failed_index_count: usize,
) -> WorthQueryPrimaryGraphIndexRefreshDenial {
    WorthQueryPrimaryGraphIndexRefreshDenial {
        kind: WorthQueryPrimaryGraphIndexRefreshDenialKind::IndexBuildRejected,
        previous_commit_id: previous.map(|commit| commit.commit_id),
        committed_mutation_id: Some(committed.commit_id),
        committed_branch_id: Some(committed.branch_id.clone()),
        requested_index_count,
        failed_index_count,
    }
}
