#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPublicationCrashPoint {
    AfterChunkWrite,
    AfterChecksumAdmission,
    AfterChunkTreeNodeDurability,
    AfterRootCandidateFormation,
    AfterReachabilityStaging,
    AfterPublicationRecordWrite,
    AfterSessionClose,
}