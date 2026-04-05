use std::collections::HashSet;

use crate::payloads::data::RecordPayload;
use crate::storage::logic::state::{EntityArena, PartitionAccess, VersionedValue};
use crate::storage::substrate::HistoricalMetadata;

#[derive(Clone, Copy)]
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
        let slot = entity_id.local_slot.0 as usize;
        let partition = self.entity_partition_for_slot(entity_id.partition_id, slot)?;
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

    pub(crate) fn relation_payload(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> Option<&'state RecordPayload> {
        let slot = relation_id.local_slot.0 as usize;
        let partition = self.relation_partition_for_slot(relation_id.partition_id, slot)?;
        if partition
            .relation_arena
            .get(&relation_id)
            .map(|slot_view| slot_view.generation())
            != Some(relation_id.generation.0)
        {
            return None;
        }
        partition
            .relation_arena
            .payload_history_at(slot)
            .and_then(|history| self.visible_payload(history))
    }

    pub(crate) fn entity_metadata(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> Option<VisibleEntityMetadata> {
        let slot = entity_id.local_slot.0 as usize;
        let partition = self.entity_partition_for_slot(entity_id.partition_id, slot)?;
        if partition
            .entity_arena
            .get(&entity_id)
            .map(|slot_view| slot_view.generation())
            != Some(entity_id.generation.0)
        {
            return None;
        }
        self.entity_metadata_at(&partition.entity_arena, entity_id.partition_id, slot)
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

    pub(crate) fn relation_metadata(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> Option<VisibleRelationMetadata> {
        let slot = relation_id.local_slot.0 as usize;
        let partition = self.relation_partition_for_slot(relation_id.partition_id, slot)?;
        if partition
            .relation_arena
            .get(&relation_id)
            .map(|slot_view| slot_view.generation())
            != Some(relation_id.generation.0)
        {
            return None;
        }
        self.relation_metadata_at(&partition.relation_arena, relation_id.partition_id, slot)
    }

    pub(crate) fn relation_metadata_at(
        &self,
        arena: &'state crate::storage::logic::state::RelationArena,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
    ) -> Option<VisibleRelationMetadata> {
        let metadata = arena
            .metadata_history_at(slot)
            .and_then(|history| self.visible_relation_metadata(history))?;
        Some(VisibleRelationMetadata {
            relation_id: crate::identity::data::RelationId::new(
                partition_id,
                slot as u64,
                metadata.generation,
            ),
            kind_id: metadata.kind_id,
            source: metadata.endpoints.source,
            target: metadata.endpoints.target,
        })
    }

    pub(crate) fn touched_entity_set(
        ids: &[crate::identity::data::EntityId],
    ) -> HashSet<crate::identity::data::EntityId> {
        ids.iter().copied().collect()
    }

    pub(crate) fn touched_visible_relation_ids(
        &self,
    ) -> Option<Vec<crate::identity::data::RelationId>> {
        let mut ids = Vec::new();
        let mut saw_any = false;
        for partition_id in self.state.partition_ids() {
            let partition = self.state.get_partition(partition_id)?;
            let Some(slots) = self.state.touched_relation_slots(partition_id) else {
                continue;
            };
            saw_any = true;
            for slot in slots {
                let Some(metadata) =
                    self.relation_metadata_at(&partition.relation_arena, partition_id, slot)
                else {
                    continue;
                };
                ids.push(metadata.relation_id);
            }
        }
        if saw_any {
            Some(ids)
        } else {
            None
        }
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
        let end = history.partition_point(|entry| entry.effective_at() <= self.version_id);
        history[..end].iter().rev().find(|entry| {
            entry.effective_at() <= self.version_id
                && entry
                    .retired_at()
                    .is_none_or(|retired| self.version_id < retired)
        })
    }

    fn visible_relation_metadata(
        &self,
        history: &'state [crate::storage::logic::state::VersionedRelationMetadata],
    ) -> Option<&'state crate::storage::logic::state::VersionedRelationMetadata> {
        let end = history.partition_point(|entry| entry.effective_at() <= self.version_id);
        history[..end].iter().rev().find(|entry| {
            entry.effective_at() <= self.version_id
                && entry
                    .retired_at()
                    .is_none_or(|retired| self.version_id < retired)
        })
    }

    fn entity_partition_for_slot(
        &self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
    ) -> Option<&'state crate::storage::logic::state::PartitionState> {
        let partition = self.state.get_partition(partition_id)?;
        if self.state.entity_slot_is_touched(partition_id, slot)
            || self.state.touched_entity_slots(partition_id).is_none()
        {
            return Some(partition);
        }
        self.state.base_partition(partition_id).or(Some(partition))
    }

    fn relation_partition_for_slot(
        &self,
        partition_id: crate::identity::data::PartitionId,
        slot: usize,
    ) -> Option<&'state crate::storage::logic::state::PartitionState> {
        let partition = self.state.get_partition(partition_id)?;
        if self.state.relation_slot_is_touched(partition_id, slot)
            || self.state.touched_relation_slots(partition_id).is_none()
            || !partition.relation_overlay_is_sparse
        {
            return Some(partition);
        }
        self.state.base_partition(partition_id).or(Some(partition))
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VisibleEntityMetadata {
    pub(crate) entity_id: crate::identity::data::EntityId,
    pub(crate) kind_id: crate::identity::data::KindId,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VisibleRelationMetadata {
    pub(crate) relation_id: crate::identity::data::RelationId,
    pub(crate) kind_id: crate::identity::data::KindId,
    pub(crate) source: crate::identity::data::EntityId,
    pub(crate) target: crate::identity::data::EntityId,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
    use std::collections::BTreeSet;

    use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
    use crate::payloads::data::RecordPayload;
    use crate::storage::logic::state::{EntityArena, PartitionState, RelationArena, SlotInit};
    use crate::storage::overlay::{
        EntityWorkingSetLayout, OverlayStateView, PartitionCloneMode, WorkingState,
    };
    use crate::storage::substrate::{EntityRecordKind, RecordKind, RelationEndpoints};

    use super::InvariantStateView;

    #[test]
    fn sparse_speculative_overlay_reads_untouched_entity_truth_from_base_partition() {
        let policy = AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        };
        let partition_id = PartitionId(1);
        let mut base_partition = PartitionState {
            partition_id,
            adjacency_policy: policy.clone(),
            relation_overlay_is_sparse: false,
            entity_arena: EntityArena::with_capacity(2),
            relation_arena: RelationArena::with_capacity(0),
            adjacency: Vec::new(),
            reverse_adjacency: Vec::new(),
        };
        let _ = base_partition.entity_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"left"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let _ = base_partition.entity_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"right"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });

        let mut base = BTreeMap::new();
        base.insert(partition_id, base_partition);
        let sparse_slots = BTreeMap::from([(partition_id, [1usize].into_iter().collect())]);
        let staged = WorkingState::from_touched_partitions_with_layout_and_sparse_slots(
            &base,
            [partition_id],
            policy,
            PartitionCloneMode::EntityOnly,
            EntityWorkingSetLayout::AoSoACandidate { chunk_width: 128 },
            Some(&sparse_slots),
            None,
        );
        let overlay = OverlayStateView::new(&base, &staged);
        let state_view = InvariantStateView::new(&overlay, VersionId(1));

        let untouched_entity = EntityId::new(partition_id, 0, 1);
        let payload = state_view
            .entity_payload(untouched_entity)
            .expect("untouched payload should read through to base");
        let metadata = state_view
            .entity_metadata(untouched_entity)
            .expect("untouched metadata should read through to base");

        assert_eq!(
            payload,
            &RecordPayload::StructuredJson(serde_json::json!({"name":"left"}))
        );
        assert_eq!(metadata.kind_id, KindId(1));
        assert_eq!(metadata.entity_id, untouched_entity);
    }

    #[test]
    fn sparse_speculative_overlay_reads_untouched_relation_truth_from_base_partition() {
        let policy = AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        };
        let partition_id = PartitionId(1);
        let mut base_partition = PartitionState {
            partition_id,
            adjacency_policy: policy.clone(),
            relation_overlay_is_sparse: false,
            entity_arena: EntityArena::with_capacity(2),
            relation_arena: RelationArena::with_capacity(1),
            adjacency: Vec::new(),
            reverse_adjacency: Vec::new(),
        };
        let (left_slot, left_generation, _) = base_partition.entity_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"left"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let (right_slot, right_generation, _) = base_partition.entity_arena.push_slot(SlotInit {
            partition_id,
            kind_id: KindId(1),
            payload: Some(RecordPayload::StructuredJson(
                serde_json::json!({"name":"right"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let left = EntityId::new(partition_id, left_slot as u64, left_generation);
        let right = EntityId::new(partition_id, right_slot as u64, right_generation);
        let (relation_slot, relation_generation, _) =
            base_partition.relation_arena.push_slot(SlotInit {
                partition_id,
                kind_id: KindId(9),
                payload: Some(RecordPayload::StructuredJson(
                    serde_json::json!({"kind":"edge"}),
                )),
                version_id: VersionId(1),
                extra: Some(RelationEndpoints {
                    source: left,
                    target: right,
                }),
            });
        let relation_id = RelationId::new(partition_id, relation_slot as u64, relation_generation);

        let mut base = BTreeMap::new();
        base.insert(partition_id, base_partition);
        let sparse_slots = BTreeMap::from([(partition_id, [0usize].into_iter().collect())]);
        let sparse_relation_partitions = BTreeSet::from([partition_id]);
        let staged = WorkingState::from_touched_partitions_with_layout_and_sparse_slots(
            &base,
            [partition_id],
            policy,
            PartitionCloneMode::GraphSparseEntities,
            EntityWorkingSetLayout::AoSoACandidate { chunk_width: 128 },
            Some(&sparse_slots),
            Some(&sparse_relation_partitions),
        );
        let overlay = OverlayStateView::new(&base, &staged);
        let state_view = InvariantStateView::new(&overlay, VersionId(1));

        let payload = state_view
            .relation_payload(relation_id)
            .expect("untouched relation payload should read through to base");
        let metadata = state_view
            .relation_metadata(relation_id)
            .expect("untouched relation metadata should read through to base");

        assert_eq!(
            payload,
            &RecordPayload::StructuredJson(serde_json::json!({"kind":"edge"}))
        );
        assert_eq!(metadata.kind_id, KindId(9));
        assert_eq!(metadata.relation_id, relation_id);
        assert_eq!(metadata.source, left);
        assert_eq!(metadata.target, right);
    }
}
