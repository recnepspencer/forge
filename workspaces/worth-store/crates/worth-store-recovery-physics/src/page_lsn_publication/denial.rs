use worth_store_physical_backend::BackendDurabilityProfileId;
use worth_store_physical_format::{PageGenerationCell, PhysicalGeneration, PhysicalPageId};

use crate::LogSequenceNumber;

use super::{PageLsn, PageLsnPublicationCounterSnapshot, RecoveryDirtyPageIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnadmittedDirtyPagePublicationDenialKind {
    MissingPageLsn,
    PageFlushBeforeWalDurability,
    StalePageLsnRequiresRedo,
    MismatchedPageGeneration,
    UnadmittedDirtyBytes,
    RollbackImageRequired,
    RollbackImageDeclarationMismatch,
    DirtyPublicationDoesNotProveDurability,
    RedoBasisLsnMismatch,
    RedoCurrentPageLsnMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnadmittedDirtyPagePublicationDenial {
    kind: UnadmittedDirtyPagePublicationDenialKind,
    profile_id: Option<BackendDurabilityProfileId>,
    dirty_identity: Option<RecoveryDirtyPageIdentity>,
    expected_page: Option<PageGenerationCell>,
    observed_page: Option<PageGenerationCell>,
    page_id: Option<PhysicalPageId>,
    expected_generation: Option<PhysicalGeneration>,
    observed_generation: Option<PhysicalGeneration>,
    page_lsn: Option<PageLsn>,
    wal_frontier: Option<LogSequenceNumber>,
    counters: PageLsnPublicationCounterSnapshot,
}

impl UnadmittedDirtyPagePublicationDenial {
    pub(crate) const fn new(
        kind: UnadmittedDirtyPagePublicationDenialKind,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            profile_id: None,
            dirty_identity: None,
            expected_page: None,
            observed_page: None,
            page_id: None,
            expected_generation: None,
            observed_generation: None,
            page_lsn: None,
            wal_frontier: None,
            counters,
        }
    }

    pub(crate) const fn page_flush_before_wal_durability(
        profile_id: BackendDurabilityProfileId,
        dirty_identity: RecoveryDirtyPageIdentity,
        page: PageGenerationCell,
        page_lsn: PageLsn,
        wal_frontier: LogSequenceNumber,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind: UnadmittedDirtyPagePublicationDenialKind::PageFlushBeforeWalDurability,
            profile_id: Some(profile_id),
            dirty_identity: Some(dirty_identity),
            expected_page: Some(page),
            observed_page: None,
            page_id: Some(page.page_id()),
            expected_generation: Some(page.generation()),
            observed_generation: None,
            page_lsn: Some(page_lsn),
            wal_frontier: Some(wal_frontier),
            counters,
        }
    }

    pub(crate) const fn rollback_image_required(
        profile_id: BackendDurabilityProfileId,
        dirty_identity: RecoveryDirtyPageIdentity,
        page: PageGenerationCell,
        page_lsn: PageLsn,
        wal_frontier: LogSequenceNumber,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind: UnadmittedDirtyPagePublicationDenialKind::RollbackImageRequired,
            profile_id: Some(profile_id),
            dirty_identity: Some(dirty_identity),
            expected_page: Some(page),
            observed_page: None,
            page_id: Some(page.page_id()),
            expected_generation: Some(page.generation()),
            observed_generation: None,
            page_lsn: Some(page_lsn),
            wal_frontier: Some(wal_frontier),
            counters,
        }
    }

    pub(crate) const fn rollback_image_declaration_mismatch(
        profile_id: BackendDurabilityProfileId,
        dirty_identity: RecoveryDirtyPageIdentity,
        expected_page: PageGenerationCell,
        observed_page: PageGenerationCell,
        page_lsn: PageLsn,
        wal_frontier: LogSequenceNumber,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind: UnadmittedDirtyPagePublicationDenialKind::RollbackImageDeclarationMismatch,
            profile_id: Some(profile_id),
            dirty_identity: Some(dirty_identity),
            expected_page: Some(expected_page),
            observed_page: Some(observed_page),
            page_id: Some(expected_page.page_id()),
            expected_generation: Some(expected_page.generation()),
            observed_generation: Some(observed_page.generation()),
            page_lsn: Some(page_lsn),
            wal_frontier: Some(wal_frontier),
            counters,
        }
    }

    pub(crate) const fn missing_page_lsn(
        expected_page: PageGenerationCell,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind: UnadmittedDirtyPagePublicationDenialKind::MissingPageLsn,
            expected_page: Some(expected_page),
            page_id: Some(expected_page.page_id()),
            expected_generation: Some(expected_page.generation()),
            counters,
            ..Self::new(
                UnadmittedDirtyPagePublicationDenialKind::MissingPageLsn,
                counters,
            )
        }
    }

    pub(crate) const fn mismatched_page_generation(
        expected_page: PageGenerationCell,
        observed_page: PageGenerationCell,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind: UnadmittedDirtyPagePublicationDenialKind::MismatchedPageGeneration,
            expected_page: Some(expected_page),
            observed_page: Some(observed_page),
            page_id: Some(expected_page.page_id()),
            expected_generation: Some(expected_page.generation()),
            observed_generation: Some(observed_page.generation()),
            counters,
            ..Self::new(
                UnadmittedDirtyPagePublicationDenialKind::MismatchedPageGeneration,
                counters,
            )
        }
    }

    pub(crate) const fn redo_basis_lsn_mismatch(
        expected_page: PageGenerationCell,
        expected_lsn: PageLsn,
        observed_lsn: PageLsn,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind: UnadmittedDirtyPagePublicationDenialKind::RedoBasisLsnMismatch,
            expected_page: Some(expected_page),
            page_id: Some(expected_page.page_id()),
            expected_generation: Some(expected_page.generation()),
            page_lsn: Some(observed_lsn),
            wal_frontier: Some(expected_lsn.lsn()),
            counters,
            ..Self::new(
                UnadmittedDirtyPagePublicationDenialKind::RedoBasisLsnMismatch,
                counters,
            )
        }
    }

    pub(crate) const fn redo_current_page_lsn_mismatch(
        expected_page: PageGenerationCell,
        expected_lsn: PageLsn,
        observed_lsn: PageLsn,
        counters: PageLsnPublicationCounterSnapshot,
    ) -> Self {
        Self {
            kind: UnadmittedDirtyPagePublicationDenialKind::RedoCurrentPageLsnMismatch,
            expected_page: Some(expected_page),
            page_id: Some(expected_page.page_id()),
            expected_generation: Some(expected_page.generation()),
            page_lsn: Some(observed_lsn),
            wal_frontier: Some(expected_lsn.lsn()),
            counters,
            ..Self::new(
                UnadmittedDirtyPagePublicationDenialKind::RedoCurrentPageLsnMismatch,
                counters,
            )
        }
    }

    pub const fn kind(&self) -> UnadmittedDirtyPagePublicationDenialKind {
        self.kind
    }

    pub const fn profile_id(&self) -> Option<BackendDurabilityProfileId> {
        self.profile_id
    }

    pub const fn dirty_identity(&self) -> Option<RecoveryDirtyPageIdentity> {
        self.dirty_identity
    }

    pub const fn expected_page(&self) -> Option<PageGenerationCell> {
        self.expected_page
    }

    pub const fn observed_page(&self) -> Option<PageGenerationCell> {
        self.observed_page
    }

    pub const fn page_id(&self) -> Option<PhysicalPageId> {
        self.page_id
    }

    pub const fn expected_generation(&self) -> Option<PhysicalGeneration> {
        self.expected_generation
    }

    pub const fn observed_generation(&self) -> Option<PhysicalGeneration> {
        self.observed_generation
    }

    pub const fn page_lsn(&self) -> Option<PageLsn> {
        self.page_lsn
    }

    pub const fn wal_frontier(&self) -> Option<LogSequenceNumber> {
        self.wal_frontier
    }

    pub const fn counters(&self) -> PageLsnPublicationCounterSnapshot {
        self.counters
    }
}
