use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::data::identity::{
    EntityId, KindId, LineageId, RelationId, StructuralFingerprint, VersionId,
};

use super::types::{RecordLifecycleState, RelationalReplayRecord};

#[derive(Debug, Clone)]
pub(super) struct VersionedValue {
    pub(super) effective_at: VersionId,
    pub(super) retired_at: Option<VersionId>,
    pub(super) value: Value,
}

#[derive(Debug, Clone)]
pub(super) struct EntityArena {
    pub(super) generations: Vec<u32>,
    pub(super) lifecycle: Vec<RecordLifecycleState>,
    pub(super) kind_ids: Vec<Option<KindId>>,
    pub(super) payloads: Vec<Option<Value>>,
    pub(super) payload_history: Vec<Vec<VersionedValue>>,
    pub(super) created_at: Vec<VersionId>,
    pub(super) retired_at: Vec<Option<VersionId>>,
    pub(super) aspect_versions: Vec<BTreeMap<String, u64>>,
    pub(super) structural_fingerprints: Vec<Option<StructuralFingerprint>>,
    pub(super) lineage_ids: Vec<Option<LineageId>>,
    pub(super) diagnostics_enrichment: Vec<BTreeMap<String, String>>,
    pub(super) branch_pins: Vec<u32>,
    pub(super) replay_pins: Vec<u32>,
    pub(super) snapshot_pins: Vec<u32>,
    pub(super) free_list: Vec<u64>,
}

impl EntityArena {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
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
    pub(super) generations: Vec<u32>,
    pub(super) lifecycle: Vec<RecordLifecycleState>,
    pub(super) kind_ids: Vec<Option<KindId>>,
    pub(super) payloads: Vec<Option<Value>>,
    pub(super) payload_history: Vec<Vec<VersionedValue>>,
    pub(super) created_at: Vec<VersionId>,
    pub(super) retired_at: Vec<Option<VersionId>>,
    pub(super) endpoints: Vec<Option<RelationEndpoints>>,
    pub(super) diagnostics_enrichment: Vec<BTreeMap<String, String>>,
    pub(super) snapshot_pins: Vec<u32>,
    pub(super) free_list: Vec<u64>,
}

impl RelationArena {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            generations: Vec::with_capacity(capacity),
            lifecycle: Vec::with_capacity(capacity),
            kind_ids: Vec::with_capacity(capacity),
            payloads: Vec::with_capacity(capacity),
            payload_history: Vec::with_capacity(capacity),
            created_at: Vec::with_capacity(capacity),
            retired_at: Vec::with_capacity(capacity),
            endpoints: Vec::with_capacity(capacity),
            diagnostics_enrichment: Vec::with_capacity(capacity),
            snapshot_pins: Vec::with_capacity(capacity),
            free_list: Vec::new(),
        }
    }
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
}

#[derive(Debug, Clone)]
pub(super) struct PublicationArtifacts {
    pub(super) snapshot: crate::data::snapshot::SnapshotHandle,
    pub(super) diagnostics_summary: crate::data::diagnostics::RelationalDiagnosticArtifact,
    pub(super) bundle: crate::data::publication::PublicationBundle<RelationalReplayRecord>,
}
