use std::collections::BTreeSet;

use crate::domain_artifacts::{GraphVersion, HadwigerCanonicalArtifact};

use super::conflict_core_extraction::ConflictCoreExtractionRequest;
use super::conflict_graph_errors::ConflictGraphError;
use super::core_minimization_certificates::{
    ConflictCoreDeletionCheckKind, ConflictCoreMinimalityCertificate,
};

pub(crate) fn validate_deletion_proof_certificate(
    request: &ConflictCoreExtractionRequest,
    certificate: &ConflictCoreMinimalityCertificate,
) -> Result<(), ConflictGraphError> {
    let graph = request.conflict_graph().graph_version();
    let vertices = graph_vertex_labels(graph);
    let edges = graph_edge_labels(graph);
    for check in certificate.deletion_checks() {
        let target_known = match check.kind() {
            ConflictCoreDeletionCheckKind::VertexRemoval => vertices.contains(check.target()),
            ConflictCoreDeletionCheckKind::EdgeRemoval => edges.contains(check.target()),
        };
        if !target_known {
            return Err(ConflictGraphError::DeletionCheckGraphMismatch {
                target: check.target().to_string(),
            });
        }
        validate_deletion_check_graph(
            request,
            check.kind(),
            check.target(),
            check.deletion_graph(),
        )?;
        validate_deletion_check_verification(request, check)?;
    }
    Ok(())
}

fn validate_deletion_check_graph(
    request: &ConflictCoreExtractionRequest,
    kind: ConflictCoreDeletionCheckKind,
    target: &str,
    deletion_graph: Option<&GraphVersion>,
) -> Result<(), ConflictGraphError> {
    let Some(deletion_graph) = deletion_graph else {
        return Ok(());
    };
    let source_graph = request.conflict_graph().graph_version();
    let expected_vertices = expected_deletion_vertices(source_graph, kind, target);
    let expected_edges = expected_deletion_edges(source_graph, kind, target);
    if graph_vertex_labels(deletion_graph) != expected_vertices
        || graph_edge_labels(deletion_graph) != expected_edges
    {
        return Err(ConflictGraphError::DeletionCheckGraphMismatch {
            target: target.to_string(),
        });
    }
    Ok(())
}

fn validate_deletion_check_verification(
    request: &ConflictCoreExtractionRequest,
    check: &super::core_minimization_certificates::ConflictCoreDeletionCheck,
) -> Result<(), ConflictGraphError> {
    let Some(verification) = check.colorability_verification() else {
        return Ok(());
    };
    let Some(deletion_graph) = check.deletion_graph() else {
        return Err(ConflictGraphError::DeletionCheckVerificationMismatch {
            target: check.target().to_string(),
        });
    };
    if verification.graph_version_reference() != &deletion_graph.reference() {
        return Err(ConflictGraphError::DeletionCheckVerificationMismatch {
            target: check.target().to_string(),
        });
    }
    if verification.color_count() != request.color_count() {
        return Err(ConflictGraphError::DeletionCheckColorCountMismatch {
            target: check.target().to_string(),
        });
    }
    Ok(())
}

fn expected_deletion_vertices(
    graph: &GraphVersion,
    kind: ConflictCoreDeletionCheckKind,
    target: &str,
) -> BTreeSet<String> {
    graph
        .vertices()
        .iter()
        .map(|vertex| vertex.vertex_label().to_string())
        .filter(|vertex| kind != ConflictCoreDeletionCheckKind::VertexRemoval || vertex != target)
        .collect()
}

fn expected_deletion_edges(
    graph: &GraphVersion,
    kind: ConflictCoreDeletionCheckKind,
    target: &str,
) -> BTreeSet<String> {
    graph
        .edges()
        .iter()
        .filter_map(|edge| {
            let (left, right) = edge.endpoints();
            let normalized = normalized_core_edge_target(left, right);
            let removes_vertex = kind == ConflictCoreDeletionCheckKind::VertexRemoval
                && (left == target || right == target);
            let removes_edge =
                kind == ConflictCoreDeletionCheckKind::EdgeRemoval && normalized == target;
            if removes_vertex || removes_edge {
                None
            } else {
                Some(normalized)
            }
        })
        .collect()
}

fn graph_vertex_labels(graph: &GraphVersion) -> BTreeSet<String> {
    graph
        .vertices()
        .iter()
        .map(|vertex| vertex.vertex_label().to_string())
        .collect()
}

fn graph_edge_labels(graph: &GraphVersion) -> BTreeSet<String> {
    graph
        .edges()
        .iter()
        .map(|edge| {
            let (left, right) = edge.endpoints();
            normalized_core_edge_target(left, right)
        })
        .collect()
}

fn normalized_core_edge_target(left: &str, right: &str) -> String {
    if right < left {
        format!("{right}:{left}")
    } else {
        format!("{left}:{right}")
    }
}
