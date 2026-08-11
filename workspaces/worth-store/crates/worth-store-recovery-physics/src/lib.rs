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
mod entry;
mod integrity_damage_map;
mod integrity_handoff;
mod integrity_input;
mod integrity_vetted_records;
mod layout_readmission;
mod memory_allocation;
mod offline_verifier;
mod operation_reconciliation;
mod page_redo;
mod partial_publication;
mod point_in_time_recovery;
mod publication;
mod recovery_blocking_integrity;
mod recovery_budget;
mod recovery_completion;
mod recovery_evidence;
mod recovery_integrity_handoff_receipt;
mod redo_replay;
mod replay_basis;
mod replay_receipt;
mod replica_bootstrap_source;
mod rollback_recovery;
mod security_metadata_admission;
#[cfg(test)]
mod security_metadata_tests;
mod security_scope_propagation;
mod source_precedence;
mod wal_prefix;
mod wal_recovery_basis;
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
    CheckpointValidationDenial, CheckpointValidationDenialKind, DurableRootSelector,
    FuzzyCheckpointCertificationModeDenial, FuzzyCheckpointCertificationModeDenialKind,
    LocatedCheckpointCandidate, RecoveredCheckpointCutoverState, RecoveredCheckpointManifestMedia,
    RecoveredCheckpointRoot, RecoveredCheckpointSelector, SharpCheckpointCertificationMode,
    StoreOwnedCheckpointLocator, SuperblockRingCheckpointPointer,
};
pub use corruption_readmission::{
    admit_recovery_corruption_readmission, classify_recovery_repair_capability,
    verify_quarantine_handoff_for_readmission, verify_store_authority_for_readmission,
    RecoveryCorruptionReadmissionDenial, RecoveryCorruptionReadmissionHandoff,
    RecoveryCorruptionRepairCapability,
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
pub use memory_allocation::{
    RecoveryMemoryAllocation, RecoveryMemoryCounterSnapshot, RecoveryMemoryObservation,
};
pub use offline_verifier::{
    CheckpointManifestBudgetMaterialization, CheckpointManifestMaterialization,
    CheckpointManifestRecoveryBasisMaterialization, CheckpointManifestSourceMaterialization,
    CheckpointPageImageMaterialization, FreshRuntimeRecoveryDriver, FreshRuntimeRecoveryExecution,
    FreshRuntimeRecoveryWitness, FreshRuntimeReopenHarnessDenial,
    FreshRuntimeReopenHarnessEvidence, OfflineRecoveryVerificationReport,
    OfflineRecoveryVerifierConclusion, PersistedRecoveryArtifactDenial,
    PersistedRecoveryArtifactDigest, PersistedRecoveryArtifactMaterialization,
    PersistedRecoveryArtifacts, RecoveryDeterminismClassification, RecoveryDeterminismReport,
    RecoveryNondeterministicMetadata, RecoveryOfflineVerifier, RecoveryOfflineVerifierDenial,
    RecoveryPersistedRecord, RecoveryPersistedRecordRole, RecoveryProfileId,
    RecoveryRuntimeClassification, RecoveryRuntimePosture, ReopenedRecoveryArtifactAdmission,
    ReopenedRecoveryArtifactAdmissionDenial, ReopenedRuntimeBoundaryEvidence,
    ReopenedRuntimeRecoverySession, RuntimeRecoveryComparisonClassification,
    RuntimeRecoveryComparisonReport, RuntimeRecoveryReport, RuntimeRecoveryReportDenial,
    WalRedoFrameMaterialization,
};
pub use operation_reconciliation::{
    classify_binding_freshness, reconcile_materialized_operation_fates, reconcile_operation_fates,
    OperationReconciliationDenial, ReconciledOperationFate, ReconciledOperationFates,
    RecoveryBindingFreshness, RecoveryOperationEvidenceInput, RecoveryOperationFate,
    RecoveryOperationIdentity,
};
pub use page_redo::{
    PageLsn, PageRedoApplicationBasis, PageRedoCounterSnapshot, PageRedoDenial, PageRedoDenialKind,
    PageRedoDigestState, PageRedoEligibility, PageRedoEligibilityKind,
};
pub use partial_publication::{
    AmbiguousPublicationReport, NonAuthoritativePublicationDenial,
    NonAuthoritativePublicationSource, PartialPublicationBeforeWalReplayRead,
    PartialPublicationClassification, PartialPublicationCounterSnapshot,
    PartialPublicationCrashEdge, PartialPublicationEvidence,
    PartialPublicationObservationAdmission, PartialPublicationObservationSet,
    PartialPublicationObservedSource, PartialPublicationPersistedBytes,
    PartialPublicationReplayReadArtifact, PartialPublicationReplayReadDenial,
    PartialPublicationReplayReadRecord, PartialPublicationReplayReadWitness,
    PartialPublicationReplayedCrashEdge, RecoveredOrRejectedPartialPublication,
    TornPublicationDenial, UnacknowledgedDurableWal, UnacknowledgedPublicationOutcome,
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
    admit_recovery_plan_cost, AdmittedRecoveryWorkBounds, BoundedRecoveryPlan,
    BoundedRecoveryReceipt, BoundedRecoverySourceAdmission, CheckpointIntervalContract,
    ForbiddenFullStoreScanRejection, RecoveryBudget, RecoveryBudgetDenial,
    RecoveryBudgetDenialKind, RecoveryCounterSnapshot, RecoveryPlanCost, RecoveryPlanCostDenial,
    RecoveryPlanLimits, RecoveryPlanningCounters, RecoveryStoreFootprint, ReopenedRecoveryDenial,
    WalTailReplayBudget,
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
    admit_physical_redo_members, decode_physical_redo_records,
    physical_redo_observation_target_identities, physical_redo_observation_targets,
    physical_redo_target_identities, plan_physical_redo, AdmittedPhysicalRedoMembers,
    AdmittedRedoFrame, ImmutablePhysicalRedoPlan, MiddleWalCorruptionDenial,
    MissingAcknowledgedWalRangeDenial, PhysicalRedoAdmissionLimits, PhysicalRedoDecision,
    PhysicalRedoDecisionKind, PhysicalRedoDecisionPrior, PhysicalRedoDecisionView,
    PhysicalRedoExtentCoordinate, PhysicalRedoGroupBinding, PhysicalRedoMemberInput,
    PhysicalRedoPlanCounters, PhysicalRedoPlanningDenial, PhysicalRedoProjection,
    PhysicalRedoRecord, PhysicalRedoTarget, PhysicalRedoTargetIdentity, RecoveredPhysicalState,
    RecoveryPageObservation, RecoveryPageSource, RecoveryRedoPlan, RedoApplicationCursor,
    RedoApplicationPageFact, RedoExecutionReceipt, RedoPlanCounterExpectation, RedoPlanningDenial,
    RedoPlanningDenialKind, RedoRecordGrammar, RedoRecordGrammarDenial,
    RedoRecordGrammarDenialKind, RedoRecordIdempotenceBasis, RedoRecordIntegrityBinding,
    RedoRecordMaterializedForm, RedoRecordOperationForm, RedoRecordTargetGeneration,
    SkippedRedoFrameReport, StaleWalGenerationDenial, TornWalTailClassification,
    WalPrefixIntegrityObservation, WalPrefixObservationScan, WalValidPrefix,
    WalValidPrefixCounters,
};
pub use replay_basis::{
    DurabilityReplayIdentity, DurabilityReplayIdentityDenial, DurabilityReplayKind,
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
    admit_physical_page_facts, admit_physical_root_slot, admit_physical_wal_tail,
    inspect_physical_wal_artifacts, select_current_previous_root, select_physical_recovery_sources,
    AdmittedCompactionCutoverDurability, AdmittedCompactionCutoverRecord, AdmittedRecoverySource,
    BackendResidueKind, BackendResidueRejection, CheckpointBaseAdmission,
    CompactionArtifactResidueReason, CompactionArtifactResidueRejection,
    CompactionCutoverRecoveryPosture, CompactionGenerationIdentity, CompactionGenerationVisibility,
    CompactionVisibleProductEvidence, CompactionVisibleProductEvidenceDenial,
    ContiguousWalTailProof, InspectedPhysicalWalArtifacts, PageLsnSkipApplyDecision,
    PhysicalCheckpointBase, PhysicalCheckpointBaseDenial, PhysicalManifestBlockCandidate,
    PhysicalPageFactDenial, PhysicalRecoveryResidue, PhysicalRecoveryResidueKind,
    PhysicalRecoverySource, PhysicalRootCandidateDenial, PhysicalRootSelectionDenial,
    PhysicalRootSlotObservation, PhysicalRootSourceCandidate, PhysicalSourceSelection,
    PhysicalSourceSelectionDenial, PhysicalSourceSelectionTrace, PhysicalWalArtifactCorruption,
    PhysicalWalArtifactInspectionDenial, PhysicalWalSegmentCandidate,
    RecoverableOldCompactionGeneration, RecoveryCandidateDiscoveryTrace,
    RecoverySourceApplicationRole, RecoverySourceCandidate, RecoverySourceDecisionKind,
    RecoverySourceDecisionOutcome, RecoverySourceDecisionRow, RecoverySourceDecisionTrace,
    SelectedCompactionProduct, SelectedPhysicalPageFacts, SelectedPhysicalRoot,
    SelectedPhysicalRootRole, SelectedPhysicalWalTail, SelectedPhysicalWalTailDenial,
    WalOnlyTailProof, WalOnlyTailProofDenial, WalTailIntegrityQuarantineHandoff, WalTailRedoSource,
};
pub use staged_wal_application::{
    StagedWalApplicationDenial, StagedWalApplicationPort, StagedWalApplicationProviderReceipt,
    StagedWalApplicationReceipt, StagedWalApplicationRequest,
};
pub use staged_wal_replay_source::{StagedWalReplaySourceDenial, StagedWalReplaySourceReceipt};
#[cfg(feature = "certification-test-authority")]
pub use wal_recovery_basis::WalAppendFailureObservation;
pub use wal_recovery_basis::{
    ReopenedWalDurabilityCrashRecord, WalAppendObservationScope, WalAppendReceipt,
    WalDurabilityCrashBasis, WalDurabilityCrashPosture, WalDurabilityCrashRecord,
    WalDurabilityObservation, WalDurabilityObservationBasis, WalDurabilityObservationDenial,
    WalDurabilityObservationDenialKind, WalFrameDigest,
};
pub use wal_topology::{LogSequenceNumber, WalLsnRange, WalSegmentGeneration, WalSegmentId};
pub use worth_store_contracts::CorruptionHandoffDamageCase;
pub use worth_store_wal::AdmittedReplayTailCursor;
