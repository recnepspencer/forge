use crate::history::data::{BranchCreateError, BranchId};

use super::HistoryAuthority;

impl<'runtime> HistoryAuthority<'runtime> {
    /// Legacy in-crate adapter retained while callers migrate to the
    /// owner-issued fork basis API. It is deliberately not part of the
    /// public facade and cannot mint a branch from a raw head projection.
    /// Owner convenience for the fork-only transition. The raw ids are
    /// descriptive selectors; this method immediately observes an exact
    /// source basis and delegates to `RelationalRuntime::fork_branch`.
    pub(crate) fn fork_branch_from(
        &mut self,
        new_branch: BranchId,
        from_branch: &BranchId,
    ) -> Result<(), BranchCreateError> {
        let (_, basis) = self
            .runtime
            .observe_fork_source(from_branch)
            .map_err(|denial| match denial {
                crate::branch::RelationalForkDenial::DuplicateTarget => {
                    BranchCreateError::branch_already_exists()
                }
                crate::branch::RelationalForkDenial::SourceBranchMissing
                | crate::branch::RelationalForkDenial::EmptySource
                | crate::branch::RelationalForkDenial::ForeignRuntime
                | crate::branch::RelationalForkDenial::StaleSource
                | crate::branch::RelationalForkDenial::MissingArtifact
                | crate::branch::RelationalForkDenial::InvalidTarget(_)
                | crate::branch::RelationalForkDenial::Cell(_) => {
                    BranchCreateError::source_branch_missing()
                }
            })?;
        self.runtime
            .fork_branch(new_branch, basis)
            .map(|_| ())
            .map_err(|denial| match denial {
                crate::branch::RelationalForkDenial::DuplicateTarget => {
                    BranchCreateError::branch_already_exists()
                }
                _ => BranchCreateError::source_branch_missing(),
            })
    }
}
