use crate::brep::topology_graph::TopologyView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopologySeedCounters {
    model_count: usize,
    body_count: usize,
    lump_count: usize,
    region_count: usize,
    shell_count: usize,
    face_count: usize,
    loop_count: usize,
    wire_count: usize,
    half_edge_count: usize,
    edge_count: usize,
    vertex_count: usize,
    validation_row_count: usize,
}

impl TopologySeedCounters {
    pub(crate) fn from_view(view: &TopologyView, validation_row_count: usize) -> Self {
        Self {
            model_count: view.models.len(),
            body_count: view.bodies.len(),
            lump_count: view.lumps.len(),
            region_count: view.regions.len(),
            shell_count: view.shells.len(),
            face_count: view.faces.len(),
            loop_count: view.loops.len(),
            wire_count: view.wires.len(),
            half_edge_count: view.half_edges.len(),
            edge_count: view.edges.len(),
            vertex_count: view.vertices.len(),
            validation_row_count,
        }
    }

    pub fn model_count(&self) -> usize {
        self.model_count
    }

    pub fn shell_count(&self) -> usize {
        self.shell_count
    }

    pub fn face_count(&self) -> usize {
        self.face_count
    }

    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    pub fn wire_count(&self) -> usize {
        self.wire_count
    }

    pub fn half_edge_count(&self) -> usize {
        self.half_edge_count
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn validation_row_count(&self) -> usize {
        self.validation_row_count
    }

    pub fn total_topology_entities(&self) -> usize {
        self.model_count
            + self.body_count
            + self.lump_count
            + self.region_count
            + self.shell_count
            + self.face_count
            + self.loop_count
            + self.wire_count
            + self.half_edge_count
            + self.edge_count
            + self.vertex_count
    }
}
