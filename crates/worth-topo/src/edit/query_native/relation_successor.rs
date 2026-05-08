use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::EntityId;
use schema::facade::TopologyRelationKind::*;

use super::relation_successor_support::{
    live_relation_for_source, matches_expected_rewire, same_loop, ContiguousSpanCandidate,
    DesiredLoopSuccessorRewire, DesiredLoopSuccessorWorkflow,
};
use crate::edit::{LoopSuccessorKind, TopologyEditAction, TopologyEditContract};

pub(super) fn supports_admitted_loop_successor_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[TopologyEditContract],
) -> bool {
    let Some(workflow) = desired_successor_workflow(contracts) else {
        return false;
    };
    single_half_edge_candidates(&workflow)
        .into_iter()
        .any(|moved_half_edge_id| {
            matches_admitted_single_half_edge_relocation_workflow(
                entity_rows,
                relation_rows,
                &workflow,
                moved_half_edge_id,
            )
        })
        || contiguous_span_candidates(entity_rows, relation_rows, &workflow)
            .into_iter()
            .any(|candidate| {
                matches_admitted_contiguous_span_relocation_workflow(
                    entity_rows,
                    relation_rows,
                    &workflow,
                    candidate,
                )
            })
}

fn desired_successor_workflow(
    contracts: &[TopologyEditContract],
) -> Option<DesiredLoopSuccessorWorkflow> {
    if contracts.len() != 6 {
        return None;
    }
    let mut next = BTreeMap::new();
    let mut prev = BTreeMap::new();
    for contract in contracts {
        let TopologyEditAction::RewireLoopSuccessor {
            relation_id,
            kind,
            half_edge_id,
            successor_half_edge_id,
        } = contract.action
        else {
            return None;
        };
        let desired = DesiredLoopSuccessorRewire {
            relation_id,
            target_half_edge_id: successor_half_edge_id,
        };
        match kind {
            LoopSuccessorKind::Next => {
                next.insert(half_edge_id, desired);
            }
            LoopSuccessorKind::Prev => {
                prev.insert(half_edge_id, desired);
            }
        }
    }
    (next.len() == 3 && prev.len() == 3).then_some(DesiredLoopSuccessorWorkflow { next, prev })
}

fn single_half_edge_candidates(workflow: &DesiredLoopSuccessorWorkflow) -> Vec<EntityId> {
    workflow
        .next
        .keys()
        .copied()
        .filter(|entity_id| workflow.prev.contains_key(entity_id))
        .collect()
}

fn matches_admitted_single_half_edge_relocation_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    workflow: &DesiredLoopSuccessorWorkflow,
    moved_half_edge_id: EntityId,
) -> bool {
    let Some(moved_next) = workflow.next.get(&moved_half_edge_id) else {
        return false;
    };
    let Some(moved_prev) = workflow.prev.get(&moved_half_edge_id) else {
        return false;
    };
    let Some(live_next) =
        live_relation_for_source(entity_rows, relation_rows, moved_half_edge_id, HalfEdgeNext)
    else {
        return false;
    };
    let Some(live_prev) =
        live_relation_for_source(entity_rows, relation_rows, moved_half_edge_id, HalfEdgePrev)
    else {
        return false;
    };
    if moved_next.relation_id != live_next.relation_id
        || moved_prev.relation_id != live_prev.relation_id
    {
        return false;
    }
    if !same_loop(
        entity_rows,
        relation_rows,
        moved_half_edge_id,
        moved_next.target_half_edge_id,
    ) || !same_loop(
        entity_rows,
        relation_rows,
        moved_half_edge_id,
        moved_prev.target_half_edge_id,
    ) {
        return false;
    }

    matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.next.get(&live_prev.target_half_edge_id),
        live_prev.target_half_edge_id,
        HalfEdgeNext,
        live_next.target_half_edge_id,
    ) && matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.prev.get(&live_next.target_half_edge_id),
        live_next.target_half_edge_id,
        HalfEdgePrev,
        live_prev.target_half_edge_id,
    ) && matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.next.get(&moved_prev.target_half_edge_id),
        moved_prev.target_half_edge_id,
        HalfEdgeNext,
        moved_half_edge_id,
    ) && matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.prev.get(&moved_next.target_half_edge_id),
        moved_next.target_half_edge_id,
        HalfEdgePrev,
        moved_half_edge_id,
    )
}

fn contiguous_span_candidates(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    workflow: &DesiredLoopSuccessorWorkflow,
) -> Vec<ContiguousSpanCandidate> {
    workflow
        .prev
        .keys()
        .copied()
        .filter(|start_half_edge_id| !workflow.next.contains_key(start_half_edge_id))
        .filter_map(|start_half_edge_id| {
            let mut span_half_edge_ids = vec![start_half_edge_id];
            let mut seen = BTreeSet::from([start_half_edge_id]);
            let mut current_half_edge_id = start_half_edge_id;
            loop {
                let live_next = live_relation_for_source(
                    entity_rows,
                    relation_rows,
                    current_half_edge_id,
                    HalfEdgeNext,
                )?;
                if workflow.prev.contains_key(&live_next.target_half_edge_id) {
                    return None;
                }
                let live_prev = live_relation_for_source(
                    entity_rows,
                    relation_rows,
                    live_next.target_half_edge_id,
                    HalfEdgePrev,
                )?;
                if live_prev.target_half_edge_id != current_half_edge_id {
                    return None;
                }
                if workflow.next.contains_key(&live_next.target_half_edge_id) {
                    return Some(ContiguousSpanCandidate {
                        start_half_edge_id,
                        end_half_edge_id: live_next.target_half_edge_id,
                        span_half_edge_ids,
                    });
                }
                if !seen.insert(live_next.target_half_edge_id) {
                    return None;
                }
                span_half_edge_ids.push(live_next.target_half_edge_id);
                current_half_edge_id = live_next.target_half_edge_id;
            }
        })
        .collect()
}

fn matches_admitted_contiguous_span_relocation_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    workflow: &DesiredLoopSuccessorWorkflow,
    candidate: ContiguousSpanCandidate,
) -> bool {
    let Some(start_prev) = workflow.prev.get(&candidate.start_half_edge_id) else {
        return false;
    };
    let Some(end_next) = workflow.next.get(&candidate.end_half_edge_id) else {
        return false;
    };
    let Some(live_start_prev) = live_relation_for_source(
        entity_rows,
        relation_rows,
        candidate.start_half_edge_id,
        HalfEdgePrev,
    ) else {
        return false;
    };
    let Some(live_end_next) = live_relation_for_source(
        entity_rows,
        relation_rows,
        candidate.end_half_edge_id,
        HalfEdgeNext,
    ) else {
        return false;
    };
    if start_prev.relation_id != live_start_prev.relation_id
        || end_next.relation_id != live_end_next.relation_id
    {
        return false;
    }

    let old_predecessor_half_edge_id = live_start_prev.target_half_edge_id;
    let old_successor_half_edge_id = live_end_next.target_half_edge_id;
    let new_predecessor_half_edge_id = start_prev.target_half_edge_id;
    let new_successor_half_edge_id = end_next.target_half_edge_id;
    let span_half_edge_ids = candidate
        .span_half_edge_ids
        .iter()
        .copied()
        .chain(std::iter::once(candidate.end_half_edge_id))
        .collect::<BTreeSet<_>>();
    if span_half_edge_ids.contains(&old_predecessor_half_edge_id)
        || span_half_edge_ids.contains(&old_successor_half_edge_id)
        || span_half_edge_ids.contains(&new_predecessor_half_edge_id)
        || span_half_edge_ids.contains(&new_successor_half_edge_id)
    {
        return false;
    }
    if !same_loop(
        entity_rows,
        relation_rows,
        candidate.start_half_edge_id,
        candidate.end_half_edge_id,
    ) || !same_loop(
        entity_rows,
        relation_rows,
        candidate.start_half_edge_id,
        new_predecessor_half_edge_id,
    ) || !same_loop(
        entity_rows,
        relation_rows,
        candidate.start_half_edge_id,
        new_successor_half_edge_id,
    ) {
        return false;
    }

    matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.next.get(&old_predecessor_half_edge_id),
        old_predecessor_half_edge_id,
        HalfEdgeNext,
        old_successor_half_edge_id,
    ) && matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.prev.get(&old_successor_half_edge_id),
        old_successor_half_edge_id,
        HalfEdgePrev,
        old_predecessor_half_edge_id,
    ) && matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.next.get(&new_predecessor_half_edge_id),
        new_predecessor_half_edge_id,
        HalfEdgeNext,
        candidate.start_half_edge_id,
    ) && matches_expected_rewire(
        entity_rows,
        relation_rows,
        workflow.prev.get(&new_successor_half_edge_id),
        new_successor_half_edge_id,
        HalfEdgePrev,
        candidate.end_half_edge_id,
    )
}
