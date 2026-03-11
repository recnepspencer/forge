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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkEntityCreateIntent {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_keys: Vec<InternedString>,
    pub payloads: Vec<RecordPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateEntityIntent {
    pub entity_id: EntityId,
    pub payload: RecordPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaceEntityIntent {
    pub entity_id: EntityId,
    pub replacement: EntitySpec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteEntityIntent {
    pub entity_id: EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkRelationCreateIntent {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_keys: Vec<InternedString>,
    pub endpoints: Vec<(EntityId, EntityId)>,
    pub payloads: Vec<Option<RecordPayload>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteRelationIntent {
    pub relation_id: RelationId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateIntent {
    Entity(EntitySpec),
    BulkEntities(BulkEntityCreateIntent),
    Relation(RelationSpec),
    BulkRelations(BulkRelationCreateIntent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityMutationIntent {
    Update(UpdateEntityIntent),
    Replace(ReplaceEntityIntent),
    Delete(DeleteEntityIntent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationMutationIntent {
    Delete(DeleteRelationIntent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationIntent {
    Create(CreateIntent),
    Entity(EntityMutationIntent),
    Relation(RelationMutationIntent),
}

impl TransactionIntent {
    pub(crate) fn to_mutation_intent(&self) -> MutationIntent {
        match self {
            Self::CreateEntity(spec) => MutationIntent::Create(CreateIntent::Entity(spec.clone())),
            Self::BulkCreateEntities {
                partition_id,
                kind_id,
                client_keys,
                payloads,
            } => MutationIntent::Create(CreateIntent::BulkEntities(BulkEntityCreateIntent {
                partition_id: *partition_id,
                kind_id: *kind_id,
                client_keys: client_keys.clone(),
                payloads: payloads.clone(),
            })),
            Self::UpdateEntity { entity_id, payload } => {
                MutationIntent::Entity(EntityMutationIntent::Update(UpdateEntityIntent {
                    entity_id: *entity_id,
                    payload: payload.clone(),
                }))
            }
            Self::ReplaceEntity {
                entity_id,
                replacement,
            } => MutationIntent::Entity(EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: *entity_id,
                replacement: replacement.clone(),
            })),
            Self::DeleteEntity { entity_id } => {
                MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
                    entity_id: *entity_id,
                }))
            }
            Self::CreateRelation(spec) => {
                MutationIntent::Create(CreateIntent::Relation(spec.clone()))
            }
            Self::BulkCreateRelations {
                partition_id,
                kind_id,
                client_keys,
                endpoints,
                payloads,
            } => MutationIntent::Create(CreateIntent::BulkRelations(
                BulkRelationCreateIntent {
                    partition_id: *partition_id,
                    kind_id: *kind_id,
                    client_keys: client_keys.clone(),
                    endpoints: endpoints.clone(),
                    payloads: payloads.clone(),
                },
            )),
            Self::DeleteRelation { relation_id } => MutationIntent::Relation(
                RelationMutationIntent::Delete(DeleteRelationIntent {
                    relation_id: *relation_id,
                }),
            ),
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

    pub(crate) fn rollback_effect(&self) -> RollbackEffect {
        self.to_mutation_intent().rollback_effect()
    }
}

impl From<MutationIntent> for TransactionIntent {
    fn from(value: MutationIntent) -> Self {
        match value {
            MutationIntent::Create(CreateIntent::Entity(spec)) => Self::CreateEntity(spec),
            MutationIntent::Create(CreateIntent::BulkEntities(spec)) => Self::BulkCreateEntities {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                client_keys: spec.client_keys,
                payloads: spec.payloads,
            },
            MutationIntent::Entity(EntityMutationIntent::Update(spec)) => Self::UpdateEntity {
                entity_id: spec.entity_id,
                payload: spec.payload,
            },
            MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => Self::ReplaceEntity {
                entity_id: spec.entity_id,
                replacement: spec.replacement,
            },
            MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => Self::DeleteEntity {
                entity_id: spec.entity_id,
            },
            MutationIntent::Create(CreateIntent::Relation(spec)) => Self::CreateRelation(spec),
            MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
                Self::BulkCreateRelations {
                    partition_id: spec.partition_id,
                    kind_id: spec.kind_id,
                    client_keys: spec.client_keys,
                    endpoints: spec.endpoints,
                    payloads: spec.payloads,
                }
            }
            MutationIntent::Relation(RelationMutationIntent::Delete(spec)) => {
                Self::DeleteRelation {
                    relation_id: spec.relation_id,
                }
            }
        }
    }
}

impl MutationIntent {
    pub(crate) fn seed_touched_partitions(
        &self,
        touched: &mut std::collections::BTreeSet<PartitionId>,
    ) {
        match self {
            Self::Create(CreateIntent::Entity(spec)) => {
                touched.insert(spec.partition_id);
            }
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                touched.insert(spec.partition_id);
            }
            Self::Entity(EntityMutationIntent::Update(spec)) => {
                touched.insert(spec.entity_id.partition_id);
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                touched.insert(spec.entity_id.partition_id);
                touched.insert(spec.replacement.partition_id);
            }
            Self::Entity(EntityMutationIntent::Delete(spec)) => {
                touched.insert(spec.entity_id.partition_id);
            }
            Self::Create(CreateIntent::Relation(spec)) => {
                touched.insert(spec.partition_id);
                touched.insert(spec.source.partition_id);
                touched.insert(spec.target.partition_id);
            }
            Self::Create(CreateIntent::BulkRelations(spec)) => {
                touched.insert(spec.partition_id);
                for (source, target) in &spec.endpoints {
                    touched.insert(source.partition_id);
                    touched.insert(target.partition_id);
                }
            }
            Self::Relation(RelationMutationIntent::Delete(spec)) => {
                touched.insert(spec.relation_id.partition_id);
            }
        }
    }

    pub(crate) fn bulk_entity_reservation(&self) -> Option<(PartitionId, usize)> {
        match self {
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                Some((spec.partition_id, spec.payloads.len()))
            }
            Self::Create(CreateIntent::Entity(_))
            | Self::Create(CreateIntent::Relation(_))
            | Self::Create(CreateIntent::BulkRelations(_))
            | Self::Entity(_)
            | Self::Relation(_) => None,
        }
    }

    pub(crate) fn bulk_relation_reservation(&self) -> Option<(PartitionId, usize)> {
        match self {
            Self::Create(CreateIntent::BulkRelations(spec)) => {
                Some((spec.partition_id, spec.endpoints.len()))
            }
            Self::Create(CreateIntent::Entity(_))
            | Self::Create(CreateIntent::BulkEntities(_))
            | Self::Create(CreateIntent::Relation(_))
            | Self::Entity(_)
            | Self::Relation(_) => None,
        }
    }

    pub(crate) fn rollback_effect(&self) -> RollbackEffect {
        match self {
            Self::Create(CreateIntent::Entity(_)) | Self::Create(CreateIntent::BulkEntities(_)) => {
                RollbackEffect::DiscardedEntityCreation
            }
            Self::Entity(EntityMutationIntent::Update(spec)) => {
                RollbackEffect::RestoredEntity(spec.entity_id)
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                RollbackEffect::RestoredEntity(spec.entity_id)
            }
            Self::Entity(EntityMutationIntent::Delete(spec)) => {
                RollbackEffect::RestoredEntity(spec.entity_id)
            }
            Self::Create(CreateIntent::Relation(_))
            | Self::Create(CreateIntent::BulkRelations(_)) => {
                RollbackEffect::DiscardedRelationCreation
            }
            Self::Relation(RelationMutationIntent::Delete(spec)) => {
                RollbackEffect::RestoredRelation(spec.relation_id)
            }
        }
    }

    pub(crate) fn existing_record_target(&self) -> Option<ExistingRecordTarget> {
        match self {
            Self::Entity(EntityMutationIntent::Update(spec)) => {
                Some(ExistingRecordTarget::Entity(spec.entity_id))
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                Some(ExistingRecordTarget::Entity(spec.entity_id))
            }
            Self::Entity(EntityMutationIntent::Delete(spec)) => {
                Some(ExistingRecordTarget::Entity(spec.entity_id))
            }
            Self::Relation(RelationMutationIntent::Delete(spec)) => {
                Some(ExistingRecordTarget::Relation(spec.relation_id))
            }
            Self::Create(_) => None,
        }
    }

    pub(crate) fn collect_relation_identities(
        &self,
        identities: &mut Vec<RelationIdentity>,
    ) {
        match self {
            Self::Create(CreateIntent::Relation(spec)) => identities.push(RelationIdentity {
                partition_id: spec.partition_id,
                kind_id: spec.kind_id,
                source: spec.source,
                target: spec.target,
            }),
            Self::Create(CreateIntent::BulkRelations(spec)) => {
                for (source, target) in &spec.endpoints {
                    identities.push(RelationIdentity {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        source: *source,
                        target: *target,
                    });
                }
            }
            Self::Create(CreateIntent::Entity(_))
            | Self::Create(CreateIntent::BulkEntities(_))
            | Self::Entity(_)
            | Self::Relation(_) => {}
        }
    }

    pub(crate) fn collect_planned_entity_field_values(
        &self,
        field: &str,
        values: &mut Vec<(Option<EntityId>, String)>,
    ) -> bool {
        match self {
            Self::Create(CreateIntent::Entity(spec)) => {
                collect_payload_field_value(None, &spec.payload, field, values);
                true
            }
            Self::Create(CreateIntent::BulkEntities(spec)) => {
                for payload in &spec.payloads {
                    collect_payload_field_value(None, payload, field, values);
                }
                true
            }
            Self::Entity(EntityMutationIntent::Update(spec)) => {
                collect_payload_field_value(Some(spec.entity_id), &spec.payload, field, values);
                true
            }
            Self::Entity(EntityMutationIntent::Replace(spec)) => {
                collect_payload_field_value(
                    Some(spec.entity_id),
                    &spec.replacement.payload,
                    field,
                    values,
                );
                true
            }
            Self::Entity(EntityMutationIntent::Delete(_))
            | Self::Create(CreateIntent::Relation(_))
            | Self::Create(CreateIntent::BulkRelations(_))
            | Self::Relation(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedCommitPlan {
    pub transaction_id: TransactionId,
    pub merged_intents: Vec<MutationIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritativeApplyPlan {
    pub transaction_id: TransactionId,
    pub version_id: VersionId,
    pub merged_intents: Vec<MutationIntent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoRecord {
    pub record: RecordRef,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitConflict {
    pub class: ConflictClass,
    pub code: DiagnosticCode,
    pub detail: String,
}

impl CommitConflict {
    pub(crate) fn new(class: ConflictClass) -> Self {
        let code = class.code();
        let detail = class.detail();
        Self { class, code, detail }
    }

    pub fn code(&self) -> DiagnosticCode {
        self.code
    }

    pub fn detail(&self) -> String {
        self.detail.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictClass {
    StaleTarget {
        target: ExistingRecordTarget,
        context: String,
    },
    InvalidRelationEndpoint {
        detail: String,
    },
    DuplicateRelationIdentity {
        detail: String,
    },
    InvariantViolation {
        code: DiagnosticCode,
        detail: String,
    },
    KindSchemaMismatch {
        detail: String,
    },
    ConflictingIntent {
        target: ExistingRecordTarget,
    },
    InvalidSavepoint {
        savepoint_id: SavepointId,
    },
    InvalidMergeParent {
        detail: String,
    },
    MergeConflictOverlap {
        detail: String,
    },
    MissingMergeBase {
        detail: String,
    },
}

impl ConflictClass {
    pub fn code(&self) -> DiagnosticCode {
        match self {
            Self::StaleTarget { .. } => DiagnosticCode::StaleHandle,
            Self::InvalidRelationEndpoint { .. } => DiagnosticCode::InvalidRelationEndpoint,
            Self::DuplicateRelationIdentity { .. } => DiagnosticCode::DuplicateRelationIdentity,
            Self::InvariantViolation { code, .. } => *code,
            Self::KindSchemaMismatch { .. } => DiagnosticCode::InvariantViolation,
            Self::ConflictingIntent { .. } => DiagnosticCode::ConflictingIntent,
            Self::InvalidSavepoint { .. } => DiagnosticCode::InvalidSavepoint,
            Self::InvalidMergeParent { .. } => DiagnosticCode::InvalidMergeParent,
            Self::MergeConflictOverlap { .. } => DiagnosticCode::MergeConflictOverlap,
            Self::MissingMergeBase { .. } => DiagnosticCode::MissingMergeBase,
        }
    }

    pub fn detail(&self) -> String {
        match self {
            Self::StaleTarget { target, context } => match target {
                ExistingRecordTarget::Entity(entity_id) => format!(
                    "entity {:?} changed before authoritative apply ({context})",
                    entity_id
                ),
                ExistingRecordTarget::Relation(relation_id) => format!(
                    "relation {:?} changed before authoritative apply ({context})",
                    relation_id
                ),
            },
            Self::InvalidRelationEndpoint { detail }
            | Self::DuplicateRelationIdentity { detail }
            | Self::KindSchemaMismatch { detail }
            | Self::InvalidMergeParent { detail }
            | Self::MergeConflictOverlap { detail }
            | Self::MissingMergeBase { detail } => detail.clone(),
            Self::InvariantViolation { detail, .. } => detail.clone(),
            Self::ConflictingIntent { target } => match target {
                ExistingRecordTarget::Entity(entity_id) => {
                    format!("conflicting entity intent for slot {}", entity_id.local_slot.0)
                }
                ExistingRecordTarget::Relation(relation_id) => {
                    format!("conflicting relation intent for slot {}", relation_id.local_slot.0)
                }
            },
            Self::InvalidSavepoint { savepoint_id } => {
                format!("savepoint {:?} does not exist", savepoint_id)
            }
        }
    }
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
