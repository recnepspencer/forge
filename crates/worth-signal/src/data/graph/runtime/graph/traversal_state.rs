use serde::{Deserialize, Serialize};

use crate::data::bitset::DenseBitset;
use crate::data::dependency::DependencyEdge;
use crate::data::graph::runtime::scratch::{ScratchLeaseKind, TraversalScratch};
use crate::data::handle::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct TraversalResources {
    #[serde(skip, default)]
    pub(in crate::data::graph) scratch: TraversalScratch,
    #[serde(skip, default)]
    pub(in crate::data::graph) scratch_lease: Option<ScratchLeaseKind>,
    #[serde(skip, default)]
    pub(in crate::data::graph) suppression_marks: DenseBitset,
    #[serde(skip, default)]
    pub(in crate::data::graph) topology_node_buffer: Vec<NodeId>,
    #[cfg_attr(not(test), allow(dead_code))]
    #[serde(skip, default)]
    pub(in crate::data::graph) topology_dependency_buffer: Vec<DependencyEdge>,
}
