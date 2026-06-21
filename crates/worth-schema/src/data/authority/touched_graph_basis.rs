use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum WorthTopologyTouchedAspect {
    TopologyStructure,
    TopologyOwnership,
    TopologyBoundary,
    TopologyRadial,
    GeometryBinding,
    GeometryEmbedding,
    GeometryProvenance,
    GeometryApproximation,
    GeometryUvAnchoring,
    GeometryCarrier,
    GeometryPrecision,
    GeometryFallback,
    LineageProvenance,
    NamingPersistentName,
    DiagnosticsDecisions,
    DiagnosticsInterpretations,
}

impl WorthTopologyTouchedAspect {
    pub const ALL: [Self; 16] = [
        Self::TopologyStructure,
        Self::TopologyOwnership,
        Self::TopologyBoundary,
        Self::TopologyRadial,
        Self::GeometryBinding,
        Self::GeometryEmbedding,
        Self::GeometryProvenance,
        Self::GeometryApproximation,
        Self::GeometryUvAnchoring,
        Self::GeometryCarrier,
        Self::GeometryPrecision,
        Self::GeometryFallback,
        Self::LineageProvenance,
        Self::NamingPersistentName,
        Self::DiagnosticsDecisions,
        Self::DiagnosticsInterpretations,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyStructure => "topology.structure",
            Self::TopologyOwnership => "topology.ownership",
            Self::TopologyBoundary => "topology.boundary",
            Self::TopologyRadial => "topology.radial",
            Self::GeometryBinding => "geometry.binding",
            Self::GeometryEmbedding => "geometry.embedding",
            Self::GeometryProvenance => "geometry.provenance",
            Self::GeometryApproximation => "geometry.approximation",
            Self::GeometryUvAnchoring => "geometry.uv_anchoring",
            Self::GeometryCarrier => "geometry.carrier",
            Self::GeometryPrecision => "geometry.precision",
            Self::GeometryFallback => "geometry.fallback",
            Self::LineageProvenance => "lineage.provenance",
            Self::NamingPersistentName => "naming.persistent_name",
            Self::DiagnosticsDecisions => "diagnostics.decisions",
            Self::DiagnosticsInterpretations => "diagnostics.interpretations",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum WorthTopologyTouchedScope {
    Entity,
    Relation,
    LocalNeighborhood,
    Loop,
    Wire,
    Shell,
    RadialNeighborhood,
    Naming,
}

impl WorthTopologyTouchedScope {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Relation => "relation",
            Self::LocalNeighborhood => "local-neighborhood",
            Self::Loop => "loop",
            Self::Wire => "wire",
            Self::Shell => "shell",
            Self::RadialNeighborhood => "radial-neighborhood",
            Self::Naming => "naming",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum WorthTopologyGraphLifecyclePosture {
    EntityCreation,
    EntityRetirement,
    ExistingRelationCreate,
    ExistingRelationRetarget,
    ExistingRelationRemoval,
}

impl WorthTopologyGraphLifecyclePosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntityCreation => "entity-creation",
            Self::EntityRetirement => "entity-retirement",
            Self::ExistingRelationCreate => "existing-relation-create",
            Self::ExistingRelationRetarget => "existing-relation-retarget",
            Self::ExistingRelationRemoval => "existing-relation-removal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum WorthTopologyTouchedOperatingWorldPosture {
    Mainline,
    Branch,
    Preview,
    ConfiguredDomainHandle,
}

impl WorthTopologyTouchedOperatingWorldPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mainline => "mainline",
            Self::Branch => "branch",
            Self::Preview => "preview",
            Self::ConfiguredDomainHandle => "configured-domain-handle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorthTopologyTouchedGraphCounters {
    entity_count: usize,
    relation_count: usize,
    relation_kind_count: usize,
    touched_aspect_count: usize,
    topology_scope_count: usize,
}

impl WorthTopologyTouchedGraphCounters {
    pub const fn from_topology_breadth(
        entity_count: usize,
        relation_count: usize,
        relation_kind_count: usize,
        touched_aspect_count: usize,
        topology_scope_count: usize,
    ) -> Self {
        Self {
            entity_count,
            relation_count,
            relation_kind_count,
            touched_aspect_count,
            topology_scope_count,
        }
    }

    pub const fn entity_count(&self) -> usize {
        self.entity_count
    }

    pub const fn relation_count(&self) -> usize {
        self.relation_count
    }

    pub const fn relation_kind_count(&self) -> usize {
        self.relation_kind_count
    }

    pub const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub const fn topology_scope_count(&self) -> usize {
        self.topology_scope_count
    }
}

pub fn worth_topology_touched_graph_digest(canonical_parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"worth.schema.topology-touched-graph-basis.v1");
    for part in canonical_parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}
