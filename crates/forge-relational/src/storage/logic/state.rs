use std::collections::{BTreeMap, BTreeSet};

use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
use crate::identity::data::{
    EntityId, KindId, LineageId, PartitionId, RelationId, StructuralFingerprint, VersionId,
};
use crate::payloads::data::RecordPayload;
use crate::replay::data::RelationalReplayRecord;
use crate::storage::data::RecordLifecycleState;
use crate::symbols::data::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DenseSlotBitSet {
    words: Vec<u64>,
}

impl DenseSlotBitSet {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
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

    pub(crate) fn set(&mut self, slot: usize, value: bool) {
        self.ensure_capacity(slot);
        let word = slot / 64;
        let bit = slot % 64;
        if value {
            self.words[word] |= 1 << bit;
        } else {
            self.words[word] &= !(1 << bit);
        }
    }

    pub(crate) fn count_ones(&self) -> usize {
        self.words
            .iter()
            .map(|word| word.count_ones() as usize)
            .sum()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VersionedPayload {
    pub(crate) effective_at: VersionId,
    pub(crate) retired_at: Option<VersionId>,
    pub(crate) value: RecordPayload,
}

pub(crate) type VersionedValue = VersionedPayload;

#[derive(Debug, Clone)]
pub(crate) enum AdjacencySet {
    Inline(Vec<RelationId>),
    Compressed(BTreeSet<RelationId>),
}

impl AdjacencySet {
    pub(crate) fn new(policy: &AdjacencyPolicy) -> Self {
        match policy.backend {
            AdjacencyBackend::InlineSmallDegreeAdjacency => {
                Self::Inline(Vec::with_capacity(policy.small_degree_inline_capacity))
            }
            AdjacencyBackend::CompressedFanoutAdjacency => Self::Compressed(BTreeSet::new()),
        }
    }

    pub(crate) fn clear(&mut self) {
        match self {
            Self::Inline(relations) => relations.clear(),
            Self::Compressed(relations) => relations.clear(),
        }
    }

    pub(crate) fn insert(&mut self, relation_id: RelationId) {
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

    pub(crate) fn remove(&mut self, relation_id: &RelationId) {
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

    pub(crate) fn ids(&self) -> Vec<RelationId> {
        match self {
            Self::Inline(relations) => relations.clone(),
            Self::Compressed(relations) => relations.iter().copied().collect(),
        }
    }

    pub(crate) fn extend_into(&self, target: &mut BTreeSet<RelationId>) {
        match self {
            Self::Inline(relations) => target.extend(relations.iter().copied()),
            Self::Compressed(relations) => target.extend(relations.iter().copied()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EntityArena {
    pub(crate) partition_ids: Vec<PartitionId>,
    pub(crate) generations: Vec<u32>,
    pub(crate) lifecycle: Vec<RecordLifecycleState>,
    pub(crate) kind_ids: Vec<Option<KindId>>,
    pub(crate) payloads: Vec<Option<RecordPayload>>,
    pub(crate) payload_history: Vec<Vec<VersionedPayload>>,
    pub(crate) created_at: Vec<VersionId>,
    pub(crate) retired_at: Vec<Option<VersionId>>,
    pub(crate) aspect_versions: Vec<BTreeMap<Symbol, u64>>,
    pub(crate) structural_fingerprints: Vec<Option<StructuralFingerprint>>,
    pub(crate) lineage_ids: Vec<Option<LineageId>>,
    pub(crate) diagnostics_enrichment: Vec<BTreeMap<Symbol, String>>,
    pub(crate) branch_pins: Vec<u32>,
    pub(crate) replay_pins: Vec<u32>,
    pub(crate) snapshot_pins: Vec<u32>,
    pub(crate) live_bitset: DenseSlotBitSet,
    pub(crate) reclaimable_bitset: DenseSlotBitSet,
    pub(crate) free_list: Vec<u64>,
}

impl EntityArena {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
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
pub(crate) struct RelationEndpoints {
    pub(crate) source: EntityId,
    pub(crate) target: EntityId,
}

#[derive(Debug, Clone)]
pub(crate) struct RelationArena {
    pub(crate) partition_ids: Vec<PartitionId>,
    pub(crate) generations: Vec<u32>,
    pub(crate) lifecycle: Vec<RecordLifecycleState>,
    pub(crate) kind_ids: Vec<Option<KindId>>,
    pub(crate) payloads: Vec<Option<RecordPayload>>,
    pub(crate) payload_history: BTreeMap<usize, Vec<VersionedPayload>>,
    pub(crate) created_at: Vec<VersionId>,
    pub(crate) retired_at: Vec<Option<VersionId>>,
    pub(crate) endpoints: Vec<Option<RelationEndpoints>>,
    pub(crate) diagnostics_enrichment: Vec<BTreeMap<Symbol, String>>,
    pub(crate) snapshot_pins: Vec<u32>,
    pub(crate) live_bitset: DenseSlotBitSet,
    pub(crate) reclaimable_bitset: DenseSlotBitSet,
    pub(crate) free_list: Vec<u64>,
}

impl RelationArena {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
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
pub(crate) struct SnapshotState {
    pub(crate) handle: crate::snapshots::data::SnapshotHandle,
    pub(crate) pinned_entities: Vec<EntityId>,
    pub(crate) pinned_relations: Vec<RelationId>,
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
    pub(crate) snapshot_state: SnapshotState,
    pub(crate) diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    pub(crate) bundle: crate::publication::data::PublicationBundle<RelationalReplayRecord>,
}
