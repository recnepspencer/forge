#![doc = include_str!("recovery_physics_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

mod staged_wal_application;
mod staged_wal_replay_source;

pub mod layout_projection;

mod backup_restore;
mod blob_replay;
mod btree_replay;
mod candidate_evaluation;
mod checkpoint_cutover;
mod corruption_readmission;
mod durable_publication;
mod entry;
mod integrity_damage_map;
mod integrity_handoff;
mod integrity_input;
mod integrity_vetted_records;
mod layout_readmission;
mod memory_envelope;
mod offline_verifier;
mod page_lsn_publication;
mod partial_publication;
mod point_in_time_recovery;
mod publication;
mod recovery_blocking_integrity;
mod recovery_budget;
mod recovery_completion;
mod recovery_evidence;
mod recovery_integrity_handoff_receipt;
mod redo_replay;
mod replay_receipt;
mod replica_bootstrap_source;
mod rollback_recovery;
mod security_metadata_admission;
#[cfg(test)]
mod security_metadata_tests;
mod security_scope_propagation;
mod source_precedence;
mod wal_durability;
mod wal_topology;

pub use backup_restore::{
    BackupRestoreReplayDenial, BackupRestoreReplayPlan, BackupRestoreReplayRequest,
    RecoveredBackupFrontierReceipt, RecoveryPhysicsBackupRestoreOwner,
};
pub use blob_replay::{
    BlobReplayAdmissionDenial, BlobReplayAdmissionDenialKind, BlobReplaySourceAdmission,
    BlobReplaySourceKind, BlobReplaySourceOutcome, BlobReplaySourceOutcomeKind,
    BlobResumeReplayReadmission,
};
pub use btree_replay::{
    AdmittedBTreeReplayPhysicalSource, AdmittedBTreeReplaySource,
    BTreeReplayPhysicalSourceIdentity, BTreeReplayRootAgreement, BTreeReplaySourceDenial,
};
pub use candidate_evaluation::{
    discover_recovery_candidates, ObservedRecoveryFrontier, RecoveryCandidate,
    RecoveryCandidateConfidence, RecoveryCandidateDiscoveryDenial, RecoveryCandidateObservation,
    RecoveryCandidateSet,
};
pub use checkpoint_cutover::{
    verify_bounded_checkpoint_backup_artifact,
    verify_bounded_checkpoint_backup_artifact_from_reader, BoundedCheckpointBackupDenial,
    BoundedCheckpointBackupObservation, BoundedCheckpointBackupVerificationRequest,
    CheckpointArtifactDurabilityCommitment, CheckpointBackupArtifact, CheckpointCandidate,
    CheckpointCandidateDiscoverySource, CheckpointCoveredLsnRange, CheckpointCutoverCrashStage,
    CheckpointCutoverReceipt, CheckpointCutoverRecoverySelection,
    CheckpointCutoverRecoverySelectionKind, CheckpointDurabilityEvidence,
    CheckpointDurabilityEvidenceSet, CheckpointDurabilityRole, CheckpointId, CheckpointLocator,
    CheckpointLocatorArtifactCommitment, CheckpointManifest, CheckpointPageLsnFrontier,
    CheckpointPublicationPlan, CheckpointRecoveryCounterSnapshot, CheckpointRedoBoundary,
    CheckpointRootPosture, CheckpointSelectorEvidence, CheckpointValidation,
    CheckpointValidationDenial, CheckpointValidationDenialKind, ContiguousWalTailProof,
    DurableRootSelector, FuzzyCheckpointCertificationModeDenial,
    FuzzyCheckpointCertificationModeDenialKind, LocatedCheckpointCandidate,
    RecoveredCheckpointCutoverState, RecoveredCheckpointManifestMedia, RecoveredCheckpointRoot,
    RecoveredCheckpointSelector, SharpCheckpointCertificationMode, StoreOwnedCheckpointLocator,
    SuperblockRingCheckpointPointer, WalRetentionAction, WalRetentionAdmittedAction,
    WalRetentionCandidateSegment, WalRetentionEligibility, WalRetentionRequest,
};
pub use corruption_readmission::{
    admit_recovery_corruption_readmission, classify_recovery_repair_capability,
    verify_quarantine_handoff_for_readmission, verify_store_authority_for_readmission,
    RecoveryCorruptionReadmissionDenial, RecoveryCorruptionReadmissionHandoff,
    RecoveryCorruptionRepairCapability,
};
#[cfg(feature = "certification-test-authority")]
pub use durable_publication::certified_durable_wal_publication_for_test;
pub use durable_publication::{
    CheckpointCrashDurabilityPosture, DurabilityRecoveryReplaySource,
    DurabilityRecoverySourcePrecedence, DurabilityReplayIdentity, DurabilityReplayKind,
    DurableCheckpointPublication, DurableManifestPublication, DurableWalPublication,
    StoreDurablePublicationDenial, StoreDurablePublicationDenialKind,
};
pub use entry::admission::RecoveryEntryAdmission;
pub use entry::basis::RecoveryEntryBasis;
pub use entry::counters::RecoveryEntryCounters;
pub use entry::denial::{
    RecoveryEntryAdmissionDecision, RecoveryEntryAdmissionDenial, RecoveryEntryAdmissionDenialKind,
    RecoveryEntryBlockedByIntegrityDamage,
};
pub use entry::identity::RecoveryEntryIdentity;
pub(crate) use entry::input_classification::{
    classify_recovery_entry_inputs, RecoveryEntryInputClassification,
};
pub use entry::replay_gate::RecoveryReplayEntryGate;
pub use integrity_damage_map::{
    classify_recovery_blocking_damage, IntegrityDamageMap, QuarantineSummary,
};
pub use integrity_handoff::damage_map;
pub use integrity_handoff::AdmittedRecoveryIntegrityInput;
pub use integrity_handoff::{
    BoundedInspectionEnvelopeEvidence, ChecksumAlgorithmScopeBasis, IntegrityHandoffAdmission,
    IntegrityHandoffCounters, IntegrityHandoffDeclaration, IntegrityHandoffDenial,
    IntegrityHandoffDenialKind, IntegrityHandoffPayload, RawBytesExcludedFromRecoveryHandoff,
};
pub use integrity_input::RecoveryPhysicsIntegrityInput;
pub use integrity_vetted_records::{
    IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameKind, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame,
};
pub use layout_projection::{
    ensure_recovery_entry_allowed, reject_decision_row, reject_locator_projection,
    BoundedWalTailLayoutReport, CheckpointCutoverLayoutReport,
    CheckpointRecoveryManifestLayoutReport, CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial,
    RecoveryLayoutAccessDenialKind, RecoveryReadmissionLayoutReport, RecoverySourceLayoutReport,
    ReplayIndexLayoutCounters, ReplayIndexLayoutReport,
};
pub use layout_readmission::{
    layout_readmission, ImportLayoutReadmissionOutcome, LayoutReadmissionAuthority,
    OfflineLayoutReadmissionOutcome, QuarantineLayoutReadmissionOutcome,
    RecoveryLayoutReadmissionAdmissionDenial, RecoveryLayoutReadmissionClass,
    RecoveryLayoutReadmissionIdentity, RecoveryLayoutReadmissionOutcomeView,
    RecoveryLayoutReadmissionWitness,
};
pub use memory_envelope::{
    RecoveryMemoryCounterSnapshot, RecoveryMemoryEnvelope, RecoveryMemoryEnvelopeDenial,
};
pub use offline_verifier::{
    CheckpointManifestBudgetMaterialization, CheckpointManifestMaterialization,
    CheckpointManifestSourceMaterialization, CheckpointPageImageMaterialization,
    FreshRuntimeRecoveryDriver, FreshRuntimeRecoveryExecution, FreshRuntimeRecoveryWitness,
    FreshRuntimeReopenHarnessDenial, FreshRuntimeReopenHarnessEvidence,
    OfflineRecoveryVerificationReport, OfflineRecoveryVerifierConclusion,
    PersistedRecoveryArtifactDenial, PersistedRecoveryArtifactDigest,
    PersistedRecoveryArtifactMaterialization, PersistedRecoveryArtifacts,
    RecoveryDeterminismClassification, RecoveryDeterminismReport, RecoveryNondeterministicMetadata,
    RecoveryOfflineVerifier, RecoveryOfflineVerifierDenial, RecoveryPersistedRecord,
    RecoveryPersistedRecordRole, RecoveryProfileId, RecoveryRuntimeClassification,
    RecoveryRuntimePosture, ReopenedRecoveryArtifactAdmission,
    ReopenedRecoveryArtifactAdmissionDenial, ReopenedRuntimeBoundaryEvidence,
    ReopenedRuntimeRecoverySession, RuntimeRecoveryComparisonClassification,
    RuntimeRecoveryComparisonReport, RuntimeRecoveryReport, RuntimeRecoveryReportDenial,
    WalRedoFrameMaterialization,
};
pub use page_lsn_publication::{
    DirtyPublicationEvidence, NoUndoPublicationEligibility, NoUndoPublicationProof,
    PageFlushRecoveryReceipt, PageLsn, PageLsnPublicationCounterSnapshot, PageRedoApplicationBasis,
    PageRedoDigestState, PageRedoEligibility, PageRedoEligibilityKind,
    PhysicalDirtyPublicationCounters, RecoveryDirtyPageIdentity, ReopenedPageRecoveryEvidence,
    RollbackImagePublicationDeclaration, RollbackImagePublicationPosture,
    StalePageRecoveryClassification, StalePageRecoveryClassificationKind,
    UnadmittedDirtyPagePublicationDenial, UnadmittedDirtyPagePublicationDenialKind,
    WalBeforeDataOrderingProof,
};
pub use partial_publication::{
    AmbiguousPublicationReport, NoUndoPartialPublicationClassification,
    NonAuthoritativePublicationDenial, NonAuthoritativePublicationSource,
    PartialPublicationBeforeWalReplayRead, PartialPublicationClassification,
    PartialPublicationCounterSnapshot, PartialPublicationCrashEdge, PartialPublicationEvidence,
    PartialPublicationObservationAdmission, PartialPublicationObservationSet,
    PartialPublicationObservedSource, PartialPublicationPersistedBytes,
    PartialPublicationReplayReadArtifact, PartialPublicationReplayReadDenial,
    PartialPublicationReplayReadRecord, PartialPublicationReplayReadWitness,
    PartialPublicationReplayedCrashEdge, RecoveredOrRejectedPartialPublication,
    RollbackImageRequiredPosture, TornPublicationDenial, UnacknowledgedDurableWal,
    UnacknowledgedPublicationOutcome, UnadmittedDurablePageMutationDenial,
};
pub use point_in_time_recovery::{
    ExactRecoveryFrontier, FrontierPartialOrder, PitrCandidatePosture,
    PitrCandidateSelectionDenial, PitrRoundingPolicy, PointInTimeCandidate,
    PointInTimeCandidateSet, PointInTimeRecoveryReceipt, PointInTimeReplayDenial,
    PointInTimeReplayPlan, PointInTimeReplayRequest, PointInTimeReplaySourceCoordinates,
    RecoveryPhysicsPointInTimeOwner, RecoveryPhysicsTimelineAuthority, RecoveryTimelineObservation,
};
pub use publication::{
    ExecutedPublicationRecoveryReceipt, PublicationCrashStage, PublicationRecoveryReplayInput,
    RecoveredPublicationStructure, RecoveredPublicationStructureKind,
};
pub use recovery_blocking_integrity::{
    RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource,
};
pub use recovery_budget::{
    AdmittedRecoveryWorkBounds, BoundedRecoveryPlan, BoundedRecoveryReceipt,
    BoundedRecoverySourceAdmission, CheckpointIntervalContract, ForbiddenFullStoreScanRejection,
    RecoveryBudget, RecoveryBudgetDenial, RecoveryBudgetDenialKind, RecoveryCounterSnapshot,
    RecoveryStoreFootprint, ReopenedRecoveryDenial, WalTailReplayBudget,
};
pub use recovery_completion::{complete_recovery, RecoveryCompletion, RecoveryCompletionDenial};
pub use recovery_evidence::{
    deny_non_applicable_surface, CurrentBasisRecoveryEvidencePosture,
    FoundationalRecoveryEvidenceBundle, NonApplicableFoundationalSurface,
    ProofProgressionRecoveryTrace, RecoveryAdmissionMechanism,
    RecoveryAttachedCounterBackedPerformanceReceipt, RecoveryCertifiedDiagnosticSupportBundle,
    RecoveryCertifiedPerformanceBundle, RecoveryCounterPerformanceReceipt,
    RecoveryCurrentBasisBoundaryBundle, RecoveryCurrentBasisEvidence,
    RecoveryEvidenceCanonicalBasis, RecoveryEvidenceConstructionSource, RecoveryEvidenceDenial,
    RecoveryEvidenceLineagePosture, RecoveryEvidenceLineageReport, RecoveryEvidencePayloadKind,
    RecoveryEvidenceRichness, RecoveryMaterializedPerformanceReport, RecoveryPerformanceSurface,
    RecoveryPerformanceSurfaceKind, RecoveryPhysicsEvidenceSource, RecoveryPhysicsReceipt,
    RecoveryPhysicsReport, RecoveryProofProgressionStep, RecoveryProofSourceFamily,
    RecoverySourceDecisionReport, RecoverySourceDiagnosticKind, StoreRecoveryEvidenceAuthority,
    NON_APPLICABLE_FOUNDATIONAL_SURFACES, RECOVERY_ADMISSION_MECHANISMS,
};
pub use recovery_integrity_handoff_receipt::RecoveryIntegrityHandoffReceipt;
pub use redo_replay::{
    AdmittedRedoFrame, MiddleWalCorruptionDenial, MissingAcknowledgedWalRangeDenial,
    RecoveredPhysicalState, RecoveryRedoPlan, RedoApplicationCursor, RedoApplicationPageFact,
    RedoExecutionReceipt, RedoPlanCounterExpectation, RedoPlanningDenial, RedoPlanningDenialKind,
    RedoRecordGrammar, RedoRecordGrammarDenial, RedoRecordGrammarDenialKind,
    RedoRecordIdempotenceBasis, RedoRecordIntegrityBinding, RedoRecordMaterializedForm,
    RedoRecordOperationForm, RedoRecordTargetGeneration, SkippedRedoFrameReport,
    StaleWalGenerationDenial, TornWalTailClassification, WalPrefixIntegrityObservation,
    WalPrefixObservationScan, WalValidPrefix, WalValidPrefixCounters,
};
pub use replay_receipt::{CheckpointValidityDecision, WalReplayReceipt};
pub use replica_bootstrap_source::{
    BootstrapSourceArtifact, BootstrapSourceArtifactFamily, BootstrapSourceEvidenceBinding,
    BootstrapSourceFrontier, BootstrapSourceResolutionCounters, BootstrapSourceResolutionDenial,
    BootstrapSourceResolutionRequest, RecoveryPhysicsBootstrapSourceOwner,
    ResolvedBootstrapRecoverySourceCut,
};
pub use rollback_recovery::{
    RecoveryPhysicsRollbackOwner, ResolvedRollbackCandidate, RollbackExecutionReceipt,
    RollbackReplayDenial, RollbackReplayPlan,
};
pub use security_metadata_admission::RecoveryRootSecurityMetadataAdmission;
pub use security_scope_propagation::{
    RecoveryCheckpointRecordSecurityMetadataEnvelope,
    RecoveryCheckpointRecordSecurityMetadataIdentity, RecoveryRootSecurityMetadataEnvelope,
    RecoverySecurityScopePropagation, RecoverySecurityScopePropagationCounters,
    RecoverySecurityScopePropagationDenial, RecoverySecurityScopePropagationInput,
    RecoveryWalRecordSecurityMetadataEnvelope, RecoveryWalRecordSecurityMetadataIdentity,
};
#[cfg(feature = "certification-test-authority")]
pub use source_precedence::RecoverySourcePrecedenceGraph;
pub use source_precedence::{
    AdmittedCompactionCutoverDurability, AdmittedCompactionCutoverRecord, AdmittedRecoverySource,
    BackendResidueKind, BackendResidueRejection, CheckpointBaseAdmission,
    CompactionArtifactResidueReason, CompactionArtifactResidueRejection,
    CompactionCutoverRecoveryPosture, CompactionGenerationIdentity, CompactionGenerationVisibility,
    CompactionVisibleProductEvidence, CompactionVisibleProductEvidenceDenial,
    PageLsnSkipApplyDecision, PhysicalRecoverySource, RecoverableOldCompactionGeneration,
    RecoveryCandidateDiscoveryTrace, RecoverySourceApplicationRole, RecoverySourceCandidate,
    RecoverySourceDecisionKind, RecoverySourceDecisionOutcome, RecoverySourceDecisionRow,
    RecoverySourceDecisionTrace, WalOnlyTailProof, WalOnlyTailProofDenial,
    WalTailIntegrityQuarantineHandoff, WalTailRedoSource,
};
pub use staged_wal_application::{
    StagedWalApplicationDenial, StagedWalApplicationPort, StagedWalApplicationProviderReceipt,
    StagedWalApplicationReceipt, StagedWalApplicationRequest,
};
pub use staged_wal_replay_source::{StagedWalReplaySourceDenial, StagedWalReplaySourceReceipt};
#[cfg(feature = "certification-test-authority")]
pub use wal_durability::execute_wal_durability_with_boundary_control;
pub use wal_durability::{
    execute_wal_durability, AcknowledgmentPrecondition, DurableAckBasis, DurableAckReceipt,
    ExecutedWalDurabilityOutcome, IllegalAcknowledgmentDenial, IllegalAcknowledgmentDenialKind,
    WalAppendDurabilityScope, WalAppendPlan, WalAppendProgress, WalAppendReceipt,
    WalDurabilityCrashBasis, WalDurabilityCrashPosture, WalDurabilityCrashRecord,
    WalDurabilityExecutionError, WalDurabilityObservation, WalDurabilityObservationSequence,
    WalFrameDigest,
};
pub use wal_topology::{LogSequenceNumber, WalLsnRange, WalSegmentGeneration, WalSegmentId};
pub use worth_store_contracts::CorruptionHandoffDamageCase;
pub use worth_store_wal::AdmittedReplayTailCursor;
