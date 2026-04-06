use std::sync::Arc;
use std::marker::PhantomData;

use crate::mapping::{
    BridgeAspectRegistrationId, BridgeMappingId, SliceFallbackPolicy, SubscriptionSliceKind,
    TruthDeltaSurfaceKind,
};
use crate::routing::{
    BridgeInvalidationIdentity, BridgeRouteIdentity, BridgeSubscriptionSliceIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePatchCoordinate {
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Arc<str>,
}

impl BridgePatchCoordinate {
    pub fn new(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: surface_label.into(),
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSnapshotReadCoordinate {
    request_key: Arc<str>,
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Option<Arc<str>>,
    slice_kind: Option<SubscriptionSliceKind>,
}

impl BridgeSnapshotReadCoordinate {
    pub fn new_coarse(
        request_key: impl Into<Arc<str>>,
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            request_key: request_key.into(),
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: None,
            slice_kind: None,
        }
    }

    pub fn new_subscription_slice(
        request_key: impl Into<Arc<str>>,
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
    ) -> Self {
        Self {
            request_key: request_key.into(),
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: Some(surface_label.into()),
            slice_kind: Some(slice_kind),
        }
    }

    pub fn request_key(&self) -> &str {
        self.request_key.as_ref()
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> Option<&str> {
        self.surface_label.as_deref()
    }

    pub fn slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        self.slice_kind.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePatchContext {
    patch_coordinate: BridgePatchCoordinate,
}

impl BridgePatchContext {
    pub fn new(patch_coordinate: BridgePatchCoordinate) -> Self {
        Self { patch_coordinate }
    }

    pub fn patch_coordinate(&self) -> &BridgePatchCoordinate {
        &self.patch_coordinate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoutingContext {
    patch_coordinate: BridgePatchCoordinate,
    truth_surface_kind: Option<TruthDeltaSurfaceKind>,
    mapping_id: Option<BridgeMappingId>,
    aspect_registration_id: Option<BridgeAspectRegistrationId>,
    slice_kind: Option<SubscriptionSliceKind>,
    slice_fallback_policy: Option<SliceFallbackPolicy>,
}

impl BridgeRoutingContext {
    pub fn new(patch_coordinate: BridgePatchCoordinate) -> Self {
        Self {
            patch_coordinate,
            truth_surface_kind: None,
            mapping_id: None,
            aspect_registration_id: None,
            slice_kind: None,
            slice_fallback_policy: None,
        }
    }

    pub fn with_truth_surface_kind(mut self, truth_surface_kind: TruthDeltaSurfaceKind) -> Self {
        self.truth_surface_kind = Some(truth_surface_kind);
        self
    }

    pub fn with_mapping_id(mut self, mapping_id: BridgeMappingId) -> Self {
        self.mapping_id = Some(mapping_id);
        self
    }

    pub fn with_aspect_registration_id(
        mut self,
        aspect_registration_id: BridgeAspectRegistrationId,
    ) -> Self {
        self.aspect_registration_id = Some(aspect_registration_id);
        self
    }

    pub fn with_slice_kind(mut self, slice_kind: SubscriptionSliceKind) -> Self {
        self.slice_kind = Some(slice_kind);
        self
    }

    pub fn with_slice_fallback_policy(
        mut self,
        slice_fallback_policy: SliceFallbackPolicy,
    ) -> Self {
        self.slice_fallback_policy = Some(slice_fallback_policy);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeDeliveryContext {
    route_identity: BridgeRouteIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    invalidation_identity: Option<BridgeInvalidationIdentity>,
    subscription_slice_identity: Option<BridgeSubscriptionSliceIdentity>,
    snapshot_read: Option<BridgeSnapshotReadCoordinate>,
}

impl BridgeDeliveryContext {
    pub fn new(route_identity: BridgeRouteIdentity, snapshot_identity: TruthSnapshotIdentity) -> Self {
        Self {
            route_identity,
            snapshot_identity,
            invalidation_identity: None,
            subscription_slice_identity: None,
            snapshot_read: None,
        }
    }

    pub(crate) fn with_invalidation_identity(
        mut self,
        invalidation_identity: BridgeInvalidationIdentity,
    ) -> Self {
        self.invalidation_identity = Some(invalidation_identity);
        self
    }

    pub(crate) fn with_subscription_slice_identity(
        mut self,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        self.subscription_slice_identity = Some(subscription_slice_identity);
        self
    }

    pub(crate) fn with_snapshot_read(mut self, snapshot_read: BridgeSnapshotReadCoordinate) -> Self {
        self.snapshot_read = Some(snapshot_read);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeReplayContext {
    route_identity: BridgeRouteIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    invalidation_identity: Option<BridgeInvalidationIdentity>,
    subscription_slice_identity: Option<BridgeSubscriptionSliceIdentity>,
}

impl BridgeReplayContext {
    pub fn new(route_identity: BridgeRouteIdentity, snapshot_identity: TruthSnapshotIdentity) -> Self {
        Self {
            route_identity,
            snapshot_identity,
            invalidation_identity: None,
            subscription_slice_identity: None,
        }
    }

    pub(crate) fn with_invalidation_identity(
        mut self,
        invalidation_identity: BridgeInvalidationIdentity,
    ) -> Self {
        self.invalidation_identity = Some(invalidation_identity);
        self
    }

    pub(crate) fn with_subscription_slice_identity(
        mut self,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        self.subscription_slice_identity = Some(subscription_slice_identity);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSnapshotContext {
    snapshot_identity: TruthSnapshotIdentity,
}

impl BridgeSnapshotContext {
    pub fn new(snapshot_identity: TruthSnapshotIdentity) -> Self {
        Self { snapshot_identity }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeErrorContext {
    None,
    Patch(BridgePatchContext),
    Routing(BridgeRoutingContext),
    Delivery(BridgeDeliveryContext),
    Replay(BridgeReplayContext),
    Snapshot(BridgeSnapshotContext),
}

impl Default for BridgeErrorContext {
    fn default() -> Self {
        Self::None
    }
}

impl BridgeErrorContext {
    pub fn patch(patch_coordinate: BridgePatchCoordinate) -> Self {
        Self::Patch(BridgePatchContext::new(patch_coordinate))
    }

    pub fn routing(patch_coordinate: BridgePatchCoordinate) -> Self {
        Self::Routing(BridgeRoutingContext::new(patch_coordinate))
    }

    pub fn delivery(
        route_identity: BridgeRouteIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        Self::Delivery(BridgeDeliveryContext::new(route_identity, snapshot_identity))
    }

    pub fn replay(
        route_identity: BridgeRouteIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        Self::Replay(BridgeReplayContext::new(route_identity, snapshot_identity))
    }

    pub fn snapshot(snapshot_identity: TruthSnapshotIdentity) -> Self {
        Self::Snapshot(BridgeSnapshotContext::new(snapshot_identity))
    }

    pub(crate) fn with_snapshot_read(self, snapshot_read: BridgeSnapshotReadCoordinate) -> Self {
        match self {
            Self::Delivery(context) => Self::Delivery(context.with_snapshot_read(snapshot_read)),
            other => other,
        }
    }

    pub(crate) fn with_invalidation_identity(self, invalidation_identity: BridgeInvalidationIdentity) -> Self {
        match self {
            Self::Delivery(context) => Self::Delivery(context.with_invalidation_identity(invalidation_identity)),
            Self::Replay(context) => Self::Replay(context.with_invalidation_identity(invalidation_identity)),
            other => other,
        }
    }

    pub(crate) fn with_subscription_slice_identity(
        self,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        match self {
            Self::Delivery(context) => {
                Self::Delivery(context.with_subscription_slice_identity(subscription_slice_identity))
            }
            Self::Replay(context) => {
                Self::Replay(context.with_subscription_slice_identity(subscription_slice_identity))
            }
            other => other,
        }
    }

    pub fn patch_coordinate(&self) -> Option<&BridgePatchCoordinate> {
        match self {
            Self::Patch(context) => Some(context.patch_coordinate()),
            Self::Routing(context) => Some(&context.patch_coordinate),
            _ => None,
        }
    }

    pub fn truth_surface_kind(&self) -> Option<TruthDeltaSurfaceKind> {
        match self {
            Self::Routing(context) => context.truth_surface_kind,
            _ => None,
        }
    }

    pub fn mapping_id(&self) -> Option<&BridgeMappingId> {
        match self {
            Self::Routing(context) => context.mapping_id.as_ref(),
            _ => None,
        }
    }

    pub fn aspect_registration_id(&self) -> Option<&BridgeAspectRegistrationId> {
        match self {
            Self::Routing(context) => context.aspect_registration_id.as_ref(),
            _ => None,
        }
    }

    pub fn slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        match self {
            Self::Routing(context) => context.slice_kind.as_ref(),
            _ => None,
        }
    }

    pub fn slice_fallback_policy(&self) -> Option<SliceFallbackPolicy> {
        match self {
            Self::Routing(context) => context.slice_fallback_policy,
            _ => None,
        }
    }

    pub fn snapshot_read(&self) -> Option<&BridgeSnapshotReadCoordinate> {
        match self {
            Self::Delivery(context) => context.snapshot_read.as_ref(),
            _ => None,
        }
    }

    pub fn route_identity(&self) -> Option<&BridgeRouteIdentity> {
        match self {
            Self::Delivery(context) => Some(&context.route_identity),
            Self::Replay(context) => Some(&context.route_identity),
            _ => None,
        }
    }

    pub fn invalidation_identity(&self) -> Option<&BridgeInvalidationIdentity> {
        match self {
            Self::Delivery(context) => context.invalidation_identity.as_ref(),
            Self::Replay(context) => context.invalidation_identity.as_ref(),
            _ => None,
        }
    }

    pub fn subscription_slice_identity(&self) -> Option<&BridgeSubscriptionSliceIdentity> {
        match self {
            Self::Delivery(context) => context.subscription_slice_identity.as_ref(),
            Self::Replay(context) => context.subscription_slice_identity.as_ref(),
            _ => None,
        }
    }

    pub fn snapshot_identity(&self) -> Option<&TruthSnapshotIdentity> {
        match self {
            Self::Delivery(context) => Some(&context.snapshot_identity),
            Self::Replay(context) => Some(&context.snapshot_identity),
            Self::Snapshot(context) => Some(&context.snapshot_identity),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTypedError<K> {
    kind: K,
    message: Arc<str>,
    context: BridgeErrorContext,
}

impl<K: Copy> BridgeTypedError<K> {
    pub(crate) fn new(kind: K, message: impl Into<Arc<str>>) -> Self {
        Self {
            kind,
            message: message.into(),
            context: BridgeErrorContext::default(),
        }
    }

    pub fn kind(&self) -> K {
        self.kind
    }

    pub(crate) fn with_context(mut self, context: BridgeErrorContext) -> Self {
        self.context = context;
        self
    }

    pub fn context(&self) -> &BridgeErrorContext {
        &self.context
    }
}

impl<K> std::fmt::Display for BridgeTypedError<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<K: std::fmt::Debug> std::error::Error for BridgeTypedError<K> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMessageError<Tag> {
    message: Arc<str>,
    _tag: PhantomData<Tag>,
}

impl<Tag> BridgeMessageError<Tag> {
    pub fn new(message: impl Into<Arc<str>>) -> Self {
        Self {
            message: message.into(),
            _tag: PhantomData,
        }
    }
}

impl<Tag> std::fmt::Display for BridgeMessageError<Tag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<Tag: std::fmt::Debug> std::error::Error for BridgeMessageError<Tag> {}

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
