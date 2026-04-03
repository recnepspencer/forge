use std::collections::BTreeMap;

use crate::config::data::AdjacencyPolicy;
use crate::identity::data::PartitionId;
use crate::storage::substrate::{EntityArena, RelationArena};

use super::{PartitionAccess, PartitionCloneMode, PartitionMutationJournal, PartitionState};

#[derive(Debug, Clone)]
pub(crate) struct WorkingState {
    pub(crate) adjacency_policy: AdjacencyPolicy,
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
            partitions,
            mutation_journal: BTreeMap::new(),
        }
    }

    pub(crate) fn from_touched_partitions(
        base_partitions: &BTreeMap<PartitionId, PartitionState>,
        touched_partitions: impl IntoIterator<Item = PartitionId>,
        adjacency_policy: AdjacencyPolicy,
        clone_mode: PartitionCloneMode,
    ) -> Self {
        let mut partitions = BTreeMap::new();
        let mut mutation_journal = BTreeMap::new();
        for partition_id in touched_partitions {
            if mutation_journal.contains_key(&partition_id) {
                continue;
            }
            mutation_journal.insert(partition_id, PartitionMutationJournal::default());
            if let Some(partition) = base_partitions.get(&partition_id) {
                partitions.insert(partition_id, partition.clone_for_overlay(clone_mode));
            }
        }
        Self {
            adjacency_policy,
            partitions,
            mutation_journal,
        }
    }

    pub(crate) fn into_partition_commits(
        self,
    ) -> BTreeMap<PartitionId, (PartitionState, PartitionMutationJournal)> {
        let WorkingState {
            partitions,
            mutation_journal,
            ..
        } = self;
        partitions
            .into_iter()
            .map(|(partition_id, partition)| {
                let journal = mutation_journal
                    .get(&partition_id)
                    .cloned()
                    .unwrap_or_default();
                (partition_id, (partition, journal))
            })
            .collect()
    }

    pub(crate) fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState {
        self.mutation_journal.entry(partition_id).or_default();
        self.partitions
            .entry(partition_id)
            .or_insert_with(|| PartitionState {
                partition_id,
                adjacency_policy: self.adjacency_policy.clone(),
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
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::config::data::AdjacencyBackend;
    use crate::identity::data::PartitionId;
    use crate::storage::logic::state::{EntityArena, PartitionState, RelationArena};

    use super::{PartitionCloneMode, WorkingState};

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
}
