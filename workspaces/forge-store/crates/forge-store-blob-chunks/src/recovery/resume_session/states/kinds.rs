#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobResumeCheckpointStateKind {
    SessionDeclared,
    SessionAdmitted,
    ChunkAppendStarted,
    ChunkBytesDurable,
    ChunkIntegrityAdmitted,
    FrontierCheckpointed,
    RootCandidateBuilt,
    RootPublicationReady,
    BlobPublished,
    SessionClosed,
    SessionAbandoned,
    SessionReclaimed,
    SessionClosedWithOrphanChunks,
}