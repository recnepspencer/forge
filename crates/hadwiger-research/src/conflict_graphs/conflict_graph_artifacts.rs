use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{
    GraphIdentity, GraphVersion, HadwigerArtifactReference, HadwigerCanonicalArtifact,
    HadwigerQueryDeclarationReference,
};

use super::conflict_graph_edges::TilingConflictEdge;
use super::conflict_graph_errors::{require_conflict_non_empty, ConflictGraphError};
use super::conflict_graph_index::ConflictGraphExtractionIndex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictGraphExtractionCounters {
    vertices_extracted: usize,
    conflict_edges_extracted: usize,
    source_evidence_records_retained: usize,
    query_declarations_performed: usize,
}

impl ConflictGraphExtractionCounters {
    pub(crate) fn new(
        vertices: usize,
        conflict_edges: usize,
        source_evidence: usize,
        query_declarations: usize,
    ) -> Self {
        Self {
            vertices_extracted: vertices,
            conflict_edges_extracted: conflict_edges,
            source_evidence_records_retained: source_evidence,
            query_declarations_performed: query_declarations,
        }
    }

    pub fn vertices_extracted(&self) -> usize {
        self.vertices_extracted
    }

    pub fn conflict_edges_extracted(&self) -> usize {
        self.conflict_edges_extracted
    }

    pub fn source_evidence_records_retained(&self) -> usize {
        self.source_evidence_records_retained
    }

    pub fn query_declarations_performed(&self) -> usize {
        self.query_declarations_performed
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.vertices_extracted,
            self.conflict_edges_extracted,
            self.source_evidence_records_retained,
            self.query_declarations_performed
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingConflictGraph {
    core: HadwigerArtifactCore,
    graph_id: String,
    graph_identity: GraphIdentity,
    graph_version: GraphVersion,
    conflict_edges: Vec<TilingConflictEdge>,
    source_references: Vec<HadwigerArtifactReference>,
    query_declaration_reference: HadwigerQueryDeclarationReference,
    required_color_count: Option<u32>,
    counters: ConflictGraphExtractionCounters,
}

impl TilingConflictGraph {
    pub(crate) fn checked(
        graph_id: impl Into<String>,
        query_declaration_reference: HadwigerQueryDeclarationReference,
        index: ConflictGraphExtractionIndex,
        required_color_count: Option<u32>,
    ) -> Result<Self, ConflictGraphError> {
        let graph_id = require_conflict_non_empty(graph_id, "conflict_graph_id")?;
        let graph_identity =
            GraphIdentity::from_query_declaration(&graph_id, query_declaration_reference.clone())?;
        let graph_version =
            graph_version_from_index(graph_identity.reference(), &graph_id, &index)?;
        let source_references = source_references_from_edges(index.edges());
        let counters = ConflictGraphExtractionCounters::new(
            index.vertices().len(),
            index.edges().len(),
            source_references.len(),
            1,
        );
        let core = artifact_core(
            HadwigerArtifactKind::TilingConflictGraph,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "tiling_conflict_graph".to_string(),
            },
            conflict_graph_parents(&graph_identity, &graph_version, &source_references),
            conflict_graph_payload(
                &graph_id,
                &query_declaration_reference,
                &graph_version,
                index.edges(),
                &counters,
                &index,
                required_color_count,
            ),
        )?;
        Ok(Self {
            core,
            graph_id,
            graph_identity,
            graph_version,
            conflict_edges: index.edges().to_vec(),
            source_references,
            query_declaration_reference,
            required_color_count,
            counters,
        })
    }

    pub fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub fn graph_identity(&self) -> &GraphIdentity {
        &self.graph_identity
    }

    pub fn graph_version(&self) -> &GraphVersion {
        &self.graph_version
    }

    pub fn conflict_edges(&self) -> &[TilingConflictEdge] {
        &self.conflict_edges
    }

    pub fn source_references(&self) -> &[HadwigerArtifactReference] {
        &self.source_references
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }

    pub fn query_declaration_digest(&self) -> &str {
        self.query_declaration_reference.declaration_digest()
    }

    pub fn required_color_count(&self) -> Option<u32> {
        self.required_color_count
    }

    pub fn counters(&self) -> &ConflictGraphExtractionCounters {
        &self.counters
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingConflictGraph, core);

pub type TilingConflictGraphExtractionReport = TilingConflictGraph;

fn graph_version_from_index(
    graph_reference: HadwigerArtifactReference,
    graph_id: &str,
    index: &ConflictGraphExtractionIndex,
) -> Result<GraphVersion, ConflictGraphError> {
    let mut builder = GraphVersion::builder(graph_reference, format!("{graph_id}:conflict-v1"));
    for vertex in index.vertices() {
        builder = builder.with_vertex(vertex)?;
    }
    for edge in index.edges() {
        builder =
            builder.with_undirected_edge(edge.left_vertex_label(), edge.right_vertex_label())?;
    }
    Ok(builder.finish()?)
}

fn source_references_from_edges(edges: &[TilingConflictEdge]) -> Vec<HadwigerArtifactReference> {
    let mut references = edges
        .iter()
        .map(|edge| edge.source_evidence_reference().clone())
        .collect::<Vec<_>>();
    references.sort_by_key(HadwigerArtifactReference::stable_token);
    references.dedup_by_key(|reference| reference.stable_token());
    references
}

fn conflict_graph_parents(
    graph_identity: &GraphIdentity,
    graph_version: &GraphVersion,
    source_references: &[HadwigerArtifactReference],
) -> Vec<HadwigerArtifactReference> {
    let mut parents = vec![graph_identity.reference(), graph_version.reference()];
    parents.extend(source_references.iter().cloned());
    parents
}

fn conflict_graph_payload(
    graph_id: &str,
    query_reference: &HadwigerQueryDeclarationReference,
    graph_version: &GraphVersion,
    edges: &[TilingConflictEdge],
    counters: &ConflictGraphExtractionCounters,
    index: &ConflictGraphExtractionIndex,
    required_color_count: Option<u32>,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "WORTH.hadwiger.conflict_graph.v1"),
        HadwigerArtifactPayloadEntry::text("graph_id", graph_id.to_string()),
        HadwigerArtifactPayloadEntry::text(
            "query_declaration_digest",
            query_reference.declaration_digest().to_string(),
        ),
        HadwigerArtifactPayloadEntry::text(
            "graph_version",
            graph_version.reference().stable_token(),
        ),
        HadwigerArtifactPayloadEntry::text("index", index.stable_token()),
        HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
    ];
    if let Some(color_count) = required_color_count {
        payload.push(HadwigerArtifactPayloadEntry::unsigned(
            "required_color_count",
            color_count as u128,
        ));
    }
    payload.extend(
        edges
            .iter()
            .map(|edge| HadwigerArtifactPayloadEntry::text("conflict_edge", edge.stable_token())),
    );
    payload
}
