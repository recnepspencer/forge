use worth_primitives::PrimitiveConstructionBirthSynopsisContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyPrimitiveConstructionBirthFamily {
    SimplexSolid,
    Orthotope,
    RegularPrism,
    RegularPyramid,
    WireBody,
    ShellWithHole,
}

impl TopologyPrimitiveConstructionBirthFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SimplexSolid => "simplex_solid",
            Self::Orthotope => "orthotope",
            Self::RegularPrism => "regular_prism",
            Self::RegularPyramid => "regular_pyramid",
            Self::WireBody => "wire_body",
            Self::ShellWithHole => "shell_with_hole",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionQueryBirthSynopsis {
    family: TopologyPrimitiveConstructionBirthFamily,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    scaffold_digest: String,
    source_birth_digest: String,
    topology_birth_class: String,
    supported_vertex_count: usize,
    supported_edge_count: usize,
    supported_loop_count: usize,
    supported_wire_count: usize,
    supported_face_count: usize,
    supported_shell_count: usize,
    supported_body_count: usize,
}

impl TopologyPrimitiveConstructionQueryBirthSynopsis {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        family: TopologyPrimitiveConstructionBirthFamily,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        scaffold_digest: String,
        source_birth_digest: String,
        topology_birth_class: String,
        supported_vertex_count: usize,
        supported_edge_count: usize,
        supported_loop_count: usize,
        supported_wire_count: usize,
        supported_face_count: usize,
        supported_shell_count: usize,
        supported_body_count: usize,
    ) -> Self {
        Self {
            family,
            birth_contract,
            scaffold_digest,
            source_birth_digest,
            topology_birth_class,
            supported_vertex_count,
            supported_edge_count,
            supported_loop_count,
            supported_wire_count,
            supported_face_count,
            supported_shell_count,
            supported_body_count,
        }
    }

    pub fn family(&self) -> TopologyPrimitiveConstructionBirthFamily {
        self.family
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub fn source_birth_digest(&self) -> &str {
        &self.source_birth_digest
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn supported_vertex_count(&self) -> usize {
        self.supported_vertex_count
    }

    pub fn supported_edge_count(&self) -> usize {
        self.supported_edge_count
    }

    pub fn supported_loop_count(&self) -> usize {
        self.supported_loop_count
    }

    pub fn supported_wire_count(&self) -> usize {
        self.supported_wire_count
    }

    pub fn supported_face_count(&self) -> usize {
        self.supported_face_count
    }

    pub fn supported_shell_count(&self) -> usize {
        self.supported_shell_count
    }

    pub fn supported_body_count(&self) -> usize {
        self.supported_body_count
    }
}
