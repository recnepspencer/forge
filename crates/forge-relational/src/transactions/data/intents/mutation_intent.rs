use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::identity::data::{KindId, PartitionId, RelationId};
use crate::payloads::data::RecordPayload;
use crate::symbols::data::InternedString;

use super::super::{EntityReference, EntitySpec, RelationSpec};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkEntityCreateIntent {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_keys: Vec<InternedString>,
    pub payloads: Vec<RecordPayload>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateEntityIntent {
    pub entity_id: crate::identity::data::EntityId,
    pub payload: RecordPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateEntityFieldsIntent {
    pub entity_id: crate::identity::data::EntityId,
    pub fields: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaceEntityIntent {
    pub entity_id: crate::identity::data::EntityId,
    pub replacement: EntitySpec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteEntityIntent {
    pub entity_id: crate::identity::data::EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkRelationCreateIntent {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_keys: Vec<InternedString>,
    pub endpoints: Vec<(EntityReference, EntityReference)>,
    pub payloads: Vec<Option<RecordPayload>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteRelationIntent {
    pub relation_id: RelationId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRelationEndpointsIntent {
    pub relation_id: RelationId,
    pub kind_id: KindId,
    pub source: EntityReference,
    pub target: EntityReference,
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
    UpdateEndpoints(UpdateRelationEndpointsIntent),
    Delete(DeleteRelationIntent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationIntent {
    Create(CreateIntent),
    Entity(EntityMutationIntent),
    Relation(RelationMutationIntent),
}
