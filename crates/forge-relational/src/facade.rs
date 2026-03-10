//! Public API boundary for `forge-relational`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

pub use crate::data::config::{
    AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CompiledLanePolicy,
    ConfigProvenance, ConfigProvenanceEntry, ConfigValueSource, CrossContextPolicy,
    DurableLogPolicy, DurableLogRetentionMode, MvccConfig, PatchSurfacePolicy,
    PublicationConfig, RelationalConfigOverride, RelationalRuntimeProfile, RetentionBackend,
    RetentionPolicy, SnapshotReleasePolicy, StorageLayoutConfig,
};
pub use crate::data::diagnostics::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry, RelationalDiagnosticsProfile,
};
pub use crate::data::diff::{
    AspectKey, PatchFragmentBudget, PatchOrdering, PatchPublicationMode, PatchRecord,
    PatchRecordKind, PatchStreamPosition, RelationalPatchRecord,
};
pub use crate::data::durability::{
    DurabilityError, DurabilityMode, DurableCheckpoint, DurableCommitEnvelope,
    RecoveryFailureClass, RecoveryPlan,
};
pub use crate::data::history::{
    BranchCreateError, BranchHead, BranchId, CommitId, CommitReference, HistoryRetentionClass,
    MergeConflictRecord, MergeInspection, VersionGraphPolicy, VersionGraphSnapshot, VersionNode,
};
pub use crate::data::identity::{
    EntityId, EntityStorageId, Generation, KindId, LineageId, LocalSlot, PartitionId, RelationId,
    RelationStorageId, Slot, StructuralFingerprint, VersionId,
};
pub use crate::data::index::{
    DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
    DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
    ReadWithStorageFallbackOutcome,
};
pub use crate::data::payload::{
    PayloadClass, PayloadCompatibility, PayloadEncoding, PayloadPolicy, RecordPayload,
};
pub use crate::data::lineage::{
    CorrespondenceCandidate, CorrespondenceResolution, LineageDivergenceSummary, LineageEventKind,
    LineageEventRecord, LineageGraphSnapshot, LineageInvariant, LineageNode,
    LineageResolutionStatus,
};
pub use crate::data::publication::{
    PublicationBundle, PublicationError, PublicationStage, PublicationStatus,
};
pub use crate::data::query::{
    PartitionHint, QueryExecutionShape, QueryWorkPacket, ReadPacketPlan, ReadTarget,
    ReductionDiscipline,
};
pub use crate::data::replay::{
    CanonicalCommitEnvelope, RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode,
    ReplayFailureClass, ReplayMismatch, ReplayObservableSurface,
};
pub use crate::data::schema::{
    EntityKindRegistration, KindResolution, RelationKindRegistration, RelationalSchemaRegistry,
    RelationPayloadClass, SchemaId, SchemaRegistryError, SchemaVersionId,
};
pub use crate::data::snapshot::{
    SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
};
pub use crate::data::symbols::{
    InternedString, StringInterner, Symbol, SymbolPolicy, SymbolTableSnapshot,
};
pub use crate::data::transaction::{
    AuthoritativeApplyPlan, AuthorityMode, CommitAuthority, CommitConflict, CommitOutcome,
    CrossContextEndpointClass, MergedCommitPlan, RecordRef, RelationScope, RollbackOutcome,
    SavepointId, TransactionCommitError, TransactionId, TransactionIntent,
    TransactionIntentBatch, TransactionOptions, UndoRecord, WorkerIntentBatch,
};
pub use crate::logic::builder::RelationalRuntimeBuilder;
pub use crate::logic::commit::CommitAuthorityContract;
pub use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
pub use crate::logic::runtime::{
    ChunkVisibilitySummary, ChunkedStorageSummary, ComplexityContract, ComplexityStatus,
    EntityReadRecord, InvariantCatalog, InvariantCheckResult, InvariantClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantRule, PacketResult,
    RecordLifecycleState, RelationReadRecord, RelationalDiagnosticsFacade, RelationalReadView,
    RelationalReplayRecord, RelationalRuntime, RelationalRuntimeConfig, RelationalTransaction,
    ReplaySchemaVersion, RuntimeComplexityCounters, StorageInvariantReport, StorageStats,
};
pub use crate::presentation::api::RelationalRuntimeApi;
pub use crate::presentation::contracts::{
    ImmutableReadContract, RelationalBoundaryContract, SerializedAuthorityContract,
};
pub use crate::presentation::harness::{
    default_harness_expectations, FixtureEntity, FixtureRelation, RelationalFixture,
    RelationalHarnessAdapter, RelationalHarnessExpectations, RelationalHarnessPlan,
    RelationalMutation,
};
