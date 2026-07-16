#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineInspectionBudget {
    max_buffer_bytes: usize,
    max_total_read_bytes: u64,
    maximum_owned_allocation_bytes: u64,
    max_elapsed: Option<std::time::Duration>,
    deadline: Option<std::time::SystemTime>,
    acquisition: OfflineMediaAcquisitionBudget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineMediaAcquisitionBudget {
    max_files: u64,
    max_directories: u64,
    max_path_bytes: u64,
    max_depth: u64,
}

impl OfflineMediaAcquisitionBudget {
    pub const fn bounded(
        max_files: u64,
        max_directories: u64,
        max_path_bytes: u64,
        max_depth: u64,
    ) -> Option<Self> {
        if max_files == 0 || max_directories == 0 || max_path_bytes == 0 || max_depth == 0 {
            None
        } else {
            Some(Self {
                max_files,
                max_directories,
                max_path_bytes,
                max_depth,
            })
        }
    }

    pub const fn canonical() -> Self {
        Self {
            max_files: 1_000_000,
            max_directories: 100_000,
            max_path_bytes: 512 * 1024 * 1024,
            max_depth: 128,
        }
    }

    pub const fn max_files(self) -> u64 {
        self.max_files
    }
    pub const fn max_directories(self) -> u64 {
        self.max_directories
    }
    pub const fn max_path_bytes(self) -> u64 {
        self.max_path_bytes
    }
    pub const fn max_depth(self) -> u64 {
        self.max_depth
    }
}

impl OfflineInspectionBudget {
    const CANONICAL_MAXIMUM_OWNED_ALLOCATION_BYTES: u64 = 256 * 1024 * 1024;

    pub const fn bounded(max_buffer_bytes: usize, max_total_read_bytes: u64) -> Option<Self> {
        if max_buffer_bytes == 0 || max_total_read_bytes == 0 {
            None
        } else {
            Some(Self {
                max_buffer_bytes,
                max_total_read_bytes,
                maximum_owned_allocation_bytes: Self::CANONICAL_MAXIMUM_OWNED_ALLOCATION_BYTES,
                max_elapsed: None,
                deadline: None,
                acquisition: OfflineMediaAcquisitionBudget::canonical(),
            })
        }
    }
    pub const fn max_buffer_bytes(self) -> usize {
        self.max_buffer_bytes
    }
    pub const fn max_total_read_bytes(self) -> u64 {
        self.max_total_read_bytes
    }
    pub const fn with_maximum_owned_allocation_bytes(
        mut self,
        maximum_owned_allocation_bytes: u64,
    ) -> Option<Self> {
        if maximum_owned_allocation_bytes == 0
            || maximum_owned_allocation_bytes < self.max_buffer_bytes as u64
        {
            None
        } else {
            self.maximum_owned_allocation_bytes = maximum_owned_allocation_bytes;
            Some(self)
        }
    }
    pub const fn maximum_owned_allocation_bytes(self) -> u64 {
        self.maximum_owned_allocation_bytes
    }
    pub const fn with_max_elapsed(mut self, max_elapsed: std::time::Duration) -> Option<Self> {
        if max_elapsed.is_zero() {
            None
        } else {
            self.max_elapsed = Some(max_elapsed);
            Some(self)
        }
    }
    pub const fn max_elapsed(self) -> Option<std::time::Duration> {
        self.max_elapsed
    }
    pub fn with_deadline(mut self, deadline: std::time::SystemTime) -> Option<Self> {
        if deadline <= std::time::UNIX_EPOCH {
            None
        } else {
            self.deadline = Some(deadline);
            Some(self)
        }
    }
    pub const fn deadline(self) -> Option<std::time::SystemTime> {
        self.deadline
    }
    pub const fn with_acquisition_budget(
        mut self,
        acquisition: OfflineMediaAcquisitionBudget,
    ) -> Self {
        self.acquisition = acquisition;
        self
    }
    pub const fn acquisition(self) -> OfflineMediaAcquisitionBudget {
        self.acquisition
    }
}
