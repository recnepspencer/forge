use super::relation_update_support::RelationUpdateQuerySupport;
use crate::edit::{WorthLoopSuccessorKind, WorthTopologyEditBatch, WorthTopologyEditContract};
use forge_query::facade::ForgeQueryWorkspace;

pub(super) fn two_half_edge_span_relocation_batch(
    workspace: &mut ForgeQueryWorkspace,
    support: &RelationUpdateQuerySupport,
    moved_start_identity: &str,
    new_successor_identity: &str,
) -> WorthTopologyEditBatch {
    successor_span_relocation_batch(
        workspace,
        support,
        moved_start_identity,
        new_successor_identity,
        2,
    )
}

pub(super) fn successor_span_relocation_batch(
    workspace: &mut ForgeQueryWorkspace,
    support: &RelationUpdateQuerySupport,
    moved_start_identity: &str,
    new_successor_identity: &str,
    span_length: usize,
) -> WorthTopologyEditBatch {
    let cycle =
        support.successor_cycle_identities(workspace, moved_start_identity, span_length + 1);
    let moved_end_identity = cycle[span_length - 1].as_str();
    let old_successor_identity = cycle[span_length].as_str();
    let moved_start_id = support.find_entity_id_by_identity(moved_start_identity);
    let moved_end_id = support.find_entity_id_by_identity(moved_end_identity);
    let old_predecessor_id = support.prev_target_half_edge_id(moved_start_identity);
    let old_successor_id = support.find_entity_id_by_identity(old_successor_identity);
    let new_predecessor_id = support.prev_target_half_edge_id(new_successor_identity);
    let new_successor_id = support.find_entity_id_by_identity(new_successor_identity);
    let old_predecessor_identity = support.find_entity_identity_by_id(old_predecessor_id);
    let old_successor_identity = support.find_entity_identity_by_id(old_successor_id);
    let new_predecessor_identity = support.find_entity_identity_by_id(new_predecessor_id);

    WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                moved_start_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            moved_start_id,
            new_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                moved_end_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            moved_end_id,
            new_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                &old_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            old_predecessor_id,
            old_successor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                &old_successor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgePrev,
            ),
            WorthLoopSuccessorKind::Prev,
            old_successor_id,
            old_predecessor_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
                &new_predecessor_identity,
                worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
            ),
            WorthLoopSuccessorKind::Next,
            new_predecessor_id,
            moved_start_id,
        ),
        WorthTopologyEditContract::rewire_loop_successor(
            support.relation_id_for_source_kind(
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
