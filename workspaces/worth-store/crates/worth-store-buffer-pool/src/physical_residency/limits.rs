#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalResidencyLimits {
    pool_budget: crate::BufferPoolBudget,
    metadata_bytes: u64,
    pin_leases: u32,
    operation_bytes: u64,
    frame_entries: u32,
    prefetch_frames: u32,
    read_ahead_frames: u32,
    write_back_frames: u32,
}

impl PhysicalResidencyLimits {
    pub fn new(
        resident_bytes: u64,
        pinned_frames: u32,
        dirty_frames: u32,
        operation_bytes: u64,
        frame_entries: u32,
    ) -> Option<Self> {
        let metadata_bytes = u64::from(frame_entries)
            .checked_mul(512)?
            .checked_add(4096)?;
        Self::new_with_metadata_budget(
            resident_bytes,
            metadata_bytes,
            pinned_frames,
            dirty_frames,
            operation_bytes,
            frame_entries,
        )
    }

    pub fn new_with_metadata_budget(
        resident_bytes: u64,
        metadata_bytes: u64,
        pinned_frames: u32,
        dirty_frames: u32,
        operation_bytes: u64,
        frame_entries: u32,
    ) -> Option<Self> {
        if resident_bytes == 0
            || pinned_frames == 0
            || metadata_bytes == 0
            || dirty_frames == 0
            || operation_bytes == 0
            || frame_entries == 0
        {
            return None;
        }
        let resident = crate::ResidentMemoryBudget::bytes(resident_bytes).ok()?;
        let pinned = crate::PinnedPageBudget::pages(pinned_frames).ok()?;
        let dirty = crate::DirtyPageBudget::pages(dirty_frames).ok()?;
        Some(Self {
            pool_budget: crate::BufferPoolBudget::declare(resident, pinned, dirty),
            metadata_bytes,
            pin_leases: pinned_frames,
            operation_bytes,
            frame_entries,
            prefetch_frames: pinned_frames,
            read_ahead_frames: pinned_frames,
            write_back_frames: dirty_frames,
        })
    }

    pub const fn with_speculative_frame_limits(
        mut self,
        prefetch_frames: u32,
        read_ahead_frames: u32,
        write_back_frames: u32,
    ) -> Option<Self> {
        if prefetch_frames == 0 || read_ahead_frames == 0 || write_back_frames == 0 {
            return None;
        }
        self.prefetch_frames = prefetch_frames;
        self.read_ahead_frames = read_ahead_frames;
        self.write_back_frames = write_back_frames;
        Some(self)
    }

    pub const fn with_pin_lease_limit(mut self, pin_leases: u32) -> Option<Self> {
        if pin_leases == 0 {
            return None;
        }
        self.pin_leases = pin_leases;
        Some(self)
    }

    pub const fn resident_bytes(self) -> u64 {
        self.pool_budget.resident_memory().as_bytes()
    }

    pub const fn pool_budget(self) -> crate::BufferPoolBudget {
        self.pool_budget
    }

    pub const fn metadata_bytes(self) -> u64 {
        self.metadata_bytes
    }

    pub const fn pinned_frames(self) -> u32 {
        self.pool_budget.pinned_pages().as_pages()
    }

    pub const fn dirty_frames(self) -> u32 {
        self.pool_budget.dirty_pages().as_pages()
    }

    pub const fn operation_bytes(self) -> u64 {
        self.operation_bytes
    }

    pub const fn frame_entries(self) -> u32 {
        self.frame_entries
    }
    pub const fn pin_leases(self) -> u32 {
        self.pin_leases
    }
    pub const fn prefetch_frames(self) -> u32 {
        self.prefetch_frames
    }
    pub const fn read_ahead_frames(self) -> u32 {
        self.read_ahead_frames
    }
    pub const fn write_back_frames(self) -> u32 {
        self.write_back_frames
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationAllocationScope {
    ForegroundRead,
    ForegroundWrite,
    Recovery,
    Scrub,
    Maintenance,
    Verification,
    Blob,
}

impl OperationAllocationScope {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::ForegroundRead => 0,
            Self::ForegroundWrite => 1,
            Self::Recovery => 2,
            Self::Scrub => 3,
            Self::Maintenance => 4,
            Self::Verification => 5,
            Self::Blob => 6,
        }
    }
}
