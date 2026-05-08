use super::shared::{entity_id_from_query_identity, relation_id_from_query_identity};
use crate::certification::error::TopologyCertificationError;
use crate::projection::{TopologyLocalRewireNeighborhoodView, TopologyLoopNeighborEvidence};
use crate::topology_operators::{LoopSuccessorKind, TopologyEditBatch, TopologyEditContract};

pub(super) fn successor_relocation_batch(
    neighborhood: &TopologyLocalRewireNeighborhoodView,
    new_successor_identity: &str,
) -> Result<TopologyEditBatch, TopologyCertificationError> {
    let moved = loop_neighbor_evidence(neighborhood, neighborhood.moved_half_edge_identity())?;
    let old_successor =
        loop_neighbor_evidence(neighborhood, neighborhood.old_successor_identity())?;
    let old_predecessor =
        loop_neighbor_evidence(neighborhood, neighborhood.old_predecessor_identity())?;
    let new_successor = loop_neighbor_evidence(neighborhood, new_successor_identity)?;
    let new_predecessor =
        loop_neighbor_evidence(neighborhood, new_successor.previous_half_edge_identity())?;

    let moved_half_edge_id = entity_id_from_query_identity(moved.half_edge_identity())?;
    let old_successor_id = entity_id_from_query_identity(old_successor.half_edge_identity())?;
    let old_predecessor_id = entity_id_from_query_identity(old_predecessor.half_edge_identity())?;
    let new_successor_id = entity_id_from_query_identity(new_successor.half_edge_identity())?;
    let new_predecessor_id = entity_id_from_query_identity(new_predecessor.half_edge_identity())?;

    TopologyEditBatch::new(vec![
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(moved.next_relation_identity())?,
            LoopSuccessorKind::Next,
            moved_half_edge_id,
            new_successor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(moved.previous_relation_identity())?,
            LoopSuccessorKind::Prev,
            moved_half_edge_id,
            new_predecessor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(old_predecessor.next_relation_identity())?,
            LoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(old_successor.previous_relation_identity())?,
            LoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(new_predecessor.next_relation_identity())?,
            LoopSuccessorKind::Next,
            new_predecessor_id,
            moved_half_edge_id,
        ),
        TopologyEditContract::rewire_loop_successor(
            relation_id_from_query_identity(new_successor.previous_relation_identity())?,
            LoopSuccessorKind::Prev,
            new_successor_id,
            moved_half_edge_id,
        ),
    ])
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))
}

fn loop_neighbor_evidence<'a>(
    neighborhood: &'a TopologyLocalRewireNeighborhoodView,
    half_edge_identity: &str,
) -> Result<&'a TopologyLoopNeighborEvidence, TopologyCertificationError> {
    neighborhood
        .cycle_half_edges()
        .iter()
        .find(|evidence| evidence.half_edge_identity() == half_edge_identity)
        .ok_or_else(|| {
            TopologyCertificationError::Query(format!(
                "local rewire neighborhood should expose cycle evidence for `{half_edge_identity}`"
            ))
        })
}
