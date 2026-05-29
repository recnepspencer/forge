use forge_query::facade::{ForgeQueryEntity, RelationName};

use crate::projection::read_views::domain::error::TopologyDomainQueryError;
use crate::projection::read_views::TopologyLoopNeighborEvidence;

use super::super::row_decode::{cycle_identities_from_successors, RetainedTopologyRows};

#[derive(Debug)]
pub(crate) struct DecodedLocalRewireNeighborhood {
    pub(crate) old_successor_identity: String,
    pub(crate) old_predecessor_identity: String,
    pub(crate) cycle_identities: Vec<String>,
    pub(crate) cycle_half_edges: Vec<TopologyLoopNeighborEvidence>,
}

pub(crate) fn decode_local_rewire_neighborhood(
    rows: &[ForgeQueryEntity],
    moved_identity: &str,
    count: usize,
    successor_relation: &RelationName,
    previous_relation: &RelationName,
    label: &str,
) -> Result<DecodedLocalRewireNeighborhood, TopologyDomainQueryError> {
    let rows = RetainedTopologyRows::new(rows);
    let moved_row = rows.row(moved_identity, label)?;
    let old_successor_identity = moved_row
        .relation_target_identity(successor_relation, label)?
        .to_string();
    let old_predecessor_identity = moved_row
        .relation_target_identity(previous_relation, label)?
        .to_string();
    let cycle_identities =
        cycle_identities_from_successors(&rows, moved_identity, count, successor_relation, label)?;
    let cycle_half_edges = build_loop_neighbor_evidence(
        &rows,
        &cycle_identities,
        successor_relation,
        previous_relation,
        label,
    )?;
    Ok(DecodedLocalRewireNeighborhood {
        old_successor_identity,
        old_predecessor_identity,
        cycle_identities,
        cycle_half_edges,
    })
}

fn build_loop_neighbor_evidence(
    rows: &RetainedTopologyRows<'_>,
    cycle_identities: &[String],
    successor_relation: &RelationName,
    previous_relation: &RelationName,
    label: &str,
) -> Result<Vec<TopologyLoopNeighborEvidence>, TopologyDomainQueryError> {
    cycle_identities
        .iter()
        .map(|identity| {
            let row = rows.row(identity, label)?;
            Ok(TopologyLoopNeighborEvidence {
                half_edge_identity: identity.clone(),
                next_half_edge_identity: row
                    .relation_target_identity(successor_relation, label)?
                    .to_string(),
                previous_half_edge_identity: row
                    .relation_target_identity(previous_relation, label)?
                    .to_string(),
                next_relation_identity: row
                    .relation_record_identity(successor_relation, label)?
                    .to_string(),
                previous_relation_identity: row
                    .relation_record_identity(previous_relation, label)?
                    .to_string(),
            })
        })
        .collect()
}
