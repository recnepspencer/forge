use std::sync::Arc;

use super::metadata::HistoryMetadataCharge;
use super::CompositeRuntimeWorldCommit;

#[derive(Debug, Clone)]
pub(crate) struct CompositeHistoryCatalogEntry {
    pub(super) commit: Arc<CompositeRuntimeWorldCommit>,
    pub(super) metadata_charge: HistoryMetadataCharge,
}

impl CompositeHistoryCatalogEntry {
    pub(crate) fn identity(&self) -> &crate::identity::CompositeCommitIdentity {
        self.commit.identity()
    }

    pub(crate) fn commit(&self) -> &CompositeRuntimeWorldCommit {
        self.commit.as_ref()
    }

    pub(super) const fn metadata_charge(&self) -> HistoryMetadataCharge {
        self.metadata_charge
    }
}
