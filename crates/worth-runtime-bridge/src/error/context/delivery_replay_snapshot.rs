use crate::routing::{
    BridgeInvalidationIdentity, BridgeRouteIdentity, BridgeSubscriptionSliceIdentity,
};
use crate::snapshot::TruthSnapshotIdentity;

use super::super::coordinates::BridgeSnapshotReadCoordinate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeDeliveryContext {
    route_identity: BridgeRouteIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    invalidation_identity: Option<BridgeInvalidationIdentity>,
    subscription_slice_identity: Option<BridgeSubscriptionSliceIdentity>,
    snapshot_read: Option<BridgeSnapshotReadCoordinate>,
}

impl BridgeDeliveryContext {
    pub fn new(
        route_identity: BridgeRouteIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
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

    pub(crate) fn with_snapshot_read(
        mut self,
        snapshot_read: BridgeSnapshotReadCoordinate,
    ) -> Self {
        self.snapshot_read = Some(snapshot_read);
        self
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn invalidation_identity(&self) -> Option<&BridgeInvalidationIdentity> {
        self.invalidation_identity.as_ref()
    }

    pub fn subscription_slice_identity(&self) -> Option<&BridgeSubscriptionSliceIdentity> {
        self.subscription_slice_identity.as_ref()
    }

    pub fn snapshot_read(&self) -> Option<&BridgeSnapshotReadCoordinate> {
        self.snapshot_read.as_ref()
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
    pub fn new(
        route_identity: BridgeRouteIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
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

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn invalidation_identity(&self) -> Option<&BridgeInvalidationIdentity> {
        self.invalidation_identity.as_ref()
    }

    pub fn subscription_slice_identity(&self) -> Option<&BridgeSubscriptionSliceIdentity> {
        self.subscription_slice_identity.as_ref()
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

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }
}
