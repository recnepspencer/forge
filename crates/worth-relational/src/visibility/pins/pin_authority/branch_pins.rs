use std::collections::{BTreeMap, BTreeSet};

use crate::visibility::snapshot_states::{
    build_partition_pins_for_branch_head, build_partition_pins_for_version,
};

use super::*;

impl<'runtime> VisibilityPinAuthority<'runtime> {
    pub(crate) fn pin_branch_version(&mut self, version_id: crate::identity::data::VersionId) {
        let pinned_partitions = build_partition_pins_for_version(self.runtime, version_id);
        self.pin_branch_partitions(pinned_partitions);
    }

    fn pin_branch_partitions(
        &mut self,
        pinned_partitions: BTreeMap<
            crate::identity::data::PartitionId,
            crate::storage::overlay::SnapshotPartitionPins,
        >,
    ) {
        for (partition_id, pins) in pinned_partitions {
            for slot in pins.entity_slots.iter_set_slots() {
                self.pin_branch_entity(crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    0,
                ));
            }
            for slot in pins.relation_slots.iter_set_slots() {
                self.pin_branch_relation(crate::identity::data::RelationId::new(
                    partition_id,
                    slot as u64,
                    0,
                ));
            }
        }
    }

    pub(crate) fn advance_branch_pins_for_changed_records(
        &mut self,
        old_version: Option<crate::identity::data::VersionId>,
        new_version: crate::identity::data::VersionId,
        changed_records: &[crate::transactions::data::RecordRef],
    ) {
        if old_version.is_none() {
            let mut entity_slots =
                BTreeMap::<crate::identity::data::PartitionId, BTreeSet<usize>>::new();
            let mut relation_slots =
                BTreeMap::<crate::identity::data::PartitionId, BTreeSet<usize>>::new();
            for record in changed_records {
                match record {
                    crate::transactions::data::RecordRef::Entity(entity_id) => {
                        entity_slots
                            .entry(entity_id.partition_id)
                            .or_default()
                            .insert(entity_id.slot_index());
                    }
                    crate::transactions::data::RecordRef::Relation(relation_id) => {
                        relation_slots
                            .entry(relation_id.partition_id)
                            .or_default()
                            .insert(relation_id.slot_index());
                    }
                }
            }
            self.runtime
                .storage_authority()
                .increment_named_pins_bulk::<EntityRecordKind>(&entity_slots, PinClass::Branch);
            self.runtime
                .storage_authority()
                .increment_named_pins_bulk::<RelationRecordKind>(&relation_slots, PinClass::Branch);
            return;
        }

        let (entity_actions, relation_actions) = {
            let current_state = self.runtime.storage_access().current_state();
            let reader = self.runtime.read_truth();
            let mut entity_actions = Vec::new();
            let mut relation_actions = Vec::new();
            for record in changed_records {
                match record {
                    crate::transactions::data::RecordRef::Entity(entity_id) => {
                        let was_visible = old_version.is_some_and(|version_id| {
                            reader
                                .authoritative_entity_record_for_id_at_version(
                                    &current_state,
                                    *entity_id,
                                    version_id,
                                )
                                .is_some()
                        });
                        let is_visible = reader
                            .authoritative_entity_record_for_id_at_version(
                                &current_state,
                                *entity_id,
                                new_version,
                            )
                            .is_some();
                        match (was_visible, is_visible) {
                            (false, true) => entity_actions.push((*entity_id, 1)),
                            (true, false) => entity_actions.push((*entity_id, -1)),
                            _ => {}
                        }
                    }
                    crate::transactions::data::RecordRef::Relation(relation_id) => {
                        let was_visible = old_version.is_some_and(|version_id| {
                            reader
                                .authoritative_relation_record_for_id_at_version(
                                    &current_state,
                                    *relation_id,
                                    version_id,
                                )
                                .is_some()
                        });
                        let is_visible = reader
                            .authoritative_relation_record_for_id_at_version(
                                &current_state,
                                *relation_id,
                                new_version,
                            )
                            .is_some();
                        match (was_visible, is_visible) {
                            (false, true) => relation_actions.push((*relation_id, 1)),
                            (true, false) => relation_actions.push((*relation_id, -1)),
                            _ => {}
                        }
                    }
                }
            }
            (entity_actions, relation_actions)
        };
        for (entity_id, delta) in entity_actions {
            if delta > 0 {
                self.pin_branch_entity(entity_id);
            } else {
                self.unpin_branch_entity(entity_id);
            }
        }
        for (relation_id, delta) in relation_actions {
            if delta > 0 {
                self.pin_branch_relation(relation_id);
            } else {
                self.unpin_branch_relation(relation_id);
            }
        }
    }

    pub(crate) fn rebuild_branch_pins_from_heads(&mut self) {
        self.runtime
            .storage_authority()
            .clear_named_pins(PinClass::Branch);
        let branch_heads = self
            .runtime
            .history
            .branch_ids_snapshot()
            .into_iter()
            .filter_map(|branch_id| {
                self.runtime
                    .history()
                    .branch_head(&branch_id)
                    .map(|head| (branch_id, head.version_id))
            })
            .collect::<Vec<_>>();
        for (branch_id, version_id) in branch_heads {
            let pins = build_partition_pins_for_branch_head(self.runtime, &branch_id, version_id);
            self.pin_branch_partitions(pins);
        }
    }

    fn pin_branch_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Branch, 1);
    }

    fn unpin_branch_entity(&mut self, entity_id: crate::identity::data::EntityId) {
        adjust_entity_pin(self.runtime, entity_id, SubstratePinClass::Branch, -1);
    }

    fn pin_branch_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Branch, 1);
    }

    fn unpin_branch_relation(&mut self, relation_id: crate::identity::data::RelationId) {
        adjust_relation_pin(self.runtime, relation_id, SubstratePinClass::Branch, -1);
    }
}
