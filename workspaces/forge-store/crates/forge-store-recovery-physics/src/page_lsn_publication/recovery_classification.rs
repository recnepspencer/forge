use forge_store_physical_backend::BackendDurabilityProfileId;
use forge_store_physical_format::PageGenerationCell;

use super::{
    page_generation_match::same_page_generation, PageFlushRecoveryReceipt, PageLsn,
    PageLsnPublicationCounterSnapshot, UnadmittedDirtyPagePublicationDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StalePageRecoveryClassificationKind {
    Current,
    RedoRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReopenedPageRecoveryEvidence {
    page_generation: PageGenerationCell,
    page_lsn: Option<PageLsn>,
}

impl ReopenedPageRecoveryEvidence {
    pub const fn from_reopened_page(
        page_generation: PageGenerationCell,
        page_lsn: Option<PageLsn>,
    ) -> Self {
        Self {
            page_generation,
            page_lsn,
        }
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn page_lsn(&self) -> Option<PageLsn> {
        self.page_lsn
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StalePageRecoveryClassification {
    kind: StalePageRecoveryClassificationKind,
    profile_id: BackendDurabilityProfileId,
    page_generation: PageGenerationCell,
    reopened_page_lsn: PageLsn,
    redo_frontier: PageLsn,
    counters: PageLsnPublicationCounterSnapshot,
}

impl StalePageRecoveryClassification {
    pub fn classify_reopened_page(
        evidence: ReopenedPageRecoveryEvidence,
        flush_receipt: &PageFlushRecoveryReceipt,
    ) -> Result<Self, UnadmittedDirtyPagePublicationDenial> {
        let reopened_page = evidence.page_generation();
        if !same_page_generation(reopened_page, flush_receipt.page_generation()) {
            return Err(
                UnadmittedDirtyPagePublicationDenial::mismatched_page_generation(
                    flush_receipt.page_generation(),
                    reopened_page,
                    flush_receipt.counters().with_generation_mismatch_denial(),
                ),
            );
        }
        let reopened_page_lsn = evidence.page_lsn();
        let Some(reopened_page_lsn) = reopened_page_lsn else {
            return Err(UnadmittedDirtyPagePublicationDenial::missing_page_lsn(
                flush_receipt.page_generation(),
                flush_receipt
                    .counters()
                    .with_missing_page_lsn_classification(),
            ));
        };
        if reopened_page_lsn.is_at_or_beyond(flush_receipt.redo_frontier()) {
            return Ok(Self {
                kind: StalePageRecoveryClassificationKind::Current,
                profile_id: flush_receipt.profile_id(),
                page_generation: reopened_page,
                reopened_page_lsn,
                redo_frontier: flush_receipt.redo_frontier(),
                counters: flush_receipt.counters().with_current_page_redo_skip(),
            });
        }
        Ok(Self {
            kind: StalePageRecoveryClassificationKind::RedoRequired,
            profile_id: flush_receipt.profile_id(),
            page_generation: reopened_page,
            reopened_page_lsn,
            redo_frontier: flush_receipt.redo_frontier(),
            counters: flush_receipt.counters().with_stale_page_redo_required(),
        })
    }

    pub const fn kind(&self) -> StalePageRecoveryClassificationKind {
        self.kind
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.profile_id
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn reopened_page_lsn(&self) -> PageLsn {
        self.reopened_page_lsn
    }

    pub const fn redo_frontier(&self) -> PageLsn {
        self.redo_frontier
    }

    pub const fn counters(&self) -> PageLsnPublicationCounterSnapshot {
        self.counters
    }
}
