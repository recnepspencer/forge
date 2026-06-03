use forge_query::facade::{ForgeQueryEntity, ForgeQueryWorkspace};
use forge_relational::facade::identity::EntityId;
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::shared::{entity_id_from_query_identity, relation_id_from_query_identity};
use crate::certification::error::TopologyCertificationError;
use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::topology_operators::{
    LoopSuccessorKind, TopologyLoopSuccessorRewireMember,
    TopologyRewireLoopSuccessorProgramDeclaration,
};

pub(super) fn scaled_successor_span_declaration(
    workspace: &mut ForgeQueryWorkspace,
    relation_rows: &[ForgeQueryEntity],
    moved_start_identity: &str,
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, TopologyCertificationError> {
    let span_length = 4;
    let cycle = TopologyReadProofHarness::current_head()
        .loop_cycle(workspace, moved_start_identity, 7)
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?
        .cycle_identities()
        .to_vec();
    let moved_end_identity = cycle[span_length - 1].as_str();
    let old_successor_identity = cycle[span_length].as_str();
    let new_successor_identity = cycle[6].as_str();
    let moved_start_id = entity_id_from_query_identity(moved_start_identity)?;
    let moved_end_id = entity_id_from_query_identity(moved_end_identity)?;
    let old_predecessor_identity =
        prev_target_half_edge_identity(relation_rows, moved_start_identity)?;
    let old_predecessor_id = entity_id_from_query_identity(&old_predecessor_identity)?;
    let old_successor_id = entity_id_from_query_identity(old_successor_identity)?;
    let new_predecessor_identity =
        prev_target_half_edge_identity(relation_rows, new_successor_identity)?;
    let new_predecessor_id = entity_id_from_query_identity(&new_predecessor_identity)?;
    let new_successor_id = entity_id_from_query_identity(new_successor_identity)?;

    Ok(TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        rewire_member(
            relation_rows,
            moved_start_identity,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            moved_start_id,
            new_predecessor_id,
        )?,
        rewire_member(
            relation_rows,
            moved_end_identity,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            moved_end_id,
            new_successor_id,
        )?,
        rewire_member(
            relation_rows,
            &old_predecessor_identity,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        )?,
        rewire_member(
            relation_rows,
            old_successor_identity,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        )?,
        rewire_member(
            relation_rows,
            &new_predecessor_identity,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            new_predecessor_id,
            moved_start_id,
        )?,
        rewire_member(
            relation_rows,
            new_successor_identity,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            new_successor_id,
            moved_end_id,
        )?,
    ]))
}

fn rewire_member(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    relation_kind: TopologyRelationKind,
    successor_kind: LoopSuccessorKind,
    source_id: EntityId,
    target_id: EntityId,
) -> Result<TopologyLoopSuccessorRewireMember, TopologyCertificationError> {
    Ok(TopologyLoopSuccessorRewireMember::new(
        relation_id_for_source_kind(relation_rows, source_identity, relation_kind)?,
        successor_kind,
        source_id,
        target_id,
    ))
}

fn prev_target_half_edge_identity(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
) -> Result<String, TopologyCertificationError> {
    relation_target_identity_for_source_kind(
        relation_rows,
        source_identity,
        TopologyRelationKind::HalfEdgePrev,
    )
}

fn relation_id_for_source_kind(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> Result<forge_relational::facade::identity::RelationId, TopologyCertificationError> {
    let relation_identity = relation_rows
        .iter()
        .find(|row| row_matches_source_kind(row, source_identity, relation_kind))
        .map(|row| row.identity())
        .ok_or_else(|| scale_pressure_span_error("span rewire relation id should resolve"))?;
    relation_id_from_query_identity(relation_identity)
}

fn relation_target_identity_for_source_kind(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> Result<String, TopologyCertificationError> {
    relation_rows
        .iter()
        .find(|row| row_matches_source_kind(row, source_identity, relation_kind))
        .and_then(|row| {
            row.external_row()
                .get("topology")
                .and_then(|value| value.get("target_identity"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
        })
        .ok_or_else(|| scale_pressure_span_error("span rewire target identity should resolve"))
}

fn row_matches_source_kind(
    row: &ForgeQueryEntity,
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> bool {
    row.external_row()
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(|value| value.as_str())
        == Some(relation_kind.kind_name())
        && row
            .external_row()
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(|value| value.as_str())
            == Some(source_identity)
}

fn scale_pressure_span_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three scale span failed: {reason}"))
}
