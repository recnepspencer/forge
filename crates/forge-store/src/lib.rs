mod authority;
mod backend;
mod bulk;
mod delta;
mod evidence;
mod facade;
mod failure;
mod layout;
mod media;
mod modes;
mod publication;
mod recovery;
mod snapshot;
mod wal;

pub use authority::{
    AdvanceCursorWitness, AuthoritativeBranchHeadRecord, AuthoritativeExportBundle,
    AuthoritativeExportRestoreRequest, CanonicalizedCommitEnvelope,
    CommitCoupledSupportAppendWitness, DurableCursorAcknowledgeRequest,
    DurableCursorResumePlan, DurableCursorResumeRequest, FetchedAuthoritativeCommit,
    EmbeddedCheckpointFetchRequest, FetchedDurableCursorIdentity, FetchedLineageSupportArtifact,
    FetchedSchemaBoundaryArtifact, FetchedSchemaSupportArtifact, HistoricalIdentityRequest,
    HistoricalIdentityResolution, PersistedAuthoritativeCommit, PersistedEmbeddedCheckpoint,
    PersistedSubscriberCheckpoint, RawRuntimeCommitEnvelope, ResumeAdmittedCursor,
    VerifiedAuthoritativeAppend,
};
pub use bulk::{
    BudgetAdmittedChunkPlan, BulkCanonicalChunkExecutionRequest, BulkChunkCommitWitness,
    BulkChunkExecutionOutcome, BulkExecutionPath, BulkIngestSourceRequest, BulkPlanKind,
    BulkSourceMember, BulkTransformRequest, CanonicalChunkPlan,
    ChunkMaterializationReceipt, ChunkOrdinal, ChunkWidthBudget, DeterministicChunkPlan,
    DurablyExecutedBulkChunk, FrozenBulkSourceManifest, FrozenTransformBasis,
    FrozenTransformTargetPartition, PlannedBulkChunk, ProgramChunkWitnessIndex,
    PublishedBulkProgressCheckpoint, RecoveredBulkChunkResume, ResumeBoundaryCandidate,
    ResumeReadyBulkProgram,
    BULK_FAMILY_VERSION,
};
pub use delta::{
    BranchDeltaAutoCompactDisposition, BranchDeltaAutoCompactOutcome, BranchDeltaFallbackClass,
    BranchDeltaLayerId, BranchDeltaLocality, BranchDeltaPerformanceEnvelope, BranchDeltaReadPlan,
    BranchDeltaReadRegime, BranchDeltaReadRequest, BranchDeltaReadResult, BranchDeltaReadStrategy,
    BranchDeltaRebuildReceipt, BranchDeltaRewritePlan, BranchDeltaRewritePolicyDecision,
    BranchDeltaRewriteReceipt, BranchDeltaRewriteRecommendation, BranchDeltaRewriteRequest,
    BranchDeltaRewriteStrategy, ComplexityStatus, Milestone7IndependentReference,
    RewriteEligibleDeltaSegment, SameBranchDescendantWitness, SharedBaseBranchCreationReceipt,
    SharedBaseBranchCreationRequest, SharedBaseBranchCreationWitness, BRANCH_DELTA_FAMILY_VERSION,
    MAX_DIRECT_LAYER_READ_DEPTH, MAX_DIRECT_LAYER_READ_RECORDS, MAX_REWRITE_LAYER_WIDTH,
    RECOMMENDED_REWRITE_LAYER_WIDTH,
};
pub use evidence::{
    AbsentModeLaneEvidence, CanonicalizationMetrics, CheckpointAuthorityReport,
    Milestone1CertificationBundle, Milestone1SemanticCertificationEvidence,
    Milestone2CertificationBundle, Milestone35CertificationBundle, Milestone36CertificationBundle,
    Milestone3CertificationBundle, Milestone4CertificationBundle, Milestone5CertificationBundle,
    Milestone5DeltaStorageReport, Milestone5ReadPathReport, Milestone6AccessStructureClaim,
    Milestone6AccessStructureContract, Milestone6AccessStructureVerification,
    Milestone6AccessStructureVerificationPath, Milestone6CertificationBundle,
    Milestone6CertificationOrigin, Milestone6CertificationSummary,
    Milestone6ComplexityPathStatus, Milestone6ComplexitySurface,
    Milestone6CounterContract, Milestone6LayoutMaterializationReport,
    Milestone6LayoutReadReport, Milestone6PhysicalLayoutReport,
    Milestone7AccessStructureClaim, Milestone7AccessStructureContract,
    Milestone7AccessStructureVerification, Milestone7AccessStructureVerificationPath,
    Milestone7CertificationBundle, Milestone7ComplexityPathStatus,
    Milestone7ComplexitySurface, Milestone7CounterContract, ObservedModeFailure,
    ObservedPublicationFailure, ObservedRecoveryFailure, ObservedRecoveryFailure356,
    OperatingModeContractMatrix, OperatingModeCounterSnapshot, OperatingModeLane,
    PersistedModeLaneEvidence, StoreCounterSnapshot, SupportDurabilityCertificationSummary,
};
pub use facade::{ForgeStore, ForgeStoreBuilder};
pub use failure::{StoreError, StoreErrorKind};
pub use layout::{
    AdmittedAspectLayoutReadPlan, AspectLayoutFallbackClass, AspectLayoutPerformanceEnvelope,
    AspectLayoutReadPlanDecision, AspectLayoutReadRequest, AspectLayoutSliceId,
    AspectLayoutTarget, AspectProjectionSet, AspectReadRegime, AspectScopeClass,
    CdcTouchedAspectScope, ChunkDeterminismWitness, ChunkModelFrozenPhysicalLayout,
    ChunkShapeVersion, DedupAdmittedBlockReuse, EntitySetUniformAspectScope,
    EquivalenceContractVersion, ExplicitBroadFallbackPlan,
    MaxAdmittedAspectSlicesPerRead, MaxAdmittedBlockDecodeBreadth,
    MaxAdmittedControlReplayBreadthForParity, MaxDeterministicChunkWidth,
    Milestone6LayoutMaterialization, Milestone7IndependentLayoutReference,
    Milestone9PhysicalChunkReference, PhysicalChunkId, RejectedAspectLayoutReadPlan,
    SingleEntityAspectScope, StructuralBlockId,
    CHUNK_SHAPE_VERSION, EQUIVALENCE_CONTRACT_VERSION,
    FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ,
    FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH,
    FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY,
    FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH, LAYOUT_FAMILY_VERSION,
    STRUCTURAL_BLOCK_FAMILY_VERSION,
};
pub use media::{DurabilityBarrierClass, DurableBackendFamily, DurableMediaReport};
pub use modes::{
    AbsentModeSemanticEvidence, AbsentRuntimeWitness, AcknowledgedDurableCommit,
    BasisBoundCheckpoint, BasisBoundCheckpointWitness, BasisFreeCheckpoint,
    ContainsCanonicalCommits, DerivedDurableCheckpointKind, DurableModeBuilder,
    DurableMutationRequest, DurableRecoveryHandle, DurableStoreHandle,
    EmbeddedCheckpointClassification, EmbeddedCheckpointPersistenceReceipt, EmbeddedModeBuilder,
    EmbeddedStoreHandle, EphemeralCheckpointKind, ExternalRuntimeCheckpointEnvelope,
    ExternalRuntimeCommitEnvelope, NoContainedCommits, VerifiedEmbeddedCheckpoint,
};
pub use publication::{
    ObservedPublicationFamilyState, PublicationBarrierContract, PublicationClassification,
    PublicationFamily, PublicationState, PublicationStrategy, PublicationWriteOutcome,
};
pub use recovery::{
    BackupRestoreCompatibilityReport, BackupRestoreIncompatibility,
    BackupRestoreIncompatibilityKind, DegradedStateReport, DurableDegradedRecovery,
    DurableRecoveryDegradedKind, DurableRecoveryOutcome, DurableRecoverySourceSummary,
    BulkRecoveryDisposition, BulkRecoverySummary, DurableMutationIdentity,
    DurableRetryResolution, MaintenanceArtifactFamily, MaintenanceRecoveryDisposition,
    MaintenanceRecoveryEntry, MaintenanceRecoveryReport, ObservedSnapshotVersionTuple,
    RecoveredBulkChunk, RecoveryOperatorAction, RecoveryOperatorActionKind,
    RecoveryOperatorDisposition, RecoveryQuarantineScope, RecoverySourceKind,
    RecoverySourceReport, RecoveryStatusReport, ResumeEligibleRecoveredBulkChunk,
    SnapshotMaintenanceRecoveryAction,
    SnapshotMaintenanceRecoveryReport, SupportArtifactFamily,
    SupportArtifactRecoveryDisposition, SupportArtifactRecoveryEntry,
    SupportArtifactRecoveryReport,
};
pub use snapshot::{
    PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
    SnapshotReadMode, SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome,
    SnapshotRestorePlan, SnapshotRestoreRequest,
};
pub use wal::{DurableMutationId, DurablePublicationPhase, RecoveryDecisionClass};

#[cfg(test)]
mod tests;
