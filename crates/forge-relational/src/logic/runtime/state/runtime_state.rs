use std::collections::BTreeMap;

use crate::storage::overlay::PartitionState;

use super::{
    AspectSemanticsSubsystem, DurabilitySubsystem, HistorySubsystem, IndexingSubsystem,
    LineageSubsystem, PublicationSubsystem, RuntimeServices, VisibilitySubsystem,
};
use crate::logic::runtime::RelationalRuntimeConfig;

#[derive(Debug)]
pub struct RelationalRuntime {
    pub(crate) config: RelationalRuntimeConfig,
    pub(crate) aspect_semantics: AspectSemanticsSubsystem,
    pub(crate) partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    pub(crate) visibility: VisibilitySubsystem,
    pub(crate) publication: PublicationSubsystem,
    pub(crate) history: HistorySubsystem,
    pub(crate) indexes: IndexingSubsystem,
    pub(crate) lineage: LineageSubsystem,
    pub(crate) durability: DurabilitySubsystem,
    pub(crate) services: RuntimeServices,
}

impl RelationalRuntime {
    pub fn config(&self) -> &RelationalRuntimeConfig {
        &self.config
    }

    #[cfg(test)]
    pub(crate) fn set_entity_structural_identity_for_test(
        &mut self,
        entity_id: crate::identity::data::EntityId,
        structural_fingerprint: Option<crate::identity::data::StructuralFingerprint>,
        lineage_id: Option<crate::identity::data::LineageId>,
    ) -> bool {
        let Some(partition) = self.partitions.get_mut(&entity_id.partition_id) else {
            return false;
        };
        if partition.entity_arena.get(&entity_id).is_none() {
            return false;
        }
        partition.entity_arena.extra[entity_id.local_slot.0 as usize] =
            crate::storage::substrate::EntityExtra {
                structural_fingerprint,
                lineage_id,
            };
        true
    }

    #[cfg(test)]
    pub(crate) fn simulate_entity_slot_reuse_for_test(
        &mut self,
        entity_id: crate::identity::data::EntityId,
        payload: crate::payloads::data::RecordPayload,
        structural_fingerprint: Option<crate::identity::data::StructuralFingerprint>,
        lineage_id: Option<crate::identity::data::LineageId>,
    ) -> Option<crate::identity::data::EntityId> {
        let replacement_version = crate::identity::data::VersionId(self.current_version_id().0 + 1);
        let partition = self.partitions.get_mut(&entity_id.partition_id)?;
        let arena = &mut partition.entity_arena;
        arena.get(&entity_id)?;
        let slot = entity_id.local_slot.0 as usize;
        let next_generation = arena.generations[slot] + 1;

        if let Some(current) = arena.payload_history[slot].last_mut() {
            current.retired_at = Some(replacement_version);
        }
        if let Some(current) = arena.metadata_history[slot].last_mut() {
            current.retired_at = Some(replacement_version);
        }

        let canonical_payload = payload.canonicalized();
        arena.generations[slot] = next_generation;
        arena.lifecycle[slot] = crate::storage::data::RecordLifecycleState::Live;
        arena.payloads[slot] = Some(canonical_payload.clone());
        arena.payload_history[slot].push(crate::storage::substrate::VersionedPayload {
            effective_at: replacement_version,
            retired_at: None,
            generation: next_generation,
            value: canonical_payload,
        });
        arena.metadata_history[slot].push(crate::storage::substrate::VersionedEntityMetadata {
            effective_at: replacement_version,
            retired_at: None,
            generation: next_generation,
            kind_id: arena.kind_ids[slot].expect("entity kind must exist for slot reuse test"),
            lineage_id,
        });
        arena.created_at[slot] = replacement_version;
        arena.retired_at[slot] = None;
        arena.extra[slot] = crate::storage::substrate::EntityExtra {
            structural_fingerprint,
            lineage_id,
        };
        arena.live_bitset.set(slot, true);
        arena.reclaimable_bitset.set(slot, false);
        self.history.next_version_id = replacement_version.0 + 1;

        Some(crate::identity::data::EntityId::new(
            entity_id.partition_id,
            entity_id.local_slot.0,
            next_generation,
        ))
    }

    #[cfg(test)]
    pub(crate) fn entity_history_len_for_test(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> usize {
        self.partitions
            .get(&entity_id.partition_id)
            .and_then(|partition| {
                partition
                    .entity_arena
                    .payload_history_at(entity_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }

    #[cfg(test)]
    pub(crate) fn relation_history_len_for_test(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> usize {
        self.partitions
            .get(&relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .payload_history_at(relation_id.local_slot.0 as usize)
            })
            .map(|history| history.len())
            .unwrap_or(0)
    }
}
