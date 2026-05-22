use forge_query::facade::{ForgeQueryEntity, RelationName};

use crate::projection::read_views::domain::error::TopologyDomainQueryError;
use crate::projection::read_views::TopologyAdjacentHalfEdgeEvidence;

use super::super::row_decode::{
    adjacent_row_identities_sharing_targets, filter_identities_by_edge_mismatch,
    RetainedTopologyRows,
};

#[derive(Debug)]
pub(crate) struct DecodedSharedVertexNeighborhood {
    pub(crate) source_edge_identity: String,
    pub(crate) source_vertex_identities: Vec<String>,
    pub(crate) vertex_adjacent_half_edge_identities: Vec<String>,
    pub(crate) vertex_adjacent_different_edge_half_edge_identities: Vec<String>,
    pub(crate) vertex_adjacent_different_edge_half_edges: Vec<TopologyAdjacentHalfEdgeEvidence>,
}

pub(crate) fn decode_shared_vertex_neighborhood(
    rows: &[ForgeQueryEntity],
    source_identity: &str,
    edge_relation: &RelationName,
    endpoint_relations: &[RelationName],
    label: &str,
) -> Result<DecodedSharedVertexNeighborhood, TopologyDomainQueryError> {
    let rows = RetainedTopologyRows::new(rows);
    let source_row = rows.row(source_identity, label)?;
    let source_edge_identity = source_row
        .relation_target_identity(edge_relation, label)?
        .to_string();
    let source_vertex_identities =
        source_row.relation_target_identities(endpoint_relations, label)?;
    let vertex_adjacent_half_edge_identities = adjacent_row_identities_sharing_targets(
        &rows,
        source_identity,
        &source_vertex_identities,
        endpoint_relations,
        label,
    )?;
    let vertex_adjacent_different_edge_half_edge_identities = filter_identities_by_edge_mismatch(
        &rows,
        &vertex_adjacent_half_edge_identities,
        &source_edge_identity,
        edge_relation,
        label,
    )?;
    let vertex_adjacent_different_edge_half_edges = build_adjacent_half_edge_evidence(
        &rows,
        &source_vertex_identities,
        &vertex_adjacent_different_edge_half_edge_identities,
        edge_relation,
        endpoint_relations,
        label,
    )?;
    Ok(DecodedSharedVertexNeighborhood {
        source_edge_identity,
        source_vertex_identities,
        vertex_adjacent_half_edge_identities,
        vertex_adjacent_different_edge_half_edge_identities,
        vertex_adjacent_different_edge_half_edges,
    })
}

fn build_adjacent_half_edge_evidence(
    rows: &RetainedTopologyRows<'_>,
    source_vertex_identities: &[String],
    adjacent_half_edge_identities: &[String],
    edge_relation: &RelationName,
    endpoint_relations: &[RelationName],
    label: &str,
) -> Result<Vec<TopologyAdjacentHalfEdgeEvidence>, TopologyDomainQueryError> {
    adjacent_half_edge_identities
        .iter()
        .map(|identity| {
            let row = rows.row(identity, label)?;
            let edge_identity = row
                .relation_target_identity(edge_relation, label)?
                .to_string();
            let shared_vertex_identities = row
                .relation_target_identities(endpoint_relations, label)?
                .into_iter()
                .filter(|candidate| source_vertex_identities.contains(candidate))
                .collect();
            Ok(TopologyAdjacentHalfEdgeEvidence {
                half_edge_identity: identity.clone(),
                edge_identity,
                shared_vertex_identities,
            })
        })
        .collect()
}
