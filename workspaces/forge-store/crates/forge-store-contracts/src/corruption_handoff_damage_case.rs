//! Shared corruption damage-case vocabulary for cross-crate handoff boundaries.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorruptionHandoffDamageCase {
    ChecksumMismatch,
    AuthenticityFailure,
    MissingChunk,
    StaleGeneration,
    CrossScopeImport,
}
