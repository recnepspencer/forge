//! Public API boundary for `forge-relational`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

pub use crate::config::data::{
    AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CompiledLanePolicy, ConfigProvenance,
    ConfigProvenanceEntry, ConfigValueSource, CrossContextPolicy, DurableLogPolicy,
    DurableLogRetentionMode, MvccConfig, PatchSurfacePolicy, PublicationConfig,
    RelationalConfigOverride, RelationalRuntimeProfile, RetentionBackend, RetentionPolicy,
    SnapshotReleasePolicy, StorageLayoutConfig,
};
pub use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry, RelationalDiagnosticsProfile,
};
pub use crate::durability::data::{
    DurabilityError, DurabilityMode, DurableCheckpoint, DurableCommitEnvelope,
    RecoveryFailureClass, RecoveryPlan,
};
pub use crate::history::data::{
    BranchCreateError, BranchHead, BranchId, CommitId, CommitReference, HistoryRetentionClass,
    MergeConflictRecord, MergeInspection, VersionGraphPolicy, VersionGraphSnapshot, VersionNode,
};
pub use crate::identity::data::{
    EntityId, EntityStorageId, Generation, KindId, LineageId, LocalSlot, PartitionId, RelationId,
    RelationStorageId, Slot, StructuralFingerprint, VersionId,
};
pub use crate::indexes::data::{
    DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
    DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
    ReadWithStorageFallbackOutcome,
};
pub use crate::lineage::data::{
    CorrespondenceCandidate, CorrespondenceResolution, LineageDivergenceSummary, LineageEventKind,
    LineageEventRecord, LineageGraphSnapshot, LineageInvariant, LineageNode,
    LineageResolutionStatus,
};
pub use crate::logic::builder::RelationalRuntimeBuilder;
pub use crate::logic::commit::CommitAuthorityContract;
pub use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
pub use crate::logic::runtime::{
    ChunkVisibilitySummary, ChunkedStorageSummary, CompiledArtifactCompatibility,
    CompiledArtifactError, CompiledExecutionArtifact, ComplexityContract, ComplexityStatus,
    EntityReadRecord, InvariantCatalog, InvariantCheckResult, InvariantClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantRule, PacketResult,
    PartitionStorageStats, RelationReadRecord, RelationalDiagnosticsFacade, RelationalReadView,
    RelationalReplayRecord, RelationalRuntime, RelationalRuntimeConfig, ReplaySchemaVersion,
    RuntimeComplexityCounters, StorageInvariantReport, StorageStats, TopologyFreezeMode,
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
    RelationalMutation,
};
pub use crate::publication::data::diff::{
    AspectKey, PatchFragmentBudget, PatchOrdering, PatchPublicationMode, PatchRecord,
    PatchRecordKind, PatchStreamPosition, RelationalPatchRecord,
};
pub use crate::publication::data::{
    PublicationBundle, PublicationError, PublicationStage, PublicationStatus,
};
pub use crate::query::data::{
    PartitionHint, QueryExecutionShape, QueryWorkPacket, ReadPacketPlan, ReadTarget,
    ReductionDiscipline,
};
pub use crate::replay::data::{
    CanonicalCommitEnvelope, RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode,
    ReplayFailureClass, ReplayMismatch, ReplayObservableSurface,
};
pub use crate::schema::data::{
    EntityKindRegistration, KindResolution, RelationKindRegistration, RelationPayloadClass,
    RelationalSchemaRegistry, SchemaId, SchemaRegistryError, SchemaVersionId,
};
pub use crate::snapshots::data::{
    SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
};
pub use crate::symbols::data::{
    InternedString, StringInterner, Symbol, SymbolPolicy, SymbolTableSnapshot,
};
pub use crate::transactions::data::{
    AuthoritativeApplyPlan, AuthorityMode, CommitAuthority, CommitConflict, CommitOutcome,
    CrossContextEndpointClass, MergedCommitPlan, RecordRef, RelationScope, RollbackOutcome,
    SavepointId, TransactionCommitError, TransactionId, TransactionIntent, TransactionIntentBatch,
    TransactionOptions, UndoRecord, WorkerIntentBatch,
};
pub use crate::transactions::logic::RelationalTransaction;
