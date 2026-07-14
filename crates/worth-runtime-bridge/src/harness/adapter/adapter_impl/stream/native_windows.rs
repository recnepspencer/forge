use super::*;
use crate::facade::TruthCommitIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::harness::adapter::adapter_impl) struct NativeStreamCommitWindow {
    commits: Vec<TruthCommitIdentity>,
}

impl NativeStreamCommitWindow {
    pub(in crate::harness::adapter::adapter_impl) fn from_commits(
        commits: impl IntoIterator<Item = TruthCommitIdentity>,
    ) -> Result<Self, BridgeHarnessError> {
        let commits = commits.into_iter().collect::<Vec<_>>();
        if commits.is_empty() {
            return Err(BridgeHarnessError::new(
                "stream harness targets require at least one commit identity",
            ));
        }
        Ok(Self { commits })
    }

    pub(super) fn commits(&self) -> &[TruthCommitIdentity] {
        &self.commits
    }
}

#[derive(Debug)]
pub(in crate::harness::adapter::adapter_impl) enum StreamHarnessTarget {
    RoutingWindow { window: NativeStreamCommitWindow },
    ReplayAuditWindow { window: NativeStreamCommitWindow },
}
