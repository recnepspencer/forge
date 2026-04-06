use crate::routing::{
    BridgeInvalidationIdentity, BridgeInvalidationTarget, BridgeRouteIdentity, BridgeSubscriptionSlice,
};
use crate::snapshot::TruthSnapshotIdentity;

use super::{BridgeRouteRecord, BridgeRouteRecordEntry};

pub type BridgeRouteExplanationEntry = BridgeRouteRecordEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteExplanation {
    route_identity: BridgeRouteIdentity,
    invalidation_identity: BridgeInvalidationIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    subscription_slice_identity: crate::routing::BridgeSubscriptionSliceIdentity,
    route_entries: Vec<BridgeRouteExplanationEntry>,
    subscription_slices: Vec<BridgeSubscriptionSlice>,
    invalidation_targets: Vec<BridgeInvalidationTarget>,
}

impl BridgeRouteExplanation {
    pub(crate) fn from_route_record(record: &BridgeRouteRecord) -> Self {
        Self {
            route_identity: record.route_identity().clone(),
            invalidation_identity: record.invalidation_identity().clone(),
            snapshot_identity: record.source_snapshot().clone(),
            subscription_slice_identity: record.subscription_slice_identity().clone(),
            route_entries: record.entries().to_vec(),
            subscription_slices: record.subscription_slices().iter().cloned().collect(),
            invalidation_targets: record.invalidation_targets().to_vec(),
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        &self.invalidation_identity
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn subscription_slice_identity(&self) -> &crate::routing::BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }

    pub fn route_entries(&self) -> &[BridgeRouteExplanationEntry] {
        &self.route_entries
    }

    pub fn subscription_slices(&self) -> &[BridgeSubscriptionSlice] {
        &self.subscription_slices
    }

    pub fn invalidation_targets(&self) -> &[BridgeInvalidationTarget] {
        &self.invalidation_targets
    }
}
