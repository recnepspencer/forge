use super::*;

impl<'runtime> VisibilityReadContext<'runtime> {
    pub fn visible_entities_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_entities_of_kind_in_partition_from_state(
                &state,
                partition_id,
                kind_id,
                version_id,
            ));
        }
        debug_assert!(entity_records_are_canonical(&records));
        records
    }

    pub fn visible_entities_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let records = self.visible_entities_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        debug_assert!(entity_records_are_canonical(&records));
        records
    }

    pub fn visible_relations_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_relations_of_kind_in_partition_from_state(
                &state,
                partition_id,
                kind_id,
                version_id,
            ));
        }
        super::truth_record_access::sort_relation_records(&mut records);
        records
    }

    pub fn visible_relations_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = self.visible_relations_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        super::truth_record_access::sort_relation_records(&mut records);
        records
    }
    pub fn entity_aspects_at_version(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.runtime.storage_access().current_state();
        let record = self.entity_record_for_id_at_version(&state, entity_id, version_id)?;
        Some(super::aspect_catalog::declared_aspects_for_entity_kind(
            self.runtime,
            record.kind.kind_id,
        ))
    }

    pub fn relation_aspects_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<Vec<AspectKey>> {
        let state = self.runtime.storage_access().current_state();
        let record = self.relation_record_for_id_at_version(&state, relation_id, version_id)?;
        Some(super::aspect_catalog::declared_aspects_for_relation_kind(
            self.runtime,
            record.kind.kind_id,
        ))
    }
}

fn entity_records_are_canonical(records: &[EntityReadRecord]) -> bool {
    records.windows(2).all(|window| {
        let left = &window[0];
        let right = &window[1];
        (
            left.entity_id.partition_id.0,
            left.entity_id.local_slot.0,
            left.entity_id.generation,
        ) <= (
            right.entity_id.partition_id.0,
            right.entity_id.local_slot.0,
            right.entity_id.generation,
        )
    })
}
