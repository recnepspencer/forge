#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundWorkBudgetSnapshot {
    free_resident_frames: u32,
    foreground_reserved_frames: u32,
    pinned_pages_used: u32,
    pinned_page_budget: u32,
}

impl BackgroundWorkBudgetSnapshot {
    pub const fn foreground_reserved(
        free_resident_frames: u32,
        foreground_reserved_frames: u32,
        pinned_pages_used: u32,
        pinned_page_budget: u32,
    ) -> Self {
        Self {
            free_resident_frames,
            foreground_reserved_frames,
            pinned_pages_used,
            pinned_page_budget,
        }
    }

    pub const fn free_resident_frames(self) -> u32 {
        self.free_resident_frames
    }

    pub const fn foreground_reserved_frames(self) -> u32 {
        self.foreground_reserved_frames
    }

    pub const fn background_available_frames(self) -> u32 {
        self.free_resident_frames
            .saturating_sub(self.foreground_reserved_frames)
    }

    pub const fn pinned_pages_used(self) -> u32 {
        self.pinned_pages_used
    }

    pub const fn pinned_page_budget(self) -> u32 {
        self.pinned_page_budget
    }

    pub const fn pin_budget_remaining(self) -> u32 {
        self.pinned_page_budget
            .saturating_sub(self.pinned_pages_used)
    }
}
