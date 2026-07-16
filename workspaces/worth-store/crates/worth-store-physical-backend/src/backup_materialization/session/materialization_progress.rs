#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBackupMaterializationProgress {
    BytesCopied(PhysicalBackupCopyProgress),
    ArtifactDurable(PhysicalBackupArtifactDurabilityProgress),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalBackupCopyProgress {
    artifact_index: usize,
    bytes_copied: u64,
    artifact_bytes_copied: u64,
    artifact_total_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalBackupArtifactDurabilityProgress {
    artifact_index: usize,
    artifact_bytes: u64,
}

impl PhysicalBackupCopyProgress {
    pub(super) const fn new(
        artifact_index: usize,
        bytes_copied: u64,
        artifact_bytes_copied: u64,
        artifact_total_bytes: u64,
    ) -> Self {
        Self {
            artifact_index,
            bytes_copied,
            artifact_bytes_copied,
            artifact_total_bytes,
        }
    }

    pub const fn artifact_index(self) -> usize {
        self.artifact_index
    }
    pub const fn bytes_copied(self) -> u64 {
        self.bytes_copied
    }
    pub const fn artifact_bytes_copied(self) -> u64 {
        self.artifact_bytes_copied
    }
    pub const fn artifact_total_bytes(self) -> u64 {
        self.artifact_total_bytes
    }
}

impl PhysicalBackupArtifactDurabilityProgress {
    pub(super) const fn new(artifact_index: usize, artifact_bytes: u64) -> Self {
        Self {
            artifact_index,
            artifact_bytes,
        }
    }

    pub const fn artifact_index(self) -> usize {
        self.artifact_index
    }
    pub const fn artifact_bytes(self) -> u64 {
        self.artifact_bytes
    }
}
