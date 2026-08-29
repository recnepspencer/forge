use serde::{Deserialize, Serialize};

use crate::data::graph::runtime::scratch::{ScratchLeaseKind, TraversalScratch};
use crate::data::handle::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct TraversalResources {
    #[serde(skip, default)]
    pub(in crate::data::graph) scratch: TraversalScratch,
    #[serde(skip, default)]
    pub(in crate::data::graph) scratch_lease: Option<ScratchLeaseKind>,
    #[serde(skip, default)]
    pub(in crate::data::graph) topology_node_buffer: Vec<NodeId>,
}
