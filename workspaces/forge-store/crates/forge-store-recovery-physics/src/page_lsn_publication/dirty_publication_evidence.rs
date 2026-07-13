use forge_store_buffer_pool::{
    DirtyPageCounterSnapshot, DirtyPageIdentity, DirtyPublicationReceipt,
};
use forge_store_physical_format::PageGenerationCell;

use super::{PageLsn, PageLsnPublicationCounterSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtyPublicationEvidence {
    dirty_identity: DirtyPageIdentity,
    page_generation: PageGenerationCell,
    page_lsn: PageLsn,
    physical_substrate_dirty_counters: DirtyPageCounterSnapshot,
    counters: PageLsnPublicationCounterSnapshot,
}

impl DirtyPublicationEvidence {
    pub fn from_physical_substrate_publication(
        receipt: DirtyPublicationReceipt,
        page_lsn: PageLsn,
    ) -> Self {
        Self {
            dirty_identity: receipt.dirty_identity(),
            page_generation: receipt.page_generation(),
            page_lsn,
            physical_substrate_dirty_counters: receipt.counters(),
            counters: PageLsnPublicationCounterSnapshot::empty().with_dirty_publication_evidence(),
        }
    }

    pub const fn dirty_identity(&self) -> DirtyPageIdentity {
        self.dirty_identity
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn page_lsn(&self) -> PageLsn {
        self.page_lsn
    }

    pub const fn physical_substrate_dirty_counters(&self) -> DirtyPageCounterSnapshot {
        self.physical_substrate_dirty_counters
    }

    pub const fn counters(&self) -> PageLsnPublicationCounterSnapshot {
        self.counters
    }
}
