use std::sync::Arc;

use super::CompositeRuntimeWorldCommit;

#[derive(Debug, Clone)]
pub(crate) struct CompositeHistoryCatalogEntry {
    pub(super) commit: Arc<CompositeRuntimeWorldCommit>,
    pub(super) metadata_bytes: usize,
}

impl CompositeHistoryCatalogEntry {
    pub(crate) fn identity(&self) -> &crate::identity::CompositeCommitIdentity {
        self.commit.identity()
    }

    pub(crate) fn commit(&self) -> &CompositeRuntimeWorldCommit {
        self.commit.as_ref()
    }

    pub(crate) const fn metadata_bytes(&self) -> usize {
        self.metadata_bytes
    }
}
