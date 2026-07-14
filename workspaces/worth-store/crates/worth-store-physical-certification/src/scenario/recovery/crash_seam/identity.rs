#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecoveryCrashSeam {
    WalAppend,
    PageFlush,
    CheckpointManifestWrite,
    CheckpointCutover,
    CompactionCutover,
    Acknowledgment,
    DirectorySync,
    RenameDurability,
}

impl RecoveryCrashSeam {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WalAppend => "storage.wal.append",
            Self::PageFlush => "storage.page.flush",
            Self::CheckpointManifestWrite => "storage.checkpoint.write",
            Self::CheckpointCutover => "storage.checkpoint.cutover",
            Self::CompactionCutover => "storage.compaction.cutover",
            Self::Acknowledgment => "storage.acknowledgment",
            Self::DirectorySync => "storage.directory.sync",
            Self::RenameDurability => "storage.rename.durability",
        }
    }
}
