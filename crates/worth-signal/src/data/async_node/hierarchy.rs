use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceCancellationReport, ResourceRequestHandle,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncNodeHierarchyReplaySummary {
    root_node: NodeId,
    hierarchy_nodes: Vec<NodeId>,
    active_request_handles: Vec<ResourceRequestHandle>,
    hierarchy_depth: u32,
    lifecycle_digest: String,
    in_flight_digest: String,
    replay_digest: String,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl AsyncNodeHierarchyReplaySummary {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        root_node: NodeId,
        hierarchy_nodes: Vec<NodeId>,
        active_request_handles: Vec<ResourceRequestHandle>,
        hierarchy_depth: u32,
        lifecycle_digest: String,
        in_flight_digest: String,
        replay_digest: String,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            root_node,
            hierarchy_nodes,
            active_request_handles,
            hierarchy_depth,
            lifecycle_digest,
            in_flight_digest,
            replay_digest,
            performance,
        }
    }

    pub fn root_node(&self) -> NodeId {
        self.root_node
    }

    pub fn hierarchy_nodes(&self) -> &[NodeId] {
        &self.hierarchy_nodes
    }

    pub fn active_request_handles(&self) -> &[ResourceRequestHandle] {
        &self.active_request_handles
    }

    pub fn hierarchy_depth(&self) -> u32 {
        self.hierarchy_depth
    }

    pub fn lifecycle_digest(&self) -> &str {
        &self.lifecycle_digest
    }

    pub fn in_flight_digest(&self) -> &str {
        &self.in_flight_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncNodeHierarchyCancellationReport {
    root_node: NodeId,
    affected_nodes: Vec<NodeId>,
    propagated_hierarchy_width: u32,
    replay_digest: String,
    cancellation: ResourceCancellationReport,
}

impl AsyncNodeHierarchyCancellationReport {
    pub(crate) fn new(
        root_node: NodeId,
        affected_nodes: Vec<NodeId>,
        propagated_hierarchy_width: u32,
        replay_digest: String,
        cancellation: ResourceCancellationReport,
    ) -> Self {
        Self {
            root_node,
            affected_nodes,
            propagated_hierarchy_width,
            replay_digest,
            cancellation,
        }
    }

    pub fn root_node(&self) -> NodeId {
        self.root_node
    }

    pub fn affected_nodes(&self) -> &[NodeId] {
        &self.affected_nodes
    }

    pub fn propagated_hierarchy_width(&self) -> u32 {
        self.propagated_hierarchy_width
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn cancellation(&self) -> &ResourceCancellationReport {
        &self.cancellation
    }
}
