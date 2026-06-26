#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoWorkClass {
    Foreground,
    Checkpoint,
    Compaction,
    Scrub,
    BlobMigration,
}
