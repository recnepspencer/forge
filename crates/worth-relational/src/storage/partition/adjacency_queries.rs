use crate::runtime::RelationalRuntime;
use crate::storage::overlay::PartitionAccess;
use std::collections::BTreeSet;

pub(crate) fn outgoing_relation_candidates_from_state(
    state: &dyn PartitionAccess,
    entity_id: crate::identity::data::EntityId,
) -> Vec<crate::identity::data::RelationId> {
    state
        .get_partition(entity_id.partition_id)
        .and_then(|partition| partition.adjacency.get(entity_id.slot_index()))
        .map(|relations| relations.as_slice().to_vec())
        .unwrap_or_default()
}

pub(crate) fn incoming_relation_candidates_from_state(
    state: &dyn PartitionAccess,
    entity_id: crate::identity::data::EntityId,
) -> Vec<crate::identity::data::RelationId> {
    state
        .get_partition(entity_id.partition_id)
        .and_then(|partition| partition.reverse_adjacency.get(entity_id.slot_index()))
        .map(|relations| relations.as_slice().to_vec())
        .unwrap_or_default()
}

pub(crate) fn outgoing_relations_for_entity(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.slot_index();
    let reader = runtime.read_truth();
    let candidates = {
        let partitions = runtime.partitions.read();
        partitions
            .get(&entity_id.partition_id)
            .and_then(|partition| partition.adjacency.get(slot))
            .map(|relations| relations.as_slice().to_vec())
            .unwrap_or_default()
    };
    candidates
        .into_iter()
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}

pub(crate) fn incoming_relations_for_entity(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.slot_index();
    let reader = runtime.read_truth();
    let candidates = {
        let partitions = runtime.partitions.read();
        partitions
            .get(&entity_id.partition_id)
            .and_then(|partition| partition.reverse_adjacency.get(slot))
            .map(|relations| relations.as_slice().to_vec())
            .unwrap_or_default()
    };
    candidates
        .into_iter()
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}

pub(crate) fn outgoing_relations_for_entity_kind(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    kind_id: crate::identity::data::KindId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.slot_index();
    let reader = runtime.read_truth();
    let candidates = {
        let partitions = runtime.partitions.read();
        partitions
            .get(&entity_id.partition_id)
            .and_then(|partition| partition.adjacency.get(slot))
            .map(|relations| relations.current_kind_slice(kind_id).to_vec())
            .unwrap_or_default()
    };
    candidates
        .into_iter()
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}

pub(crate) fn incoming_relations_for_entity_kind(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    kind_id: crate::identity::data::KindId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.slot_index();
    let reader = runtime.read_truth();
    let candidates = {
        let partitions = runtime.partitions.read();
        partitions
            .get(&entity_id.partition_id)
            .and_then(|partition| partition.reverse_adjacency.get(slot))
            .map(|relations| relations.current_kind_slice(kind_id).to_vec())
            .unwrap_or_default()
    };
    candidates
        .into_iter()
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}

pub(crate) fn all_relations_for_entity(
    runtime: &RelationalRuntime,
    entity_id: crate::identity::data::EntityId,
    version_id: crate::identity::data::VersionId,
) -> Vec<crate::identity::data::RelationId> {
    let slot = entity_id.slot_index();
    let reader = runtime.read_truth();
    let mut relation_ids = BTreeSet::new();
    {
        let partitions = runtime.partitions.read();
        if let Some(partition) = partitions.get(&entity_id.partition_id) {
            if let Some(relations) = partition.adjacency.get(slot) {
                relation_ids.extend(relations.as_slice().iter().copied());
            }
            if let Some(relations) = partition.reverse_adjacency.get(slot) {
                relation_ids.extend(relations.as_slice().iter().copied());
            }
        }
    }
    relation_ids
        .into_iter()
        .filter(|relation_id| reader.relation_visible_at_version(*relation_id, version_id))
        .collect()
}
