#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryPhysicsCrashLane {
    WalAppend,
    PageFlush,
    CheckpointWrite,
    CheckpointCutover,
    CompactionCutover,
    Acknowledgment,
    DirectorySync,
    RenameDurability,
}

impl RecoveryPhysicsCrashLane {
    pub const REQUIRED_S4_LANES: [Self; 8] = [
        Self::WalAppend,
        Self::PageFlush,
        Self::CheckpointWrite,
        Self::CheckpointCutover,
        Self::CompactionCutover,
        Self::Acknowledgment,
        Self::DirectorySync,
        Self::RenameDurability,
    ];

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::WalAppend => "wal_append",
            Self::PageFlush => "page_flush",
            Self::CheckpointWrite => "checkpoint_write",
            Self::CheckpointCutover => "checkpoint_cutover",
            Self::CompactionCutover => "compaction_cutover",
            Self::Acknowledgment => "acknowledgment",
            Self::DirectorySync => "directory_sync",
            Self::RenameDurability => "rename_durability",
        }
    }

    pub const fn crash_seam(&self) -> &'static str {
        match self {
            Self::WalAppend => "storage.wal.append",
            Self::PageFlush => "storage.page.flush",
            Self::CheckpointWrite => "storage.checkpoint.write",
            Self::CheckpointCutover => "storage.checkpoint.cutover",
            Self::CompactionCutover => "storage.compaction.cutover",
            Self::Acknowledgment => "storage.acknowledgment",
            Self::DirectorySync => "storage.directory.sync",
            Self::RenameDurability => "storage.rename.durability",
        }
    }
}
