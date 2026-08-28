use crate::history::data::{BranchCreateError, BranchId};
use crate::history::HistoryAuthority;
use crate::runtime::RelationalRuntime;

use super::{
    AdmittedRelationalForkSourceBasis, RelationalForkDenial, RelationalForkOutcome,
    RelationalForkSourceDescriptor,
};

impl RelationalRuntime {
    pub fn fork_port(&self) -> crate::branch::RelationalForkPort {
        use crate::capabilities::VisibilityPolicySource;
        crate::branch::RelationalForkPort::new(
            self.runtime_instance_id(),
            self.owner_binding(),
            self.history.fork_binding().with_visibility(
                self.visibility.cache_binding(),
                self.protect_branch_heads(),
                self.services.instrumentation.clone(),
            ),
        )
    }

    pub fn observe_fork_source(
        &self,
        source_branch: &BranchId,
    ) -> Result<
        (
            RelationalForkSourceDescriptor,
            AdmittedRelationalForkSourceBasis,
        ),
        RelationalForkDenial,
    > {
        self.fork_port().observe_fork_source(source_branch)
    }

    pub fn fork_branch(
        &self,
        target_branch: BranchId,
        source: AdmittedRelationalForkSourceBasis,
    ) -> Result<RelationalForkOutcome, RelationalForkDenial> {
        self.fork_port().fork_branch(target_branch, source)
    }
}

impl HistoryAuthority<'_> {
    /// In-crate compatibility adapter for replay and preservation callers.
    /// It observes an owner fork token and delegates to `fork_branch`; it is
    /// not a public currentness door and is not a second fork authority.
    pub(crate) fn fork_branch_from(
        &mut self,
        new_branch: BranchId,
        from_branch: &BranchId,
    ) -> Result<(), BranchCreateError> {
        let runtime = self.runtime();
        let (_, basis) = runtime
            .observe_fork_source(from_branch)
            .map_err(|_| BranchCreateError::source_branch_missing())?;
        runtime
            .fork_branch(new_branch, basis)
            .map(|_| ())
            .map_err(|denial| match denial {
                RelationalForkDenial::DuplicateTarget => BranchCreateError::branch_already_exists(),
                _ => BranchCreateError::source_branch_missing(),
            })
    }
}
