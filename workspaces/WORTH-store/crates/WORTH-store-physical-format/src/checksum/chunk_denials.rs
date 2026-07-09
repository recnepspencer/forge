#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalChunkChecksumDenial {
    EmptyChunkPayload,
    ChecksumMismatch,
    UnsupportedAlgorithm,
    StorePhysicalAuthorityRequired,
}
