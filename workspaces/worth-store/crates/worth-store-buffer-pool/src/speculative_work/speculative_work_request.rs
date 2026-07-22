use crate::{AllocationRequest, DirtyPageCount};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefetchWindow {
    resident_frames: u32,
}

impl PrefetchWindow {
    pub fn resident_frames(resident_frames: u32) -> Result<Self, SpeculativeWorkRequestDenial> {
        if resident_frames == 0 {
            return Err(SpeculativeWorkRequestDenial::WindowIsZero);
        }
        Ok(Self { resident_frames })
    }

    pub const fn as_resident_frames(self) -> u32 {
        self.resident_frames
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadAheadRequest {
    window: PrefetchWindow,
    allocation: Option<AllocationRequest>,
}

impl ReadAheadRequest {
    pub const fn new(window: PrefetchWindow, allocation: Option<AllocationRequest>) -> Self {
        Self { window, allocation }
    }

    pub const fn window(self) -> PrefetchWindow {
        self.window
    }

    pub const fn allocation(self) -> Option<AllocationRequest> {
        self.allocation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefetchRequest {
    window: PrefetchWindow,
    allocation: Option<AllocationRequest>,
}

impl PrefetchRequest {
    pub const fn new(window: PrefetchWindow, allocation: Option<AllocationRequest>) -> Self {
        Self { window, allocation }
    }

    pub const fn window(self) -> PrefetchWindow {
        self.window
    }

    pub const fn allocation(self) -> Option<AllocationRequest> {
        self.allocation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteBehindRequest {
    dirty_pages: DirtyPageCount,
    allocation: Option<AllocationRequest>,
}

impl WriteBehindRequest {
    pub fn dirty_pages(
        dirty_pages: DirtyPageCount,
        allocation: Option<AllocationRequest>,
    ) -> Result<Self, SpeculativeWorkRequestDenial> {
        if dirty_pages.as_pages() == 0 {
            return Err(SpeculativeWorkRequestDenial::DirtyPagesAreZero);
        }
        Ok(Self {
            dirty_pages,
            allocation,
        })
    }

    pub const fn dirty_page_count(self) -> DirtyPageCount {
        self.dirty_pages
    }

    pub const fn allocation(self) -> Option<AllocationRequest> {
        self.allocation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeculativeWorkRequestDenial {
    WindowIsZero,
    DirtyPagesAreZero,
}
