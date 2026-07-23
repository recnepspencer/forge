#[cfg(not(feature = "legacy-certification-models"))]
use worth_store_buffer_pool::PhysicalFrameKey;
#[cfg(feature = "legacy-certification-models")]
use worth_store_buffer_pool::{
    DirtyPageCounterSnapshot, DirtyPageIdentity, DirtyPublicationReceipt,
};
#[cfg(not(feature = "legacy-certification-models"))]
use worth_store_physical_backend::{ArtifactRangeWriteDurability, CompletedArtifactRangeWrite};
use worth_store_physical_format::PageGenerationCell;

use super::{PageLsn, PageLsnPublicationCounterSnapshot};

#[cfg(feature = "legacy-certification-models")]
pub type RecoveryDirtyPageIdentity = DirtyPageIdentity;
#[cfg(not(feature = "legacy-certification-models"))]
pub type RecoveryDirtyPageIdentity = PhysicalFrameKey;

#[cfg(feature = "legacy-certification-models")]
pub type PhysicalDirtyPublicationCounters = DirtyPageCounterSnapshot;

#[cfg(not(feature = "legacy-certification-models"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalDirtyPublicationCounters {
    completed_bytes: u64,
    durability: ArtifactRangeWriteDurability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtyPublicationEvidence {
    dirty_identity: RecoveryDirtyPageIdentity,
    page_generation: PageGenerationCell,
    page_lsn: PageLsn,
    physical_substrate_dirty_counters: PhysicalDirtyPublicationCounters,
    counters: PageLsnPublicationCounterSnapshot,
}

impl DirtyPublicationEvidence {
    #[cfg(feature = "legacy-certification-models")]
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

    #[cfg(not(feature = "legacy-certification-models"))]
    pub fn from_completed_physical_write(
        receipt: &CompletedArtifactRangeWrite,
        page_generation: PageGenerationCell,
        page_lsn: PageLsn,
    ) -> Self {
        Self {
            dirty_identity: PhysicalFrameKey::new(receipt.store(), receipt.coordinate()),
            page_generation,
            page_lsn,
            physical_substrate_dirty_counters: PhysicalDirtyPublicationCounters {
                completed_bytes: receipt.completed_bytes(),
                durability: receipt.durability(),
            },
            counters: PageLsnPublicationCounterSnapshot::empty().with_dirty_publication_evidence(),
        }
    }

    pub const fn dirty_identity(&self) -> RecoveryDirtyPageIdentity {
        self.dirty_identity
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn page_lsn(&self) -> PageLsn {
        self.page_lsn
    }

    pub const fn physical_substrate_dirty_counters(&self) -> PhysicalDirtyPublicationCounters {
        self.physical_substrate_dirty_counters
    }

    pub const fn counters(&self) -> PageLsnPublicationCounterSnapshot {
        self.counters
    }
}

#[cfg(not(feature = "legacy-certification-models"))]
impl PhysicalDirtyPublicationCounters {
    pub const fn completed_bytes(self) -> u64 {
        self.completed_bytes
    }

    pub const fn durability(self) -> ArtifactRangeWriteDurability {
        self.durability
    }
}
