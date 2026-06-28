use crate::LogSequenceNumber;
use forge_store_physical_format::PhysicalPageId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedRedoFrameReport {
    frame_lsn: LogSequenceNumber,
    target_page: PhysicalPageId,
    reason: SkippedRedoFrameReason,
}

impl SkippedRedoFrameReport {
    pub(crate) const fn already_current_page(
        frame_lsn: LogSequenceNumber,
        target_page: PhysicalPageId,
    ) -> Self {
        Self {
            frame_lsn,
            target_page,
            reason: SkippedRedoFrameReason::AlreadyCurrentPageLsn,
        }
    }

    pub const fn frame_lsn(&self) -> LogSequenceNumber {
        self.frame_lsn
    }

    pub const fn target_page(&self) -> PhysicalPageId {
        self.target_page
    }

    pub const fn reason(&self) -> SkippedRedoFrameReason {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkippedRedoFrameReason {
    AlreadyCurrentPageLsn,
}
