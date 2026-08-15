use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::data::dependency::{DependencySnapshotShapeStore, DependencySnapshotStore};
use crate::data::graph::{DependencyEdgeStore, ReverseSubscriptionIndex, SubscriberEdgeStore};
use crate::data::handle::NodeId;

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
    pub(in crate::data::graph) pending_revalidation_waiters: BTreeMap<NodeId, BTreeSet<NodeId>>,
}
