use serde::{Deserialize, Serialize};

use crate::identity::data::{EntityId, RelationId, VersionId};
use crate::payloads::data::RecordPayload;
use crate::query::data::QueryExecutionShape;
use crate::schema::data::KindResolution;
use crate::snapshots::data::SnapshotHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordLifecycleState {
    Live,
    DeletedRetained,
    PinnedBySnapshot,
    PinnedByBranch,
    PinnedByReplayRetention,
    Reclaimable,
    Reusable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityReadRecord {
    pub entity_id: EntityId,
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
        let mut entities = Vec::new();
        let mut relations = Vec::new();
        for target in &packet.targets {
            match target {
                crate::query::data::ReadTarget::Entity(entity_id) => {
                    if let Some(record) = self
                        .entities
                        .iter()
                        .find(|record| &record.entity_id == entity_id)
                    {
                        entities.push(record.clone());
                    }
                }
                crate::query::data::ReadTarget::Relation(relation_id) => {
                    if let Some(record) = self
                        .relations
                        .iter()
                        .find(|record| &record.relation_id == relation_id)
                    {
                        relations.push(record.clone());
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
            .iter()
            .find(|record| record.entity_id == entity_id)
    }

    pub fn get_relation(&self, relation_id: RelationId) -> Option<&RelationReadRecord> {
        self.relations
            .iter()
            .find(|record| record.relation_id == relation_id)
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
