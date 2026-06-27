use crate::{EvictionProtectionSummary, ResidentFrameTable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeculativeWorkBudgetSnapshot {
    resident_capacity_frames: u32,
    resident_frame_count: u32,
    free_frame_count: u32,
    protected_resident_frames: u32,
    protection_summary: EvictionProtectionSummary,
    dirty_pages_used: u32,
    dirty_page_budget: u32,
    pinned_pages_used: u32,
    pinned_page_budget: u32,
}

impl SpeculativeWorkBudgetSnapshot {
    pub const fn resident_capacity_frames(self) -> u32 {
        self.resident_capacity_frames
    }

    pub const fn resident_frame_count(self) -> u32 {
        self.resident_frame_count
    }

    pub const fn free_frame_count(self) -> u32 {
        self.free_frame_count
    }

    pub const fn protected_resident_frames(self) -> u32 {
        self.protected_resident_frames
    }

    pub const fn protection_summary(self) -> EvictionProtectionSummary {
        self.protection_summary
    }

    pub const fn dirty_pages_used(self) -> u32 {
        self.dirty_pages_used
    }

    pub const fn dirty_page_budget(self) -> u32 {
        self.dirty_page_budget
    }

    pub const fn dirty_pages_remaining(self) -> u32 {
        self.dirty_page_budget.saturating_sub(self.dirty_pages_used)
    }

    pub const fn dirty_pages_exceed_budget(self) -> bool {
        self.dirty_pages_used > self.dirty_page_budget
    }

    pub const fn pinned_pages_used(self) -> u32 {
        self.pinned_pages_used
    }

    pub const fn pinned_page_budget(self) -> u32 {
        self.pinned_page_budget
    }

    pub const fn pinned_pages_remaining(self) -> u32 {
        self.pinned_page_budget
            .saturating_sub(self.pinned_pages_used)
    }

    pub const fn pinned_pages_exceed_budget(self, requested_pages: u32) -> bool {
        requested_pages > self.pinned_pages_remaining()
    }

    pub const fn all_resident_frames_protected(self) -> bool {
        self.resident_frame_count > 0 && self.resident_frame_count == self.protected_resident_frames
    }
}

impl ResidentFrameTable {
    pub fn speculative_work_budget_snapshot(&self) -> SpeculativeWorkBudgetSnapshot {
        let mut protected_resident_frames = 0;
        let mut protection_summary = EvictionProtectionSummary::empty();
        for slot in &self.resident_slots {
            if let Ok(record) = self.record_at_slot(*slot) {
                let frame_protection = record.eviction_protection_summary();
                if !frame_protection.is_empty() {
                    protected_resident_frames += 1;
                    protection_summary = protection_summary.merge(frame_protection);
                }
            }
        }

        SpeculativeWorkBudgetSnapshot {
            resident_capacity_frames: self.frames.len() as u32,
            resident_frame_count: self.resident_slots.len() as u32,
            free_frame_count: self.free_slots.len() as u32,
            protected_resident_frames,
            protection_summary,
            dirty_pages_used: self.dirty_counters.dirty_pages().as_pages(),
            dirty_page_budget: self.entry.admission().budget().dirty_pages().as_pages(),
            pinned_pages_used: self.pin_counters.active_pinned_pages() as u32,
            pinned_page_budget: self.entry.admission().budget().pinned_pages().as_pages(),
        }
    }
}
