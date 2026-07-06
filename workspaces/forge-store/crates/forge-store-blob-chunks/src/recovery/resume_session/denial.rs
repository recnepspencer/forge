#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobResumeUnfinishedState {
    SessionDeclaredWithoutAdmission,
    SessionAdmittedWithoutChunkAppend,
    ChunkAppendWithoutDurableBytes,
    ChunkBytesWithoutChecksumAdmission,
    ChecksumAdmissionWithoutDurableFrontier,
    DurableFrontierWithoutRootNode,
    RootNodeWithoutReachabilityStaging,
    BlobPublishedAwaitingSessionCloseout,
    SessionClosed,
    SessionAbandonedAwaitingReclaim,
    SessionReclaimed,
    ClosedSessionWithOrphanChunks,
    MissingChunkTail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobResumeDenial {
    EmptyDeclaredBlob,
    WrongWalRecordKind,
    MissingDurableBytes,
    ChunkOrdinalMismatch,
    ChunkSecurityScopeMismatch,
    ChunkTailMissing {
        expected_total_bytes: u64,
        actual_total_bytes: u64,
    },
    FrontierMissingChunk,
    RootCandidateMismatch,
    StaleSessionId,
    WrongSecurityScope,
    WrongStoreAuthority,
    CopiedCheckpointAuthority,
    MissingS7ReclaimProof,
}
