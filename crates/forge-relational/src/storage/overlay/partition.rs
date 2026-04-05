use std::collections::{BTreeMap, BTreeSet};

use crate::config::data::AdjacencyPolicy;
use crate::identity::data::PartitionId;

use crate::storage::partition::{AdjacencySet, DenseSlotBitSet};
use crate::storage::substrate::{EntityArena, RelationArena};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PartitionCloneMode {
    Full,
    EntityOnly,
    GraphSparseEntities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntityWorkingSetLayout {
    CanonicalSoA,
    AoSoACandidate { chunk_width: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct EntityChunkPlanSummary {
    pub(crate) chunk_width: usize,
    pub(crate) chunk_count: usize,
    pub(crate) slot_count: usize,
}

pub(crate) fn summarize_entity_chunk_plan(
    slot_count: usize,
    layout: EntityWorkingSetLayout,
) -> EntityChunkPlanSummary {
    match layout {
        EntityWorkingSetLayout::CanonicalSoA => EntityChunkPlanSummary::default(),
        EntityWorkingSetLayout::AoSoACandidate { chunk_width } => EntityChunkPlanSummary {
            chunk_width,
            chunk_count: if slot_count == 0 {
                0
            } else {
                slot_count.div_ceil(chunk_width)
            },
            slot_count,
        },
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PartitionState {
    pub(crate) partition_id: PartitionId,
    pub(crate) adjacency_policy: AdjacencyPolicy,
    pub(crate) relation_overlay_is_sparse: bool,
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
                relation_overlay_is_sparse: false,
                entity_arena: self.entity_arena.clone(),
                relation_arena: RelationArena::with_capacity(0),
                adjacency: Vec::new(),
                reverse_adjacency: Vec::new(),
            },
            PartitionCloneMode::GraphSparseEntities => Self {
                partition_id: self.partition_id,
                adjacency_policy: self.adjacency_policy.clone(),
                relation_overlay_is_sparse: false,
                entity_arena: EntityArena::with_capacity(0),
                relation_arena: self.relation_arena.clone(),
                adjacency: self.adjacency.clone(),
                reverse_adjacency: self.reverse_adjacency.clone(),
            },
        }
    }

    pub(crate) fn clone_entity_slots_for_overlay(
        &self,
        touched_entity_slots: &BTreeSet<usize>,
    ) -> Self {
        Self {
            partition_id: self.partition_id,
            adjacency_policy: self.adjacency_policy.clone(),
            relation_overlay_is_sparse: false,
            entity_arena: self
                .entity_arena
                .sparse_clone_slots_for_overlay(touched_entity_slots),
            relation_arena: RelationArena::with_capacity(0),
            adjacency: Vec::new(),
            reverse_adjacency: Vec::new(),
        }
    }

    pub(crate) fn clone_graph_with_sparse_entity_slots(
        &self,
        touched_entity_slots: &BTreeSet<usize>,
    ) -> Self {
        Self {
            partition_id: self.partition_id,
            adjacency_policy: self.adjacency_policy.clone(),
            relation_overlay_is_sparse: false,
            entity_arena: self
                .entity_arena
                .sparse_clone_slots_for_overlay(touched_entity_slots),
            relation_arena: self.relation_arena.clone(),
            adjacency: self.adjacency.clone(),
            reverse_adjacency: self.reverse_adjacency.clone(),
        }
    }

    pub(crate) fn clone_graph_with_sparse_entity_slots_and_relation_shape(
        &self,
        touched_entity_slots: &BTreeSet<usize>,
    ) -> Self {
        Self {
            partition_id: self.partition_id,
            adjacency_policy: self.adjacency_policy.clone(),
            relation_overlay_is_sparse: true,
            entity_arena: self
                .entity_arena
                .sparse_clone_slots_for_overlay(touched_entity_slots),
            relation_arena: self.relation_arena.sparse_shape_clone_for_overlay(),
            adjacency: self.adjacency.clone(),
            reverse_adjacency: self.reverse_adjacency.clone(),
        }
    }

    pub(crate) fn clone_graph_with_sparse_relation_shape(&self) -> Self {
        Self {
            partition_id: self.partition_id,
            adjacency_policy: self.adjacency_policy.clone(),
            relation_overlay_is_sparse: true,
            entity_arena: EntityArena::with_capacity(0),
            relation_arena: self.relation_arena.sparse_shape_clone_for_overlay(),
            adjacency: self.adjacency.clone(),
            reverse_adjacency: self.reverse_adjacency.clone(),
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
