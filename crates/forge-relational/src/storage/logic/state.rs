#[path = "bitsets.rs"]
mod bitsets;
#[path = "adjacency.rs"]
mod adjacency;
#[path = "working_state.rs"]
mod working_state;

use std::collections::BTreeMap;

use crate::identity::data::{
    EntityId, KindId, LineageId, PartitionId, StructuralFingerprint, VersionId,
};
use crate::payloads::data::RecordPayload;
use crate::storage::data::RecordLifecycleState;
use crate::symbols::data::Symbol;

pub(crate) use adjacency::AdjacencySet;
pub(crate) use bitsets::DenseSlotBitSet;
pub(crate) use working_state::{
    BorrowedWorkingState, PartitionAccess, PartitionState, PublicationArtifacts,
    SnapshotPartitionPins, SnapshotState, WorkingState,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LifecycleCounts {
    pub(crate) live: usize,
    pub(crate) deleted: usize,
    pub(crate) reusable: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct VersionedPayload {
    pub(crate) effective_at: VersionId,
    pub(crate) retired_at: Option<VersionId>,
    pub(crate) value: RecordPayload,
}

pub(crate) type VersionedValue = VersionedPayload;

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

    pub(crate) fn reserve_additional(&mut self, additional: usize) {
        self.partition_ids.reserve(additional);
        self.generations.reserve(additional);
        self.lifecycle.reserve(additional);
        self.kind_ids.reserve(additional);
        self.payloads.reserve(additional);
        self.payload_history.reserve(additional);
        self.created_at.reserve(additional);
        self.retired_at.reserve(additional);
        self.aspect_versions.reserve(additional);
        self.structural_fingerprints.reserve(additional);
        self.lineage_ids.reserve(additional);
        self.diagnostics_enrichment.reserve(additional);
        self.branch_pins.reserve(additional);
        self.replay_pins.reserve(additional);
        self.snapshot_pins.reserve(additional);
        self.free_list.reserve(additional);
    }

    pub(crate) fn allocate(
        &mut self,
        partition_id: PartitionId,
        kind_id: KindId,
        payload: RecordPayload,
        version_id: VersionId,
    ) -> (usize, u32, bool) {
        let payload = payload.canonicalized();
        if let Some(slot) = self.free_list.pop() {
            let idx = slot as usize;
            self.partition_ids[idx] = partition_id;
            self.lifecycle[idx] = RecordLifecycleState::Live;
            self.kind_ids[idx] = Some(kind_id);
            self.payloads[idx] = Some(payload.clone());
            self.payload_history[idx] = vec![VersionedPayload {
                effective_at: version_id,
                retired_at: None,
                value: payload,
            }];
            self.created_at[idx] = version_id;
            self.retired_at[idx] = None;
            self.generations[idx] += 1;
            self.aspect_versions[idx].clear();
            self.structural_fingerprints[idx] = None;
            self.lineage_ids[idx] = None;
            self.diagnostics_enrichment[idx].clear();
            self.branch_pins[idx] = 0;
            self.replay_pins[idx] = 0;
            self.snapshot_pins[idx] = 0;
            self.live_bitset.set(idx, true);
            self.reclaimable_bitset.set(idx, false);
            return (idx, self.generations[idx], true);
        }

        let slot = self.generations.len();
        self.partition_ids.push(partition_id);
        self.generations.push(1);
        self.lifecycle.push(RecordLifecycleState::Live);
        self.kind_ids.push(Some(kind_id));
        self.payloads.push(Some(payload.clone()));
        self.payload_history.push(vec![VersionedPayload {
            effective_at: version_id,
            retired_at: None,
            value: payload,
        }]);
        self.created_at.push(version_id);
        self.retired_at.push(None);
        self.aspect_versions.push(BTreeMap::new());
        self.structural_fingerprints.push(None);
        self.lineage_ids.push(None);
        self.diagnostics_enrichment.push(BTreeMap::new());
        self.branch_pins.push(0);
        self.replay_pins.push(0);
        self.snapshot_pins.push(0);
        self.live_bitset.set(slot, true);
        self.reclaimable_bitset.set(slot, false);
        (slot, 1, false)
    }

    pub(crate) fn apply_payload_update(
        &mut self,
        slot: usize,
        payload: RecordPayload,
        version_id: VersionId,
    ) {
        let payload = payload.canonicalized();
        self.payloads[slot] = Some(payload.clone());
        if let Some(current) = self.payload_history[slot].last_mut() {
            current.retired_at = Some(version_id);
        }
        self.payload_history[slot].push(VersionedPayload {
            effective_at: version_id,
            retired_at: None,
            value: payload,
        });
    }

    pub(crate) fn retire(&mut self, slot: usize, version_id: VersionId) {
        self.retired_at[slot] = Some(version_id);
        self.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
        self.live_bitset.set(slot, false);
        self.reclaimable_bitset.set(slot, true);
        if let Some(current) = self.payload_history[slot].last_mut() {
            current.retired_at = Some(version_id);
        }
    }

    pub(crate) fn lifecycle_counts(&self) -> LifecycleCounts {
        lifecycle_counts(&self.lifecycle)
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
    pub(crate) aspect_versions: Vec<BTreeMap<Symbol, u64>>,
    pub(crate) diagnostics_enrichment: Vec<BTreeMap<Symbol, String>>,
    pub(crate) branch_pins: Vec<u32>,
    pub(crate) replay_pins: Vec<u32>,
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
            aspect_versions: Vec::with_capacity(capacity),
            diagnostics_enrichment: Vec::with_capacity(capacity),
            branch_pins: Vec::with_capacity(capacity),
            replay_pins: Vec::with_capacity(capacity),
            snapshot_pins: Vec::with_capacity(capacity),
            live_bitset: DenseSlotBitSet::with_capacity(capacity),
            reclaimable_bitset: DenseSlotBitSet::with_capacity(capacity),
            free_list: Vec::new(),
        }
    }

    pub(crate) fn reserve_additional(&mut self, additional: usize) {
        self.partition_ids.reserve(additional);
        self.generations.reserve(additional);
        self.lifecycle.reserve(additional);
        self.kind_ids.reserve(additional);
        self.payloads.reserve(additional);
        self.created_at.reserve(additional);
        self.retired_at.reserve(additional);
        self.endpoints.reserve(additional);
        self.aspect_versions.reserve(additional);
        self.diagnostics_enrichment.reserve(additional);
        self.branch_pins.reserve(additional);
        self.replay_pins.reserve(additional);
        self.snapshot_pins.reserve(additional);
        self.free_list.reserve(additional);
    }

    pub(crate) fn allocate(
        &mut self,
        partition_id: PartitionId,
        kind_id: KindId,
        payload: Option<RecordPayload>,
        version_id: VersionId,
        endpoints: RelationEndpoints,
    ) -> (usize, u32) {
        let canonical_payload = payload.map(|value| value.canonicalized());
        if let Some(slot) = self.free_list.pop() {
            let idx = slot as usize;
            self.partition_ids[idx] = partition_id;
            self.lifecycle[idx] = RecordLifecycleState::Live;
            self.kind_ids[idx] = Some(kind_id);
            self.payloads[idx] = canonical_payload.clone();
            if let Some(payload) = canonical_payload {
                self.payload_history.insert(
                    idx,
                    vec![VersionedPayload {
                        effective_at: version_id,
                        retired_at: None,
                        value: payload,
                    }],
                );
            } else {
                self.payload_history.remove(&idx);
            }
            self.created_at[idx] = version_id;
            self.retired_at[idx] = None;
            self.endpoints[idx] = Some(endpoints);
            self.aspect_versions[idx].clear();
            self.diagnostics_enrichment[idx].clear();
            self.branch_pins[idx] = 0;
            self.replay_pins[idx] = 0;
            self.snapshot_pins[idx] = 0;
            self.generations[idx] += 1;
            self.live_bitset.set(idx, true);
            self.reclaimable_bitset.set(idx, false);
            return (idx, self.generations[idx]);
        }

        let slot = self.generations.len();
        self.partition_ids.push(partition_id);
        self.generations.push(1);
        self.lifecycle.push(RecordLifecycleState::Live);
        self.kind_ids.push(Some(kind_id));
        self.payloads.push(canonical_payload.clone());
        if let Some(payload) = canonical_payload {
            self.payload_history.insert(
                slot,
                vec![VersionedPayload {
                    effective_at: version_id,
                    retired_at: None,
                    value: payload,
                }],
            );
        }
        self.created_at.push(version_id);
        self.retired_at.push(None);
        self.endpoints.push(Some(endpoints));
        self.aspect_versions.push(BTreeMap::new());
        self.diagnostics_enrichment.push(BTreeMap::new());
        self.branch_pins.push(0);
        self.replay_pins.push(0);
        self.snapshot_pins.push(0);
        self.live_bitset.set(slot, true);
        self.reclaimable_bitset.set(slot, false);
        (slot, 1)
    }

    pub(crate) fn retire(&mut self, slot: usize, version_id: VersionId) {
        self.retired_at[slot] = Some(version_id);
        self.lifecycle[slot] = RecordLifecycleState::DeletedRetained;
        self.live_bitset.set(slot, false);
        self.reclaimable_bitset.set(slot, true);
        if let Some(current) = self
            .payload_history
            .get_mut(&slot)
            .and_then(|history| history.last_mut())
        {
            current.retired_at = Some(version_id);
        }
    }

    pub(crate) fn lifecycle_counts(&self) -> LifecycleCounts {
        lifecycle_counts(&self.lifecycle)
    }
}

pub(crate) fn lifecycle_counts(lifecycle: &[RecordLifecycleState]) -> LifecycleCounts {
    let mut counts = LifecycleCounts::default();
    for state in lifecycle {
        match state {
            RecordLifecycleState::Live => counts.live += 1,
            RecordLifecycleState::Reusable => counts.reusable += 1,
            RecordLifecycleState::DeletedRetained
            | RecordLifecycleState::PinnedBySnapshot
            | RecordLifecycleState::PinnedByBranch
            | RecordLifecycleState::PinnedByReplayRetention
            | RecordLifecycleState::Reclaimable => counts.deleted += 1,
        }
    }
    counts
}
