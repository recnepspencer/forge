use serde::{Deserialize, Serialize};

use crate::identity::data::{EntityId, LineageId, RelationId, VersionId};
use crate::payloads::data::RecordPayload;
use crate::query::data::QueryExecutionShape;
use crate::schema::data::KindResolution;
use crate::snapshots::data::SnapshotHandle;
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordLifecycleState {
    Live,
    DeletedRetained,
    RetainedDanglingForAudit,
    PinnedBySnapshot,
    PinnedByBranch,
    PinnedByReplayRetention,
    Reclaimable,
    Reusable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityReadRecord {
    pub entity_id: EntityId,
    pub lineage_id: Option<LineageId>,
    pub kind: KindResolution,
    pub lifecycle: RecordLifecycleState,
    pub created_at_version: VersionId,
    pub retired_at_version: Option<VersionId>,
    pub payload: RecordPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationReadRecord {
    pub relation_id: RelationId,
    pub kind: KindResolution,
    pub lifecycle: RecordLifecycleState,
    pub created_at_version: VersionId,
    pub retired_at_version: Option<VersionId>,
    pub source: EntityId,
    pub target: EntityId,
    pub payload: Option<RecordPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketResult {
    pub execution_shape: QueryExecutionShape,
    pub entities: Vec<EntityReadRecord>,
    pub relations: Vec<RelationReadRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedReadOutcome {
    pub result: PacketResult,
    pub used_index_generation: Option<crate::indexes::data::DerivedIndexGenerationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReadView {
    pub(crate) snapshot: SnapshotHandle,
    pub(crate) entities: Vec<EntityReadRecord>,
    pub(crate) relations: Vec<RelationReadRecord>,
}

impl RelationalReadView {
    pub fn snapshot(&self) -> &SnapshotHandle {
        &self.snapshot
    }

    pub fn entities(&self) -> &[EntityReadRecord] {
        &self.entities
    }

    pub fn relations(&self) -> &[RelationReadRecord] {
        &self.relations
    }

    pub fn execute_packet(&self, packet: &crate::query::data::QueryWorkPacket) -> PacketResult {
        let entity_index = self
            .entities
            .iter()
            .enumerate()
            .map(|(index, record)| (record.entity_id, index))
            .collect::<std::collections::BTreeMap<_, _>>();
        let relation_index = self
            .relations
            .iter()
            .enumerate()
            .map(|(index, record)| (record.relation_id, index))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut entities = Vec::new();
        let mut relations = Vec::new();
        for target in &packet.targets {
            match target {
                RecordRef::Entity(entity_id) => {
                    if let Some(index) = entity_index.get(entity_id) {
                        entities.push(self.entities[*index].clone());
                    }
                }
                RecordRef::Relation(relation_id) => {
                    if let Some(index) = relation_index.get(relation_id) {
                        relations.push(self.relations[*index].clone());
                    }
                }
            }
        }
        PacketResult {
            execution_shape: QueryExecutionShape::BulkPacketized,
            entities,
            relations,
        }
    }

    pub fn get_entity(&self, entity_id: EntityId) -> Option<&EntityReadRecord> {
        self.entities
            .binary_search_by_key(&entity_id, |record| record.entity_id)
            .ok()
            .map(|index| &self.entities[index])
    }

    pub fn get_relation(&self, relation_id: RelationId) -> Option<&RelationReadRecord> {
        self.relations
            .binary_search_by_key(&relation_id, |record| record.relation_id)
            .ok()
            .map(|index| &self.relations[index])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageStats {
    pub entity_slots: usize,
    pub entity_chunks: usize,
    pub live_entities: usize,
    pub deleted_entities: usize,
    pub reusable_entity_slots: usize,
    pub relation_slots: usize,
    pub relation_chunks: usize,
    pub live_relations: usize,
    pub deleted_relations: usize,
    pub reusable_relation_slots: usize,
    pub snapshot_count: usize,
    pub published_snapshot_handle_count: usize,
    pub cached_visibility_version_count: usize,
    pub protected_visibility_version_count: usize,
    pub recent_visibility_cache_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionStorageStats {
    pub partition_id: crate::identity::data::PartitionId,
    pub entity_slots: usize,
    pub entity_chunks: usize,
    pub live_entities: usize,
    pub deleted_entities: usize,
    pub reusable_entity_slots: usize,
    pub relation_slots: usize,
    pub relation_chunks: usize,
    pub live_relations: usize,
    pub deleted_relations: usize,
    pub reusable_relation_slots: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionPassOutcome {
    pub entity_reclaimable: usize,
    pub entity_reclaimed: usize,
    pub entity_chunks_scanned: usize,
    pub relation_reclaimable: usize,
    pub relation_reclaimed: usize,
    pub relation_chunks_scanned: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionPlan {
    pub retention_fence_version: VersionId,
    pub active_snapshot_count: usize,
    pub branch_pinned_entities: usize,
    pub replay_pinned_entities: usize,
    pub snapshot_pinned_entities: usize,
    pub branch_pinned_relations: usize,
    pub replay_pinned_relations: usize,
    pub snapshot_pinned_relations: usize,
    pub reclaimable_entities: usize,
    pub reclaimable_relations: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkVisibilitySummary {
    pub chunk_index: usize,
    pub slot_start: usize,
    pub slot_len: usize,
    pub visible_records: usize,
    pub retained_records: usize,
    pub reclaimable_records: usize,
    pub earliest_created_at: Option<VersionId>,
    pub latest_retired_at: Option<VersionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkedStorageSummary {
    pub entity_chunks: Vec<ChunkVisibilitySummary>,
    pub relation_chunks: Vec<ChunkVisibilitySummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkDiagnostics {
    pub version_id: VersionId,
    pub entity_chunks_total: usize,
    pub entity_chunks_with_visible_records: usize,
    pub entity_chunks_with_retained_records: usize,
    pub relation_chunks_total: usize,
    pub relation_chunks_with_visible_records: usize,
    pub relation_chunks_with_retained_records: usize,
}
