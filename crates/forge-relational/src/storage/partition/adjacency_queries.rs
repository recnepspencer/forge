use crate::logic::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn outgoing_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        let slot = entity_id.local_slot.0 as usize;
        self.partition(entity_id.partition_id)
            .and_then(|partition| partition.adjacency.get(slot))
            .into_iter()
            .flat_map(|relations: &crate::storage::partition::AdjacencySet| {
                relations.as_slice().iter().copied()
            })
            .filter(|relation_id| self.relation_visible_at_version(*relation_id, version_id))
            .collect()
    }

    pub fn incoming_relations_for_entity(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::identity::data::RelationId> {
        let slot = entity_id.local_slot.0 as usize;
        self.partition(entity_id.partition_id)
            .and_then(|partition| partition.reverse_adjacency.get(slot))
            .into_iter()
            .flat_map(|relations: &crate::storage::partition::AdjacencySet| {
                relations.as_slice().iter().copied()
            })
            .filter(|relation_id| self.relation_visible_at_version(*relation_id, version_id))
            .collect()
    }
}
