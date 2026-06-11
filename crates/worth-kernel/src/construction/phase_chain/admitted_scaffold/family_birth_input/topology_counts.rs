use worth_primitives::PrimitiveConstructionTopologyContract;

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
    pub fn from_contract(contract: PrimitiveConstructionTopologyContract) -> Self {
        Self {
            vertex_count: contract.vertex_count(),
            edge_count: contract.edge_count(),
            loop_count: contract.loop_count(),
            wire_count: contract.wire_count(),
            face_count: contract.face_count(),
            shell_count: contract.shell_count(),
            body_count: contract.body_count(),
        }
    }

    #[cfg(test)]
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    #[cfg(test)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    #[cfg(test)]
    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    #[cfg(test)]
    pub fn wire_count(&self) -> usize {
        self.wire_count
    }

    #[cfg(test)]
    pub fn face_count(&self) -> usize {
        self.face_count
    }

    #[cfg(test)]
    pub fn shell_count(&self) -> usize {
        self.shell_count
    }

    #[cfg(test)]
    pub fn body_count(&self) -> usize {
        self.body_count
    }
}
