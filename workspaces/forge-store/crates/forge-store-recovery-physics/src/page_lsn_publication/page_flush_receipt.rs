use forge_store_physical_backend::{BackendDurabilityProfile, BackendDurabilityProfileId};
use forge_store_physical_format::PageGenerationCell;

use crate::LogSequenceNumber;

use super::{
    NoUndoPublicationEligibility, PageLsn, PageLsnPublicationCounterSnapshot,
    RollbackImagePublicationDeclaration, RollbackImagePublicationPosture,
    UnadmittedDirtyPagePublicationDenial, WalBeforeDataOrderingProof,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageFlushRecoveryReceipt {
    profile_id: BackendDurabilityProfileId,
    page_generation: PageGenerationCell,
    page_lsn: PageLsn,
    wal_frontier: LogSequenceNumber,
    rollback_posture: RollbackImagePublicationPosture,
    counters: PageLsnPublicationCounterSnapshot,
}

impl PageFlushRecoveryReceipt {
    pub fn publish(eligibility: NoUndoPublicationEligibility) -> Self {
        Self {
            profile_id: eligibility.profile_id(),
            page_generation: eligibility.page_generation(),
            page_lsn: eligibility.page_lsn(),
            wal_frontier: eligibility.wal_frontier(),
            rollback_posture: eligibility.rollback_posture(),
            counters: eligibility.counters().with_page_flush_receipt(),
        }
    }

    pub fn publish_admitted_redo_only<P: BackendDurabilityProfile>(
        ordering: WalBeforeDataOrderingProof<P>,
    ) -> Self {
        Self::publish(NoUndoPublicationEligibility::admitted_dirty_publication(
            ordering,
        ))
    }

    pub fn publish_rollback_image_protected<P: BackendDurabilityProfile>(
        ordering: WalBeforeDataOrderingProof<P>,
        declaration: RollbackImagePublicationDeclaration,
    ) -> Result<Self, UnadmittedDirtyPagePublicationDenial> {
        Ok(Self::publish(
            NoUndoPublicationEligibility::rollback_image_protected(ordering, declaration)?,
        ))
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.profile_id
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn page_lsn(&self) -> PageLsn {
        self.page_lsn
    }

    pub const fn redo_frontier(&self) -> PageLsn {
        self.page_lsn
    }

    pub const fn wal_frontier(&self) -> LogSequenceNumber {
        self.wal_frontier
    }

    pub const fn rollback_posture(&self) -> RollbackImagePublicationPosture {
        self.rollback_posture
    }

    pub const fn counters(&self) -> PageLsnPublicationCounterSnapshot {
        self.counters
    }
}
