use worth_relational::facade::history::BranchId;

use crate::domain_computation::provider_session::WorthQueryMutationGraphWorkCompletion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCommitTerminalKind {
    Executed,
    Recovered,
}

/// Opaque, non-authoritative description of how an application commit became
/// observable to this caller.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::{
///     WorthQueryApplicationCommitTerminalEvidence,
///     WorthQueryApplicationCommitTerminalKind,
/// };
/// use worth_relational::facade::history::BranchId;
///
/// let forged = WorthQueryApplicationCommitTerminalEvidence {
///     kind: WorthQueryApplicationCommitTerminalKind::Recovered,
///     branch: BranchId("forged".to_owned()),
///     execution: None,
///     retry_inspection: None,
/// };
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryApplicationCommitTerminalEvidence {
    kind: WorthQueryApplicationCommitTerminalKind,
    branch: BranchId,
    execution: Option<WorthQueryMutationGraphWorkCompletion>,
    retry_inspection: Option<WorthQueryMutationGraphWorkCompletion>,
}

impl Eq for WorthQueryApplicationCommitTerminalEvidence {}

impl WorthQueryApplicationCommitTerminalEvidence {
    pub(super) fn executed(completion: WorthQueryMutationGraphWorkCompletion) -> Self {
        Self {
            kind: WorthQueryApplicationCommitTerminalKind::Executed,
            branch: completion.relational_branch().clone(),
            execution: Some(completion),
            retry_inspection: None,
        }
    }

    pub(super) const fn recovered(branch: BranchId) -> Self {
        Self {
            kind: WorthQueryApplicationCommitTerminalKind::Recovered,
            branch,
            execution: None,
            retry_inspection: None,
        }
    }

    pub(super) fn with_retry_inspection(
        mut self,
        completion: WorthQueryMutationGraphWorkCompletion,
    ) -> Option<Self> {
        if self.kind != WorthQueryApplicationCommitTerminalKind::Recovered
            || completion.relational_branch() != &self.branch
        {
            return None;
        }
        self.retry_inspection = Some(completion);
        Some(self)
    }

    pub const fn kind(&self) -> WorthQueryApplicationCommitTerminalKind {
        self.kind
    }

    pub const fn branch(&self) -> &BranchId {
        &self.branch
    }

    pub const fn execution(&self) -> Option<&WorthQueryMutationGraphWorkCompletion> {
        self.execution.as_ref()
    }

    pub const fn retry_inspection(&self) -> Option<&WorthQueryMutationGraphWorkCompletion> {
        self.retry_inspection.as_ref()
    }
}
