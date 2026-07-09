use std::collections::BTreeMap;

use crate::transactions::data::EntityReference;

pub(super) type PlannedSuccessorMap = BTreeMap<EntityReference, Vec<EntityReference>>;

pub(super) fn planned_successor_map(
    planned_edges: &[super::super::super::request::PlannedRelationEdge],
) -> PlannedSuccessorMap {
    let mut successors = BTreeMap::new();
    for edge in planned_edges {
        successors
            .entry(edge.source.clone())
            .or_insert_with(Vec::new)
            .push(edge.target.clone());
    }
    successors
}

pub(super) fn planned_successor_count(planned_successors: &PlannedSuccessorMap) -> usize {
    planned_successors.values().map(Vec::len).sum()
}
