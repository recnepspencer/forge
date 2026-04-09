use super::typed::BridgeTypedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeBuildErrorKind {
    MissingRelationalSource,
    MissingSignalSink,
    MissingMappingRegistrations,
    MissingSourceAdapter,
    DuplicateMappingRegistration,
    AmbiguousMappingRegistration,
    DuplicateAspectRegistration,
    AmbiguousAspectRegistration,
    DuplicateSourceDeclaration,
    AmbiguousSourceDeclaration,
    DuplicateStructuralDeclaration,
    AmbiguousStructuralDeclaration,
    DuplicateMergeDeclaration,
    AmbiguousMergeDeclaration,
    MergeOntologyLoweringMismatch,
    MergeAuthorityBasisMismatch,
    StructuralComparisonModeMismatch,
    SourceCapabilityMismatch,
    BuilderConfigurationConflict,
    InvalidFineGrainedFallbackPolicy,
}

pub type BridgeBuildError = BridgeTypedError<BridgeBuildErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeRouteErrorKind {
    EmptyBulkWorkloadRequest,
    UnsupportedProducerEnvelope,
    MissingMappingRegistration,
    AmbiguousSliceMappingRegistration,
    UnsupportedTruthPatchScope,
    UnsupportedTruthDeltaSurface,
    UnsupportedSubscriptionSlice,
    InconsistentNormalizedSurfaceDigest,
    SliceReadPacketConstructionFailure,
    InvalidLoweringContract,
}

pub type BridgeRouteError = BridgeTypedError<BridgeRouteErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeDeliveryErrorKind {
    SourceContractMismatch,
    InvalidFallbackAdmission,
    BulkDeliveryRejected,
    HistoricalPolicyRejected,
    HistoricalTruthViewUnavailable,
    HistoricalBranchMismatch,
    HistoricalCommitMismatch,
    HistoricalSelectorMissingCommit,
    SnapshotAcquisitionFailure,
    SnapshotReadFailure,
    SnapshotReadContractViolation,
    SnapshotIdentityMismatch,
    StructuralContractMismatch,
    StructuralPlanRejected,
    SignalSinkRejection,
}

pub type BridgeDeliveryError = BridgeTypedError<BridgeDeliveryErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeReplayErrorKind {
    ReplayArtifactsDisabled,
    CanonicalArtifactCompatibilityFailure,
    StructuralReplayBasisTruncated,
    BulkPlanReplayMismatch,
    HistoricalEvaluationDeclarationMismatch,
    HistoricalEvaluationPolicyMismatch,
    HistoricalEvaluationAuthorityMismatch,
    RouteMismatch,
    InvalidationMismatch,
    SubscriptionSliceMismatch,
    ContinuityRequestMismatch,
    ContinuityResolutionMismatch,
    ContinuityArtifactMismatch,
    DigestMismatch,
    PlanningContractMismatch,
    LoweringContractMismatch,
}

pub type BridgeReplayError = BridgeTypedError<BridgeReplayErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeStreamErrorKind {
    UnsupportedConsumerShape,
    UnsupportedResumeMode,
    ProtocolVersionMismatch,
    CheckpointContractMismatch,
    CheckpointStreamMismatch,
    CheckpointTruncated,
    IllegalCoalescingBoundary,
    NonIdempotentDuplicateObservation,
    BackpressurePolicyViolation,
    StreamReplayMismatch,
    StreamDeliveryRejected,
    InvalidStreamMaterial,
}

pub type BridgeStreamError = BridgeTypedError<BridgeStreamErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeContinuityErrorKind {
    MissingLineageContext,
    MissingLineageSource,
    UnsupportedContinuityClass,
    InvalidContinuityRequestSet,
    LineageAuthorityMismatch,
    HistoricalResolutionFailure,
}

pub type BridgeContinuityError = BridgeTypedError<BridgeContinuityErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeMergeErrorKind {
    MergeContractMismatch,
    MergeContinuityDenied,
    MergeStructuralContradiction,
    MergeCausalFrontierTruncated,
    MergePolicyRejected,
    MergeDeletionDenied,
    MergeTopologyRewireDenied,
}

pub type BridgeMergeError = BridgeTypedError<BridgeMergeErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSpeculationErrorKind {
    PreviewRequestKindMismatch,
    PreviewSessionIdentityConflict,
    PreviewBranchBindingMismatch,
    IllegalPreviewLifecycleTransition,
    PromotionAdmissibilityMismatch,
    PreviewReuseEquivalenceMismatch,
    PreviewResidueClassificationMismatch,
}

pub type BridgeSpeculationError = BridgeTypedError<BridgeSpeculationErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeLineageSourceErrorKind {
    UnsupportedContinuityClass,
    HistoricalResolutionFailure,
}

pub type BridgeLineageSourceError = BridgeTypedError<BridgeLineageSourceErrorKind>;
