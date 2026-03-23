use std::collections::HashSet;

use crate::payloads::data::RecordPayload;
use crate::storage::logic::state::{EntityArena, PartitionAccess, VersionedValue};

pub(crate) struct InvariantStateView<'state> {
    state: &'state dyn PartitionAccess,
    version_id: crate::identity::data::VersionId,
}

impl<'state> InvariantStateView<'state> {
    pub(crate) fn new(
        state: &'state dyn PartitionAccess,
        version_id: crate::identity::data::VersionId,
    ) -> Self {
        Self { state, version_id }
    }

    pub(crate) fn state(&self) -> &'state dyn PartitionAccess {
        self.state
    }

    pub(crate) fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub(crate) fn touched_visible_entity_ids(
        &self,
    ) -> Option<Vec<crate::identity::data::EntityId>> {
        let mut ids = Vec::new();
        let mut saw_any = false;
        for partition_id in self.state.partition_ids() {
            let partition = self.state.get_partition(partition_id)?;
            let Some(slots) = self.state.touched_entity_slots(partition_id) else {
                continue;
            };
            saw_any = true;
            for slot in slots {
                let Some(metadata) = partition
                    .entity_arena
                    .metadata_history_at(slot)
                    .and_then(|history| self.visible_entity_metadata(history))
                else {
                    continue;
                };
                ids.push(crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    metadata.generation,
                ));
            }
        }
        if saw_any {
            Some(ids)
        } else {
            None
        }
    }

    pub(crate) fn entity_payload(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> Option<&'state RecordPayload> {
        let partition = self.state.get_partition(entity_id.partition_id)?;
        let slot = entity_id.local_slot.0 as usize;
        if partition
            .entity_arena
            .get(&entity_id)
            .map(|slot_view| slot_view.generation())
            != Some(entity_id.generation.0)
        {
            return None;
        }
        partition
            .entity_arena
            .payload_history_at(slot)
            .and_then(|history| self.visible_payload(history))
    }

    pub(crate) fn entity_visible_at_version(&self, arena: &EntityArena, slot: usize) -> bool {
        arena
            .metadata_history_at(slot)
            .and_then(|history| self.visible_entity_metadata(history))
            .is_some()
    }

    pub(crate) fn entity_metadata_at(
        &self,
        arena: &'state EntityArena,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
    ) -> Option<VisibleEntityMetadata> {
        let metadata = arena
            .metadata_history_at(slot)
            .and_then(|history| self.visible_entity_metadata(history))?;
        Some(VisibleEntityMetadata {
            entity_id: crate::identity::data::EntityId::new(
                partition_id,
                slot as u64,
                metadata.generation,
            ),
            kind_id: metadata.kind_id,
        })
    }

    pub(crate) fn touched_entity_set(
        ids: &[crate::identity::data::EntityId],
    ) -> HashSet<crate::identity::data::EntityId> {
        ids.iter().copied().collect()
    }

    pub(crate) fn visible_payload(
        &self,
        history: &'state [VersionedValue],
    ) -> Option<&'state RecordPayload> {
        let end = history.partition_point(|entry| entry.effective_at <= self.version_id);
        history[..end]
            .iter()
            .rev()
            .find(|entry| {
                entry.effective_at <= self.version_id
                    && entry
                        .retired_at
                        .is_none_or(|retired| self.version_id < retired)
            })
            .map(|entry| &entry.value)
    }

    fn visible_entity_metadata(
        &self,
        history: &'state [crate::storage::logic::state::VersionedEntityMetadata],
    ) -> Option<&'state crate::storage::logic::state::VersionedEntityMetadata> {
        let end = history.partition_point(|entry| entry.effective_at <= self.version_id);
        history[..end].iter().rev().find(|entry| {
            entry.effective_at <= self.version_id
                && entry
                    .retired_at
                    .is_none_or(|retired| self.version_id < retired)
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VisibleEntityMetadata {
    pub(crate) entity_id: crate::identity::data::EntityId,
    pub(crate) kind_id: crate::identity::data::KindId,
}
