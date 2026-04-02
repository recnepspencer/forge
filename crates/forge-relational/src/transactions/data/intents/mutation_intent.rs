use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::payloads::data::RecordPayload;
use crate::symbols::data::InternedString;

use super::super::{EntitySpec, RelationSpec};

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
pub struct UpdateEntityFieldsIntent {
    pub entity_id: EntityId,
    pub fields: BTreeMap<String, Value>,
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
    UpdateFields(UpdateEntityFieldsIntent),
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
