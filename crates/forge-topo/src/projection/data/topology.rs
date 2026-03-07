use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use forge_spec::facade::SpecNodeId;

use crate::projection::data::handles::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId,
    ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedVertexId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectedEntityRef {
    Body(ProjectedBodyId),
    Lump(ProjectedLumpId),
    Region(ProjectedRegionId),
    Shell(ProjectedShellId),
    Face(ProjectedFaceId),
    Loop(ProjectedLoopId),
    HalfEdge(ProjectedHalfEdgeId),
    Edge(ProjectedEdgeId),
    Vertex(ProjectedVertexId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedBodyData {
    pub spec_id: SpecNodeId,
    pub lumps: Vec<ProjectedLumpId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedLumpData {
    pub spec_id: SpecNodeId,
    pub body: ProjectedBodyId,
    pub regions: Vec<ProjectedRegionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedRegionData {
    pub spec_id: SpecNodeId,
    pub lump: ProjectedLumpId,
    pub shells: Vec<ProjectedShellId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedShellData {
    pub spec_id: SpecNodeId,
    pub region: ProjectedRegionId,
    pub faces: Vec<ProjectedFaceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedFaceData {
    pub spec_id: SpecNodeId,
    pub shell: ProjectedShellId,
    pub outer_loop: ProjectedLoopId,
    pub inner_loops: Vec<ProjectedLoopId>,
    pub surface_binding: Option<SpecNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedLoopData {
    pub spec_id: SpecNodeId,
    pub face: ProjectedFaceId,
    pub half_edge: ProjectedHalfEdgeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedHalfEdgeData {
    pub spec_id: SpecNodeId,
    pub radial_next: ProjectedHalfEdgeId,
    pub next: ProjectedHalfEdgeId,
    pub prev: ProjectedHalfEdgeId,
    pub face: ProjectedFaceId,
    pub origin: ProjectedVertexId,
    pub edge: ProjectedEdgeId,
    pub coedge_binding: Option<SpecNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedEdgeData {
    pub spec_id: SpecNodeId,
    pub half_edge: ProjectedHalfEdgeId,
    pub curve_binding: Option<SpecNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedVertexData {
    pub spec_id: SpecNodeId,
    pub primary_half_edge: Option<ProjectedHalfEdgeId>,
    pub geometry_binding: Option<SpecNodeId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectedTopology {
    pub(crate) bodies: Vec<ProjectedBodyData>,
    pub(crate) lumps: Vec<ProjectedLumpData>,
    pub(crate) regions: Vec<ProjectedRegionData>,
    pub(crate) shells: Vec<ProjectedShellData>,
    pub(crate) faces: Vec<ProjectedFaceData>,
    pub(crate) loops: Vec<ProjectedLoopData>,
    pub(crate) half_edges: Vec<ProjectedHalfEdgeData>,
    pub(crate) edges: Vec<ProjectedEdgeData>,
    pub(crate) vertices: Vec<ProjectedVertexData>,
    #[serde(skip)]
    pub(crate) entity_index: HashMap<SpecNodeId, ProjectedEntityRef>,
}

impl ProjectedTopology {
    pub fn body_count(&self) -> usize { self.bodies.len() }
    pub fn lump_count(&self) -> usize { self.lumps.len() }
    pub fn region_count(&self) -> usize { self.regions.len() }
    pub fn shell_count(&self) -> usize { self.shells.len() }
    pub fn face_count(&self) -> usize { self.faces.len() }
    pub fn loop_count(&self) -> usize { self.loops.len() }
    pub fn half_edge_count(&self) -> usize { self.half_edges.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
    pub fn vertex_count(&self) -> usize { self.vertices.len() }

    pub fn bodies(&self) -> &[ProjectedBodyData] { &self.bodies }
    pub fn lumps(&self) -> &[ProjectedLumpData] { &self.lumps }
    pub fn regions(&self) -> &[ProjectedRegionData] { &self.regions }
    pub fn shells(&self) -> &[ProjectedShellData] { &self.shells }
    pub fn faces(&self) -> &[ProjectedFaceData] { &self.faces }
    pub fn loops(&self) -> &[ProjectedLoopData] { &self.loops }
    pub fn half_edges(&self) -> &[ProjectedHalfEdgeData] { &self.half_edges }
    pub fn edges(&self) -> &[ProjectedEdgeData] { &self.edges }
    pub fn vertices(&self) -> &[ProjectedVertexData] { &self.vertices }

    pub fn body(&self, id: ProjectedBodyId) -> &ProjectedBodyData { &self.bodies[id.index()] }
    pub fn lump(&self, id: ProjectedLumpId) -> &ProjectedLumpData { &self.lumps[id.index()] }
    pub fn region(&self, id: ProjectedRegionId) -> &ProjectedRegionData { &self.regions[id.index()] }
    pub fn shell(&self, id: ProjectedShellId) -> &ProjectedShellData { &self.shells[id.index()] }
    pub fn face(&self, id: ProjectedFaceId) -> &ProjectedFaceData { &self.faces[id.index()] }
    pub fn loop_data(&self, id: ProjectedLoopId) -> &ProjectedLoopData { &self.loops[id.index()] }
    pub fn half_edge(&self, id: ProjectedHalfEdgeId) -> &ProjectedHalfEdgeData { &self.half_edges[id.index()] }
    pub fn edge(&self, id: ProjectedEdgeId) -> &ProjectedEdgeData { &self.edges[id.index()] }
    pub fn vertex(&self, id: ProjectedVertexId) -> &ProjectedVertexData { &self.vertices[id.index()] }

    pub fn resolve(&self, spec_id: SpecNodeId) -> Option<ProjectedEntityRef> {
        self.entity_index.get(&spec_id).copied()
    }

    pub(crate) fn rebuild_index(&mut self) {
        self.entity_index.clear();
        for (idx, body) in self.bodies.iter().enumerate() {
            self.entity_index.insert(body.spec_id, ProjectedEntityRef::Body(ProjectedBodyId::new(idx as u32)));
        }
        for (idx, lump) in self.lumps.iter().enumerate() {
            self.entity_index.insert(lump.spec_id, ProjectedEntityRef::Lump(ProjectedLumpId::new(idx as u32)));
        }
        for (idx, region) in self.regions.iter().enumerate() {
            self.entity_index.insert(region.spec_id, ProjectedEntityRef::Region(ProjectedRegionId::new(idx as u32)));
        }
        for (idx, shell) in self.shells.iter().enumerate() {
            self.entity_index.insert(shell.spec_id, ProjectedEntityRef::Shell(ProjectedShellId::new(idx as u32)));
        }
        for (idx, face) in self.faces.iter().enumerate() {
            self.entity_index.insert(face.spec_id, ProjectedEntityRef::Face(ProjectedFaceId::new(idx as u32)));
        }
        for (idx, loop_data) in self.loops.iter().enumerate() {
            self.entity_index.insert(loop_data.spec_id, ProjectedEntityRef::Loop(ProjectedLoopId::new(idx as u32)));
        }
        for (idx, half_edge) in self.half_edges.iter().enumerate() {
            self.entity_index.insert(half_edge.spec_id, ProjectedEntityRef::HalfEdge(ProjectedHalfEdgeId::new(idx as u32)));
        }
        for (idx, edge) in self.edges.iter().enumerate() {
            self.entity_index.insert(edge.spec_id, ProjectedEntityRef::Edge(ProjectedEdgeId::new(idx as u32)));
        }
        for (idx, vertex) in self.vertices.iter().enumerate() {
            self.entity_index.insert(vertex.spec_id, ProjectedEntityRef::Vertex(ProjectedVertexId::new(idx as u32)));
        }
    }
}
