use std::marker::PhantomData;

use worth_store_physical_backend::{BackendDurabilityProfile, BackendDurabilityProfileId};
use worth_store_physical_format::PageGenerationCell;

use crate::{DurableAckReceipt, LogSequenceNumber};

use super::{
    DirtyPublicationEvidence, PageLsn, PageLsnPublicationCounterSnapshot,
    UnadmittedDirtyPagePublicationDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalBeforeDataOrderingProof<P: BackendDurabilityProfile> {
    profile: PhantomData<P>,
    evidence: DirtyPublicationEvidence,
    wal_frontier: LogSequenceNumber,
    counters: PageLsnPublicationCounterSnapshot,
}

impl<P: BackendDurabilityProfile> WalBeforeDataOrderingProof<P> {
    pub fn prove(
        evidence: DirtyPublicationEvidence,
        durable_wal: &DurableAckReceipt<P>,
    ) -> Result<Self, UnadmittedDirtyPagePublicationDenial> {
        let ack_range = durable_wal.ack_basis().lsn_range();
        let wal_frontier = ack_range.end_exclusive();
        if !evidence.page_lsn().is_covered_by_wal_range(ack_range) {
            return Err(
                UnadmittedDirtyPagePublicationDenial::page_flush_before_wal_durability(
                    P::ID,
                    evidence.dirty_identity(),
                    evidence.page_generation(),
                    evidence.page_lsn(),
                    wal_frontier,
                    evidence.counters().with_wal_before_data_denial(),
                ),
            );
        }
        Ok(Self {
            profile: PhantomData,
            wal_frontier,
            counters: evidence.counters().with_wal_before_data_proof(),
            evidence,
        })
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        P::ID
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.evidence.page_generation()
    }

    pub const fn page_lsn(&self) -> PageLsn {
        self.evidence.page_lsn()
    }

    pub const fn wal_frontier(&self) -> LogSequenceNumber {
        self.wal_frontier
    }

    pub const fn counters(&self) -> PageLsnPublicationCounterSnapshot {
        self.counters
    }

    pub(crate) const fn evidence(&self) -> &DirtyPublicationEvidence {
        &self.evidence
    }
}
