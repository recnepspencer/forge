mod delivery_replay_snapshot;
mod mapping_freeze;
mod patch_routing;

pub use delivery_replay_snapshot::{
    BridgeDeliveryContext, BridgeReplayContext, BridgeSnapshotContext,
};
pub use mapping_freeze::BridgeMappingFreezeContext;
pub use patch_routing::{BridgePatchContext, BridgeRoutingContext};

use crate::mapping::{
    BridgeAspectRegistrationId, BridgeMappingId, SliceWideningPolicy, SubscriptionSliceKind,
    TruthDeltaSurfaceKind,
};
use crate::routing::{
    BridgeInvalidationIdentity, BridgeRouteIdentity, BridgeSubscriptionSliceIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

use super::coordinates::{BridgePatchTargetCoordinate, BridgeSnapshotReadCoordinate};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BridgeErrorContext {
    #[default]
    None,
    MappingFreeze(BridgeMappingFreezeContext),
    Patch(BridgePatchContext),
    Routing(BridgeRoutingContext),
    Delivery(BridgeDeliveryContext),
    Replay(BridgeReplayContext),
    Snapshot(BridgeSnapshotContext),
}

impl BridgeErrorContext {
    pub fn patch(patch_target_coordinate: BridgePatchTargetCoordinate) -> Self {
        Self::Patch(BridgePatchContext::new(patch_target_coordinate))
    }

    pub fn mapping_freeze(context: BridgeMappingFreezeContext) -> Self {
        Self::MappingFreeze(context)
    }

    pub fn routing(patch_target_coordinate: BridgePatchTargetCoordinate) -> Self {
        Self::Routing(BridgeRoutingContext::new(patch_target_coordinate))
    }

    pub fn delivery(
        route_identity: BridgeRouteIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        Self::Delivery(BridgeDeliveryContext::new(
            route_identity,
            snapshot_identity,
        ))
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

    pub(crate) fn with_invalidation_identity(
        self,
        invalidation_identity: BridgeInvalidationIdentity,
    ) -> Self {
        match self {
            Self::Delivery(context) => {
                Self::Delivery(context.with_invalidation_identity(invalidation_identity))
            }
            Self::Replay(context) => {
                Self::Replay(context.with_invalidation_identity(invalidation_identity))
            }
            other => other,
        }
    }

    pub(crate) fn with_subscription_slice_identity(
        self,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    ) -> Self {
        match self {
            Self::Delivery(context) => Self::Delivery(
                context.with_subscription_slice_identity(subscription_slice_identity),
            ),
            Self::Replay(context) => {
                Self::Replay(context.with_subscription_slice_identity(subscription_slice_identity))
            }
            other => other,
        }
    }

    pub fn patch_target_coordinate(&self) -> Option<&BridgePatchTargetCoordinate> {
        match self {
            Self::Patch(context) => Some(context.patch_target_coordinate()),
            Self::Routing(context) => Some(context.patch_target_coordinate()),
            _ => None,
        }
    }

    pub fn truth_surface_kind(&self) -> Option<TruthDeltaSurfaceKind> {
        match self {
            Self::Routing(context) => context.truth_surface_kind(),
            _ => None,
        }
    }

    pub fn mapping_id(&self) -> Option<&BridgeMappingId> {
        match self {
            Self::Routing(context) => context.mapping_id(),
            _ => None,
        }
    }

    pub fn aspect_registration_id(&self) -> Option<&BridgeAspectRegistrationId> {
        match self {
            Self::Routing(context) => context.aspect_registration_id(),
            _ => None,
        }
    }

    pub fn slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        match self {
            Self::Routing(context) => context.slice_kind(),
            _ => None,
        }
    }

    pub fn slice_widening_policy(&self) -> Option<SliceWideningPolicy> {
        match self {
            Self::Routing(context) => context.slice_widening_policy(),
            _ => None,
        }
    }

    pub fn mapping_freeze_context(&self) -> Option<&BridgeMappingFreezeContext> {
        match self {
            Self::MappingFreeze(context) => Some(context),
            _ => None,
        }
    }

    pub fn snapshot_read(&self) -> Option<&BridgeSnapshotReadCoordinate> {
        match self {
            Self::Delivery(context) => context.snapshot_read(),
            _ => None,
        }
    }

    pub fn route_identity(&self) -> Option<&BridgeRouteIdentity> {
        match self {
            Self::Delivery(context) => Some(context.route_identity()),
            Self::Replay(context) => Some(context.route_identity()),
            _ => None,
        }
    }

    pub fn invalidation_identity(&self) -> Option<&BridgeInvalidationIdentity> {
        match self {
            Self::Delivery(context) => context.invalidation_identity(),
            Self::Replay(context) => context.invalidation_identity(),
            _ => None,
        }
    }

    pub fn subscription_slice_identity(&self) -> Option<&BridgeSubscriptionSliceIdentity> {
        match self {
            Self::Delivery(context) => context.subscription_slice_identity(),
            Self::Replay(context) => context.subscription_slice_identity(),
            _ => None,
        }
    }

    pub fn snapshot_identity(&self) -> Option<&TruthSnapshotIdentity> {
        match self {
            Self::Delivery(context) => Some(context.snapshot_identity()),
            Self::Replay(context) => Some(context.snapshot_identity()),
            Self::Snapshot(context) => Some(context.snapshot_identity()),
            _ => None,
        }
    }
}
