#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobOrphanReclaimDenial {
    MissingSessionDigest,
    MissingChunkDigest,
    EmptyPartialChunk,
    AlreadyReachable,
    MissingS7ReclaimBarrier,
    OrphanReclaimIdentityMismatch,
}