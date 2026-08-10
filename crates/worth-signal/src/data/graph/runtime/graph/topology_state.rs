use serde::{Deserialize, Serialize};

use crate::data::dependency::{DependencySnapshotShapeStore, DependencySnapshotStore};
use crate::data::graph::{DependencyEdgeStore, SubscriberEdgeStore};

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
}
