use crate::{DirtyPageBudget, PinnedPageBudget, ResidentMemoryBudget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolBudget {
    resident_memory: ResidentMemoryBudget,
    pinned_pages: PinnedPageBudget,
    dirty_pages: DirtyPageBudget,
}

impl BufferPoolBudget {
    pub const fn declare(
        resident_memory: ResidentMemoryBudget,
        pinned_pages: PinnedPageBudget,
        dirty_pages: DirtyPageBudget,
    ) -> Self {
        Self {
            resident_memory,
            pinned_pages,
            dirty_pages,
        }
    }

    pub const fn resident_memory(&self) -> ResidentMemoryBudget {
        self.resident_memory
    }

    pub const fn pinned_pages(&self) -> PinnedPageBudget {
        self.pinned_pages
    }

    pub const fn dirty_pages(&self) -> DirtyPageBudget {
        self.dirty_pages
    }
}
