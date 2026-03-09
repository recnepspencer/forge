use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticsScope {
    Transaction,
    Snapshot,
    History,
    Replay,
    PatchPublication,
    Lineage,
    QueryPlanning,
    Invariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticsArtifactKind {
    MinimalSummary,
    DetailedTrace,
    Failure,
    Rollback,
    Comparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DeterminismExpectation {
    Required,
    Measured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticCode {
    StaleHandle,
    InvalidRelationEndpoint,
    DuplicateRelationIdentity,
    ConflictingIntent,
    InvalidSavepoint,
    SavepointRolledBack,
    InvariantViolation,
    CanonicalOrderingViolation,
    DeterministicMergeViolation,
    SidecarConsistencyFailure,
    InvalidSnapshotHandle,
    SnapshotExpired,
    RetentionPinningConflict,
    EntityCreated,
    EntityUpdated,
    EntityDeleted,
    RelationCreated,
    RelationDeleted,
    CommitPublished,
    DiagnosticsPublicationFailure,
    ReplaySchemaVersionMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalDiagnosticsEntry {
    pub code: DiagnosticCode,
    pub message: String,
    pub fields: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalDiagnosticArtifact {
    pub scope: DiagnosticsScope,
    pub kind: DiagnosticsArtifactKind,
    pub determinism: DeterminismExpectation,
    pub entries: Vec<RelationalDiagnosticsEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalDiagnosticsProfile {
    pub capture_failures: bool,
    pub capture_rollbacks: bool,
    pub capture_comparisons: bool,
    pub detailed_traces_enabled: bool,
    pub max_entries_per_artifact: usize,
}

impl Default for RelationalDiagnosticsProfile {
    fn default() -> Self {
        Self {
            capture_failures: true,
            capture_rollbacks: true,
            capture_comparisons: true,
            detailed_traces_enabled: false,
            max_entries_per_artifact: 256,
        }
    }
}
