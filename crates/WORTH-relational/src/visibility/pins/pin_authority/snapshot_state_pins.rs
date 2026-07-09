use super::*;

impl<'runtime> VisibilityPinAuthority<'runtime> {
    pub(crate) fn pin_snapshot_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn unpin_snapshot_state(&mut self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.unpin_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.unpin_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    fn pin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        self.runtime
            .storage_authority()
            .pin_snapshot_record::<EntityRecordKind>(entity_id);
    }

    fn unpin_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        let retention_fence = self
            .runtime
            .visibility
            .retention_fence_version(self.runtime.current_version_id());
        self.runtime
            .storage_authority()
            .unpin_snapshot_record::<EntityRecordKind>(entity_id, retention_fence);
    }

    fn pin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        self.runtime
            .storage_authority()
            .pin_snapshot_record::<RelationRecordKind>(relation_id);
    }

    fn unpin_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        let retention_fence = self
            .runtime
            .visibility
            .retention_fence_version(self.runtime.current_version_id());
        self.runtime
            .storage_authority()
            .unpin_snapshot_record::<RelationRecordKind>(relation_id, retention_fence);
    }
}
