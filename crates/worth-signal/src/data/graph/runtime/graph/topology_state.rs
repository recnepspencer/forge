use crate::data::dependency::{DependencySnapshotShapeStore, DependencySnapshotStore};
use crate::data::graph::{DependencyEdgeStore, ReverseSubscriptionIndex, SubscriberEdgeStore};
use crate::data::handle::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct EdgeTopology {
    #[serde(default)]
    pub(in crate::data::graph) dependency_snapshots: DependencySnapshotStore,
    #[serde(default)]
    pub(in crate::data::graph) dependency_snapshot_shapes: DependencySnapshotShapeStore,
    #[serde(default)]
    pub(in crate::data::graph) dependency_edges: DependencyEdgeStore,
    #[serde(default)]
    pub(in crate::data::graph) subscriber_edges: SubscriberEdgeStore,
    #[serde(skip, default)]
    pub(in crate::data::graph) reverse_subscriptions: ReverseSubscriptionIndex,
    #[serde(skip, default)]
    pub(in crate::data::graph) pending_revalidation_waiters: im::OrdMap<NodeId, im::OrdSet<NodeId>>,
}

impl EdgeTopology {
    pub(crate) fn operational_clone(&self) -> Self {
        Self {
            dependency_snapshots: self.dependency_snapshots.operational_clone(),
            dependency_snapshot_shapes: self.dependency_snapshot_shapes.operational_clone(),
            dependency_edges: self.dependency_edges.operational_clone(),
            subscriber_edges: self.subscriber_edges.operational_clone(),
            reverse_subscriptions: self.reverse_subscriptions.operational_clone(),
            pending_revalidation_waiters: self
                .pending_revalidation_waiters
                .iter()
                .map(|(node, waiters)| (*node, waiters.iter().copied().collect::<im::OrdSet<_>>()))
                .collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        self.dependency_snapshots
            .shares_storage_with(&other.dependency_snapshots)
            && self
                .dependency_snapshot_shapes
                .shares_storage_with(&other.dependency_snapshot_shapes)
            && self
                .dependency_edges
                .shares_storage_with(&other.dependency_edges)
            && self
                .subscriber_edges
                .shares_storage_with(&other.subscriber_edges)
            && self
                .reverse_subscriptions
                .shares_storage_with(&other.reverse_subscriptions)
            && self
                .pending_revalidation_waiters
                .ptr_eq(&other.pending_revalidation_waiters)
    }
}
