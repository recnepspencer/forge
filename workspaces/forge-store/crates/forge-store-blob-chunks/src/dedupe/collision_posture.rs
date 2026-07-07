#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkDedupeCollisionPosture {
    VerifiedEquivalent,
    DigestCollisionDenied,
    DigestAlgorithmQuarantined,
    DedupeIndexPartitioned,
    ChunkRewrittenUnderNewDigestBasis,
}
