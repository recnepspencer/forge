use super::super::shared::{entity_id_from_query_identity, relation_id_from_query_identity_label};
use crate::certification::error::TopologyCertificationError;
use crate::query_domain::{TopologyLocalRewireNeighborhoodView, TopologyLoopNeighborEvidence};
use crate::topology_operators::{
    LoopSuccessorKind, TopologyLoopSuccessorRewireMember,
    TopologyRewireLoopSuccessorProgramDeclaration,
};

pub(in crate::certification::topology_operator_closeout) fn successor_relocation_declaration(
    neighborhood: &TopologyLocalRewireNeighborhoodView,
    new_successor_identity: &str,
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, TopologyCertificationError> {
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

    Ok(TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(moved.next_relation_identity())?,
            LoopSuccessorKind::Next,
            moved_half_edge_id,
            new_successor_id,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(moved.previous_relation_identity())?,
            LoopSuccessorKind::Prev,
            moved_half_edge_id,
            new_predecessor_id,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(old_predecessor.next_relation_identity())?,
            LoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(old_successor.previous_relation_identity())?,
            LoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(new_predecessor.next_relation_identity())?,
            LoopSuccessorKind::Next,
            new_predecessor_id,
            moved_half_edge_id,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(new_successor.previous_relation_identity())?,
            LoopSuccessorKind::Prev,
            new_successor_id,
            moved_half_edge_id,
        ),
    ]))
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
