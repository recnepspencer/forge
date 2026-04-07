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
    RouteMismatch,
    InvalidationMismatch,
    SubscriptionSliceMismatch,
    DigestMismatch,
    PlanningContractMismatch,
    LoweringContractMismatch,
}

pub type BridgeReplayError = BridgeTypedError<BridgeReplayErrorKind>;
