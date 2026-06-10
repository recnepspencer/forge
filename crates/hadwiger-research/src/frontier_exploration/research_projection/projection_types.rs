use crate::domain_artifacts::core_artifact::{impl_hadwiger_artifact, HadwigerArtifactCore};
use crate::domain_artifacts::GraphVersion;
use crate::frontier_seeds::{FrontierGraphSeedArtifact, RetainedFrontierColoringProof};
use crate::mathematical_verification::{
    KColorabilityVerificationChecked, UnitDistanceVerificationChecked,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierResearchProjectionRequest {
    pub(super) projection_id: String,
    pub(super) seed: FrontierGraphSeedArtifact,
    pub(super) graph_version: GraphVersion,
    pub(super) unit_checked: Option<UnitDistanceVerificationChecked>,
    pub(super) color_checked: Option<KColorabilityVerificationChecked>,
    pub(super) retained_proof: Option<RetainedFrontierColoringProof>,
}

impl FrontierResearchProjectionRequest {
    pub fn new(
        projection_id: impl Into<String>,
        seed: &FrontierGraphSeedArtifact,
        graph_version: &GraphVersion,
    ) -> Self {
        Self {
            projection_id: projection_id.into(),
            seed: seed.clone(),
            graph_version: graph_version.clone(),
            unit_checked: None,
            color_checked: None,
            retained_proof: None,
        }
    }

    pub fn with_unit_distance_verification(
        mut self,
        checked: &UnitDistanceVerificationChecked,
    ) -> Self {
        self.unit_checked = Some(checked.clone());
        self
    }

    pub fn with_colorability_verification(
        mut self,
        checked: &KColorabilityVerificationChecked,
    ) -> Self {
        self.color_checked = Some(checked.clone());
        self
    }

    pub fn with_retained_proof_manifest(mut self, proof: &RetainedFrontierColoringProof) -> Self {
        self.retained_proof = Some(proof.clone());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierProjectionNode {
    node_id: String,
    kind: String,
    label: String,
}

impl FrontierProjectionNode {
    pub(super) fn new(
        node_id: impl Into<String>,
        kind: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            kind: kind.into(),
            label: label.into(),
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierProjectionEdge {
    source: String,
    target: String,
    relation: String,
}

impl FrontierProjectionEdge {
    pub(super) fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        relation: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            relation: relation.into(),
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierDegreeBucket {
    degree: usize,
    vertex_count: usize,
}

impl FrontierDegreeBucket {
    pub(super) fn new(degree: usize, vertex_count: usize) -> Self {
        Self {
            degree,
            vertex_count,
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierPressureVertex {
    vertex_label: String,
    degree: usize,
    triangle_count: usize,
    pressure_score: usize,
}

impl FrontierPressureVertex {
    pub(super) fn new(
        vertex_label: String,
        degree: usize,
        triangle_count: usize,
        pressure_score: usize,
    ) -> Self {
        Self {
            vertex_label,
            degree,
            triangle_count,
            pressure_score,
        }
    }

    pub fn vertex_label(&self) -> &str {
        &self.vertex_label
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn triangle_count(&self) -> usize {
        self.triangle_count
    }

    pub fn pressure_score(&self) -> usize {
        self.pressure_score
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierPressureSatellite {
    vertex_label: String,
    degree: usize,
    common_spokes: Vec<String>,
}

impl FrontierPressureSatellite {
    pub(super) fn new(vertex_label: String, degree: usize, common_spokes: Vec<String>) -> Self {
        Self {
            vertex_label,
            degree,
            common_spokes,
        }
    }

    pub fn vertex_label(&self) -> &str {
        &self.vertex_label
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn common_spokes(&self) -> &[String] {
        &self.common_spokes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierPressureHaloMotif {
    hub_vertex: String,
    satellites: Vec<FrontierPressureSatellite>,
    pressure_score: usize,
    novelty_signature: String,
}

impl FrontierPressureHaloMotif {
    pub(super) fn new(
        hub_vertex: String,
        satellites: Vec<FrontierPressureSatellite>,
        pressure_score: usize,
        novelty_signature: String,
    ) -> Self {
        Self {
            hub_vertex,
            satellites,
            pressure_score,
            novelty_signature,
        }
    }

    pub fn hub_vertex(&self) -> &str {
        &self.hub_vertex
    }

    pub fn satellites(&self) -> &[FrontierPressureSatellite] {
        &self.satellites
    }

    pub fn pressure_score(&self) -> usize {
        self.pressure_score
    }

    pub fn novelty_signature(&self) -> &str {
        &self.novelty_signature
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierResearchProjectionGraph {
    core: HadwigerArtifactCore,
    query_declaration_digest: String,
    nodes: Vec<FrontierProjectionNode>,
    edges: Vec<FrontierProjectionEdge>,
    degree_histogram: Vec<FrontierDegreeBucket>,
    pressure_vertices: Vec<FrontierPressureVertex>,
    pressure_halo_motifs: Vec<FrontierPressureHaloMotif>,
}

impl FrontierResearchProjectionGraph {
    pub(super) fn new(
        core: HadwigerArtifactCore,
        query_declaration_digest: String,
        nodes: Vec<FrontierProjectionNode>,
        edges: Vec<FrontierProjectionEdge>,
        degree_histogram: Vec<FrontierDegreeBucket>,
        pressure_vertices: Vec<FrontierPressureVertex>,
        pressure_halo_motifs: Vec<FrontierPressureHaloMotif>,
    ) -> Self {
        Self {
            core,
            query_declaration_digest,
            nodes,
            edges,
            degree_histogram,
            pressure_vertices,
            pressure_halo_motifs,
        }
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn nodes(&self) -> &[FrontierProjectionNode] {
        &self.nodes
    }

    pub fn edges(&self) -> &[FrontierProjectionEdge] {
        &self.edges
    }

    pub fn degree_histogram(&self) -> &[FrontierDegreeBucket] {
        &self.degree_histogram
    }

    pub fn top_pressure_vertices(&self, limit: usize) -> Vec<&FrontierPressureVertex> {
        self.pressure_vertices.iter().take(limit).collect()
    }

    pub fn pressure_halo_motifs(&self) -> &[FrontierPressureHaloMotif] {
        &self.pressure_halo_motifs
    }

    pub fn best_pressure_halo_motif(&self) -> Option<&FrontierPressureHaloMotif> {
        self.pressure_halo_motifs.first()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(FrontierResearchProjectionGraph, core);
