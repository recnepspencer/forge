use super::*;

impl<'runtime> VisibilityPinAuthority<'runtime> {
    pub(crate) fn pin_replay_state(&self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_replay_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_replay_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn unpin_replay_state(&self, state: &SnapshotState) {
        for (partition_id, pins) in &state.pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.unpin_replay_entity(crate::identity::data::EntityId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.unpin_replay_relation(crate::identity::data::RelationId::new(
                    *partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    fn pin_replay_entity(&self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Replay, 1);
    }

    fn unpin_replay_entity(&self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Replay, -1);
    }

    fn pin_replay_relation(&self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Replay, 1);
    }

    fn unpin_replay_relation(&self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Replay, -1);
    }
}
