use super::*;

impl<'runtime> VisibilityReadContext<'runtime> {
    pub(crate) fn authoritative_entity_record_at_version(
        &self,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<EntityReadRecord> {
        let state = self.runtime.storage_access().current_state();
        self.authoritative_entity_record_for_id_at_version(&state, entity_id, version_id)
    }

    pub(crate) fn authoritative_relation_record_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<RelationReadRecord> {
        let state = self.runtime.storage_access().current_state();
        self.authoritative_relation_record_for_id_at_version(&state, relation_id, version_id)
    }

    pub(crate) fn authoritative_entity_record_for_id_at_version(
        &self,
        state: &impl PartitionAccess,
        entity_id: crate::identity::data::EntityId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<EntityReadRecord> {
        let partition = state.get_partition(entity_id.partition_id)?;
        let slot = entity_id.slot_index();
        if version_id == self.runtime.current_version_id() {
            materialize_current_authoritative_entity_record(
                self.runtime,
                partition,
                entity_id.partition_id,
                slot,
            )
            .filter(|record| {
                entity_id.generation.is_zero()
                    || record.entity_id.generation == entity_id.generation
            })
        } else {
            materialize_authoritative_entity_record_at_version(
                self.runtime,
                partition,
                entity_id.partition_id,
                slot,
                version_id,
            )
            .filter(|record| {
                entity_id.generation.is_zero()
                    || record.entity_id.generation == entity_id.generation
            })
        }
    }

    pub(crate) fn authoritative_relation_record_for_id_at_version(
        &self,
        state: &impl PartitionAccess,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<RelationReadRecord> {
        let partition = state.get_partition(relation_id.partition_id)?;
        let slot = relation_id.slot_index();
        if version_id == self.runtime.current_version_id() {
            materialize_current_authoritative_relation_record(
                self.runtime,
                partition,
                relation_id.partition_id,
                slot,
            )
            .filter(|record| {
                relation_id.generation.is_zero()
                    || record.relation_id.generation == relation_id.generation
            })
        } else {
            materialize_authoritative_relation_record_at_version(
                self.runtime,
                partition,
                relation_id.partition_id,
                slot,
                version_id,
            )
            .filter(|record| {
                relation_id.generation.is_zero()
                    || record.relation_id.generation == relation_id.generation
            })
        }
    }

    pub(crate) fn visible_entities_of_kind_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let mut records = Vec::new();
        let current_version = VersionSource::current_version_id(self.runtime);
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.entity_arena.live_bitset.iter_set_slots() {
                if !slot_kind_matches_current(&partition.entity_arena, slot, kind_id) {
                    continue;
                }
                if let Some(record) = materialize_current_authoritative_entity_record(
                    self.runtime,
                    partition,
                    partition_id,
                    slot,
                ) {
                    records.push(record);
                }
            }
        } else {
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_entity_slot_scans += partition.entity_arena.slot_count();
            });
            for slot in 0..partition.entity_arena.slot_count() {
                if !entity_slot_matches_kind_at_version(
                    partition,
                    slot,
                    kind_id,
                    version_id,
                    current_version,
                ) {
                    continue;
                }
                if let Some(record) = materialize_authoritative_entity_record_at_version(
                    self.runtime,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        records
    }

    pub(crate) fn visible_relations_of_kind_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        kind_id: crate::identity::data::KindId,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let mut records = Vec::new();
        let current_version = VersionSource::current_version_id(self.runtime);
        let Some(partition) = state.get_partition(partition_id) else {
            return records;
        };
        if version_id == current_version {
            for slot in partition.relation_arena.live_bitset.iter_set_slots() {
                if !slot_kind_matches_current(&partition.relation_arena, slot, kind_id) {
                    continue;
                }
                if let Some(record) = materialize_current_authoritative_relation_record(
                    self.runtime,
                    partition,
                    partition_id,
                    slot,
                ) {
                    records.push(record);
                }
            }
        } else {
            self.runtime.services.instrumentation.count(|counters| {
                counters.visibility_relation_slot_scans += partition.relation_arena.slot_count();
            });
            for slot in 0..partition.relation_arena.slot_count() {
                if !relation_slot_matches_kind_at_version(
                    partition,
                    slot,
                    kind_id,
                    version_id,
                    current_version,
                ) {
                    continue;
                }
                if let Some(record) = materialize_authoritative_relation_record_at_version(
                    self.runtime,
                    partition,
                    partition_id,
                    slot,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        records
    }

    pub(crate) fn visible_entity_slots_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<(crate::identity::data::PartitionId, DenseSlotBitSet)> {
        let mut partitions = Vec::new();
        for partition_id in state.partition_ids() {
            if let Some(entity_slots) =
                self.visible_entity_slots_in_partition_from_state(state, partition_id, version_id)
            {
                partitions.push((partition_id, entity_slots));
            }
        }
        partitions
    }

    pub(crate) fn all_authoritative_entity_records_at_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<EntityReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            let Some(slots) =
                self.visible_entity_slots_in_partition_from_state(&state, partition_id, version_id)
            else {
                continue;
            };
            for slot in slots.iter_set_slots() {
                let entity_id = crate::identity::data::EntityId::new(partition_id, slot as u64, 0);
                if let Some(record) = self
                    .authoritative_entity_record_for_id_at_version(&state, entity_id, version_id)
                {
                    records.push(record);
                }
            }
        }
        records
    }

    pub(crate) fn visible_entity_slots_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<DenseSlotBitSet> {
        visible_slots_in_partition_from_state::<crate::storage::substrate::EntityRecordKind>(
            self.runtime,
            state,
            partition_id,
            version_id,
            |runtime, scanned| {
                runtime.services.instrumentation.count(|counters| {
                    counters.visibility_entity_slot_scans += scanned;
                });
            },
        )
    }

    pub(crate) fn visible_relation_slots_from_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<(crate::identity::data::PartitionId, DenseSlotBitSet)> {
        let mut partitions = Vec::new();
        for partition_id in state.partition_ids() {
            if let Some(relation_slots) =
                self.visible_relation_slots_in_partition_from_state(state, partition_id, version_id)
            {
                partitions.push((partition_id, relation_slots));
            }
        }
        partitions
    }

    pub(crate) fn all_authoritative_relation_records_at_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Vec<RelationReadRecord> {
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            let Some(slots) = self.visible_relation_slots_in_partition_from_state(
                &state,
                partition_id,
                version_id,
            ) else {
                continue;
            };
            for slot in slots.iter_set_slots() {
                let relation_id =
                    crate::identity::data::RelationId::new(partition_id, slot as u64, 0);
                if let Some(record) = self.authoritative_relation_record_for_id_at_version(
                    &state,
                    relation_id,
                    version_id,
                ) {
                    records.push(record);
                }
            }
        }
        sort_authoritative_relation_records(&mut records);
        records
    }

    pub(crate) fn visible_relation_slots_in_partition_from_state(
        &self,
        state: &impl PartitionAccess,
        partition_id: crate::identity::data::PartitionId,
        version_id: crate::identity::data::VersionId,
    ) -> Option<DenseSlotBitSet> {
        visible_slots_in_partition_from_state::<crate::storage::substrate::RelationRecordKind>(
            self.runtime,
            state,
            partition_id,
            version_id,
            |runtime, scanned| {
                runtime.services.instrumentation.count(|counters| {
                    counters.visibility_relation_slot_scans += scanned;
                });
            },
        )
    }

    pub(crate) fn relation_visible_at_version(
        &self,
        relation_id: crate::identity::data::RelationId,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let current_state = self.runtime.storage_access().current_state();
        self.authoritative_relation_record_for_id_at_version(
            &current_state,
            relation_id,
            version_id,
        )
        .is_some()
    }
}

pub(super) fn sort_authoritative_relation_records(records: &mut [RelationReadRecord]) {
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
