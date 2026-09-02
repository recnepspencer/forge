use std::sync::Arc;

use super::CompositeRuntimeWorldCommit;

#[derive(Debug, Clone)]
pub(crate) struct CompositeHistoryTraversal {
    pub(super) commits: Vec<Arc<CompositeRuntimeWorldCommit>>,
    pub(super) next_parent: Option<crate::identity::CompositeCommitIdentity>,
}

impl CompositeHistoryTraversal {
    pub(crate) fn commits(&self) -> &[Arc<CompositeRuntimeWorldCommit>] {
        &self.commits
    }

    pub(crate) fn next_parent(&self) -> Option<&crate::identity::CompositeCommitIdentity> {
        self.next_parent.as_ref()
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.next_parent.is_none()
    }

    pub(crate) fn visited_count(&self) -> usize {
        self.commits.len()
    }
}
