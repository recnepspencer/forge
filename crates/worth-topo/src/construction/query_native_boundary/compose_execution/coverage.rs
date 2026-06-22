use crate::construction::query_native_boundary::digest_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyPrimitiveConstructionBirthTopologyKind {
    Vertex,
    Edge,
    Loop,
    Wire,
    Face,
    Shell,
    Body,
}

impl TopologyPrimitiveConstructionBirthTopologyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertex => ".vertex",
            Self::Edge => ".edge",
            Self::Loop => ".loop",
            Self::Wire => ".wire",
            Self::Face => ".face",
            Self::Shell => ".shell",
            Self::Body => ".body",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionBirthMaterializationCoverage {
    committed_topology_kinds: Vec<TopologyPrimitiveConstructionBirthTopologyKind>,
    unmaterialized_topology_kinds: Vec<TopologyPrimitiveConstructionBirthTopologyKind>,
    coverage_digest: String,
}

impl TopologyPrimitiveConstructionBirthMaterializationCoverage {
    pub(crate) fn anchor_only(
        unmaterialized_topology_kinds: Vec<TopologyPrimitiveConstructionBirthTopologyKind>,
    ) -> Self {
        let committed_topology_kinds = vec![TopologyPrimitiveConstructionBirthTopologyKind::Vertex];
        let coverage_digest = digest_parts(&[
            "primitive-construction-birth-materialization-coverage".to_string(),
            committed_topology_kinds
                .iter()
                .map(|kind| kind.as_str())
                .collect::<Vec<_>>()
                .join("|"),
            unmaterialized_topology_kinds
                .iter()
                .map(|kind| kind.as_str())
                .collect::<Vec<_>>()
                .join("|"),
        ]);
        Self {
            committed_topology_kinds,
            unmaterialized_topology_kinds,
            coverage_digest,
        }
    }

    pub fn committed_topology_kinds(&self) -> &[TopologyPrimitiveConstructionBirthTopologyKind] {
        &self.committed_topology_kinds
    }

    pub fn unmaterialized_topology_kinds(
        &self,
    ) -> &[TopologyPrimitiveConstructionBirthTopologyKind] {
        &self.unmaterialized_topology_kinds
    }

    pub fn coverage_digest(&self) -> &str {
        &self.coverage_digest
    }
}
