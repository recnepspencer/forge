use forge_query::facade::{ForgeQueryEntity, RelationName};

use crate::projection::read_views::domain::error::TopologyReadError;
use crate::projection::read_views::TopologyRadialCandidateEvidence;

use super::super::row_decode::{
    edge_identity_by_row, filter_row_identities_by_edge_match, RetainedTopologyRows,
};

#[derive(Debug)]
pub(crate) struct DecodedRadialNeighborhood {
    pub(crate) source_edge_identity: String,
    pub(crate) current_target_half_edge_identity: String,
    pub(crate) current_target_edge_identity: String,
    pub(crate) source_radial_next_relation_identity: String,
    pub(crate) same_edge_half_edge_identities: Vec<String>,
    pub(crate) different_edge_half_edge_identities: Vec<String>,
    pub(crate) different_edge_half_edges: Vec<TopologyRadialCandidateEvidence>,
}

pub(crate) fn decode_radial_neighborhood(
    rows: &[ForgeQueryEntity],
    source_identity: &str,
    edge_relation: &RelationName,
    radial_next_relation: &RelationName,
    label: &str,
) -> Result<DecodedRadialNeighborhood, TopologyReadError> {
    let rows = RetainedTopologyRows::new(rows);
    let source_row = rows.row(source_identity, label)?;
    let source_edge_identity = source_row
        .relation_target_identity(edge_relation, label)?
        .to_string();
    let current_target_half_edge_identity = source_row
        .relation_target_identity(radial_next_relation, label)?
        .to_string();
    let current_target_edge_identity = edge_identity_by_row(
        &rows,
        &current_target_half_edge_identity,
        edge_relation,
        label,
    )?;
    let source_radial_next_relation_identity = source_row
        .relation_record_identity(radial_next_relation, label)?
        .to_string();
    let same_edge_half_edge_identities = filter_row_identities_by_edge_match(
        &rows,
        source_identity,
        &source_edge_identity,
        edge_relation,
        true,
        label,
    )?;
    let different_edge_half_edge_identities = filter_row_identities_by_edge_match(
        &rows,
        source_identity,
        &source_edge_identity,
        edge_relation,
        false,
        label,
    )?;
    let different_edge_half_edges = build_radial_candidate_evidence(
        &rows,
        &different_edge_half_edge_identities,
        edge_relation,
        label,
    )?;
    Ok(DecodedRadialNeighborhood {
        source_edge_identity,
        current_target_half_edge_identity,
        current_target_edge_identity,
        source_radial_next_relation_identity,
        same_edge_half_edge_identities,
        different_edge_half_edge_identities,
        different_edge_half_edges,
    })
}

fn build_radial_candidate_evidence(
    rows: &RetainedTopologyRows<'_>,
    identities: &[String],
    edge_relation: &RelationName,
    label: &str,
) -> Result<Vec<TopologyRadialCandidateEvidence>, TopologyReadError> {
    identities
        .iter()
        .map(|identity| {
            Ok(TopologyRadialCandidateEvidence {
                half_edge_identity: identity.clone(),
                edge_identity: rows
                    .row(identity, label)?
                    .relation_target_identity(edge_relation, label)?
                    .to_string(),
            })
        })
        .collect()
}
