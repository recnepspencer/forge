use std::collections::{BTreeMap, BTreeSet};

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
}

#[derive(Debug, Clone)]
pub(super) struct VersionedPayload {
    pub(super) effective_at: VersionId,
    pub(super) retired_at: Option<VersionId>,
    pub(super) value: RecordPayload,
}

pub(super) type VersionedValue = VersionedPayload;

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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(super) struct EntityArenaSet {
    pub(super) partition_id: PartitionId,
    pub(super) arena: EntityArena,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(super) struct RelationArenaSet {
    pub(super) partition_id: PartitionId,
    pub(super) arena: RelationArena,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(super) struct PartitionState {
    pub(super) partition_id: PartitionId,
    pub(super) entity_arena: EntityArena,
    pub(super) relation_arena: RelationArena,
    pub(super) adjacency: Vec<BTreeSet<RelationId>>,
    pub(super) reverse_adjacency: Vec<BTreeSet<RelationId>>,
}

#[derive(Debug, Clone)]
pub(super) struct SnapshotState {
    pub(super) handle: crate::data::snapshot::SnapshotHandle,
    pub(super) pinned_entities: Vec<EntityId>,
    pub(super) pinned_relations: Vec<RelationId>,
}

#[derive(Debug, Clone)]
pub(super) struct WorkingState {
    pub(super) entity_arena: EntityArena,
    pub(super) relation_arena: RelationArena,
    pub(super) adjacency: Vec<BTreeSet<RelationId>>,
    pub(super) reverse_adjacency: Vec<BTreeSet<RelationId>>,
}

#[derive(Debug, Clone)]
pub(super) struct PublicationArtifacts {
    pub(super) snapshot: crate::data::snapshot::SnapshotHandle,
    pub(super) snapshot_state: SnapshotState,
    pub(super) diagnostics_summary: crate::data::diagnostics::RelationalDiagnosticArtifact,
    pub(super) bundle: crate::data::publication::PublicationBundle<RelationalReplayRecord>,
}
