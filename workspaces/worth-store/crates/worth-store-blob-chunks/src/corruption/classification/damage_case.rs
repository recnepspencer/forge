/// Physical damage kind — distinct from [`super::super::types::BlobCorruptionDetectionSource`]
/// (where damage was observed) and from quarantine lifecycle posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobDamageCase {
    ChecksumMismatch,
    AuthenticityFailure,
    MissingChunk,
    StaleGeneration,
    CrossScopeImport,
}
