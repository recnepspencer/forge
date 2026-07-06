#![doc = include_str!("recovery_physics_compile_fail_proofs.md")]
#![forbid(unsafe_code)]

mod blob_replay;
mod checkpoint_cutover;
mod durable_publication;
mod integrity_damage_map;
mod integrity_input;
mod integrity_vetted_records;
mod memory_envelope;
mod offline_verifier;
mod page_lsn_publication;
mod partial_publication;
mod recovery_blocking_integrity;
mod recovery_budget;
mod recovery_entry_admission;
mod recovery_entry_basis;
mod recovery_entry_counters;
mod recovery_entry_denial;
mod recovery_entry_identity;
mod recovery_entry_input_classification;
mod recovery_evidence;
mod recovery_integrity_handoff_receipt;
mod recovery_replay_entry_gate;
mod redo_replay;
mod replay_receipt;
mod s4_closeout;
mod s4_integrity_handoff_admission;
mod s4_integrity_handoff_denial;
mod s4_integrity_handoff_payload;
mod s4_recovery_physics_integrity_readiness;
mod s5_publication_recovery;
mod security_metadata_admission;
#[cfg(test)]
mod security_metadata_tests;
mod security_scope_propagation;
mod source_precedence;
mod wal_durability;
mod wal_topology;

pub use blob_replay::{
    BlobReplayAdmissionDenial, BlobReplayAdmissionDenialKind, BlobReplaySourceAdmission,
    BlobReplaySourceKind, BlobReplaySourceOutcome, BlobReplaySourceOutcomeKind,
    BlobResumeReplayReadmission,
};
pub use checkpoint_cutover::{
    CheckpointArtifactDurabilityCommitment, CheckpointCandidate,
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
pub use durable_publication::{
    CheckpointCrashDurabilityPosture, DurabilityRecoveryReplaySource,
    DurabilityRecoverySourcePrecedence, DurabilityReplayIdentity, DurabilityReplayKind,
    DurableCheckpointPublication, DurableManifestPublication, DurableWalPublication,
    StoreDurablePublicationDenial, StoreDurablePublicationDenialKind,
};
pub use integrity_damage_map::{IntegrityDamageMap, QuarantineSummary};
pub use integrity_input::RecoveryPhysicsIntegrityInput;
pub use integrity_vetted_records::{
    IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameKind, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame,
};
pub use memory_envelope::{RecoveryMemoryEnvelope, RecoveryMemoryEnvelopeDenial};
pub use offline_verifier::{
    FreshRuntimeRecoveryDriver, FreshRuntimeRecoveryExecution, FreshRuntimeRecoveryWitness,
    FreshRuntimeReopenHarnessDenial, FreshRuntimeReopenHarnessEvidence,
    OfflineRecoveryVerificationReport, OfflineRecoveryVerifierConclusion,
    PersistedRecoveryArtifactDenial, PersistedRecoveryArtifactDigest, PersistedRecoveryArtifacts,
    RecoveryDeterminismClassification, RecoveryDeterminismReport, RecoveryNondeterministicMetadata,
    RecoveryOfflineVerifier, RecoveryOfflineVerifierDenial, RecoveryPersistedRecord,
    RecoveryPersistedRecordRole, RecoveryProfileId, RecoveryRuntimeClassification,
    RecoveryRuntimePosture, ReopenedRecoveryArtifactAdmission,
    ReopenedRecoveryArtifactAdmissionDenial, ReopenedRuntimeBoundaryEvidence,
    ReopenedRuntimeRecoverySession, RuntimeRecoveryComparisonClassification,
    RuntimeRecoveryComparisonReport, RuntimeRecoveryReport, RuntimeRecoveryReportDenial,
    S4CheckpointManifestMaterialization, S4CheckpointPageImageMaterialization,
    S4PersistedRecoveryArtifactMaterialization, S4WalRedoFrameMaterialization,
};
pub use page_lsn_publication::{
    DirtyPublicationEvidence, NoUndoPublicationEligibility, NoUndoPublicationProof,
    PageFlushRecoveryReceipt, PageLsn, PageLsnPublicationCounterSnapshot, PageRedoApplicationBasis,
    PageRedoDigestState, PageRedoEligibility, PageRedoEligibilityKind,
    ReopenedPageRecoveryEvidence, RollbackImagePublicationDeclaration,
    RollbackImagePublicationPosture, StalePageRecoveryClassification,
    StalePageRecoveryClassificationKind, UnadmittedDirtyPagePublicationDenial,
    UnadmittedDirtyPagePublicationDenialKind, WalBeforeDataOrderingProof,
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
pub use recovery_blocking_integrity::{
    RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource,
};
pub use recovery_budget::{
    AdmittedRecoveryWorkBounds, BoundedRecoveryPlan, BoundedRecoveryReceipt,
    BoundedRecoverySourceAdmission, CheckpointIntervalContract, ForbiddenFullStoreScanRejection,
    RecoveryBudget, RecoveryBudgetDenial, RecoveryBudgetDenialKind, RecoveryCounterSnapshot,
    RecoveryStoreFootprint, ReopenedRecoveryDenial, WalTailReplayBudget,
};
pub use recovery_entry_admission::RecoveryEntryAdmission;
pub use recovery_entry_basis::RecoveryEntryBasis;
pub use recovery_entry_counters::RecoveryEntryCounters;
pub use recovery_entry_denial::{
    RecoveryEntryAdmissionDecision, RecoveryEntryAdmissionDenial, RecoveryEntryAdmissionDenialKind,
    RecoveryEntryBlockedByIntegrityDamage,
};
pub use recovery_entry_identity::RecoveryEntryIdentity;
pub(crate) use recovery_entry_input_classification::{
    classify_recovery_entry_inputs, RecoveryEntryInputClassification,
};
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
pub use recovery_replay_entry_gate::RecoveryReplayEntryGate;
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
pub use s4_closeout::{
    CrashSeamRecoveryObservation, FreshRuntimeCrashRecoveryEvidence, RecoveryBoundednessEvidence,
    RecoveryPhysicsCertificationBundle, RecoveryPhysicsCloseoutCollector,
    RecoveryPhysicsCloseoutDenial, RecoveryPhysicsCloseoutEvidence, RecoveryPhysicsCloseoutReport,
    RecoveryPhysicsCloseoutSuiteLane, RecoveryPhysicsCloseoutSuiteRequirement,
    RecoveryPhysicsCloseoutSuiteStatus, RecoveryPhysicsStabilityAssumption, RecoveryWorkBound,
    S4CrashFaultSchedulerEvidence, S4CrashHarnessTranscriptSource, S4LoweredCrashHarnessEvidence,
    S4RecoveryCrashSeam, S5PhysicalIsolationRecoveryReadiness, S5RecoveryReadinessAdmission,
    S5RecoveryReadinessDenial, SyntheticRecoveryShortcutEvidence, SyntheticRecoveryShortcutKind,
    SyntheticRecoveryShortcutRejection, SyntheticRecoveryShortcutRejectionBoundary,
    SyntheticRecoveryShortcutRejectionReport, WalCheckpointLsnRecoveryPhysicsSuite,
};
pub use s4_integrity_handoff_admission::S4IntegrityHandoffAdmission;
pub use s4_integrity_handoff_denial::{S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind};
pub use s4_integrity_handoff_payload::{
    BoundedInspectionEnvelopeEvidence, RawBytesExcludedFromRecoveryHandoff,
    S4ChecksumAlgorithmScopeBasis, S4IntegrityHandoffCounters, S4IntegrityHandoffPayload,
    S4IntegrityHandoffPayloadDeclaration,
};
pub use s4_recovery_physics_integrity_readiness::S4RecoveryPhysicsIntegrityReadiness;
pub use s5_publication_recovery::{
    ExecutedS5PublicationRecoveryReceipt, S5PublicationCrashStage,
    S5PublicationRecoveryReplayInput, S5RecoveredPublicationStructure,
    S5RecoveredPublicationStructureKind,
};
pub use security_metadata_admission::RecoveryRootSecurityMetadataAdmission;
pub use security_scope_propagation::{
    RecoveryCheckpointRecordSecurityMetadataEnvelope,
    RecoveryCheckpointRecordSecurityMetadataIdentity, RecoveryRootSecurityMetadataEnvelope,
    RecoverySecurityScopePropagation, RecoverySecurityScopePropagationCounters,
    RecoverySecurityScopePropagationDenial, RecoverySecurityScopePropagationInput,
    RecoveryWalRecordSecurityMetadataEnvelope, RecoveryWalRecordSecurityMetadataIdentity,
};
pub use source_precedence::{
    AdmittedCompactionCutoverDurability, AdmittedCompactionCutoverRecord, AdmittedRecoverySource,
    BackendResidueKind, BackendResidueRejection, CheckpointBaseAdmission,
    CompactionArtifactResidueReason, CompactionArtifactResidueRejection,
    CompactionCutoverRecoveryPosture, CompactionGenerationIdentity, CompactionGenerationVisibility,
    CompactionVisibleProductEvidence, CompactionVisibleProductEvidenceDenial,
    PageLsnSkipApplyDecision, RecoverableOldCompactionGeneration, RecoveryCandidateDiscoveryTrace,
    RecoverySourceApplicationRole, RecoverySourceCandidate, RecoverySourceDecisionKind,
    RecoverySourceDecisionOutcome, RecoverySourceDecisionRow, RecoverySourceDecisionTrace,
    RecoverySourcePrecedenceGraph, WalOnlyTailProof, WalOnlyTailProofDenial,
    WalTailIntegrityQuarantineHandoff, WalTailRedoSource,
};
pub use wal_durability::{
    AcknowledgmentPrecondition, DurableAckBasis, DurableAckReceipt, IllegalAcknowledgmentDenial,
    IllegalAcknowledgmentDenialKind, WalAppendDurabilityScope, WalAppendPlan, WalAppendProgress,
    WalAppendReceipt, WalDurabilityCrashBasis, WalDurabilityCrashPosture, WalDurabilityCrashRecord,
    WalDurabilityObservation, WalDurabilityObservationSequence, WalFrameDigest,
};
pub use wal_topology::{
    LogSequenceNumber, ReplayCursor, ReplayCursorSegment, WalFrameOrderingProof, WalLsnRange,
    WalSegmentGeneration, WalSegmentId, WalSegmentScanRecord, WalTopologyDenial,
    WalTopologyDenialKind, WalTopologyScan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoverySource {
    Checkpoint,
    WalTail,
    Manifest,
    Quarantine,
}
