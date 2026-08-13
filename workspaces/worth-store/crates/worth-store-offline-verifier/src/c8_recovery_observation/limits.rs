#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryObserverLimitsDenial {
    ZeroDirectoryEntryLimit,
    ZeroDirectoryLimit,
    ZeroArtifactLimit,
    ZeroByteLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryObserverLimits {
    maximum_directory_entries: u64,
    maximum_directories: u64,
    maximum_artifacts: u64,
    maximum_bytes: u64,
}

impl RecoveryObserverLimits {
    pub const fn new(
        maximum_directory_entries: u64,
        maximum_directories: u64,
        maximum_artifacts: u64,
        maximum_bytes: u64,
    ) -> Result<Self, RecoveryObserverLimitsDenial> {
        if maximum_directory_entries == 0 {
            return Err(RecoveryObserverLimitsDenial::ZeroDirectoryEntryLimit);
        }
        if maximum_directories == 0 {
            return Err(RecoveryObserverLimitsDenial::ZeroDirectoryLimit);
        }
        if maximum_artifacts == 0 {
            return Err(RecoveryObserverLimitsDenial::ZeroArtifactLimit);
        }
        if maximum_bytes == 0 {
            return Err(RecoveryObserverLimitsDenial::ZeroByteLimit);
        }
        Ok(Self {
            maximum_directory_entries,
            maximum_directories,
            maximum_artifacts,
            maximum_bytes,
        })
    }

    pub(super) const fn maximum_directory_entries(self) -> u64 {
        self.maximum_directory_entries
    }

    pub(super) const fn maximum_directories(self) -> u64 {
        self.maximum_directories
    }

    pub(super) const fn maximum_artifacts(self) -> u64 {
        self.maximum_artifacts
    }

    pub(super) const fn maximum_bytes(self) -> u64 {
        self.maximum_bytes
    }
}
