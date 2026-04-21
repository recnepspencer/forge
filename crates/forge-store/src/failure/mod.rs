use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum StoreErrorKind {
    InvalidRuntimeOwnershipMode,
    EmbeddedModeLifecycleViolation,
    AbsentModeStoreDependencyViolation,
    ModeCapabilityViolation,
    CrossModeCanonicalBoundaryViolation,
    CheckpointCommitSurfaceConfusion,
    EmbeddedCheckpointAuthorityViolation,
    ConflictingAuthorityOwner,
    UnsupportedModeConstruction,
    HostedRuntimeStartupFailure,
    HostedRuntimeShutdownFailure,
    HostedRuntimeReplayPurityViolation,
    ExternalRuntimeArtifactRejection,
    ExternalRuntimeCheckpointRejection,
    ModeSelectionContractViolation,
    HostedRuntimeMutationProducedNoCommit,
    WalRecordCorruption,
    WalCanonicalizationVersionUnsupported,
    WalDigestMismatch,
    DurableRecordFramingInvalid,
    DurableTailTruncated,
    DurableTornWriteDetected,
    DurableBarrierContractViolation,
    DurableDirectoryDurabilityGap,
    DurablePublicationMarkerGap,
    DurableRecordAuthenticityInvalid,
    DurableFamilyVersionUnsupported,
    DurablePublicationStateGap,
    AcknowledgmentBoundaryViolation,
    RecoveryDuplicateSuppressionFailure,
    RecoveryAuthoritativeArtifactMissing,
    RecoveryBranchHeadMismatch,
    RecoveryReplayParityViolation,
    RecoveryRequiresFullRebuild,
    RecoveryIntegrityFailure,
    RecoverySourcePrecedenceViolation,
    RecoverySourceConflict,
    InterruptedMaintenancePublicationGap,
    RecoveryQuiescenceViolation,
    RecoveryQuarantineRequired,
    RecoverySalvageRequired,
    BackupRestoreCompatibilityViolation,
    CompatibilityArtifactFrameMalformed,
    CompatibilityArtifactManifestMalformed,
    CompatibilityManifestPublicationGap,
    CompatibilityArtifactFamilyUndeclared,
    CompatibilityArtifactFormatUnsupported,
    CompatibilityArtifactSemanticVersionUnsupported,
    CompatibilityEdgeMissing,
    CompatibilityAdapterParityFailure,
    CompatibilityAuthoritativePartialTruthRejected,
    CompatibilityDerivedReuseIncompatible,
    CompatibilityDerivedRebuildIncompatible,
    CompatibilityRollingUpgradeRejected,
    CompatibilityRestoreRejected,
    CompatibilityRestoreOutOfScopeScanRejected,
    DisasterRecoverySourceInsufficient,
    RecoveryTrustedTruthAmbiguous,
    RecoveryOperatorDecisionRequired,
    SnapshotBasisAmbiguous,
    SnapshotBasisUnsupported,
    SnapshotCaptureSourceNotImmutable,
    SnapshotPublicationStateGap,
    SnapshotDigestMismatch,
    SnapshotReadBasisMismatch,
    SnapshotUnsupportedReadMode,
    SnapshotRestoreTargetIllegal,
    SnapshotTailRangeGap,
    SnapshotRestoreParityViolation,
    SnapshotRebuildFailure,
    SnapshotRebuildParityViolation,
    SnapshotShadowAuthorityViolation,
    SnapshotFamilyVersionUnsupported,
    SnapshotIntegrityFailure,
    BranchDeltaBasisAmbiguous,
    BranchDeltaBasisUnsupported,
    BranchDeltaPublicationGap,
    BranchDeltaDigestMismatch,
    BranchBaseCopyViolation,
    BranchDeltaReadTargetIllegal,
    BranchDeltaReadBudgetExceeded,
    BranchDeltaTargetRequiresMergeAwareWidening,
    BranchDeltaReplayParityViolation,
    BranchDeltaRewriteTargetIllegal,
    BranchDeltaRewriteBudgetExceeded,
    BranchDeltaRewriteParityViolation,
    BranchDeltaReplacementGap,
    BranchDeltaRebuildFailure,
    BranchDeltaShadowAuthorityViolation,
    ConcurrentArtifactBoundaryViolation,
    BranchDeltaFamilyVersionUnsupported,
    BranchDeltaIntegrityFailure,
    AspectScopeUnsupported,
    AspectScopeAmbiguous,
    AspectLayoutFallbackRequired,
    AspectLayoutReadTargetIllegal,
    AspectLayoutArtifactMissing,
    StructuralBlockEquivalenceViolation,
    PhysicalChunkDeterminismViolation,
    ConcurrentBulkBoundaryViolation,
    ConcurrentSupportBoundaryViolation,
    BulkProgramVersionUnsupported,
    BulkSourceIdentityUnavailable,
    BulkPlanDeterminismViolation,
    BulkCheckpointPublicationGap,
    BulkTransformBasisDrift,
    BulkCanonicalLoweringViolation,
    BulkChunkContractUnsupported,
    BulkChunkWidthBudgetExceeded,
    BulkCheckpointDigestMismatch,
    BulkResumeBoundaryAmbiguous,
    BulkChunkWitnessGap,
    BulkChunkDuplicateCommit,
    CommitSupportPublicationGap,
    RetentionPolicyUnsupported,
    PlacementPolicyUnsupported,
    PlacementExecutionOriginIllegal,
    PlacementRawLocatorBoundaryViolation,
    PlacementWitnessConstructionViolation,
    TierResidencyManifestViolation,
    TierTransferVerificationFailed,
    TierCutoverViolation,
    TierRecallExecutionViolation,
    RetentionClosureViolation,
    RetentionClosureBasisMissing,
    PolicyExpiredRangeIllegal,
    CompactionPlanBasisAmbiguous,
    CompactionCutoverViolation,
    CompactionProductShadowAuthorityViolation,
    ReclaimEligibilityViolation,
    ReclaimLiveBasisConflict,
    BasisSurvivalAmbiguous,
    MaintenanceDeclarationMissing,
    MaintenanceAdmissionViolation,
    MaintenanceLifecycleViolation,
    MaintenanceResumeAmbiguous,
    MaintenanceCheckpointViolation,
    SupportAuthorityTaxonomyViolation,
    SchemaBoundaryArtifactMissing,
    SchemaBoundaryBasisMismatch,
    SchemaBoundaryVersionUnsupported,
    LineageArtifactMissing,
    LineageArtifactDrift,
    HistoricalIdentityResolutionGap,
    StableBasisShapeViolation,
    StableBasisArtifactMissing,
    StableBasisVersionUnsupported,
    StableBasisSchemaMismatch,
    StableBasisSupportContextMismatch,
    StableBasisLayoutPostureViolation,
    StableBasisRetainedStateDegraded,
    StableBasisRetainedStateRejected,
    ContinuationCursorIncompatibility,
    ContinuationBranchIncompatibility,
    ContinuationSchemaIncompatibility,
    ContinuationScopeIncompatibility,
    ContinuationBudgetExceeded,
    ContinuationBatchGap,
    ContinuationBatchDuplicate,
    ContinuationBatchOrderingViolation,
    ContinuationIllegalAdvance,
    CursorEquivalenceViolation,
    CursorCheckpointMissing,
    CursorBasisMismatch,
    CursorSchemaBasisMismatch,
    CursorRegression,
    CursorResumeAmbiguous,
    SubscriberCheckpointConflict,
    CheckpointShapeViolation,
    CheckpointBasisMissing,
    CheckpointContainedCommitMissing,
    CheckpointClassificationUnsupported,
    SupportArtifactRecoveryGap,
    HostedRuntimeRestartMisuse,
    DurableRetryResolutionRequired,
    NonCanonicalEnvelope,
    UnknownBranch,
    OrphanParentReference,
    IllegalBranchHeadTransition,
    DuplicateArtifactIdentity,
    FetchedArtifactDigestMismatch,
    UnsupportedCanonicalizationVersion,
    BackendIntegrityViolation,
    AuthoritativeAppendAtomicityViolation,
    CommitNotFound,
    BranchHeadNotFound,
    Io,
    Serialization,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StoreError {
    kind: StoreErrorKind,
    message: String,
}

impl StoreError {
    pub fn new(kind: StoreErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &StoreErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn unknown_branch(branch_id: &BranchId) -> Self {
        Self::new(
            StoreErrorKind::UnknownBranch,
            format!("branch `{}` is not registered in forge-store", branch_id.0),
        )
    }

    pub fn orphan_parent(commit_id: CommitId, missing_parent: CommitId) -> Self {
        Self::new(
            StoreErrorKind::OrphanParentReference,
            format!(
                "commit {} references missing parent {}",
                commit_id.0, missing_parent.0
            ),
        )
    }

    pub fn duplicate_conflict(commit_id: CommitId) -> Self {
        Self::new(
            StoreErrorKind::DuplicateArtifactIdentity,
            format!(
                "commit {} already exists with a different canonical digest",
                commit_id.0
            ),
        )
    }

    pub fn digest_mismatch(commit_id: CommitId) -> Self {
        Self::new(
            StoreErrorKind::FetchedArtifactDigestMismatch,
            format!(
                "fetched commit {} failed canonical digest verification",
                commit_id.0
            ),
        )
    }

    pub fn backend_integrity(message: impl Into<String>) -> Self {
        Self::new(StoreErrorKind::BackendIntegrityViolation, message)
    }

    pub fn invalid_runtime_ownership(message: impl Into<String>) -> Self {
        Self::new(StoreErrorKind::InvalidRuntimeOwnershipMode, message)
    }

    pub fn mode_capability_violation(message: impl Into<String>) -> Self {
        Self::new(StoreErrorKind::ModeCapabilityViolation, message)
    }

    pub fn embedded_checkpoint_authority_violation(message: impl Into<String>) -> Self {
        Self::new(
            StoreErrorKind::EmbeddedCheckpointAuthorityViolation,
            message,
        )
    }

    pub fn external_runtime_artifact_rejection(message: impl Into<String>) -> Self {
        Self::new(StoreErrorKind::ExternalRuntimeArtifactRejection, message)
    }

    pub fn external_runtime_checkpoint_rejection(message: impl Into<String>) -> Self {
        Self::new(StoreErrorKind::ExternalRuntimeCheckpointRejection, message)
    }

    pub fn recovery_integrity(message: impl Into<String>) -> Self {
        Self::new(StoreErrorKind::RecoveryIntegrityFailure, message)
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StoreError {}

impl From<std::io::Error> for StoreError {
    fn from(value: std::io::Error) -> Self {
        Self::new(StoreErrorKind::Io, value.to_string())
    }
}

impl From<serde_json::Error> for StoreError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(StoreErrorKind::Serialization, value.to_string())
    }
}
