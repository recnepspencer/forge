use std::collections::{BTreeMap, BTreeSet};

use crate::config::data::AdjacencyPolicy;
use crate::identity::data::PartitionId;
use crate::replay::data::RelationalReplayRecord;

use super::{AdjacencySet, DenseSlotBitSet, EntityArena, RelationArena};

#[derive(Debug, Clone)]
pub(crate) struct PartitionState {
    pub(crate) partition_id: PartitionId,
    pub(crate) adjacency_policy: AdjacencyPolicy,
    pub(crate) entity_arena: EntityArena,
    pub(crate) relation_arena: RelationArena,
    pub(crate) adjacency: Vec<AdjacencySet>,
    pub(crate) reverse_adjacency: Vec<AdjacencySet>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PartitionMutationJournal {
    pub(crate) entity_slots: BTreeSet<usize>,
    pub(crate) relation_slots: BTreeSet<usize>,
    pub(crate) adjacency_slots: BTreeSet<usize>,
    pub(crate) reverse_adjacency_slots: BTreeSet<usize>,
}

#[derive(Debug, Clone)]
pub(crate) struct SnapshotPartitionPins {
    pub(crate) entity_slots: DenseSlotBitSet,
    pub(crate) relation_slots: DenseSlotBitSet,
}

#[derive(Debug, Clone)]
pub(crate) struct SnapshotState {
    pub(crate) handle: crate::snapshots::data::SnapshotHandle,
    pub(crate) pinned_partitions: BTreeMap<PartitionId, SnapshotPartitionPins>,
    pub(crate) pinned_entity_count: usize,
    pub(crate) pinned_relation_count: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct WorkingState {
    pub(crate) adjacency_policy: AdjacencyPolicy,
    pub(crate) partitions: BTreeMap<PartitionId, PartitionState>,
    pub(crate) mutation_journal: BTreeMap<PartitionId, PartitionMutationJournal>,
}

pub(crate) trait PartitionAccess {
    fn get_partition(&self, partition_id: PartitionId) -> Option<&PartitionState>;
    fn partition_ids(&self) -> Vec<PartitionId>;

    fn touched_entity_slots(&self, _partition_id: PartitionId) -> Option<Vec<usize>> {
        None
    }

    fn touched_relation_slots(&self, _partition_id: PartitionId) -> Option<Vec<usize>> {
        None
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedWorkingState<'a> {
    pub(crate) partitions: &'a BTreeMap<PartitionId, PartitionState>,
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

    pub(crate) fn apply_to_runtime(
        self,
        runtime_partitions: &mut BTreeMap<PartitionId, PartitionState>,
    ) {
        *runtime_partitions = self.partitions;
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

impl<'a> BorrowedWorkingState<'a> {
    pub(crate) fn new(partitions: &'a BTreeMap<PartitionId, PartitionState>) -> Self {
        Self { partitions }
    }
}

impl PartitionAccess for BorrowedWorkingState<'_> {
    fn get_partition(&self, partition_id: PartitionId) -> Option<&PartitionState> {
        self.partitions.get(&partition_id)
    }

    fn partition_ids(&self) -> Vec<PartitionId> {
        self.partitions.keys().copied().collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PublicationArtifacts {
    pub(crate) snapshot: crate::snapshots::data::SnapshotHandle,
    pub(crate) diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    pub(crate) bundle: crate::publication::data::PublicationBundle<RelationalReplayRecord>,
}
