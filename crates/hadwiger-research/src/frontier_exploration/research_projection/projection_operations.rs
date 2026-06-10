use crate::domain_artifacts::core_artifact::{
    HadwigerArtifactAuthorityOwner, HadwigerArtifactKind, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::domain_declarations::{declare_research_request_checked, MotifSeedDeclaration};
use crate::query_entry::HadwigerResearchHandle;

use super::projection_index::{
    degree_histogram, pressure_halo_motifs, pressure_vertices, ProjectionIndex,
};
use super::projection_types::{
    FrontierDegreeBucket, FrontierPressureHaloMotif, FrontierProjectionEdge,
    FrontierProjectionNode, FrontierResearchProjectionGraph, FrontierResearchProjectionRequest,
};

pub fn build_frontier_research_projection_graph_checked(
    handle: &HadwigerResearchHandle,
    request: FrontierResearchProjectionRequest,
) -> Result<FrontierResearchProjectionGraph, HadwigerArtifactShapeError> {
    let query_declaration_digest = declare_projection_intent(handle, &request)?;
    let index = ProjectionIndex::from_graph(&request.graph_version);
    let nodes = projection_nodes(&request, &index);
    let edges = projection_edges(&request, &index);
    let degree_histogram = degree_histogram(&index);
    let pressure_vertices = pressure_vertices(&index);
    let pressure_halo_motifs = pressure_halo_motifs(&index);
    let core = artifact_core(
        HadwigerArtifactKind::FrontierResearchProjectionGraph,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "frontier_research_projection_graph".to_string(),
        },
        projection_parents(&request),
        projection_payload(
            &request,
            &query_declaration_digest,
            &degree_histogram,
            &pressure_halo_motifs,
        ),
    )?;
    Ok(FrontierResearchProjectionGraph::new(
        core,
        query_declaration_digest,
        nodes,
        edges,
        degree_histogram,
        pressure_vertices,
        pressure_halo_motifs,
    ))
}

fn declare_projection_intent(
    handle: &HadwigerResearchHandle,
    request: &FrontierResearchProjectionRequest,
) -> Result<String, HadwigerArtifactShapeError> {
    match declare_research_request_checked(
        handle,
        MotifSeedDeclaration::new(format!("{}-projection", request.projection_id))
            .with_source_family(request.seed.source_family())
            .with_novelty_signature(format!("{}:projection", request.seed.source_digest())),
    ) {
        forge_query::facade::ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => Ok(
            crate::domain_artifacts::HadwigerQueryDeclarationReference::from(declaration)
                .declaration_digest()
                .to_string(),
        ),
        _ => Err(shape_error("frontier_projection_declaration")),
    }
}

fn projection_nodes(
    request: &FrontierResearchProjectionRequest,
    index: &ProjectionIndex,
) -> Vec<FrontierProjectionNode> {
    let mut nodes = vec![
        FrontierProjectionNode::new("seed", "seed", request.seed.seed_id()),
        FrontierProjectionNode::new("graph", "graph_version", request.graph_version.version_id()),
    ];
    nodes.extend(
        index.vertices.iter().map(|vertex| {
            FrontierProjectionNode::new(format!("vertex:{vertex}"), "vertex", vertex)
        }),
    );
    if let Some(proof) = &request.retained_proof {
        nodes.push(FrontierProjectionNode::new(
            "retained-proof-manifest",
            "retained_proof_manifest",
            proof.proof_id(),
        ));
    }
    for motif in pressure_halo_motifs(index) {
        nodes.push(FrontierProjectionNode::new(
            format!("motif:pressure-halo:{}", motif.hub_vertex()),
            "pressure_halo_motif",
            motif.novelty_signature(),
        ));
    }
    nodes
}

fn projection_edges(
    request: &FrontierResearchProjectionRequest,
    index: &ProjectionIndex,
) -> Vec<FrontierProjectionEdge> {
    let mut edges = vec![FrontierProjectionEdge::new(
        "seed",
        "graph",
        "has_graph_version",
    )];
    if request.unit_checked.is_some() {
        edges.push(FrontierProjectionEdge::new(
            "graph",
            "unit-distance-replay",
            "has_exact_geometry_replay",
        ));
    }
    if request.color_checked.is_some() {
        edges.push(FrontierProjectionEdge::new(
            "graph",
            "colorability-replay",
            "has_unsat_certificate_replay",
        ));
    }
    if request.retained_proof.is_some() {
        edges.push(FrontierProjectionEdge::new(
            "graph",
            "retained-proof-manifest",
            "has_retained_proof_manifest",
        ));
    }
    for (left, neighbors) in &index.adjacency {
        for right in neighbors {
            if left < right {
                edges.push(FrontierProjectionEdge::new(
                    format!("vertex:{left}"),
                    format!("vertex:{right}"),
                    "unit_edge",
                ));
            }
        }
    }
    for motif in pressure_halo_motifs(index) {
        let motif_id = format!("motif:pressure-halo:{}", motif.hub_vertex());
        edges.push(FrontierProjectionEdge::new(
            motif_id.clone(),
            format!("vertex:{}", motif.hub_vertex()),
            "has_hub",
        ));
        for satellite in motif.satellites() {
            edges.push(FrontierProjectionEdge::new(
                motif_id.clone(),
                format!("vertex:{}", satellite.vertex_label()),
                "has_satellite",
            ));
        }
    }
    edges
}

fn projection_parents(
    request: &FrontierResearchProjectionRequest,
) -> Vec<crate::domain_artifacts::HadwigerArtifactReference> {
    let mut parents = vec![request.seed.reference(), request.graph_version.reference()];
    if let Some(unit_checked) = &request.unit_checked {
        parents.push(unit_checked.verification().reference());
    }
    if let Some(color_checked) = &request.color_checked {
        parents.push(color_checked.colorability_verification().reference());
    }
    parents
}

fn projection_payload(
    request: &FrontierResearchProjectionRequest,
    query_declaration_digest: &str,
    histogram: &[FrontierDegreeBucket],
    motifs: &[FrontierPressureHaloMotif],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut entries = vec![
        HadwigerArtifactPayloadEntry::text("projection_id", &request.projection_id),
        HadwigerArtifactPayloadEntry::text("seed_id", request.seed.seed_id()),
        HadwigerArtifactPayloadEntry::text("graph_version", request.graph_version.version_id()),
        HadwigerArtifactPayloadEntry::text("query_declaration_digest", query_declaration_digest),
        HadwigerArtifactPayloadEntry::unsigned(
            "vertex_count",
            request.graph_version.vertex_count() as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "edge_count",
            request.graph_version.edge_count() as u128,
        ),
    ];
    for bucket in histogram {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "degree_bucket",
            format!("{}:{}", bucket.degree(), bucket.vertex_count()),
        ));
    }
    for motif in motifs {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "pressure_halo_motif",
            motif.novelty_signature(),
        ));
    }
    if let Some(proof) = &request.retained_proof {
        entries.push(HadwigerArtifactPayloadEntry::text(
            "retained_proof_id",
            proof.proof_id(),
        ));
        entries.push(HadwigerArtifactPayloadEntry::text(
            "retained_proof_sha256",
            proof.proof_sha256(),
        ));
        entries.push(HadwigerArtifactPayloadEntry::text(
            "retained_proof_cnf_digest",
            proof.cnf_digest(),
        ));
    }
    entries
}

fn shape_error(field: &'static str) -> HadwigerArtifactShapeError {
    HadwigerArtifactShapeError::EmptyField { field }
}
