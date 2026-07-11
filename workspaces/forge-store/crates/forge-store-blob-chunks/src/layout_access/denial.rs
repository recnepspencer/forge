#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobLayoutAccessDenialKind {
    ChunkTreeRootCannotStandInForBlobObjectLayoutAuthority,
    StreamingFrontierCannotStandInForChunkTreeLayoutAuthority,
    FullBlobBufferCannotStandInForStreamingLayoutAuthority,
    DigestOnlyCandidateCannotStandInForDedupeLayoutAuthority,
    EmptyReachabilityProofCannotStandInForReachabilityLayoutAuthority,
    ReclaimReceiptCannotStandInForRetentionLayoutAuthority,
    RetentionLayoutRequiresExplicitDisposition,
    RetentionLayoutRequiresProtectedHoldEvidence,
    ReclaimLayoutRequiresReachabilityBoundPolicyExecution,
    ScopeSafeAbsenceRequiresReclaimReleaseMatch,
    CompactionLayoutRequiresPlanBoundEquivalence,
    PublishedGenerationDoesNotMatchVerifiedRead,
    VerifiedReadDoesNotMatchStreamingRequest,
    StreamingLayoutRequiresExactCounters,
    StreamingLayoutRequiresConstantMemory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobLayoutAccessDenial {
    kind: BlobLayoutAccessDenialKind,
}

impl BlobLayoutAccessDenial {
    pub(crate) const fn new(kind: BlobLayoutAccessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> BlobLayoutAccessDenialKind {
        self.kind
    }
}
