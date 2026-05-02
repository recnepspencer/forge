use forge_query::facade::ForgeQueryEntity;

use super::relation_update_support::{
    find_entity_id_by_identity, prev_target_half_edge_id, query_entity_id_from_row,
    relation_id_for_source_kind, successor_cycle_identities,
};
use crate::edit::{WorthLoopSuccessorKind, WorthTopologyEditBatch, WorthTopologyEditContract};

pub(super) fn two_half_edge_span_relocation_batch(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    moved_start_identity: &str,
    new_successor_identity: &str,
) -> WorthTopologyEditBatch {
    successor_span_relocation_batch(
        entity_rows,
        relation_rows,
        moved_start_identity,
        new_successor_identity,
        2,
    )
}

pub(super) fn successor_span_relocation_batch(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    moved_start_identity: &str,
    new_successor_identity: &str,
    span_length: usize,
) -> WorthTopologyEditBatch {
    let cycle = successor_cycle_identities(
        entity_rows,
        relation_rows,
        moved_start_identity,
        span_length + 1,
    );
    let moved_end_identity = cycle[span_length - 1].as_str();
    let old_successor_identity = cycle[span_length].as_str();
    let moved_start_id = find_entity_id_by_identity(entity_rows, moved_start_identity);
    let moved_end_id = find_entity_id_by_identity(entity_rows, moved_end_identity);
    let old_predecessor_id =
        prev_target_half_edge_id(entity_rows, relation_rows, moved_start_identity);
    let old_successor_id = find_entity_id_by_identity(entity_rows, old_successor_identity);
    let new_predecessor_id =
        prev_target_half_edge_id(entity_rows, relation_rows, new_successor_identity);
    let new_successor_id = find_entity_id_by_identity(entity_rows, new_successor_identity);
    let old_predecessor_identity = entity_rows
        .iter()
        .find(|row| query_entity_id_from_row(row) == old_predecessor_id)
        .expect("old predecessor should remain visible")
        .identity
        .clone();
    let old_successor_identity = entity_rows
        .iter()
        .find(|row| query_entity_id_from_row(row) == old_successor_id)
        .expect("old successor should remain visible")
        .identity
        .clone();
    let new_predecessor_identity = entity_rows
        .iter()
        .find(|row| query_entity_id_from_row(row) == new_predecessor_id)
        .expect("new predecessor should remain visible")
        .identity
        .clone();

    WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                moved_start_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            moved_start_id,
            new_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                moved_end_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            moved_end_id,
            new_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                &old_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                &old_successor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                &new_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            new_predecessor_id,
            moved_start_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            relation_id_for_source_kind(
                relation_rows,
                new_successor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            new_successor_id,
            moved_end_id,
        ),
    ])
    .expect("non-empty successor span relocation batch")
}
