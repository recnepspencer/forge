#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LargeStoreFixtureProfile {
    StoreLargerThanMemory,
    CheckpointHeavy,
    CompactionHeavy,
    ForegroundUnderBackgroundIo,
    BlobLargerThanMemoryReadiness,
}

impl LargeStoreFixtureProfile {
    pub const ALL: [Self; 5] = [
        Self::StoreLargerThanMemory,
        Self::CheckpointHeavy,
        Self::CompactionHeavy,
        Self::ForegroundUnderBackgroundIo,
        Self::BlobLargerThanMemoryReadiness,
    ];

    pub const fn scale_declaration(self) -> FixtureScaleDeclaration {
        match self {
            Self::StoreLargerThanMemory => FixtureScaleDeclaration::new(
                self,
                FixtureStorageScale::new(16 * 1024 * 1024 * 1024, 256 * 1024 * 1024),
                FixtureActivityScale::new(1, 0, 4 * 1024 * 1024, 0),
                FixtureRecoveryScale::new(2 * 1024 * 1024 * 1024, 4 * 1024 * 1024, 64 * 1024),
                None,
            ),
            Self::CheckpointHeavy => FixtureScaleDeclaration::new(
                self,
                FixtureStorageScale::new(4 * 1024 * 1024 * 1024, 512 * 1024 * 1024),
                FixtureActivityScale::new(32, 0, 8 * 1024 * 1024, 128 * 1024 * 1024),
                FixtureRecoveryScale::new(512 * 1024 * 1024, 128 * 1024 * 1024, 64 * 1024),
                None,
            ),
            Self::CompactionHeavy => FixtureScaleDeclaration::new(
                self,
                FixtureStorageScale::new(8 * 1024 * 1024 * 1024, 512 * 1024 * 1024),
                FixtureActivityScale::new(4, 12, 16 * 1024 * 1024, 512 * 1024 * 1024),
                FixtureRecoveryScale::new(1024 * 1024 * 1024, 512 * 1024 * 1024, 8 * 1024 * 1024),
                None,
            ),
            Self::ForegroundUnderBackgroundIo => FixtureScaleDeclaration::new(
                self,
                FixtureStorageScale::new(8 * 1024 * 1024 * 1024, 384 * 1024 * 1024),
                FixtureActivityScale::new(8, 8, 256 * 1024 * 1024, 2 * 1024 * 1024 * 1024),
                FixtureRecoveryScale::new(
                    2 * 1024 * 1024 * 1024,
                    512 * 1024 * 1024,
                    16 * 1024 * 1024,
                ),
                None,
            ),
            Self::BlobLargerThanMemoryReadiness => FixtureScaleDeclaration::new(
                self,
                FixtureStorageScale::new(64 * 1024 * 1024 * 1024, 512 * 1024 * 1024),
                FixtureActivityScale::new(2, 2, 32 * 1024 * 1024, 256 * 1024 * 1024),
                FixtureRecoveryScale::new(32 * 1024 * 1024 * 1024, 256 * 1024 * 1024, 1024 * 1024),
                Some(FixtureProfileNonClaim::BlobCorrectnessNotCertified),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureProfileNonClaim {
    BlobCorrectnessNotCertified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureStorageScale {
    declared_store_bytes: u64,
    resident_memory_budget_bytes: u64,
}

impl FixtureStorageScale {
    pub const fn new(declared_store_bytes: u64, resident_memory_budget_bytes: u64) -> Self {
        Self {
            declared_store_bytes,
            resident_memory_budget_bytes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureActivityScale {
    checkpoint_count: u32,
    compaction_run_count: u32,
    foreground_io_bytes: u64,
    background_io_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureRecoveryScale {
    blob_bytes: u64,
    wal_tail_bytes: u64,
    damaged_region_bytes: u64,
}

impl FixtureRecoveryScale {
    pub const fn new(blob_bytes: u64, wal_tail_bytes: u64, damaged_region_bytes: u64) -> Self {
        Self {
            blob_bytes,
            wal_tail_bytes,
            damaged_region_bytes,
        }
    }
}

impl FixtureActivityScale {
    pub const fn new(
        checkpoint_count: u32,
        compaction_run_count: u32,
        foreground_io_bytes: u64,
        background_io_bytes: u64,
    ) -> Self {
        Self {
            checkpoint_count,
            compaction_run_count,
            foreground_io_bytes,
            background_io_bytes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureScaleDeclaration {
    profile: LargeStoreFixtureProfile,
    declared_store_bytes: u64,
    resident_memory_budget_bytes: u64,
    checkpoint_count: u32,
    compaction_run_count: u32,
    foreground_io_bytes: u64,
    background_io_bytes: u64,
    blob_bytes: u64,
    wal_tail_bytes: u64,
    damaged_region_bytes: u64,
    non_claim: Option<FixtureProfileNonClaim>,
}

impl FixtureScaleDeclaration {
    pub const fn new(
        profile: LargeStoreFixtureProfile,
        storage: FixtureStorageScale,
        activity: FixtureActivityScale,
        recovery: FixtureRecoveryScale,
        non_claim: Option<FixtureProfileNonClaim>,
    ) -> Self {
        Self {
            profile,
            declared_store_bytes: storage.declared_store_bytes,
            resident_memory_budget_bytes: storage.resident_memory_budget_bytes,
            checkpoint_count: activity.checkpoint_count,
            compaction_run_count: activity.compaction_run_count,
            foreground_io_bytes: activity.foreground_io_bytes,
            background_io_bytes: activity.background_io_bytes,
            blob_bytes: recovery.blob_bytes,
            wal_tail_bytes: recovery.wal_tail_bytes,
            damaged_region_bytes: recovery.damaged_region_bytes,
            non_claim,
        }
    }

    pub const fn profile(&self) -> LargeStoreFixtureProfile {
        self.profile
    }

    pub const fn declared_store_bytes(&self) -> u64 {
        self.declared_store_bytes
    }

    pub const fn resident_memory_budget_bytes(&self) -> u64 {
        self.resident_memory_budget_bytes
    }

    pub const fn checkpoint_count(&self) -> u32 {
        self.checkpoint_count
    }

    pub const fn compaction_run_count(&self) -> u32 {
        self.compaction_run_count
    }

    pub const fn foreground_io_bytes(&self) -> u64 {
        self.foreground_io_bytes
    }

    pub const fn background_io_bytes(&self) -> u64 {
        self.background_io_bytes
    }

    pub const fn blob_bytes(&self) -> u64 {
        self.blob_bytes
    }

    pub const fn wal_tail_bytes(&self) -> u64 {
        self.wal_tail_bytes
    }

    pub const fn damaged_region_bytes(&self) -> u64 {
        self.damaged_region_bytes
    }

    pub const fn non_claim(&self) -> Option<FixtureProfileNonClaim> {
        self.non_claim
    }

    pub const fn declares_larger_than_memory_shape(&self) -> bool {
        self.declared_store_bytes > self.resident_memory_budget_bytes
    }
}
