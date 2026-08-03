use worth_store_physical_format::PageGenerationCell;

use super::{PageLsn, PageRedoCounterSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageRedoDenialKind {
    MismatchedPageGeneration,
    RedoBasisLsnMismatch,
    RedoCurrentPageLsnMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRedoDenial {
    kind: PageRedoDenialKind,
    expected_page: PageGenerationCell,
    observed_page: Option<PageGenerationCell>,
    expected_lsn: Option<PageLsn>,
    observed_lsn: Option<PageLsn>,
    counters: PageRedoCounterSnapshot,
}

impl PageRedoDenial {
    pub(super) const fn mismatched_page_generation(
        expected_page: PageGenerationCell,
        observed_page: PageGenerationCell,
        counters: PageRedoCounterSnapshot,
    ) -> Self {
        Self {
            kind: PageRedoDenialKind::MismatchedPageGeneration,
            expected_page,
            observed_page: Some(observed_page),
            expected_lsn: None,
            observed_lsn: None,
            counters,
        }
    }

    pub(super) const fn redo_basis_lsn_mismatch(
        expected_page: PageGenerationCell,
        expected_lsn: PageLsn,
        observed_lsn: PageLsn,
        counters: PageRedoCounterSnapshot,
    ) -> Self {
        Self {
            kind: PageRedoDenialKind::RedoBasisLsnMismatch,
            expected_page,
            observed_page: None,
            expected_lsn: Some(expected_lsn),
            observed_lsn: Some(observed_lsn),
            counters,
        }
    }

    pub(super) const fn redo_current_page_lsn_mismatch(
        expected_page: PageGenerationCell,
        expected_lsn: PageLsn,
        observed_lsn: PageLsn,
        counters: PageRedoCounterSnapshot,
    ) -> Self {
        Self {
            kind: PageRedoDenialKind::RedoCurrentPageLsnMismatch,
            expected_page,
            observed_page: None,
            expected_lsn: Some(expected_lsn),
            observed_lsn: Some(observed_lsn),
            counters,
        }
    }

    pub const fn kind(&self) -> PageRedoDenialKind {
        self.kind
    }

    pub const fn expected_page(&self) -> PageGenerationCell {
        self.expected_page
    }

    pub const fn observed_page(&self) -> Option<PageGenerationCell> {
        self.observed_page
    }

    pub const fn expected_lsn(&self) -> Option<PageLsn> {
        self.expected_lsn
    }

    pub const fn observed_lsn(&self) -> Option<PageLsn> {
        self.observed_lsn
    }

    pub const fn counters(&self) -> PageRedoCounterSnapshot {
        self.counters
    }
}
