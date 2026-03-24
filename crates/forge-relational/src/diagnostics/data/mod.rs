use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DiagnosticsScope {
    Schema,
    Transaction,
    Snapshot,
    Retention,
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
    RelationEndpointKindViolation,
    RelationCardinalityViolation,
    RelationUniquenessViolation,
    RelationSymmetryViolation,
    RelationEndpointDeletionIntegrityViolation,
    RelationContractCompatibilityMismatch,
    ConflictingIntent,
    InvalidSavepoint,
    InvalidMergeParent,
    MissingMergeBase,
    MergeConflictOverlap,
    SavepointRolledBack,
    InvariantViolation,
    PreparationFailure,
    PreparationFallback,
    InvariantProofBoundaryObserved,
    StorageInconsistencyDetected,
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
    RelationRetainedForAudit,
    ReplayRetentionPinned,
    ReplayRetentionReleased,
    RetentionPlanInspected,
    MergeBaseResolved,
    MergeCommitPublished,
    CommitPublished,
    LineageCandidateRecorded,
    LineagePromotionPublished,
    LineagePromotionExecutionFailed,
    DurableAppendSucceeded,
    DurableAppendFailed,
    CheckpointCreated,
    CheckpointFailed,
    RecoveryCheckpointSelected,
    RecoveryRangeReplayed,
    DurableRecoveryCompatibilityEvaluated,
    SubscriberContractEvaluated,
    SubscriberBoundaryEvaluated,
    DurableCorruptionDetected,
    DurableCompactionCompleted,
    DiagnosticsPublicationFailure,
    ReplaySchemaVersionMismatch,
    SchemaContinuityViolation,
    SchemaTransitionTraced,
    SchemaTransitionClassified,
    SchemaLineageTraced,
    SchemaBridgeDescriptorConstructed,
    SchemaReconciliationResolved,
    SchemaInterpretationSensitivityClassified,
    SchemaDescriptorVersionSelected,
    SubscriberContractUpgradeDecision,
    SubscriberRenegotiationDecision,
    SnapshotReadPathInspected,
    PublishedSnapshotHandleRead,
    VisibilityCacheHit,
    VisibilityCacheMissReconstructed,
    VisibilityCacheRecentAdmissionCandidate,
    VisibilityCacheProtectedRead,
    VisibilityCacheTransientRead,
    AspectHistoryResolved,
    LineageAspectHistoryResolved,
    AspectDeltaFailure,
    AspectEvaluationTraced,
    AspectEmissionTraced,
    AspectDeclarationTraced,
    AspectLoweringTraced,
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
    pub collect_all_invariant_failures: bool,
    pub max_entries_per_artifact: usize,
}

impl Default for RelationalDiagnosticsProfile {
    fn default() -> Self {
        Self {
            capture_failures: true,
            capture_rollbacks: true,
            capture_comparisons: true,
            detailed_traces_enabled: false,
            collect_all_invariant_failures: false,
            max_entries_per_artifact: 256,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalDiagnosticsFacade {
    pub artifacts: Vec<RelationalDiagnosticArtifact>,
}

impl RelationalDiagnosticsFacade {
    pub fn artifacts(&self) -> &[RelationalDiagnosticArtifact] {
        &self.artifacts
    }

    pub fn by_scope(&self, scope: DiagnosticsScope) -> Vec<&RelationalDiagnosticArtifact> {
        self.artifacts
            .iter()
            .filter(|artifact| artifact.scope == scope)
            .collect()
    }

    pub fn minimal_summaries(&self) -> Vec<&RelationalDiagnosticArtifact> {
        self.artifacts
            .iter()
            .filter(|artifact| artifact.kind == DiagnosticsArtifactKind::MinimalSummary)
            .collect()
    }
}
