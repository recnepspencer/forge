#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionTopologyCounts {
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    wire_count: usize,
    face_count: usize,
    shell_count: usize,
    body_count: usize,
}

impl PrimitiveConstructionTopologyCounts {
    pub fn new(
        vertex_count: usize,
        edge_count: usize,
        loop_count: usize,
        wire_count: usize,
        face_count: usize,
        shell_count: usize,
        body_count: usize,
    ) -> Self {
        Self {
            vertex_count,
            edge_count,
            loop_count,
            wire_count,
            face_count,
            shell_count,
            body_count,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    pub fn wire_count(&self) -> usize {
        self.wire_count
    }

    pub fn face_count(&self) -> usize {
        self.face_count
    }

    pub fn shell_count(&self) -> usize {
        self.shell_count
    }

    pub fn body_count(&self) -> usize {
        self.body_count
    }
}
