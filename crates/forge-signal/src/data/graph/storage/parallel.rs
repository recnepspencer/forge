use crate::data::dependency::{DependencyEdge, DependencySnapshot, DependencySnapshotId};
use crate::data::handle::NodeId;

use super::super::signal_graph::SignalGraph;
use super::{DependencySetId, SubscriberSetId};

impl SignalGraph {
    pub(crate) fn store_dependency_snapshot(
        &mut self,
        snapshot: DependencySnapshot,
    ) -> DependencySnapshotId {
        self.topology.dependency_snapshots.insert(snapshot)
    }

    pub(crate) fn store_dependency_edges(
        &mut self,
        edges: &[DependencyEdge],
    ) -> DependencySetId {
        self.topology.dependency_edges.insert_from_slice(edges)
    }

    pub(crate) fn store_subscribers(&mut self, subscribers: &[NodeId]) -> SubscriberSetId {
        self.topology.subscriber_edges.insert_from_slice(subscribers)
    }

}
