use serde::{Deserialize, Serialize};

use crate::brep::topology_graph::{
    TopologyBody, TopologyEdge, TopologyFace, TopologyHalfEdge, TopologyLoop, TopologyLump,
    TopologyModel, TopologyRegion, TopologyShell, TopologyVertex, TopologyWire,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyView {
    pub models: Vec<TopologyModel>,
    pub bodies: Vec<TopologyBody>,
    pub lumps: Vec<TopologyLump>,
    pub regions: Vec<TopologyRegion>,
    pub shells: Vec<TopologyShell>,
    pub faces: Vec<TopologyFace>,
    pub loops: Vec<TopologyLoop>,
    pub wires: Vec<TopologyWire>,
    pub half_edges: Vec<TopologyHalfEdge>,
    pub edges: Vec<TopologyEdge>,
    pub vertices: Vec<TopologyVertex>,
}




