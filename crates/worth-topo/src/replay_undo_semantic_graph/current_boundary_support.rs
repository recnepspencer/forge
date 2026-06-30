use forge_query::facade::ForgeQueryEntity;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::read_views::TopologyLocalRewireNeighborhoodView;
use crate::projection::read_views::TopologyLoopNeighborEvidence;
use crate::query_native_runtime_boundary::{row_text_at, TopologyNativeQueryRowField};
use crate::topology_operators::{
    TopologyLoopSuccessorRewireMember, TopologyRewireLoopSuccessorProgramDeclaration,
};

use super::CurrentReplayUndoTopologyBoundaryError;

pub(super) fn first_source_identity_for_relation_kind(
    relation_rows: &[ForgeQueryEntity],
    relation_kind: TopologyRelationKind,
) -> Result<String, CurrentReplayUndoTopologyBoundaryError> {
    relation_rows
        .iter()
        .find_map(|row| {
            (row_text_at(
                row,
                TopologyNativeQueryRowField::TopologyKind.row_segments(),
            ) == Some(relation_kind.kind_name()))
            .then(|| {
                row_text_at(
                    row,
                    TopologyNativeQueryRowField::TopologySourceIdentity.row_segments(),
                )
                .map(str::to_string)
            })
            .flatten()
        })
        .ok_or_else(|| {
            CurrentReplayUndoTopologyBoundaryError::new(format!(
                "current replay/undo topology query rows should expose `{}` source identities",
                relation_kind.kind_name()
            ))
        })
}

pub(super) fn successor_candidate_with_retained_predecessor(
    neighborhood: &TopologyLocalRewireNeighborhoodView,
    preferred_offset: usize,
    context: &str,
) -> Result<String, CurrentReplayUndoTopologyBoundaryError> {
    neighborhood
        .cycle_identities()
        .iter()
        .nth(preferred_offset)
        .filter(|candidate| {
            candidate_has_complete_relocation_evidence(neighborhood, candidate.as_str())
        })
        .cloned()
        .or_else(|| {
            neighborhood
                .cycle_identities()
                .iter()
                .skip(1)
                .find(|candidate| {
                    candidate_has_complete_relocation_evidence(neighborhood, candidate.as_str())
                })
                .cloned()
        })
        .ok_or_else(|| {
            CurrentReplayUndoTopologyBoundaryError::new(format!(
                "{context} should expose a successor candidate with retained predecessor evidence"
            ))
        })
}

pub(super) fn successor_relocation_declaration(
    neighborhood: &TopologyLocalRewireNeighborhoodView,
    new_successor_identity: &str,
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, CurrentReplayUndoTopologyBoundaryError> {
    let moved = loop_neighbor_evidence(neighborhood, neighborhood.moved_half_edge_identity())?;
    let old_successor =
        loop_neighbor_evidence(neighborhood, neighborhood.old_successor_identity())?;
    let old_predecessor =
        loop_neighbor_evidence(neighborhood, neighborhood.old_predecessor_identity())?;
    let new_successor = loop_neighbor_evidence(neighborhood, new_successor_identity)?;
    let new_predecessor =
        loop_neighbor_evidence(neighborhood, new_successor.previous_half_edge_identity())?;
    Ok(TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(moved.next_relation_identity())?,
            crate::facade::LoopSuccessorKind::Next,
            entity_id_from_query_identity(moved.half_edge_identity())?,
            entity_id_from_query_identity(new_successor.half_edge_identity())?,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(moved.previous_relation_identity())?,
            crate::facade::LoopSuccessorKind::Prev,
            entity_id_from_query_identity(moved.half_edge_identity())?,
            entity_id_from_query_identity(new_predecessor.half_edge_identity())?,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(old_predecessor.next_relation_identity())?,
            crate::facade::LoopSuccessorKind::Next,
            entity_id_from_query_identity(old_predecessor.half_edge_identity())?,
            entity_id_from_query_identity(old_successor.half_edge_identity())?,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(old_successor.previous_relation_identity())?,
            crate::facade::LoopSuccessorKind::Prev,
            entity_id_from_query_identity(old_successor.half_edge_identity())?,
            entity_id_from_query_identity(old_predecessor.half_edge_identity())?,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(new_predecessor.next_relation_identity())?,
            crate::facade::LoopSuccessorKind::Next,
            entity_id_from_query_identity(new_predecessor.half_edge_identity())?,
            entity_id_from_query_identity(moved.half_edge_identity())?,
        ),
        TopologyLoopSuccessorRewireMember::new(
            relation_id_from_query_identity_label(new_successor.previous_relation_identity())?,
            crate::facade::LoopSuccessorKind::Prev,
            entity_id_from_query_identity(new_successor.half_edge_identity())?,
            entity_id_from_query_identity(moved.half_edge_identity())?,
        ),
    ]))
}

fn candidate_has_complete_relocation_evidence(
    neighborhood: &TopologyLocalRewireNeighborhoodView,
    candidate: &str,
) -> bool {
    successor_relocation_declaration(neighborhood, candidate).is_ok()
}

fn loop_neighbor_evidence<'a>(
    neighborhood: &'a TopologyLocalRewireNeighborhoodView,
    half_edge_identity: &str,
) -> Result<&'a TopologyLoopNeighborEvidence, CurrentReplayUndoTopologyBoundaryError> {
    neighborhood
        .cycle_half_edges()
        .iter()
        .find(|evidence| evidence.half_edge_identity() == half_edge_identity)
        .ok_or_else(|| {
            CurrentReplayUndoTopologyBoundaryError::new(format!(
                "current replay/undo topology local rewire neighborhood should expose cycle evidence for `{half_edge_identity}`"
            ))
        })
}

fn entity_id_from_query_identity(
    identity: &str,
) -> Result<forge_relational::facade::identity::EntityId, CurrentReplayUndoTopologyBoundaryError> {
    let [partition_id, local_slot, generation] = query_identity_parts(identity, "entity")?;
    Ok(forge_relational::facade::identity::EntityId::new(
        forge_relational::facade::identity::PartitionId(partition_id as u32),
        local_slot,
        generation as u32,
    ))
}

fn relation_id_from_query_identity_label(
    identity: &str,
) -> Result<forge_relational::facade::identity::RelationId, CurrentReplayUndoTopologyBoundaryError>
{
    let [partition_id, local_slot, generation] = query_identity_parts(identity, "relation")?;
    Ok(forge_relational::facade::identity::RelationId::new(
        forge_relational::facade::identity::PartitionId(partition_id as u32),
        local_slot,
        generation as u32,
    ))
}

fn query_identity_parts(
    identity: &str,
    expected_kind: &str,
) -> Result<[u64; 3], CurrentReplayUndoTopologyBoundaryError> {
    let mut parts = identity.split(':');
    let kind = parts.next().unwrap_or_default();
    if kind != expected_kind {
        return Err(CurrentReplayUndoTopologyBoundaryError::new(format!(
            "expected `{expected_kind}` query identity, got `{identity}`"
        )));
    }
    let partition_id = parse_query_identity_part(parts.next(), identity, "partition")?;
    let local_slot = parse_query_identity_part(parts.next(), identity, "local slot")?;
    let generation = parse_query_identity_part(parts.next(), identity, "generation")?;
    if parts.next().is_some() {
        return Err(CurrentReplayUndoTopologyBoundaryError::new(format!(
            "query identity `{identity}` had too many fields"
        )));
    }
    Ok([partition_id, local_slot, generation])
}

fn parse_query_identity_part(
    part: Option<&str>,
    identity: &str,
    label: &str,
) -> Result<u64, CurrentReplayUndoTopologyBoundaryError> {
    part.ok_or_else(|| {
        CurrentReplayUndoTopologyBoundaryError::new(format!(
            "query identity `{identity}` is missing {label}"
        ))
    })?
    .parse()
    .map_err(|_| {
        CurrentReplayUndoTopologyBoundaryError::new(format!(
            "query identity `{identity}` has invalid {label}"
        ))
    })
}
