mod authority;
mod backend;
mod bulk;
mod delta;
mod evidence;
mod facade;
mod failure;
mod layout;
mod live_query;
mod maintenance;
mod media;
mod modes;
mod publication;
mod recovery;
mod retention;
mod snapshot;
mod tiering;
mod wal;

pub use authority::{
    AdvanceCursorWitness, AuthoritativeBranchHeadRecord, AuthoritativeExportBundle,
    AuthoritativeExportRestoreRequest, CanonicalizedCommitEnvelope,
    CommitCoupledSupportAppendWitness, DurableCursorAcknowledgeRequest, DurableCursorResumePlan,
    DurableCursorResumeRequest, EmbeddedCheckpointFetchRequest, FetchedAuthoritativeCommit,
    FetchedDurableCursorIdentity, FetchedLineageSupportArtifact, FetchedSchemaBoundaryArtifact,
    FetchedSchemaSupportArtifact, HistoricalIdentityRequest, HistoricalIdentityResolution,
    PersistedAuthoritativeCommit, PersistedEmbeddedCheckpoint, PersistedSubscriberCheckpoint,
    RawRuntimeCommitEnvelope, ResumeAdmittedCursor, VerifiedAuthoritativeAppend,
};
pub use bulk::{
    BudgetAdmittedChunkPlan, BulkCanonicalChunkExecutionRequest, BulkCheckpointPolicy,
    BulkChunkCommitWitness, BulkChunkExecutionOutcome, BulkExecutionPath, BulkIngestSourceRequest,
    BulkPlanKind, BulkSourceMember, BulkTransformRequest, CanonicalChunkPlan,
    ChunkMaterializationReceipt, ChunkOrdinal, ChunkWidthBudget, DeterministicChunkPlan,
    DurablyExecutedBulkChunk, FrozenBulkSourceManifest, FrozenTransformBasis,
    FrozenTransformTargetPartition, PlannedBulkChunk, ProgramChunkWitnessIndex,
    PublishedBulkProgressCheckpoint, RecoveredBulkChunkResume, ResumeBoundaryCandidate,
    ResumeReadyBulkProgram, BULK_FAMILY_VERSION,
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
    Milestone10ArtifactReport, Milestone10CertificationBundle, Milestone10CertificationSummary,
    Milestone10ComplexityPathStatus, Milestone10ComplexitySurface, Milestone10CounterContract,
    Milestone11ComplexityPathStatus, Milestone11ComplexitySurface, Milestone11CounterContract,
    Milestone13ComplexityPathStatus, Milestone13ComplexitySurface, Milestone13CounterContract,
    Milestone11LocalityScopeCount, Milestone11MaintenanceReport,
    Milestone11ReservationFamilyCount, Milestone11SchedulerTopologyReport,
    Milestone11WorkClassCount, Milestone1CertificationBundle,
    Milestone1SemanticCertificationEvidence, Milestone2CertificationBundle,
    Milestone35CertificationBundle, Milestone36CertificationBundle, Milestone3CertificationBundle,
    Milestone4CertificationBundle, Milestone5CertificationBundle, Milestone5DeltaStorageReport,
    Milestone5ReadPathReport, Milestone6AccessStructureClaim, Milestone6AccessStructureContract,
    Milestone6AccessStructureVerification, Milestone6AccessStructureVerificationPath,
    Milestone6CertificationBundle, Milestone6CertificationOrigin, Milestone6CertificationSummary,
    Milestone6ComplexityPathStatus, Milestone6ComplexitySurface, Milestone6CounterContract,
    Milestone6LayoutMaterializationReport, Milestone6LayoutReadReport,
    Milestone6PhysicalLayoutReport, Milestone7AccessStructureClaim,
    Milestone7AccessStructureContract, Milestone7AccessStructureVerification,
    Milestone7AccessStructureVerificationPath, Milestone7CertificationBundle,
    Milestone7ComplexityPathStatus, Milestone7ComplexitySurface, Milestone7CounterContract,
    Milestone9CertificationBundle, Milestone9CertificationSummary, ObservedModeFailure,
    ObservedPublicationFailure, ObservedRecoveryFailure, ObservedRecoveryFailure356,
    OperatingModeContractMatrix, OperatingModeCounterSnapshot, OperatingModeLane,
    PersistedModeLaneEvidence, StoreCounterSnapshot, SupportDurabilityCertificationSummary,
};
pub use facade::{ForgeStore, ForgeStoreBuilder};
pub use failure::{StoreError, StoreErrorKind};
pub use layout::{
    AdmittedAspectLayoutReadPlan, AspectLayoutControlTruth, AspectLayoutFallbackClass,
    AspectLayoutPerformanceEnvelope, AspectLayoutReadExecutionDecision,
    AspectLayoutReadExecutionResult, AspectLayoutReadPlanDecision, AspectLayoutReadRequest,
    AspectLayoutSliceId, AspectLayoutTarget, AspectProjectionSet, AspectReadRegime,
    AspectScopeClass, CdcTouchedAspectScope, ChunkDeterminismWitness,
    ChunkModelFrozenPhysicalLayout, ChunkShapeVersion, DedupAdmittedBlockReuse,
    DedupBackedReadResult, EntitySetUniformAspectScope, EquivalenceContractVersion,
    ExplicitBroadFallbackPlan, MaxAdmittedAspectSlicesPerRead, MaxAdmittedBlockDecodeBreadth,
    MaxAdmittedControlReplayBreadthForParity, MaxDeterministicChunkWidth,
    Milestone6ChunkModelExport, Milestone6DerivedArtifactRebuildReport,
    Milestone6LayoutMaterialization, Milestone6LayoutSupportLane, Milestone6LayoutSupportPolicy,
    Milestone6LayoutSupportPublicationDisposition, Milestone6PreparedLayoutSupport,
    Milestone6ResolvedLayoutSupportLane, Milestone7IndependentLayoutReference,
    Milestone9PhysicalChunkReference, PhysicalChunkId, RejectedAspectLayoutReadPlan,
    SingleEntityAspectScope, StructuralBlockId, StructuralBlockLookup, StructuralBlockLookupResult,
    CHUNK_SHAPE_VERSION, EQUIVALENCE_CONTRACT_VERSION,
    FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ, FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH,
    FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY,
    FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH, LAYOUT_FAMILY_VERSION,
    STRUCTURAL_BLOCK_FAMILY_VERSION,
};
pub use live_query::{
    AcknowledgedContinuationAdvance, AdmittedNarrowBatchReceipt, BroadenedBatchReceipt,
    CaughtUpContinuationBatch, ContinuationAdvanceReceipt, ContinuationBatchBudget,
    ContinuationBatchId, ContinuationBatchResult, ContinuationCompatibilityWitness,
    ContinuationRetentionDescriptor, ContinuationRetentionStatus, ContinuationStrategy,
    ControlLaneBatchReceipt, CursorContinuationPlan, CursorContinuationRequest, FetchWidth,
    LiveQueryBasisEvidence, LiveQueryComplexityStatus, LiveQueryContinuationSessionEvidence,
    MaxBatchItems, MaxCoveredCommits, MaxMaterializedBytes, MaxSupportRowsPerBatch,
    Milestone8CertificationBundle, Milestone8CertificationRequest, Milestone8CertificationSummary,
    Milestone8TruthSurface, StableBasisHandle, StableBasisId, StableBasisLayoutPosture,
    StableBasisReadPlan, StableBasisReadRequest, StableBasisReadScope,
};
pub use maintenance::{
    AdmittedMaintenanceDeclaration, AuthoritativeReclaimMaintenanceDeclaration,
    AdmittedMaintenanceWork, BackgroundPacedMaintenancePlan, BackgroundReservationFamily,
    CancelledMaintenanceWork, CompactionMaintenanceDeclaration, CompletedMaintenance,
    CpuBudgetUnits, DeferredMaintenancePlan, DiscoveredMaintenanceWork,
    EscalatedMaintenancePlan, ExecutingMaintenanceWork, FailedMaintenance,
    ForegroundLatencyGuard, ForegroundReservationFamily, ForegroundReservationWitness,
    ForegroundReservedMaintenancePlan, FreshnessWindow, IoBudgetUnits, LocalityScopeToken,
    MaintenanceAdmissionReceipt, MaintenanceAdmissionRejection, MaintenanceBatch,
    MaintenanceBatchClass, MaintenanceBatchSummary, MaintenanceDebtFamily,
    MaintenanceDeclaration, MaintenanceDeclarationClass, MaintenanceDeclarationId,
    MaintenanceDescriptorDemand, MaintenanceEquivalenceKey, MaintenanceEscalationDecision,
    MaintenanceExecutionPosture, MaintenanceExecutionStatus, MaintenanceExecutionTransition,
    MaintenanceFailureKind, MaintenanceForegroundImpact,
    MaintenanceLocalityScope, MaintenancePlanFamily, MaintenanceQuantum,
    MaintenanceReadmissionStatus,
    MaintenanceReservationFamily, MaintenanceStatusReport, MaintenanceWorkClass,
    MaintenanceWorkDescriptor, MaintenanceWorkIdentity, MemoryBudgetUnits, PacingWindow,
    PlanGeneration, PublicationSlotBudget, QuantumBudgetReceipt,
    RebuildMaintenanceDeclaration, ReclaimMaintenanceDeclaration,
    RecoveredMaintenanceDescriptor, ReservedMaintenanceWork, RestartMaintenanceAdmission,
    RetentionMaintenanceDeclaration, MaintenanceReservationTransition, SupersessionEpoch,
    SupersededMaintenanceWitness, TierWorkContainerClass,
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
    BackupRestoreIncompatibilityKind, BulkRecoveryDisposition, BulkRecoverySummary,
    DegradedStateReport, DurableDegradedRecovery, DurableMutationIdentity,
    DurableRecoveryDegradedKind, DurableRecoveryOutcome, DurableRecoverySourceSummary,
    DurableRetryResolution, MaintenanceArtifactFamily, MaintenanceRecoveryDisposition,
    MaintenanceRecoveryEntry, MaintenanceRecoveryReport, ObservedSnapshotVersionTuple,
    RecoveredBulkChunk, RecoveryOperatorAction, RecoveryOperatorActionKind,
    RecoveryOperatorDisposition, RecoveryQuarantineScope, RecoverySourceKind, RecoverySourceReport,
    RecoveryStatusReport, ResumeEligibleRecoveredBulkChunk, SnapshotMaintenanceRecoveryAction,
    SnapshotMaintenanceRecoveryReport, SupportArtifactFamily, SupportArtifactRecoveryDisposition,
    SupportArtifactRecoveryEntry, SupportArtifactRecoveryReport,
};
pub use retention::{
    AggressiveRetentionDebtMarker, AuthoritativeRangeReclaimUnit, AuthoritativeReclaimReport,
    BasisSurvivalVerdict, BranchHistoryWindowPolicy, CompactionBackedRetentionPlan,
    CompactionCandidateRejection, CompactionCutoverReport, CompactionCutoverWitness,
    CompactionPlan, CompactionPublicationReport, ConservativeRetentionPlan,
    ConservativeRetentionPolicy, DeltaLayerCompactionUnit, DerivedFamilyReclaimUnit,
    DerivedFamilyRetentionPolicy, LayoutFamilyCompactionUnit, LoweredCompactionDeclaration,
    LoweredRebuildDeclaration, LoweredReclaimDeclaration, LoweredRetentionMaintenanceBatch,
    PinnedSnapshotPolicy, PolicyExpiredAuthorityRange, PublishedCompactionProduct,
    RebuildDebtSummary, RebuildRequiredRetentionPlan, ReclaimEligibilityWitness,
    ReclaimExecutionReport, RetainedAuthoritativeRange, RetainedHeadSet,
    RetainedRangeRebuildReport, RetainedRangeRebuildUnit, RetainedReadCostSurface,
    RetainedReadPath, RetentionCandidatePlan, RetentionClosureSummary, RetentionClosureWitness,
    RetentionMaintenanceVerification, RetentionPlanningReport, RetentionPolicyClass,
    RetentionTargetStateVerification, SnapshotCompactionUnit, StableBasisSet,
    SupersededPhysicalFamily, COMPACTION_PRODUCT_FAMILY_VERSION, RETENTION_FAMILY_VERSION,
};
pub use snapshot::{
    PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId, SnapshotImageBundle,
    SnapshotReadMode, SnapshotReadRequest, SnapshotReadResult, SnapshotRestoreOutcome,
    SnapshotRestorePlan, SnapshotRestoreRequest,
};
pub use tiering::{
    AdaptivePlacementDebtMarker, AuthoritativeTierMovePlan, AuthoritativeTierMoveUnit,
    AuthoritativeTierResidency, AuthoritativePlacementPlanningReport, BroadenedRecallPlan,
    CanonicalResidencyManifest, ColdDerivedFamilyPolicy, ColdRecallLease, ColdRecallTierPath,
    ConservativePlacementPolicy, DeltaRecallUnit, DerivedTierMovePlan,
    DerivedPlacementPlanningReport, DerivedTierMoveUnit, DerivedTierResidency,
    FamilyLocalPlacementPlan, FamilyLocalRecallUnit,
    HotnessClassificationVerdict, LayoutFamilyRecallUnit, PlacementArtifactFamily,
    PlacementBoundArtifactRef, PlacementBudgetClass, PlacementDemandSummary,
    PlacementExecutionOrigin, PlacementNonAuthorityWitness, PlacementObservationScopeClass,
    PlacementObservationUnit,
    PlacementPolicyClass, PlacementResolvedReadHandle, PlacementStabilityPlan,
    ReadPlacementPlanningReport, RecallAmplificationBudget, RecallBreadthSummary,
    RecallCoalescingKey, RecallCompletionWitness, RecallCostClass, RecallEligibilityWitness,
    RecallPreparationPlan, ResidentReadLease, RetainedRangePlacementPlan,
    RetiredTierReplica, SchedulerPlacementWorkToken, SnapshotRecallUnit,
    TIERING_FAMILY_VERSION, TierCoolingCandidate, TierCutoverWitness,
    TierLocalityFootprint, TierMoveBreadthSummary, TierMoveRejection,
    TierPlacementEvidence, TierPromotionCandidate, TierResidenceClass,
    TierTransferIntent, TransferredTierReplica, VerifiedTierReplica,
    WorkingSetDebtSummary, WorkingSetObservationWindow,
};
pub use wal::{DurableMutationId, DurablePublicationPhase, RecoveryDecisionClass};

#[cfg(test)]
mod tests;
