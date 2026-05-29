use super::RelationalDiagnosticFields;
use serde::{Deserialize, Serialize};

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
pub enum DiagnosticsDeliveryClass {
    MustBeHot,
    CanDefer,
    ReconstructableFromReplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalArtifactPolicy {
    pub delivery_class: DiagnosticsDeliveryClass,
    pub enabled: bool,
    pub max_entries: usize,
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
    RelationUpdated,
    RelationDeleted,
    RelationRetainedForAudit,
    ReplayRetentionPinned,
    ReplayRetentionReleased,
    RetentionPlanInspected,
    MergeBaseResolved,
    MergeExecutionPublished,
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
    pub fields: RelationalDiagnosticFields,
}

impl RelationalDiagnosticsEntry {
    pub fn new(
        code: DiagnosticCode,
        message: impl Into<String>,
        fields: RelationalDiagnosticFields,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            fields,
        }
    }

    pub fn canonicalized(mut self) -> Self {
        self.fields = canonicalize_diagnostic_fields(self.fields);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalDiagnosticArtifact {
    pub scope: DiagnosticsScope,
    pub kind: DiagnosticsArtifactKind,
    pub determinism: DeterminismExpectation,
    pub entries: Vec<RelationalDiagnosticsEntry>,
}

impl RelationalDiagnosticArtifact {
    pub fn new(
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        determinism: DeterminismExpectation,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) -> Self {
        Self {
            scope,
            kind,
            determinism,
            entries,
        }
        .canonicalized()
    }

    pub fn canonicalized(mut self) -> Self {
        self.entries = self
            .entries
            .into_iter()
            .map(RelationalDiagnosticsEntry::canonicalized)
            .collect();
        self
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

pub(crate) fn canonicalize_diagnostic_fields(
    fields: RelationalDiagnosticFields,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticFields::from_diagnostic_value(fields.root().clone())
}
