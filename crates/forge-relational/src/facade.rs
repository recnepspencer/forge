//! Public API boundary for `forge-relational`.
//! External crates should import through this module rather than reaching into
//! internal crate structure directly.

pub mod config {
    pub use crate::config::data::{
        AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CheckpointPolicy,
        CompiledLanePolicy, ConfigProvenance, ConfigProvenanceEntry, ConfigValueSource,
        CrossContextPolicy, DurabilityPolicy, DurableLogPolicy, DurableLogRetentionMode,
        MvccConfig, PatchSurfacePolicy, PublicationConfig, RelationalConfigOverride,
        RelationalRuntimeProfile, RetentionBackend, RetentionPolicy, SnapshotReleasePolicy,
        StorageLayoutConfig, VisibilityCachePolicy,
    };
}

pub mod diagnostics {
    pub use crate::diagnostics::data::{
        DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
        RelationalDiagnosticArtifact, RelationalDiagnosticsEntry, RelationalDiagnosticsProfile,
    };
    pub use crate::diagnostics::facade::RelationalDiagnosticsFacade;
}

pub mod durability {
    pub use crate::durability::data::{
        CheckpointCoverage, CompactionOutcome, CompactionPlan, CompactionPolicy, DurabilityError,
        DurabilityMode, DurableCheckpoint, DurableCheckpointId, DurableCheckpointManifest,
        DurableIntegrityStatus, DurableSegmentId, DurableSegmentManifest, DurableStore,
        DurableStoreLayout, PartitionCheckpointImage, RecoveryAuthorityParity,
        RecoveryCompatibilityCheck,
        RecoveryCompatibilityMismatch, RecoveryCoverage, RecoveryCursor, RecoveryFailureClass,
        RecoveryIntegrityReport, RecoveryPlan, RecoveryVerificationMode,
        RecoveryVerificationOutcome, RecoveryVerificationPlan,
        RelationIntegrityContractFamily,
        SegmentRetentionClass,
    };
}

pub mod errors {
    pub use crate::errors::data::{
        ErrorContext, ErrorOperation, RelationalError, RelationalSubsystem, SuggestedFix,
    };
}

pub mod history {
    pub use crate::history::data::{
        AspectFilter, AspectFilterMode, AspectHistoryCommitSpan, AspectHistoryDigest,
        AspectHistoryEntry, AspectHistoryLineageEventSpan, AspectHistoryOrigin,
        AspectHistoryQueryResult, AspectHistoryResolutionTrace, AspectResolutionContext,
        BranchCreateError, BranchCreateErrorClass, BranchHead, BranchId, CommitId, CommitReference,
        HistoryAspectQueryTarget, HistoryRetentionClass, LineageAspectHistory,
        LineageAspectHistoryQueryResult, LineageAspectResolutionDigest, MergeConflictRecord,
        MergeInspection, RequestedAspectSet, VersionGraphPolicy, VersionGraphSnapshot, VersionNode,
    };
}

pub mod identity {
    pub use crate::identity::data::{
        EntityId, EntityStorageId, Generation, KindId, LineageId, LocalSlot, PartitionId,
        RelationId, RelationStorageId, StructuralFingerprint, VersionBound, VersionId,
    };
}

pub mod inspection {
    pub use crate::inspection::data::{
        CommitInspection, ConnectivityComponentSummary, ConnectivityInspectionRequest,
        ConnectivityInspectionBudget, ConnectivityInspectionSummary, GraphInspectionBudget,
        GraphInspectionRequest, GraphInspectionSummary,
        HistoricalAspectObservation, HistoricalAvailabilityObservation, HistoricalInspectionMode,
        HistoricalOpenResult, HistoricalRecordInspection, HistoricalRecordObservation,
        HistoricalRecordValue, HistoricalSnapshotView, InspectionAccessPath,
        InspectionAvailability, InspectionDegradation, InspectionOrigin, InspectionRecordClass,
        InspectionResolutionContext, InspectionScope, KindInspectionRequest,
        KindInspectionSummary, NeighborInspectionResult, PinStateObservation,
        RecentCommitInspectionRequest, RecentCommitInspectionWindow, ReclaimEligibility,
        RecordRetentionInspection, RetentionExecutionInspection, RetentionInspectionRequest,
        RetentionInspectionSummary,
        RetentionStateObservation, SavepointInspectionSurface, SnapshotPinInspection,
        StructuralIdentityComparison, StructuralIdentityComparisonVerdict,
        StructuralIdentityEvidence, StructuralIdentityQueryRequest,
        TransactionInspectionSurface, TransactionIntentCounts,
    };
    pub use crate::inspection::logic::InspectionAccess;
}

pub mod indexes {
    pub use crate::indexes::data::{
        DerivedIndexBuildOutcome, DerivedIndexBuildRequest, DerivedIndexCompatibility,
        DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexGenerationId, DerivedIndexId,
        DerivedIndexKind, DerivedIndexPayload, DerivedIndexPublicationStatus,
        ReadWithStorageFallbackOutcome,
    };
}

pub mod lineage {
    pub use crate::lineage::data::{
          CorrespondenceCandidate, CorrespondenceCandidateId, CorrespondencePromotionExecutionFailureClass,
          CorrespondencePromotionOutcome, CorrespondencePromotionRejectionClass, CorrespondenceResolution,
          HistoricalLineageResolution, HistoricalLineageResolutionMetrics,
          HistoricalLineageResolutionDigestBasis, HistoricalResolutionBoundednessBasis,
          HistoricalResolutionDigestMode, HistoricalResolutionRequest,
          HistoricalResolutionTrace, LineageArtifactCounters, LineageCheckpointArtifact,
          LineageCheckpointCounters, LineageCheckpointDigestBasis, LineageDecisionKind,
          LineageDecisionLogDigestBasis, LineageDigestBasis, LineageDivergenceMetrics,
          LineageDivergenceRequest, LineageDivergenceSummary, LineageDivergenceTraversalBasis,
          LineageEventBatchDigestBasis, LineageEventKind, LineageEventRecord,
          LineageGraphDigestBasis, LineageGraphDigestMode, LineageGraphMetrics,
          LineageGraphRequest, LineageGraphSnapshot, LineageGraphTraversalBasis,
          LineageInvariant, LineageNode, LineageResolutionStatus, RecordHistoryRequest,
      };
  }

pub mod runtime {
    pub use crate::logic::builder::RelationalRuntimeBuilder;
    pub use crate::logic::commit::CommitAuthorityContract;
    pub use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
    pub use crate::logic::runtime::{
        ChunkVisibilitySummary, ChunkedStorageSummary, CompiledArtifactCompatibility,
        CompiledArtifactError, CompiledExecutionArtifact, ComplexityContract, ComplexityStatus,
        EntityReadRecord, EntityRecordProjection, HarnessAuditMode, InvariantCatalog,
        InvariantCheckResult, InvariantClass, InvariantExecutionPoint, InvariantFailureEffect,
        InvariantRegistration, InvariantRule, PacketResult, PartitionStorageStats,
        RelationReadRecord, RelationRecordProjection, RelationalReadView, RelationalReplayRecord,
        RelationalRuntime, RelationalRuntimeConfig, ReplaySchemaVersion, RuntimeComplexityCounters,
        SnapshotGuard, StorageStats, TopologyFreezeMode, VisibilityProjectionView,
    };
    pub use crate::presentation::api::RelationalRuntimeApi;
    pub use crate::presentation::contracts::{
        ImmutableReadContract, RelationalBoundaryContract, SerializedAuthorityContract,
    };
}

pub mod payloads {
    pub use crate::payloads::data::{
        PayloadClass, PayloadCompatibility, PayloadEncoding, PayloadPolicy, RecordPayload,
    };
}

pub mod harness {
    pub use crate::presentation::harness::{
        default_harness_expectations, FixtureEntity, FixtureRelation, RelationalFixture,
        RelationalHarnessAdapter, RelationalHarnessError, RelationalHarnessExpectations,
        RelationalHarnessPlan,
    };
}

pub mod publication {
    pub use crate::publication::bundle::{PublicationBundle, PublicationStage, PublicationStatus};
    pub use crate::publication::cdc::data::{
        SubscriberCheckpoint, SubscriberRecoveryDecision, SubscriberRecoveryDisposition,
        SubscriberRecoverySource, SubscriberResumeRequest, SubscriberStreamBatch,
        SubscriberStreamFailure, SubscriberStreamFailureClass,
    };
    pub use crate::publication::data::PublicationError;
    pub use crate::publication::patch::data::{
        AspectKey, CanonicalAspectSet, PatchFragmentBudget, PatchOrdering, PatchPublicationMode,
        PatchRecord, PatchRecordKind, PatchStreamBatch, PatchStreamPosition, PatchStreamReadError,
        PatchStreamReadErrorClass, PatchStreamRequest, RecordStructuralChange,
        RelationalPatchRecord,
    };
}

pub mod query {
    pub use crate::query::data::{
        PartitionHint, QueryExecutionShape, QueryWorkPacket, ReadPacketPlan, ReductionDiscipline,
    };
}

pub mod replay {
    pub use crate::replay::data::{
        CanonicalCommitAuthorityKind, CanonicalCommitEnvelope, CertifiedLineageSurfaceComparisonBasis,
        CertifiedLineageSurfaceDigest, LineageCertifiedSurfaceKind, RelationalReplayOutcome,
        RelationalReplayRequest,
        ReplayAuthorityBasisKind, ReplayError, ReplayExecutionMode, ReplayFailureClass,
        ReplayLineageAuthorityBasis, ReplayLineageDigestMode, ReplayMismatch, ReplayMismatchClass,
        ReplayObservableSurface, ReplaySnapshotSurface, ReplayVerificationLayer,
        ReplayVerificationMode, ReplayVerificationPlan,
    };
}

pub mod schema {
    pub use crate::publication::patch::data::AspectKey;
    pub use crate::schema::data::{
        AspectBinding, AspectComparator, AspectDeclarationTrace, AspectDeclarationTraceRow,
        AspectLoweringTrace, AspectLoweringTraceRow, AspectPlanRevision, AspectPrecision,
        CardinalityContractDeclaration, CompatibilityObservation, ContractId, DeclaredAspect,
        DescriptorCanonicalizationCompatibilityPolicy, DescriptorCanonicalizationVersion,
        DescriptorSemanticsCompatibilityPolicy, DescriptorSemanticsVersion,
        EndpointDeletionIntegrityDeclaration, EndpointDeletionIntegrityMode,
        EndpointKindContractDeclaration, EntityKindRegistration, FreeFormSchemaDiffIntent,
        HistoricalInterpretationSensitivity, KindAspectDeclarations, KindResolution,
        LoweredAspectBinding, LoweredAspectComparator, LoweredAspectExtractor,
        LoweredExecutableAspectBindingKind, LoweredAspectPlan, LoweredCardinalityMaximumContract,
        LoweredCardinalityMinimumContract,
        LoweredEndpointDeletionIntegrityContract, LoweredEndpointKindContract,
        LoweredSchemaTransitionPlan, LoweredRelationIntegrityPlan, LoweredSymmetryContract,
        LoweredUniquenessContract, ProposedSchemaTransition, RelationIntegrityDeclarations,
        RelationIntegrityPlanCatalog, RelationIntegrityPlanRevision, RelationKindRegistration,
        RelationPayloadClass, RelationalSchemaRegistry, SchemaBoundaryFingerprint, SchemaBridgeDescriptor,
        SchemaBridgeabilityClassification, SchemaDiffAtom, SchemaDiffDetail, SchemaElementKind,
        SchemaElementRef, SchemaId, SchemaLineageArtifact, SchemaReconciliationClassification,
        SchemaReconciliationDescriptor, SchemaReconciliationOrderingMode, SchemaReconciliationPolicy,
        SchemaRegistryError, SchemaRegistryErrorClass, SchemaStratum, SchemaSubscriberImpact,
        SubscriberBoundaryVisibility, MinimumCardinalityEnforcement, PairMinimumSemantics,
        SchemaPublicationImpact, SchemaTransitionArtifact, SchemaTransitionBarrier,
        SchemaTransitionSummary, SchemaVersionId, SchemaContinuationClassification,
        SchemaContinuationDescriptor, SchemaLineageOrderingSemantics, SymmetryContractDeclaration,
        SymmetryMode, UniquenessContractDeclaration, UniquenessScope, ValidatedSchemaTransition,
    };
}

pub mod snapshots {
    pub use crate::snapshots::data::{
        SnapshotHandle, SnapshotId, SnapshotInspectionSummary, SnapshotReadPolicy,
    };
}

pub mod storage {
    pub use crate::storage::data::RecordLifecycleState;
}

pub mod symbols {
    pub use crate::symbols::data::{
        InternedString, StringInterner, Symbol, SymbolPolicy, SymbolTableSnapshot,
    };
}

pub mod transactions {
    pub use crate::transactions::data::{
        AspectEmissionTrace, AspectEvaluationTrace, AspectEvaluationTraceRow,
        AspectLifecycleTransitionClass, AspectTagAccuracyReport, AspectTraceEvidence,
        AuthoritativeApplyPlan, AuthorityMode, BulkEntityCreateIntent, BulkRelationCreateIntent,
        CommitAspectSummary, CommitAuthority, CommitChangeSummary, CommitConflict,
        CommitHistorySummary, CommitLog, CommitOutcome, CommitPatchBudgetSummary, CommitPhase,
        CommitPhaseTiming, CommitPublicationSummary, CommitResult, CommitSchemaSummary,
        CommitStructuralSummary,
        CommitSummary, CommitTopology, CommitTraceEvent, ConflictClass, CreateIntent,
        CrossContextEndpointClass, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent,
        MergedCommitPlan, MutationIntent, PatchVsTruthDeltaReport, RecordRef,
        RelationMutationIntent, RelationScope, ReplaceEntityIntent, RollbackEffect,
        RollbackOutcome, RollbackSummary, SavepointId, TransactionCommitError, TransactionId,
        TransactionOptions, UndoRecord, UpdateEntityIntent, WorkerIntentBatch,
    };
    pub use crate::transactions::logic::RelationalTransaction;
}
