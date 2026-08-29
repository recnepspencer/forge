use crate::history::data::{BranchCreateError, BranchId};
use crate::history::HistoryAuthority;
use crate::runtime::RelationalRuntime;

use super::{
    AdmittedRelationalForkSourceBasis, RelationalForkDenial, RelationalForkOutcome,
    RelationalForkSourceDescriptor,
};

impl RelationalRuntime {
    /// The independently borrowable fork service for this runtime.
    ///
    /// Fork is a separate owner transition, not a variant of publication. The
    /// service owns two operations:
    /// [`RelationalForkPort::observe_fork_source`](crate::branch::RelationalForkPort::observe_fork_source),
    /// which issues a linear fork-only source token, and
    /// [`RelationalForkPort::fork_branch`](crate::branch::RelationalForkPort::fork_branch),
    /// which consumes that token exactly once to create a fresh reference cell
    /// sharing the exact immutable source root. An empty branch produces no
    /// fork source, so a branch must have committed before it can be forked.
    ///
    /// Like
    /// [`RelationalRuntime::preparation_port`](crate::runtime::RelationalRuntime::preparation_port),
    /// this service is obtained from a shared borrow and is
    /// `Clone + Send + Sync`. The runnable owner workflow is
    /// `examples/branch_local_mvcc.rs`.
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
