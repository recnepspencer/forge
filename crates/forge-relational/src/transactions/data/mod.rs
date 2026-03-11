use serde::{Deserialize, Serialize};

use crate::diagnostics::data::DiagnosticCode;
use crate::history::data::BranchId;
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::payloads::data::RecordPayload;
use crate::publication::data::{PublicationError, PublicationStatus};
use crate::snapshots::data::SnapshotHandle;
use crate::symbols::data::{InternedString, StringInterner, SymbolPolicy};

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
    pub target_branch: Option<BranchId>,
    pub merge_parent_branches: Vec<BranchId>,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            allow_nested_savepoints: true,
            diagnostics_required: true,
            deterministic_merge_required: true,
            target_branch: None,
            merge_parent_branches: Vec::new(),
        }
    }
}

impl TransactionOptions {
    pub fn merge_from_branches(mut self, branches: Vec<BranchId>) -> Self {
        self.merge_parent_branches = branches;
        self
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExistingRecordTarget {
    Entity(EntityId),
    Relation(RelationId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RelationIdentity {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub source: EntityId,
    pub target: EntityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationScope {
    SamePartition,
    CrossPartition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossContextEndpointClass {
    SamePartitionEndpoints,
    CrossPartitionEndpoints,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitySpec {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: InternedString,
    pub payload: RecordPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationSpec {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: InternedString,
    pub source: EntityId,
    pub target: EntityId,
    pub payload: Option<RecordPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionIntent {
    CreateEntity(EntitySpec),
    BulkCreateEntities {
        partition_id: PartitionId,
        kind_id: KindId,
        client_keys: Vec<InternedString>,
        payloads: Vec<RecordPayload>,
    },
    UpdateEntity {
        entity_id: EntityId,
        payload: RecordPayload,
    },
    ReplaceEntity {
        entity_id: EntityId,
        replacement: EntitySpec,
    },
    DeleteEntity {
        entity_id: EntityId,
    },
    CreateRelation(RelationSpec),
    BulkCreateRelations {
        partition_id: PartitionId,
        kind_id: KindId,
        client_keys: Vec<InternedString>,
        endpoints: Vec<(EntityId, EntityId)>,
        payloads: Vec<Option<RecordPayload>>,
    },
    DeleteRelation {
        relation_id: RelationId,
    },
}

impl TransactionIntent {
    pub(crate) fn seed_touched_partitions(
        &self,
        touched: &mut std::collections::BTreeSet<PartitionId>,
    ) {
        match self {
            Self::CreateEntity(spec) => {
                touched.insert(spec.partition_id);
            }
            Self::BulkCreateEntities { partition_id, .. } => {
                touched.insert(*partition_id);
            }
            Self::UpdateEntity { entity_id, .. } | Self::DeleteEntity { entity_id } => {
                touched.insert(entity_id.partition_id);
            }
            Self::ReplaceEntity {
                entity_id,
                replacement,
            } => {
                touched.insert(entity_id.partition_id);
                touched.insert(replacement.partition_id);
            }
            Self::CreateRelation(spec) => {
                touched.insert(spec.partition_id);
                touched.insert(spec.source.partition_id);
                touched.insert(spec.target.partition_id);
            }
            Self::BulkCreateRelations {
                partition_id,
                endpoints,
                ..
            } => {
                touched.insert(*partition_id);
                for (source, target) in endpoints {
                    touched.insert(source.partition_id);
                    touched.insert(target.partition_id);
                }
            }
            Self::DeleteRelation { relation_id } => {
                touched.insert(relation_id.partition_id);
            }
        }
    }

    pub(crate) fn collect_raw_client_keys(&self, raw_values: &mut Vec<String>) {
        match self {
            Self::CreateEntity(spec) => {
                if let InternedString::Raw(raw) = &spec.client_key {
                    raw_values.push(raw.clone());
                }
            }
            Self::BulkCreateEntities { client_keys, .. }
            | Self::BulkCreateRelations { client_keys, .. } => {
                for client_key in client_keys {
                    if let InternedString::Raw(raw) = client_key {
                        raw_values.push(raw.clone());
                    }
                }
            }
            Self::CreateRelation(spec) => {
                if let InternedString::Raw(raw) = &spec.client_key {
                    raw_values.push(raw.clone());
                }
            }
            Self::UpdateEntity { .. }
            | Self::ReplaceEntity { .. }
            | Self::DeleteEntity { .. }
            | Self::DeleteRelation { .. } => {}
        }
    }

    pub(crate) fn normalize_client_keys(
        &mut self,
        interner: &mut StringInterner,
        policy: SymbolPolicy,
    ) {
        match self {
            Self::CreateEntity(spec) => {
                spec.client_key = normalize_interned_string(interner, policy, spec.client_key.clone());
            }
            Self::BulkCreateEntities { client_keys, .. }
            | Self::BulkCreateRelations { client_keys, .. } => {
                for client_key in client_keys {
                    *client_key =
                        normalize_interned_string(interner, policy, client_key.clone());
                }
            }
            Self::CreateRelation(spec) => {
                spec.client_key = normalize_interned_string(interner, policy, spec.client_key.clone());
            }
            Self::UpdateEntity { .. }
            | Self::ReplaceEntity { .. }
            | Self::DeleteEntity { .. }
            | Self::DeleteRelation { .. } => {}
        }
    }

    pub(crate) fn bulk_entity_reservation(&self) -> Option<(PartitionId, usize)> {
        match self {
            Self::BulkCreateEntities {
                partition_id,
                payloads,
                ..
            } => Some((*partition_id, payloads.len())),
            _ => None,
        }
    }

    pub(crate) fn bulk_relation_reservation(&self) -> Option<(PartitionId, usize)> {
        match self {
            Self::BulkCreateRelations {
                partition_id,
                endpoints,
                ..
            } => Some((*partition_id, endpoints.len())),
            _ => None,
        }
    }

    pub(crate) fn rollback_effect(&self) -> RollbackEffect {
        match self {
            Self::CreateEntity(_) | Self::BulkCreateEntities { .. } => {
                RollbackEffect::DiscardedEntityCreation
            }
            Self::UpdateEntity { entity_id, .. }
            | Self::ReplaceEntity { entity_id, .. }
            | Self::DeleteEntity { entity_id } => RollbackEffect::RestoredEntity(*entity_id),
            Self::CreateRelation(_) | Self::BulkCreateRelations { .. } => {
                RollbackEffect::DiscardedRelationCreation
            }
            Self::DeleteRelation { relation_id } => RollbackEffect::RestoredRelation(*relation_id),
        }
    }

    pub(crate) fn existing_record_target(&self) -> Option<ExistingRecordTarget> {
        match self {
            Self::UpdateEntity { entity_id, .. }
            | Self::ReplaceEntity { entity_id, .. }
            | Self::DeleteEntity { entity_id } => Some(ExistingRecordTarget::Entity(*entity_id)),
            Self::DeleteRelation { relation_id } => Some(ExistingRecordTarget::Relation(*relation_id)),
            Self::CreateEntity(_)
            | Self::BulkCreateEntities { .. }
            | Self::CreateRelation(_)
            | Self::BulkCreateRelations { .. } => None,
        }
    }

    pub(crate) fn collect_relation_identities(
        &self,
        identities: &mut Vec<RelationIdentity>,
    ) {
        match self {
            Self::CreateRelation(spec) => identities.push(RelationIdentity {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                source: spec.source,
                target: spec.target,
            }),
            Self::BulkCreateRelations {
                partition_id,
                kind_id,
                endpoints,
                ..
            } => {
                for (source, target) in endpoints {
                    identities.push(RelationIdentity {
                        partition_id: *partition_id,
                        kind_id: *kind_id,
                        source: *source,
                        target: *target,
                    });
                }
            }
            Self::CreateEntity(_)
            | Self::BulkCreateEntities { .. }
            | Self::UpdateEntity { .. }
            | Self::ReplaceEntity { .. }
            | Self::DeleteEntity { .. }
            | Self::DeleteRelation { .. } => {}
        }
    }

    pub(crate) fn collect_planned_entity_field_values(
        &self,
        field: &str,
        values: &mut Vec<(Option<EntityId>, String)>,
    ) -> bool {
        match self {
            Self::CreateEntity(spec) => {
                collect_payload_field_value(None, &spec.payload, field, values);
                true
            }
            Self::BulkCreateEntities { payloads, .. } => {
                for payload in payloads {
                    collect_payload_field_value(None, payload, field, values);
                }
                true
            }
            Self::UpdateEntity { entity_id, payload } => {
                collect_payload_field_value(Some(*entity_id), payload, field, values);
                true
            }
            Self::ReplaceEntity {
                entity_id,
                replacement,
            } => {
                collect_payload_field_value(Some(*entity_id), &replacement.payload, field, values);
                true
            }
            Self::DeleteEntity { .. }
            | Self::CreateRelation(_)
            | Self::BulkCreateRelations { .. }
            | Self::DeleteRelation { .. } => false,
        }
    }
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
    pub commit: crate::history::data::CommitReference,
    pub version_id: VersionId,
    pub snapshot: SnapshotHandle,
    pub changed_records: Vec<RecordRef>,
    pub publication_status: PublicationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackEffect {
    RestoredEntity(EntityId),
    RestoredRelation(RelationId),
    DiscardedEntityCreation,
    DiscardedRelationCreation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackOutcome {
    pub transaction_id: TransactionId,
    pub effects: Vec<RollbackEffect>,
}

fn normalize_interned_string(
    interner: &mut StringInterner,
    policy: SymbolPolicy,
    value: InternedString,
) -> InternedString {
    match policy {
        SymbolPolicy::Disabled => value,
        SymbolPolicy::PreferInterned | SymbolPolicy::RequireInterned => interner.normalize(value),
    }
}

fn collect_payload_field_value(
    entity_id: Option<EntityId>,
    payload: &RecordPayload,
    field: &str,
    values: &mut Vec<(Option<EntityId>, String)>,
) {
    if let Some(value) = payload
        .as_json()
        .and_then(|value| value.get(field))
        .and_then(|value| value.as_str())
    {
        values.push((entity_id, value.to_string()));
    }
}
