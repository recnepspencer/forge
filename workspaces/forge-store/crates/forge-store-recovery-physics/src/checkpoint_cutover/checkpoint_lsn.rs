use forge_store_physical_format::PageGenerationCell;

use crate::{LogSequenceNumber, PageLsn, WalLsnRange, WalTopologyDenial};

use super::{
    CheckpointRecoveryCounterSnapshot, CheckpointValidationDenial, CheckpointValidationDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointCoveredLsnRange {
    range: WalLsnRange,
}

impl CheckpointCoveredLsnRange {
    pub fn new(
        start: LogSequenceNumber,
        end_exclusive: LogSequenceNumber,
    ) -> Result<Self, WalTopologyDenial> {
        Ok(Self {
            range: WalLsnRange::new(start, end_exclusive)?,
        })
    }

    pub const fn from_wal_range(range: WalLsnRange) -> Self {
        Self { range }
    }

    pub const fn range(self) -> WalLsnRange {
        self.range
    }

    pub const fn contains(self, lsn: LogSequenceNumber) -> bool {
        self.range.contains(lsn)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointRedoBoundary {
    page_lsn: PageLsn,
}

impl CheckpointRedoBoundary {
    pub const fn from_page_lsn(page_lsn: PageLsn) -> Self {
        Self { page_lsn }
    }

    pub const fn page_lsn(self) -> PageLsn {
        self.page_lsn
    }

    pub const fn lsn(self) -> LogSequenceNumber {
        self.page_lsn.lsn()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPageLsnFrontier {
    pages: Vec<(PageGenerationCell, PageLsn)>,
}

impl CheckpointPageLsnFrontier {
    pub fn from_pages(
        pages: impl IntoIterator<Item = (PageGenerationCell, PageLsn)>,
    ) -> Result<Self, CheckpointValidationDenial> {
        let pages: Vec<_> = pages.into_iter().collect();
        if pages.is_empty() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::TornManifest,
                CheckpointRecoveryCounterSnapshot::new().with_manifest_validation(),
            ));
        }
        Ok(Self { pages })
    }

    pub(crate) fn require_covers_redo_boundary(
        &self,
        boundary: CheckpointRedoBoundary,
        counters: CheckpointRecoveryCounterSnapshot,
    ) -> Result<(), CheckpointValidationDenial> {
        for (_, page_lsn) in &self.pages {
            if !page_lsn.is_at_or_beyond(boundary.page_lsn()) {
                return Err(CheckpointValidationDenial::new(
                    CheckpointValidationDenialKind::StalePageLsnFrontier,
                    counters.with_manifest_validation(),
                )
                .with_page_lsn_pair(boundary.page_lsn(), *page_lsn));
            }
        }
        Ok(())
    }

    pub fn pages(&self) -> &[(PageGenerationCell, PageLsn)] {
        &self.pages
    }
}
