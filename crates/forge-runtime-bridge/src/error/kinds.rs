use super::typed::BridgeTypedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeBuildErrorKind {
    MissingRelationalSource,
    MissingSignalSink,
    MissingMappingRegistrations,
    DuplicateMappingRegistration,
    AmbiguousMappingRegistration,
    DuplicateAspectRegistration,
    AmbiguousAspectRegistration,
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
    InvalidFallbackAdmission,
    BulkDeliveryRejected,
    SnapshotAcquisitionFailure,
    SnapshotReadFailure,
    SnapshotReadContractViolation,
    SnapshotIdentityMismatch,
    SignalSinkRejection,
}

pub type BridgeDeliveryError = BridgeTypedError<BridgeDeliveryErrorKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeReplayErrorKind {
    ReplayArtifactsDisabled,
    CanonicalArtifactCompatibilityFailure,
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
pub enum BridgeLineageSourceErrorKind {
    UnsupportedContinuityClass,
    HistoricalResolutionFailure,
}

pub type BridgeLineageSourceError = BridgeTypedError<BridgeLineageSourceErrorKind>;
