use serde::{Deserialize, Serialize};

use crate::data::topology_view::{
    WorthTopologyBody, WorthTopologyEdge, WorthTopologyFace, WorthTopologyHalfEdge,
    WorthTopologyLoop, WorthTopologyLump, WorthTopologyModel, WorthTopologyRegion,
    WorthTopologyShell, WorthTopologyVertex, WorthTopologyWire,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyView {
    pub models: Vec<WorthTopologyModel>,
    pub bodies: Vec<WorthTopologyBody>,
    pub lumps: Vec<WorthTopologyLump>,
    pub regions: Vec<WorthTopologyRegion>,
    pub shells: Vec<WorthTopologyShell>,
    pub faces: Vec<WorthTopologyFace>,
    pub loops: Vec<WorthTopologyLoop>,
    pub wires: Vec<WorthTopologyWire>,
    pub half_edges: Vec<WorthTopologyHalfEdge>,
    pub edges: Vec<WorthTopologyEdge>,
    pub vertices: Vec<WorthTopologyVertex>,
}
