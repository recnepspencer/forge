use std::collections::{BTreeMap, BTreeSet};

use crate::config::data::AdjacencyPolicy;
use crate::identity::data::PartitionId;

use crate::storage::partition::{AdjacencySet, DenseSlotBitSet};
use crate::storage::substrate::{EntityArena, RelationArena};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PartitionCloneMode {
    Full,
    EntityOnly,
}

#[derive(Debug, Clone)]
pub(crate) struct PartitionState {
    pub(crate) partition_id: PartitionId,
    pub(crate) adjacency_policy: AdjacencyPolicy,
    pub(crate) entity_arena: EntityArena,
    pub(crate) relation_arena: RelationArena,
    pub(crate) adjacency: Vec<AdjacencySet>,
    pub(crate) reverse_adjacency: Vec<AdjacencySet>,
}

impl PartitionState {
    pub(crate) fn clone_for_overlay(&self, mode: PartitionCloneMode) -> Self {
        match mode {
            PartitionCloneMode::Full => self.clone(),
            PartitionCloneMode::EntityOnly => Self {
                partition_id: self.partition_id,
                adjacency_policy: self.adjacency_policy.clone(),
                entity_arena: self.entity_arena.clone(),
                relation_arena: RelationArena::with_capacity(0),
                adjacency: Vec::new(),
                reverse_adjacency: Vec::new(),
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PartitionMutationJournal {
    pub(crate) entity_slots: BTreeSet<usize>,
    pub(crate) relation_slots: BTreeSet<usize>,
    pub(crate) adjacency_slots: BTreeSet<usize>,
    pub(crate) reverse_adjacency_slots: BTreeSet<usize>,
    pub(crate) entity_free_list_changed: bool,
    pub(crate) relation_free_list_changed: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SnapshotPartitionPins {
    pub(crate) entity_slots: DenseSlotBitSet,
    pub(crate) relation_slots: DenseSlotBitSet,
    pub(crate) retained_relation_slots: DenseSlotBitSet,
}

#[derive(Debug, Clone)]
pub(crate) struct SnapshotState {
    pub(crate) handle: crate::snapshots::data::SnapshotHandle,
    pub(crate) pinned_partitions: BTreeMap<PartitionId, SnapshotPartitionPins>,
    pub(crate) pinned_entity_count: usize,
    pub(crate) pinned_relation_count: usize,
}
