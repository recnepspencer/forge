use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::diagnostics::DiagnosticCode;
use crate::data::identity::{EntityId, KindId, RelationId, VersionId};
use crate::data::publication::{PublicationError, PublicationStatus};
use crate::data::snapshot::SnapshotHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SavepointId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityMode {
    SerializedCommit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAuthority {
    pub mode: AuthorityMode,
    pub label: String,
}

impl Default for CommitAuthority {
    fn default() -> Self {
        Self {
            mode: AuthorityMode::SerializedCommit,
            label: "single-writer deterministic commit".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionOptions {
    pub allow_nested_savepoints: bool,
    pub diagnostics_required: bool,
    pub deterministic_merge_required: bool,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            allow_nested_savepoints: true,
            diagnostics_required: true,
            deterministic_merge_required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerIntentBatch {
    pub name: String,
    pub partition_key: Option<String>,
    pub worker_local_only: bool,
    pub intents: Vec<TransactionIntent>,
}

impl WorkerIntentBatch {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            partition_key: None,
            worker_local_only: true,
            intents: Vec::new(),
        }
    }

    pub fn with_partition_key(mut self, partition_key: impl Into<String>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }

    pub fn push(mut self, intent: TransactionIntent) -> Self {
        self.intents.push(intent);
        self
    }
}

pub type TransactionIntentBatch = WorkerIntentBatch;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordRef {
    Entity(EntityId),
    Relation(RelationId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitySpec {
    pub kind_id: KindId,
    pub client_key: String,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationSpec {
    pub kind_id: KindId,
    pub client_key: String,
    pub source: EntityId,
    pub target: EntityId,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionIntent {
    CreateEntity(EntitySpec),
    UpdateEntity { entity_id: EntityId, payload: Value },
    DeleteEntity { entity_id: EntityId },
    CreateRelation(RelationSpec),
    DeleteRelation { relation_id: RelationId },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedCommitPlan {
    pub transaction_id: TransactionId,
    pub merged_intents: Vec<TransactionIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeApplyPlan {
    pub transaction_id: TransactionId,
    pub version_id: VersionId,
    pub merged_intents: Vec<TransactionIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoRecord {
    pub record: RecordRef,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitConflict {
    pub code: DiagnosticCode,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionCommitError {
    Conflict(CommitConflict),
    Publication(PublicationError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitOutcome {
    pub transaction_id: TransactionId,
    pub version_id: VersionId,
    pub snapshot: SnapshotHandle,
    pub changed_records: Vec<RecordRef>,
    pub publication_status: PublicationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackOutcome {
    pub transaction_id: TransactionId,
    pub restored_records: Vec<RecordRef>,
}
