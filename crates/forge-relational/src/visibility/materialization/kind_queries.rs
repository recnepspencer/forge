use crate::logic::runtime::RelationalRuntime;
use crate::storage::logic::state::PartitionAccess;

impl RelationalRuntime {
    pub fn visible_entities_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::EntityReadRecord> {
        let state = self.current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_entities_of_kind_in_partition_from_state(
                &state,
                partition_id,
                kind_id,
                version_id,
            ));
        }
        sort_entity_records(&mut records);
        records
    }

    pub fn visible_entities_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::EntityReadRecord> {
        let state = self.current_state();
        let mut records = self.visible_entities_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        sort_entity_records(&mut records);
        records
    }

    pub fn visible_relations_of_kind(
        &self,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::RelationReadRecord> {
        let state = self.current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            records.extend(self.visible_relations_of_kind_in_partition_from_state(
                &state,
                partition_id,
                kind_id,
                version_id,
            ));
        }
        sort_relation_records(&mut records);
        records
    }

    pub fn visible_relations_of_kind_in_partition(
        &self,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<crate::storage::data::RelationReadRecord> {
        let state = self.current_state();
        let mut records = self.visible_relations_of_kind_in_partition_from_state(
            &state,
            partition_id,
            kind_id,
            version_id,
        );
        sort_relation_records(&mut records);
        records
    }
}

fn sort_entity_records(records: &mut [crate::storage::data::EntityReadRecord]) {
    records.sort_by_key(|record| {
        (
            record.entity_id.partition_id.0,
            record.entity_id.local_slot.0,
            record.entity_id.generation,
        )
    });
}

fn sort_relation_records(records: &mut [crate::storage::data::RelationReadRecord]) {
    records.sort_by_key(|record| {
        (
            record.source.partition_id.0,
            record.source.local_slot.0,
            record.target.partition_id.0,
            record.target.local_slot.0,
            record.relation_id.partition_id.0,
            record.relation_id.local_slot.0,
        )
    });
}
