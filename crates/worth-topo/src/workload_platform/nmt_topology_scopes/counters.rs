#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NmtTopologyScopeCounters {
    face_count: usize,
    edge_count: usize,
    loop_count: usize,
    boundary_half_edge_count: usize,
    non_manifold_edge_count: usize,
    scope_entity_count: usize,
}

impl NmtTopologyScopeCounters {
    pub(crate) fn new(
        face_count: usize,
        edge_count: usize,
        loop_count: usize,
        boundary_half_edge_count: usize,
        non_manifold_edge_count: usize,
    ) -> Self {
        Self {
            face_count,
            edge_count,
            loop_count,
            boundary_half_edge_count,
            non_manifold_edge_count,
            scope_entity_count: face_count + edge_count + loop_count,
        }
    }

    pub fn face_count(self) -> usize {
        self.face_count
    }

    pub fn edge_count(self) -> usize {
        self.edge_count
    }

    pub fn loop_count(self) -> usize {
        self.loop_count
    }

    pub fn boundary_half_edge_count(self) -> usize {
        self.boundary_half_edge_count
    }

    pub fn non_manifold_edge_count(self) -> usize {
        self.non_manifold_edge_count
    }

    pub fn scope_entity_count(self) -> usize {
        self.scope_entity_count
    }
}
