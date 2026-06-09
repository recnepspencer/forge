use std::collections::BTreeSet;

use forge_query::facade::ForgeQueryDeclaredFamilyChecked;

use super::seed_artifacts::{FrontierGraphSeedArtifact, FrontierGraphSeedImportReport};
use super::seed_imports::{FrontierGraphSeedImport, FrontierSeedFormat};
use crate::domain_artifacts::{
    GraphIdentity, GraphVersion, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
    HadwigerQueryDeclarationReference,
};
use crate::domain_declarations::CandidateGraphDeclaration;
use crate::query_entry::HadwigerResearchHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrontierSeedError {
    EmptyField { field: &'static str },
    UnsupportedFormat,
    MalformedHeader,
    MalformedEdge { line: usize },
    DuplicateEdge { left: String, right: String },
    SelfLoop { vertex: String },
    MissingEndpoint { vertex: String },
    EdgeCountMismatch { declared: usize, actual: usize },
    QueryDeclarationNotAdmitted,
    Artifact(HadwigerArtifactShapeError),
}

impl From<HadwigerArtifactShapeError> for FrontierSeedError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(value)
    }
}

pub fn import_frontier_graph_seed_checked(
    handle: &HadwigerResearchHandle,
    seed_import: FrontierGraphSeedImport,
) -> Result<FrontierGraphSeedImportReport, FrontierSeedError> {
    let parsed = parse_seed_edges(seed_import.format(), seed_import.edge_list())?;
    let declared = handle.declare_checked(
        CandidateGraphDeclaration::new(seed_import.seed_id())
            .with_graph_version(seed_import.version_id())
            .with_source_note(seed_import.source_family()),
    );
    let query_reference =
        admitted_reference(declared).ok_or(FrontierSeedError::QueryDeclarationNotAdmitted)?;
    let graph_identity =
        GraphIdentity::from_query_declaration(seed_import.seed_id(), query_reference.clone())?;
    let mut builder = GraphVersion::builder(graph_identity.reference(), seed_import.version_id());
    for vertex in &parsed.vertices {
        builder = builder.with_vertex(vertex)?;
    }
    for (left, right) in &parsed.edges {
        builder = builder.with_undirected_edge(left, right)?;
    }
    let graph_version = builder.finish()?;
    let seed_artifact = FrontierGraphSeedArtifact::checked(
        graph_version.reference(),
        query_reference,
        seed_import.seed_id(),
        seed_import.source_family(),
        seed_import.source_url(),
        seed_import.source_digest(),
        parsed.vertices.len(),
        parsed.edges.len(),
        seed_import
            .algebraic_embedding_certificate()
            .map(ToOwned::to_owned),
    )?;
    Ok(FrontierGraphSeedImportReport::new(
        graph_identity,
        graph_version,
        seed_artifact,
    ))
}

struct ParsedSeedGraph {
    vertices: BTreeSet<String>,
    edges: BTreeSet<(String, String)>,
}

fn parse_seed_edges(
    format: FrontierSeedFormat,
    edge_list: &str,
) -> Result<ParsedSeedGraph, FrontierSeedError> {
    match format {
        FrontierSeedFormat::DimacsEdgeList => parse_dimacs_edges(edge_list),
    }
}

fn parse_dimacs_edges(edge_list: &str) -> Result<ParsedSeedGraph, FrontierSeedError> {
    let mut declared_vertex_count = None;
    let mut declared_edge_count = None;
    let mut vertices = BTreeSet::new();
    let mut edges = BTreeSet::new();
    for (index, line) in edge_list.lines().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() || parts[0] == "c" {
            continue;
        }
        match parts[0] {
            "p" if parts.len() == 4 && parts[1] == "edge" => {
                declared_vertex_count = Some(parse_usize(parts[2])?);
                declared_edge_count = Some(parse_usize(parts[3])?);
            }
            "e" if parts.len() == 3 => {
                let left = normalize_vertex(parts[1])?;
                let right = normalize_vertex(parts[2])?;
                if left == right {
                    return Err(FrontierSeedError::SelfLoop { vertex: left });
                }
                vertices.insert(left.clone());
                vertices.insert(right.clone());
                let edge = normalized_edge(left, right);
                if !edges.insert(edge.clone()) {
                    return Err(FrontierSeedError::DuplicateEdge {
                        left: edge.0,
                        right: edge.1,
                    });
                }
            }
            _ => return Err(FrontierSeedError::MalformedEdge { line: index + 1 }),
        }
    }
    let declared_vertices = declared_vertex_count.ok_or(FrontierSeedError::MalformedHeader)?;
    let declared_edges = declared_edge_count.ok_or(FrontierSeedError::MalformedHeader)?;
    for vertex_index in 1..=declared_vertices {
        vertices.insert(vertex_index.to_string());
    }
    if declared_edges != edges.len() {
        return Err(FrontierSeedError::EdgeCountMismatch {
            declared: declared_edges,
            actual: edges.len(),
        });
    }
    Ok(ParsedSeedGraph { vertices, edges })
}

fn normalize_vertex(value: &str) -> Result<String, FrontierSeedError> {
    if value.trim().is_empty() {
        Err(FrontierSeedError::EmptyField { field: "vertex" })
    } else {
        Ok(value.to_string())
    }
}

fn parse_usize(value: &str) -> Result<usize, FrontierSeedError> {
    value
        .parse::<usize>()
        .map_err(|_| FrontierSeedError::MalformedHeader)
}

fn normalized_edge(left: String, right: String) -> (String, String) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}

fn admitted_reference(
    checked: ForgeQueryDeclaredFamilyChecked<
        crate::query_entry::HadwigerResearchDomainEntry,
        CandidateGraphDeclaration,
    >,
) -> Option<HadwigerQueryDeclarationReference> {
    match checked {
        ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => Some(declaration.into()),
        _ => None,
    }
}
