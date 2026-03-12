//! Public API boundary for `forge-relational`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

pub use crate::config::data::{
    AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CompiledLanePolicy, ConfigProvenance,
    ConfigProvenanceEntry, ConfigValueSource, CrossContextPolicy, CheckpointPolicy,
    DurabilityPolicy, DurableLogPolicy, DurableLogRetentionMode, MvccConfig,
    PatchSurfacePolicy, PublicationConfig, RelationalConfigOverride,
    RelationalRuntimeProfile, RetentionBackend, RetentionPolicy, SnapshotReleasePolicy,
    StorageLayoutConfig, VisibilityCachePolicy,
};
pub use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry, RelationalDiagnosticsProfile,
};
pub use crate::diagnostics::facade::RelationalDiagnosticsFacade;
pub use crate::durability::data::{
    CheckpointCoverage, CompactionOutcome, CompactionPlan, CompactionPolicy, DurabilityError,
    DurabilityMode, DurableCheckpoint, DurableCheckpointId, DurableCheckpointManifest,
    DurableIntegrityStatus, DurableSegmentId, DurableSegmentManifest,
    DurableStore, DurableStoreLayout, PartitionCheckpointImage, RecoveryCompatibilityCheck,
    RecoveryCoverage, RecoveryCursor, RecoveryFailureClass, RecoveryIntegrityReport, RecoveryPlan,
    SegmentRetentionClass,
};
pub use crate::errors::data::{
    ErrorContext, ErrorOperation, RelationalError, RelationalSubsystem, SuggestedFix,
};
pub use crate::history::data::{
    BranchCreateError, BranchCreateErrorClass, BranchHead, BranchId, CommitId, CommitReference,
    HistoryRetentionClass, MergeConflictRecord, MergeInspection, VersionGraphPolicy,
    VersionGraphSnapshot, VersionNode,
};
pub use crate::identity::data::{
    EntityId, EntityStorageId, Generation, KindId, LineageId, LocalSlot, PartitionId, RelationId,
    RelationStorageId, Slot, StructuralFingerprint, VersionBound, VersionId,
};
pub use crate::indexes::data::{
    DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
    DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
    ReadWithStorageFallbackOutcome,
};
pub use crate::lineage::data::{
    CorrespondenceCandidate, CorrespondenceResolution, HistoricalLineageResolution,
    LineageDivergenceSummary, LineageEventKind, LineageEventRecord, LineageGraphSnapshot,
    LineageInvariant, LineageNode, LineageResolutionStatus,
};
pub use crate::logic::builder::RelationalRuntimeBuilder;
pub use crate::logic::commit::CommitAuthorityContract;
pub use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
pub use crate::logic::runtime::{
    ChunkVisibilitySummary, ChunkedStorageSummary, CompiledArtifactCompatibility,
    CompiledArtifactError, CompiledExecutionArtifact, ComplexityContract, ComplexityStatus,
    EntityReadRecord, InvariantCatalog, InvariantCheckResult, InvariantClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantRule, PacketResult,
    PartitionStorageStats, RelationReadRecord, RelationalReadView, RelationalReplayRecord,
    RelationalRuntime, RelationalRuntimeConfig, ReplaySchemaVersion, RuntimeComplexityCounters,
    SnapshotGuard, StorageInvariantReport, StorageStats, TopologyFreezeMode,
};
pub use crate::payloads::data::{
    PayloadClass, PayloadCompatibility, PayloadEncoding, PayloadPolicy, RecordPayload,
};
pub use crate::presentation::api::RelationalRuntimeApi;
pub use crate::presentation::contracts::{
    ImmutableReadContract, RelationalBoundaryContract, SerializedAuthorityContract,
};
pub use crate::presentation::harness::{
    default_harness_expectations, FixtureEntity, FixtureRelation, RelationalFixture,
    RelationalHarnessAdapter, RelationalHarnessExpectations, RelationalHarnessPlan,
};
pub use crate::publication::data::diff::{
    AspectKey, PatchFragmentBudget, PatchOrdering, PatchPublicationMode, PatchRecord,
    PatchRecordKind, PatchStreamBatch, PatchStreamPosition, PatchStreamReadError,
    PatchStreamReadErrorClass, PatchStreamRequest, RelationalPatchRecord,
};
pub use crate::publication::data::{
    PublicationBundle, PublicationError, PublicationStage, PublicationStatus,
};
pub use crate::query::data::{
    PartitionHint, QueryExecutionShape, QueryWorkPacket, ReadPacketPlan,
    ReductionDiscipline,
};
pub use crate::replay::data::{
    CanonicalCommitEnvelope, RelationalReplayOutcome, RelationalReplayRequest, ReplayError,
    ReplayExecutionMode, ReplayFailureClass, ReplayMismatch, ReplayMismatchClass,
    ReplayObservableSurface,
};
pub use crate::schema::data::{
    EntityKindRegistration, KindResolution, RelationKindRegistration, RelationPayloadClass,
    RelationalSchemaRegistry, SchemaId, SchemaRegistryError, SchemaRegistryErrorClass,
    SchemaVersionId,
};
pub use crate::snapshots::data::{
    SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
};
pub use crate::storage::data::RecordLifecycleState;
pub use crate::symbols::data::{
    InternedString, StringInterner, Symbol, SymbolPolicy, SymbolTableSnapshot,
};
pub use crate::transactions::data::{
    AuthoritativeApplyPlan, AuthorityMode, BulkEntityCreateIntent, BulkRelationCreateIntent,
    CommitAuthority, CommitConflict, CommitOutcome, ConflictClass, CreateIntent,
    CrossContextEndpointClass, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent,
    MergedCommitPlan, MutationIntent, RecordRef, RelationMutationIntent, RelationScope,
    ReplaceEntityIntent, RollbackEffect, RollbackOutcome, SavepointId, TransactionCommitError,
    TransactionId, TransactionOptions, UndoRecord,
    UpdateEntityIntent, WorkerIntentBatch,
};
pub use crate::transactions::logic::RelationalTransaction;
