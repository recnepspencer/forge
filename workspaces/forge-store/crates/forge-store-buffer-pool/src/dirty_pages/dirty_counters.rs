use crate::{DirtyByteCount, DirtyPageCount};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirtyPageCounterSnapshot {
    dirty_pages: DirtyPageCount,
    dirty_bytes: DirtyByteCount,
    scheduled_not_durable_pages: DirtyPageCount,
    scheduled_not_durable_bytes: DirtyByteCount,
    scheduled_dirty_overlap_pages: DirtyPageCount,
    scheduled_dirty_overlap_bytes: DirtyByteCount,
    dirty_mark_attempt_count: u64,
    newly_dirty_count: u64,
    already_dirty_count: u64,
    dirty_budget_denial_count: u64,
    publication_plan_attempt_count: u64,
    publication_plan_denial_count: u64,
    publication_receipt_count: u64,
    write_scheduling_attempt_count: u64,
    write_scheduling_denial_count: u64,
    stale_publication_plan_denial_count: u64,
    dirty_shutdown_unflushed_count: u64,
    dirty_discard_denial_count: u64,
}

impl DirtyPageCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            dirty_pages: DirtyPageCount::from_observed_pages(0),
            dirty_bytes: DirtyByteCount::from_observed_bytes(0),
            scheduled_not_durable_pages: DirtyPageCount::from_observed_pages(0),
            scheduled_not_durable_bytes: DirtyByteCount::from_observed_bytes(0),
            scheduled_dirty_overlap_pages: DirtyPageCount::from_observed_pages(0),
            scheduled_dirty_overlap_bytes: DirtyByteCount::from_observed_bytes(0),
            dirty_mark_attempt_count: 0,
            newly_dirty_count: 0,
            already_dirty_count: 0,
            dirty_budget_denial_count: 0,
            publication_plan_attempt_count: 0,
            publication_plan_denial_count: 0,
            publication_receipt_count: 0,
            write_scheduling_attempt_count: 0,
            write_scheduling_denial_count: 0,
            stale_publication_plan_denial_count: 0,
            dirty_shutdown_unflushed_count: 0,
            dirty_discard_denial_count: 0,
        }
    }

    pub(crate) fn with_newly_dirty_behind_scheduled_write(self, frame_bytes: u64) -> Self {
        Self {
            dirty_pages: DirtyPageCount::from_observed_pages(self.dirty_pages.as_pages() + 1),
            dirty_bytes: DirtyByteCount::from_observed_bytes(
                self.dirty_bytes.as_bytes() + frame_bytes,
            ),
            scheduled_dirty_overlap_pages: DirtyPageCount::from_observed_pages(
                self.scheduled_dirty_overlap_pages.as_pages() + 1,
            ),
            scheduled_dirty_overlap_bytes: DirtyByteCount::from_observed_bytes(
                self.scheduled_dirty_overlap_bytes.as_bytes() + frame_bytes,
            ),
            newly_dirty_count: self.newly_dirty_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_dirty_mark_attempt(self) -> Self {
        Self {
            dirty_mark_attempt_count: self.dirty_mark_attempt_count + 1,
            ..self
        }
    }

    pub(crate) fn with_newly_dirty(self, frame_bytes: u64) -> Self {
        Self {
            dirty_pages: DirtyPageCount::from_observed_pages(self.dirty_pages.as_pages() + 1),
            dirty_bytes: DirtyByteCount::from_observed_bytes(
                self.dirty_bytes.as_bytes() + frame_bytes,
            ),
            newly_dirty_count: self.newly_dirty_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_already_dirty(self) -> Self {
        Self {
            already_dirty_count: self.already_dirty_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_dirty_budget_denial(self) -> Self {
        Self {
            dirty_budget_denial_count: self.dirty_budget_denial_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_publication_plan_attempt(self) -> Self {
        Self {
            publication_plan_attempt_count: self.publication_plan_attempt_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_publication_plan_denial(self) -> Self {
        Self {
            publication_plan_denial_count: self.publication_plan_denial_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_write_scheduling_denial(self) -> Self {
        Self {
            write_scheduling_denial_count: self.write_scheduling_denial_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_stale_publication_plan_denial(self) -> Self {
        Self {
            stale_publication_plan_denial_count: self.stale_publication_plan_denial_count + 1,
            ..self
        }
    }

    pub(crate) fn with_first_publication_receipt(self, frame_bytes: u64) -> Self {
        Self {
            dirty_pages: DirtyPageCount::from_observed_pages(self.dirty_pages.as_pages() - 1),
            dirty_bytes: DirtyByteCount::from_observed_bytes(
                self.dirty_bytes.as_bytes() - frame_bytes,
            ),
            scheduled_not_durable_pages: DirtyPageCount::from_observed_pages(
                self.scheduled_not_durable_pages.as_pages() + 1,
            ),
            scheduled_not_durable_bytes: DirtyByteCount::from_observed_bytes(
                self.scheduled_not_durable_bytes.as_bytes() + frame_bytes,
            ),
            publication_receipt_count: self.publication_receipt_count + 1,
            write_scheduling_attempt_count: self.write_scheduling_attempt_count + 1,
            ..self
        }
    }

    pub(crate) fn with_additional_publication_receipt(self, frame_bytes: u64) -> Self {
        Self {
            dirty_pages: DirtyPageCount::from_observed_pages(self.dirty_pages.as_pages() - 1),
            dirty_bytes: DirtyByteCount::from_observed_bytes(
                self.dirty_bytes.as_bytes() - frame_bytes,
            ),
            scheduled_dirty_overlap_pages: DirtyPageCount::from_observed_pages(
                self.scheduled_dirty_overlap_pages.as_pages() - 1,
            ),
            scheduled_dirty_overlap_bytes: DirtyByteCount::from_observed_bytes(
                self.scheduled_dirty_overlap_bytes.as_bytes() - frame_bytes,
            ),
            publication_receipt_count: self.publication_receipt_count + 1,
            write_scheduling_attempt_count: self.write_scheduling_attempt_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_dirty_shutdown_unflushed(self) -> Self {
        Self {
            dirty_shutdown_unflushed_count: self.dirty_shutdown_unflushed_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_dirty_discard_denial(self) -> Self {
        Self {
            dirty_discard_denial_count: self.dirty_discard_denial_count + 1,
            ..self
        }
    }

    pub const fn dirty_pages(self) -> DirtyPageCount {
        self.dirty_pages
    }

    pub const fn dirty_bytes(self) -> DirtyByteCount {
        self.dirty_bytes
    }

    pub const fn scheduled_not_durable_pages(self) -> DirtyPageCount {
        self.scheduled_not_durable_pages
    }

    pub const fn scheduled_not_durable_bytes(self) -> DirtyByteCount {
        self.scheduled_not_durable_bytes
    }

    pub const fn scheduled_dirty_overlap_pages(self) -> DirtyPageCount {
        self.scheduled_dirty_overlap_pages
    }

    pub const fn scheduled_dirty_overlap_bytes(self) -> DirtyByteCount {
        self.scheduled_dirty_overlap_bytes
    }

    pub fn unflushed_dirty_pages(self) -> DirtyPageCount {
        DirtyPageCount::from_observed_pages(
            self.dirty_pages.as_pages() + self.scheduled_not_durable_pages.as_pages()
                - self.scheduled_dirty_overlap_pages.as_pages(),
        )
    }

    pub fn unflushed_dirty_bytes(self) -> DirtyByteCount {
        DirtyByteCount::from_observed_bytes(
            self.dirty_bytes.as_bytes() + self.scheduled_not_durable_bytes.as_bytes()
                - self.scheduled_dirty_overlap_bytes.as_bytes(),
        )
    }

    pub fn has_unflushed_dirty_state(self) -> bool {
        self.unflushed_dirty_pages().as_pages() > 0
    }

    pub const fn dirty_mark_attempt_count(self) -> u64 {
        self.dirty_mark_attempt_count
    }

    pub const fn newly_dirty_count(self) -> u64 {
        self.newly_dirty_count
    }

    pub const fn already_dirty_count(self) -> u64 {
        self.already_dirty_count
    }

    pub const fn dirty_budget_denial_count(self) -> u64 {
        self.dirty_budget_denial_count
    }

    pub const fn publication_plan_attempt_count(self) -> u64 {
        self.publication_plan_attempt_count
    }

    pub const fn publication_plan_denial_count(self) -> u64 {
        self.publication_plan_denial_count
    }

    pub const fn publication_receipt_count(self) -> u64 {
        self.publication_receipt_count
    }

    pub const fn write_scheduling_attempt_count(self) -> u64 {
        self.write_scheduling_attempt_count
    }

    pub const fn write_scheduling_denial_count(self) -> u64 {
        self.write_scheduling_denial_count
    }

    pub const fn stale_publication_plan_denial_count(self) -> u64 {
        self.stale_publication_plan_denial_count
    }

    pub const fn dirty_shutdown_unflushed_count(self) -> u64 {
        self.dirty_shutdown_unflushed_count
    }

    pub const fn dirty_discard_denial_count(self) -> u64 {
        self.dirty_discard_denial_count
    }
}
