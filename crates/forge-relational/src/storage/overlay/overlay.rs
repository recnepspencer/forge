use std::collections::{BTreeMap, BTreeSet};

use crate::config::data::AdjacencyPolicy;
use crate::identity::data::PartitionId;
use crate::storage::substrate::{EntityArena, RelationArena};

use super::{PartitionAccess, PartitionMutationJournal, PartitionState};

#[derive(Debug, Clone)]
pub(crate) struct WorkingState {
    pub(crate) adjacency_policy: AdjacencyPolicy,
    pub(crate) partitions: BTreeMap<PartitionId, PartitionState>,
    pub(crate) mutation_journal: BTreeMap<PartitionId, PartitionMutationJournal>,
}

impl WorkingState {
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
    ) -> Self {
        let mut partitions = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for partition_id in touched_partitions {
            if !seen.insert(partition_id) {
                continue;
            }
            if let Some(partition) = base_partitions.get(&partition_id) {
                partitions.insert(partition_id, partition.clone());
            }
        }
        Self::new(partitions, adjacency_policy)
    }

    pub(crate) fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState {
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

    pub(crate) fn mark_relation_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .relation_slots
            .insert(slot);
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

#[derive(Debug, Clone)]
pub(crate) struct RelationalDraft {
    pub(crate) touched_partitions: BTreeSet<PartitionId>,
    pub(crate) working: WorkingState,
}

impl RelationalDraft {
    pub(crate) fn from_touched_partitions(
        base_partitions: &BTreeMap<PartitionId, PartitionState>,
        touched_partitions: impl IntoIterator<Item = PartitionId>,
        adjacency_policy: AdjacencyPolicy,
    ) -> Self {
        let touched_partitions = touched_partitions.into_iter().collect::<BTreeSet<_>>();
        Self {
            working: WorkingState::from_touched_partitions(
                base_partitions,
                touched_partitions.iter().copied(),
                adjacency_policy,
            ),
            touched_partitions,
        }
    }

    pub(crate) fn commit(self) -> BTreeMap<PartitionId, PartitionState> {
        self.working.partitions
    }

    pub(crate) fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState {
        self.working.get_partition_mut(partition_id)
    }

    pub(crate) fn mark_entity_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.working.mark_entity_slot_touched(partition_id, slot);
    }

    pub(crate) fn mark_relation_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.working.mark_relation_slot_touched(partition_id, slot);
    }

    pub(crate) fn mark_adjacency_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.working.mark_adjacency_slot_touched(partition_id, slot);
    }

    pub(crate) fn mark_reverse_adjacency_slot_touched(
        &mut self,
        partition_id: PartitionId,
        slot: usize,
    ) {
        self.working
            .mark_reverse_adjacency_slot_touched(partition_id, slot);
    }

    pub(crate) fn reserve_entity_slots(&mut self, partition_id: PartitionId, additional: usize) {
        self.working.reserve_entity_slots(partition_id, additional);
    }

    pub(crate) fn reserve_relation_slots(&mut self, partition_id: PartitionId, additional: usize) {
        self.working.reserve_relation_slots(partition_id, additional);
    }

    pub(crate) fn mutation_journal(&self) -> &BTreeMap<PartitionId, PartitionMutationJournal> {
        &self.working.mutation_journal
    }

    pub(crate) fn touched_partitions(&self) -> &BTreeSet<PartitionId> {
        &self.touched_partitions
    }
}

impl PartitionAccess for RelationalDraft {
    fn get_partition(&self, partition_id: PartitionId) -> Option<&PartitionState> {
        self.working.get_partition(partition_id)
    }

    fn partition_ids(&self) -> Vec<PartitionId> {
        self.working.partition_ids()
    }

    fn touched_entity_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.working.touched_entity_slots(partition_id)
    }

    fn touched_relation_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.working.touched_relation_slots(partition_id)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::config::data::AdjacencyBackend;
    use crate::identity::data::PartitionId;
    use crate::storage::logic::state::{EntityArena, PartitionState, RelationArena};

    use super::WorkingState;

    #[test]
    fn touched_partition_overlay_only_clones_selected_partitions() {
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

        let overlay = WorkingState::from_touched_partitions(&base, [right], policy);

        assert!(!overlay.partitions.contains_key(&left));
        assert!(overlay.partitions.contains_key(&right));
    }
}
