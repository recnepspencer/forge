use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::config::data::AdjacencyPolicy;
use crate::identity::data::PartitionId;
use crate::storage::substrate::{EntityArena, RelationArena};

use super::{
    EntityWorkingSetLayout, PartitionAccess, PartitionCloneMode, PartitionMutationJournal,
    PartitionState,
};

#[derive(Debug, Clone)]
pub(crate) struct WorkingState {
    pub(crate) adjacency_policy: AdjacencyPolicy,
    pub(crate) clone_mode: PartitionCloneMode,
    pub(crate) entity_working_set_layout: EntityWorkingSetLayout,
    pub(crate) partitions: BTreeMap<PartitionId, PartitionState>,
    pub(crate) mutation_journal: BTreeMap<PartitionId, PartitionMutationJournal>,
}

impl WorkingState {
    #[cfg(test)]
    pub(crate) fn new(
        partitions: BTreeMap<PartitionId, PartitionState>,
        adjacency_policy: AdjacencyPolicy,
    ) -> Self {
        Self {
            adjacency_policy,
            clone_mode: PartitionCloneMode::Full,
            entity_working_set_layout: EntityWorkingSetLayout::CanonicalSoA,
            partitions,
            mutation_journal: BTreeMap::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_touched_partitions(
        base_partitions: &BTreeMap<PartitionId, PartitionState>,
        touched_partitions: impl IntoIterator<Item = PartitionId>,
        adjacency_policy: AdjacencyPolicy,
        clone_mode: PartitionCloneMode,
    ) -> Self {
        Self::from_touched_partitions_with_layout(
            base_partitions,
            touched_partitions,
            adjacency_policy,
            clone_mode,
            EntityWorkingSetLayout::CanonicalSoA,
        )
    }

    pub(crate) fn from_touched_partitions_with_layout(
        base_partitions: &BTreeMap<PartitionId, PartitionState>,
        touched_partitions: impl IntoIterator<Item = PartitionId>,
        adjacency_policy: AdjacencyPolicy,
        clone_mode: PartitionCloneMode,
        entity_working_set_layout: EntityWorkingSetLayout,
    ) -> Self {
        Self::from_touched_partitions_with_layout_and_sparse_slots(
            base_partitions,
            touched_partitions,
            adjacency_policy,
            clone_mode,
            entity_working_set_layout,
            None,
            None,
        )
    }

    pub(crate) fn from_touched_partitions_with_layout_and_sparse_slots(
        base_partitions: &BTreeMap<PartitionId, PartitionState>,
        touched_partitions: impl IntoIterator<Item = PartitionId>,
        adjacency_policy: AdjacencyPolicy,
        clone_mode: PartitionCloneMode,
        entity_working_set_layout: EntityWorkingSetLayout,
        sparse_entity_slots: Option<&BTreeMap<PartitionId, BTreeSet<usize>>>,
        sparse_relation_overlay_partitions: Option<&BTreeSet<PartitionId>>,
    ) -> Self {
        let mut partitions = BTreeMap::new();
        let mut mutation_journal = BTreeMap::new();
        for partition_id in touched_partitions {
            if mutation_journal.contains_key(&partition_id) {
                continue;
            }
            mutation_journal.insert(partition_id, PartitionMutationJournal::default());
            if let Some(partition) = base_partitions.get(&partition_id) {
                let partition_state = match (
                    clone_mode,
                    sparse_entity_slots.and_then(|slots| slots.get(&partition_id)),
                    sparse_relation_overlay_partitions
                        .is_some_and(|partitions| partitions.contains(&partition_id)),
                ) {
                    (PartitionCloneMode::GraphSparseEntities, Some(touched_slots), true)
                        if !touched_slots.is_empty() =>
                    {
                        partition
                            .clone_graph_with_sparse_entity_slots_and_relation_shape(touched_slots)
                    }
                    (PartitionCloneMode::EntityOnly, Some(touched_slots), _)
                        if !touched_slots.is_empty() =>
                    {
                        partition.clone_entity_slots_for_overlay(touched_slots)
                    }
                    (PartitionCloneMode::GraphSparseEntities, Some(touched_slots), _)
                        if !touched_slots.is_empty() =>
                    {
                        partition.clone_graph_with_sparse_entity_slots(touched_slots)
                    }
                    (PartitionCloneMode::GraphSparseEntities, None, true) => {
                        partition.clone_graph_with_sparse_relation_shape()
                    }
                    _ => partition.clone_for_overlay(clone_mode),
                };
                partitions.insert(partition_id, partition_state);
            }
        }
        Self {
            adjacency_policy,
            clone_mode,
            entity_working_set_layout,
            partitions,
            mutation_journal,
        }
    }

    pub(crate) fn into_partition_commits(
        self,
    ) -> (
        PartitionCloneMode,
        BTreeMap<PartitionId, (PartitionState, PartitionMutationJournal)>,
    ) {
        let WorkingState {
            clone_mode,
            partitions,
            mutation_journal,
            ..
        } = self;
        (
            clone_mode,
            partitions
                .into_iter()
                .map(|(partition_id, partition)| {
                    let journal = mutation_journal
                        .get(&partition_id)
                        .cloned()
                        .unwrap_or_default();
                    (partition_id, (partition, journal))
                })
                .collect(),
        )
    }

    pub(crate) fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState {
        self.mutation_journal.entry(partition_id).or_default();
        self.partitions
            .entry(partition_id)
            .or_insert_with(|| PartitionState {
                partition_id,
                adjacency_policy: self.adjacency_policy.clone(),
                relation_overlay_is_sparse: false,
                entity_arena: EntityArena::with_capacity(0),
                relation_arena: RelationArena::with_capacity(0),
                adjacency: Vec::new(),
                reverse_adjacency: Vec::new(),
            })
    }

    pub(crate) fn mark_entity_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .entity_slots
            .insert(slot);
    }

    pub(crate) fn mark_entity_free_list_changed(&mut self, partition_id: PartitionId) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .entity_free_list_changed = true;
    }

    pub(crate) fn mark_relation_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .relation_slots
            .insert(slot);
    }

    pub(crate) fn mark_relation_free_list_changed(&mut self, partition_id: PartitionId) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .relation_free_list_changed = true;
    }

    pub(crate) fn mark_adjacency_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .adjacency_slots
            .insert(slot);
    }

    pub(crate) fn mark_reverse_adjacency_slot_touched(
        &mut self,
        partition_id: PartitionId,
        slot: usize,
    ) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .reverse_adjacency_slots
            .insert(slot);
    }

    pub(crate) fn reserve_entity_slots(&mut self, partition_id: PartitionId, additional: usize) {
        if additional == 0 {
            return;
        }
        let partition = self.get_partition_mut(partition_id);
        partition.entity_arena.reserve_additional(additional);
        partition.adjacency.reserve(additional);
        partition.reverse_adjacency.reserve(additional);
    }

    pub(crate) fn reserve_relation_slots(&mut self, partition_id: PartitionId, additional: usize) {
        if additional == 0 {
            return;
        }
        let partition = self.get_partition_mut(partition_id);
        partition.relation_arena.reserve_additional(additional);
    }

    pub(crate) fn mutation_journal(&self) -> &BTreeMap<PartitionId, PartitionMutationJournal> {
        &self.mutation_journal
    }

    pub(crate) fn touched_partition_count(&self) -> usize {
        self.mutation_journal.len()
    }

    pub(crate) fn entity_working_set_layout(&self) -> EntityWorkingSetLayout {
        self.entity_working_set_layout
    }

    pub(crate) fn clone_mode(&self) -> PartitionCloneMode {
        self.clone_mode
    }
}

impl PartitionAccess for WorkingState {
    fn get_partition(&self, partition_id: PartitionId) -> Option<&PartitionState> {
        self.partitions.get(&partition_id)
    }

    fn partition_ids(&self) -> Vec<PartitionId> {
        self.partitions.keys().copied().collect()
    }

    fn touched_entity_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.mutation_journal
            .get(&partition_id)
            .map(|journal| journal.entity_slots.iter().copied().collect())
    }

    fn touched_relation_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.mutation_journal
            .get(&partition_id)
            .map(|journal| journal.relation_slots.iter().copied().collect())
    }

    fn entity_slot_is_touched(&self, partition_id: PartitionId, slot: usize) -> bool {
        self.mutation_journal
            .get(&partition_id)
            .is_some_and(|journal| journal.entity_slots.contains(&slot))
    }

    fn relation_slot_is_touched(&self, partition_id: PartitionId, slot: usize) -> bool {
        self.mutation_journal
            .get(&partition_id)
            .is_some_and(|journal| journal.relation_slots.contains(&slot))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::config::data::AdjacencyBackend;
    use crate::identity::data::{EntityId, KindId, PartitionId, VersionId};
    use crate::storage::logic::state::RelationEndpoints;
    use crate::storage::logic::state::{EntityArena, PartitionState, RelationArena};
    use crate::storage::substrate::{EntityRecordKind, RecordKind, SlotInit};

    use super::{EntityWorkingSetLayout, PartitionCloneMode, WorkingState};

    #[test]
    fn touched_partition_working_state_only_clones_selected_partitions() {
        let policy = crate::config::data::AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        };
        let left = PartitionId(1);
        let right = PartitionId(2);
        let mut base = BTreeMap::new();
        base.insert(
            left,
            PartitionState {
                partition_id: left,
                adjacency_policy: policy.clone(),
                relation_overlay_is_sparse: false,
                entity_arena: EntityArena::with_capacity(1),
                relation_arena: RelationArena::with_capacity(0),
                adjacency: Vec::new(),
                reverse_adjacency: Vec::new(),
            },
        );
        base.insert(
            right,
            PartitionState {
                partition_id: right,
                adjacency_policy: policy.clone(),
                relation_overlay_is_sparse: false,
                entity_arena: EntityArena::with_capacity(1),
                relation_arena: RelationArena::with_capacity(0),
                adjacency: Vec::new(),
                reverse_adjacency: Vec::new(),
            },
        );

        let overlay =
            WorkingState::from_touched_partitions(&base, [right], policy, PartitionCloneMode::Full);

        assert!(!overlay.partitions.contains_key(&left));
        assert!(overlay.partitions.contains_key(&right));
        assert_eq!(overlay.touched_partition_count(), 1);
    }

    #[test]
    fn touched_partition_working_state_preserves_candidate_layout_metadata() {
        let policy = crate::config::data::AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        };
        let partition = PartitionId(1);
        let mut base = BTreeMap::new();
        base.insert(
            partition,
            PartitionState {
                partition_id: partition,
                adjacency_policy: policy.clone(),
                relation_overlay_is_sparse: false,
                entity_arena: EntityArena::with_capacity(8),
                relation_arena: RelationArena::with_capacity(0),
                adjacency: Vec::new(),
                reverse_adjacency: Vec::new(),
            },
        );

        let overlay = WorkingState::from_touched_partitions_with_layout(
            &base,
            [partition],
            policy,
            PartitionCloneMode::EntityOnly,
            EntityWorkingSetLayout::AoSoACandidate { chunk_width: 256 },
        );

        assert_eq!(
            overlay.entity_working_set_layout(),
            EntityWorkingSetLayout::AoSoACandidate { chunk_width: 256 }
        );
    }

    #[test]
    fn sparse_entity_overlay_only_materializes_touched_slot_payload_history() {
        let policy = crate::config::data::AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        };
        let partition = PartitionId(1);
        let mut base_partition = PartitionState {
            partition_id: partition,
            adjacency_policy: policy.clone(),
            relation_overlay_is_sparse: false,
            entity_arena: EntityArena::with_capacity(2),
            relation_arena: RelationArena::with_capacity(0),
            adjacency: Vec::new(),
            reverse_adjacency: Vec::new(),
        };
        let _ = base_partition.entity_arena.push_slot(SlotInit {
            partition_id: partition,
            kind_id: KindId(1),
            payload: Some(crate::payloads::data::RecordPayload::StructuredJson(
                serde_json::json!({"name":"left"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });
        let _ = base_partition.entity_arena.push_slot(SlotInit {
            partition_id: partition,
            kind_id: KindId(1),
            payload: Some(crate::payloads::data::RecordPayload::StructuredJson(
                serde_json::json!({"name":"right"}),
            )),
            version_id: VersionId(1),
            extra: EntityRecordKind::empty_extra(),
        });

        let mut base = BTreeMap::new();
        base.insert(partition, base_partition);
        let mut sparse_slots = BTreeMap::new();
        sparse_slots.insert(partition, [1usize].into_iter().collect());

        let overlay = WorkingState::from_touched_partitions_with_layout_and_sparse_slots(
            &base,
            [partition],
            policy,
            PartitionCloneMode::EntityOnly,
            EntityWorkingSetLayout::AoSoACandidate { chunk_width: 256 },
            Some(&sparse_slots),
            None,
        );

        let partition_state = overlay
            .partitions
            .get(&partition)
            .expect("partition present");
        assert!(partition_state.entity_arena.payload_history[0].is_empty());
        assert_eq!(partition_state.entity_arena.payload_history[1].len(), 1);
        assert!(partition_state.entity_arena.payloads[0].is_none());
        assert!(partition_state.entity_arena.payloads[1].is_some());
    }

    #[test]
    fn sparse_relation_overlay_keeps_relation_shape_without_full_relation_payload_clone() {
        let policy = crate::config::data::AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        };
        let partition = PartitionId(11);
        let mut base_partition = PartitionState {
            partition_id: partition,
            adjacency_policy: policy.clone(),
            relation_overlay_is_sparse: false,
            entity_arena: EntityArena::with_capacity(0),
            relation_arena: RelationArena::with_capacity(2),
            adjacency: Vec::new(),
            reverse_adjacency: Vec::new(),
        };
        let _ = base_partition.relation_arena.push_slot(SlotInit {
            partition_id: partition,
            kind_id: KindId(2),
            payload: Some(crate::payloads::data::RecordPayload::StructuredJson(
                serde_json::json!({"edge":"left"}),
            )),
            version_id: VersionId(1),
            extra: Some(RelationEndpoints {
                source: EntityId::new(PartitionId(1), 0, 1),
                target: EntityId::new(PartitionId(2), 0, 1),
            }),
        });

        let mut base = BTreeMap::new();
        base.insert(partition, base_partition);
        let sparse_relation_partitions = BTreeSet::from([partition]);

        let overlay = WorkingState::from_touched_partitions_with_layout_and_sparse_slots(
            &base,
            [partition],
            policy,
            PartitionCloneMode::GraphSparseEntities,
            EntityWorkingSetLayout::CanonicalSoA,
            None,
            Some(&sparse_relation_partitions),
        );

        let partition_state = overlay
            .partitions
            .get(&partition)
            .expect("partition present");
        assert!(partition_state.relation_overlay_is_sparse);
        assert!(partition_state.relation_arena.payloads[0].is_none());
        assert!(partition_state.relation_arena.payload_history[0].is_empty());
    }
}
