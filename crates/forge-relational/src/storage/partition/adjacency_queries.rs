use crate::logic::runtime::RelationalRuntime;
use std::collections::BTreeSet;

pub(crate) fn outgoing_relations_for_entity(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.local_slot.0 as usize;
    let reader = runtime.visibility_reads();
    runtime
        .storage_access()
        .partition_state(entity_id.partition_id)
        .and_then(|partition| partition.adjacency.get(slot))
        .into_iter()
        .flat_map(|relations: &crate::storage::partition::AdjacencySet| {
            relations.as_slice().iter().copied()
        })
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}

pub(crate) fn incoming_relations_for_entity(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.local_slot.0 as usize;
    let reader = runtime.visibility_reads();
    runtime
        .storage_access()
        .partition_state(entity_id.partition_id)
        .and_then(|partition| partition.reverse_adjacency.get(slot))
        .into_iter()
        .flat_map(|relations: &crate::storage::partition::AdjacencySet| {
            relations.as_slice().iter().copied()
        })
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}

pub(crate) fn all_relations_for_entity(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.local_slot.0 as usize;
    let reader = runtime.visibility_reads();
    let mut relation_ids = BTreeSet::new();
    if let Some(partition) = runtime.storage_access().partition_state(entity_id.partition_id) {
        if let Some(relations) = partition.adjacency.get(slot) {
            relation_ids.extend(relations.as_slice().iter().copied());
        }
        if let Some(relations) = partition.reverse_adjacency.get(slot) {
            relation_ids.extend(relations.as_slice().iter().copied());
        }
    }
    relation_ids
        .into_iter()
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}
