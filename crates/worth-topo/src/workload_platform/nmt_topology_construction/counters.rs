use crate::brep::topology_graph::TopologyView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NmtTopologyConstructionCounters {
    layer_count: usize,
    model_count: usize,
    shell_count: usize,
    face_count: usize,
    loop_count: usize,
    wire_count: usize,
    half_edge_count: usize,
    edge_count: usize,
    vertex_count: usize,
    boundary_half_edge_count: usize,
    non_manifold_edge_count: usize,
    validation_row_count: usize,
}

impl NmtTopologyConstructionCounters {
    pub(crate) fn from_view(
        view: &TopologyView,
        layer_count: usize,
        boundary_half_edge_count: usize,
        non_manifold_edge_count: usize,
        validation_row_count: usize,
    ) -> Self {
        Self {
            layer_count,
            model_count: view.models.len(),
            shell_count: view.shells.len(),
            face_count: view.faces.len(),
            loop_count: view.loops.len(),
            wire_count: view.wires.len(),
            half_edge_count: view.half_edges.len(),
            edge_count: view.edges.len(),
            vertex_count: view.vertices.len(),
            boundary_half_edge_count,
            non_manifold_edge_count,
            validation_row_count,
        }
    }

    pub fn layer_count(&self) -> usize {
        self.layer_count
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

    pub fn boundary_half_edge_count(&self) -> usize {
        self.boundary_half_edge_count
    }

    pub fn non_manifold_edge_count(&self) -> usize {
        self.non_manifold_edge_count
    }

    pub fn validation_row_count(&self) -> usize {
        self.validation_row_count
    }
}
