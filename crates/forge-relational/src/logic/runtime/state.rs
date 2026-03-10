use std::collections::{BTreeMap, BTreeSet};

use crate::data::config::{AdjacencyBackend, AdjacencyPolicy};
use crate::data::identity::{
    EntityId, KindId, LineageId, PartitionId, RelationId, StructuralFingerprint, VersionId,
};
use crate::data::payload::RecordPayload;
use crate::data::symbols::Symbol;

use super::types::{RecordLifecycleState, RelationalReplayRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DenseSlotBitSet {
    words: Vec<u64>,
}

impl DenseSlotBitSet {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            words: vec![0; capacity.div_ceil(64)],
        }
    }

    fn ensure_capacity(&mut self, slot: usize) {
        let required = slot / 64 + 1;
        if self.words.len() < required {
            self.words.resize(required, 0);
        }
    }

    pub(super) fn set(&mut self, slot: usize, value: bool) {
        self.ensure_capacity(slot);
        let word = slot / 64;
        let bit = slot % 64;
        if value {
            self.words[word] |= 1 << bit;
        } else {
            self.words[word] &= !(1 << bit);
        }
    }

    pub(super) fn count_ones(&self) -> usize {
        self.words.iter().map(|word| word.count_ones() as usize).sum()
    }
}

#[derive(Debug, Clone)]
pub(super) struct VersionedPayload {
    pub(super) effective_at: VersionId,
    pub(super) retired_at: Option<VersionId>,
    pub(super) value: RecordPayload,
}

pub(super) type VersionedValue = VersionedPayload;

#[derive(Debug, Clone)]
pub(super) enum AdjacencySet {
    Inline(Vec<RelationId>),
    Compressed(BTreeSet<RelationId>),
}

impl AdjacencySet {
    pub(super) fn new(policy: &AdjacencyPolicy) -> Self {
        match policy.backend {
            AdjacencyBackend::InlineSmallDegreeAdjacency => {
                Self::Inline(Vec::with_capacity(policy.small_degree_inline_capacity))
            }
            AdjacencyBackend::CompressedFanoutAdjacency => Self::Compressed(BTreeSet::new()),
        }
    }

    pub(super) fn clear(&mut self) {
        match self {
            Self::Inline(relations) => relations.clear(),
            Self::Compressed(relations) => relations.clear(),
        }
    }

    pub(super) fn insert(&mut self, relation_id: RelationId) {
        match self {
            Self::Inline(relations) => match relations.binary_search(&relation_id) {
                Ok(_) => {}
                Err(index) => relations.insert(index, relation_id),
            },
            Self::Compressed(relations) => {
                relations.insert(relation_id);
            }
        }
    }

    pub(super) fn remove(&mut self, relation_id: &RelationId) {
        match self {
            Self::Inline(relations) => {
                if let Ok(index) = relations.binary_search(relation_id) {
                    relations.remove(index);
                }
            }
            Self::Compressed(relations) => {
                relations.remove(relation_id);
            }
        }
    }

    pub(super) fn ids(&self) -> Vec<RelationId> {
        match self {
            Self::Inline(relations) => relations.clone(),
            Self::Compressed(relations) => relations.iter().copied().collect(),
        }
    }

    pub(super) fn extend_into(&self, target: &mut BTreeSet<RelationId>) {
        match self {
            Self::Inline(relations) => target.extend(relations.iter().copied()),
            Self::Compressed(relations) => target.extend(relations.iter().copied()),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct EntityArena {
    pub(super) partition_ids: Vec<PartitionId>,
    pub(super) generations: Vec<u32>,
    pub(super) lifecycle: Vec<RecordLifecycleState>,
    pub(super) kind_ids: Vec<Option<KindId>>,
    pub(super) payloads: Vec<Option<RecordPayload>>,
    pub(super) payload_history: Vec<Vec<VersionedPayload>>,
    pub(super) created_at: Vec<VersionId>,
    pub(super) retired_at: Vec<Option<VersionId>>,
    pub(super) aspect_versions: Vec<BTreeMap<Symbol, u64>>,
    pub(super) structural_fingerprints: Vec<Option<StructuralFingerprint>>,
    pub(super) lineage_ids: Vec<Option<LineageId>>,
    pub(super) diagnostics_enrichment: Vec<BTreeMap<Symbol, String>>,
    pub(super) branch_pins: Vec<u32>,
    pub(super) replay_pins: Vec<u32>,
    pub(super) snapshot_pins: Vec<u32>,
    pub(super) live_bitset: DenseSlotBitSet,
    pub(super) reclaimable_bitset: DenseSlotBitSet,
    pub(super) free_list: Vec<u64>,
}

impl EntityArena {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            partition_ids: Vec::with_capacity(capacity),
            generations: Vec::with_capacity(capacity),
            lifecycle: Vec::with_capacity(capacity),
            kind_ids: Vec::with_capacity(capacity),
            payloads: Vec::with_capacity(capacity),
            payload_history: Vec::with_capacity(capacity),
            created_at: Vec::with_capacity(capacity),
            retired_at: Vec::with_capacity(capacity),
            aspect_versions: Vec::with_capacity(capacity),
            structural_fingerprints: Vec::with_capacity(capacity),
            lineage_ids: Vec::with_capacity(capacity),
            diagnostics_enrichment: Vec::with_capacity(capacity),
            branch_pins: Vec::with_capacity(capacity),
            replay_pins: Vec::with_capacity(capacity),
            snapshot_pins: Vec::with_capacity(capacity),
            live_bitset: DenseSlotBitSet::with_capacity(capacity),
            reclaimable_bitset: DenseSlotBitSet::with_capacity(capacity),
            free_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct RelationEndpoints {
    pub(super) source: EntityId,
    pub(super) target: EntityId,
}

#[derive(Debug, Clone)]
pub(super) struct RelationArena {
    pub(super) partition_ids: Vec<PartitionId>,
    pub(super) generations: Vec<u32>,
    pub(super) lifecycle: Vec<RecordLifecycleState>,
    pub(super) kind_ids: Vec<Option<KindId>>,
    pub(super) payloads: Vec<Option<RecordPayload>>,
    pub(super) payload_history: BTreeMap<usize, Vec<VersionedPayload>>,
    pub(super) created_at: Vec<VersionId>,
    pub(super) retired_at: Vec<Option<VersionId>>,
    pub(super) endpoints: Vec<Option<RelationEndpoints>>,
    pub(super) diagnostics_enrichment: Vec<BTreeMap<Symbol, String>>,
    pub(super) snapshot_pins: Vec<u32>,
    pub(super) live_bitset: DenseSlotBitSet,
    pub(super) reclaimable_bitset: DenseSlotBitSet,
    pub(super) free_list: Vec<u64>,
}

impl RelationArena {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            partition_ids: Vec::with_capacity(capacity),
            generations: Vec::with_capacity(capacity),
            lifecycle: Vec::with_capacity(capacity),
            kind_ids: Vec::with_capacity(capacity),
            payloads: Vec::with_capacity(capacity),
            payload_history: BTreeMap::new(),
            created_at: Vec::with_capacity(capacity),
            retired_at: Vec::with_capacity(capacity),
            endpoints: Vec::with_capacity(capacity),
            diagnostics_enrichment: Vec::with_capacity(capacity),
            snapshot_pins: Vec::with_capacity(capacity),
            live_bitset: DenseSlotBitSet::with_capacity(capacity),
            reclaimable_bitset: DenseSlotBitSet::with_capacity(capacity),
            free_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct PartitionState {
    pub(super) partition_id: PartitionId,
    pub(super) adjacency_policy: AdjacencyPolicy,
    pub(super) entity_arena: EntityArena,
    pub(super) relation_arena: RelationArena,
    pub(super) adjacency: Vec<AdjacencySet>,
    pub(super) reverse_adjacency: Vec<AdjacencySet>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct PartitionMutationJournal {
    pub(super) entity_slots: BTreeSet<usize>,
    pub(super) relation_slots: BTreeSet<usize>,
    pub(super) adjacency_slots: BTreeSet<usize>,
    pub(super) reverse_adjacency_slots: BTreeSet<usize>,
}

#[derive(Debug, Clone)]
pub(super) struct SnapshotState {
    pub(super) handle: crate::data::snapshot::SnapshotHandle,
    pub(super) pinned_entities: Vec<EntityId>,
    pub(super) pinned_relations: Vec<RelationId>,
}

#[derive(Debug, Clone)]
pub(super) struct WorkingState {
    pub(super) adjacency_policy: AdjacencyPolicy,
    pub(super) partitions: BTreeMap<PartitionId, PartitionState>,
    pub(super) mutation_journal: BTreeMap<PartitionId, PartitionMutationJournal>,
}

pub(super) trait PartitionAccess {
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
pub(super) struct BorrowedWorkingState<'a> {
    pub(super) partitions: &'a BTreeMap<PartitionId, PartitionState>,
}

impl WorkingState {
    pub(super) fn new(
        partitions: BTreeMap<PartitionId, PartitionState>,
        adjacency_policy: AdjacencyPolicy,
    ) -> Self {
        Self {
            adjacency_policy,
            partitions,
            mutation_journal: BTreeMap::new(),
        }
    }

    pub(super) fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState {
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

    pub(super) fn apply_to_runtime(self, runtime_partitions: &mut BTreeMap<PartitionId, PartitionState>) {
        *runtime_partitions = self.partitions;
    }

    pub(super) fn mark_entity_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .entity_slots
            .insert(slot);
    }

    pub(super) fn mark_relation_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .relation_slots
            .insert(slot);
    }

    pub(super) fn mark_adjacency_slot_touched(&mut self, partition_id: PartitionId, slot: usize) {
        self.mutation_journal
            .entry(partition_id)
            .or_default()
            .adjacency_slots
            .insert(slot);
    }

    pub(super) fn mark_reverse_adjacency_slot_touched(
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
    pub(super) fn new(partitions: &'a BTreeMap<PartitionId, PartitionState>) -> Self {
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
pub(super) struct PublicationArtifacts {
    pub(super) snapshot: crate::data::snapshot::SnapshotHandle,
    pub(super) snapshot_state: SnapshotState,
    pub(super) diagnostics_summary: crate::data::diagnostics::RelationalDiagnosticArtifact,
    pub(super) bundle: crate::data::publication::PublicationBundle<RelationalReplayRecord>,
}
