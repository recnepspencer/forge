use serde::{Deserialize, Serialize};

use crate::identity::data::{KindId, PartitionId, RelationId};
use crate::symbols::data::ClientKey;
use worth_foundational::facade::PortableRecordAspectPatch;

use super::super::AspectFieldPatch;

use super::super::{EntityReference, EntitySpec, RelationSpec};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkEntityCreateIntent {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_keys: Vec<ClientKey>,
    pub field_patches: Vec<AspectFieldPatch>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityAspectCreateIntent {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: ClientKey,
    pub aspect_patch: PortableRecordAspectPatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyEntityAspectPatchIntent {
    pub entity_id: crate::identity::data::EntityId,
    pub aspect_patch: PortableRecordAspectPatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateEntityFieldsIntent {
    pub entity_id: crate::identity::data::EntityId,
    pub fields: AspectFieldPatch,
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
    pub client_keys: Vec<ClientKey>,
    pub endpoints: Vec<(EntityReference, EntityReference)>,
    pub field_patches: Vec<AspectFieldPatch>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationAspectCreateIntent {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: ClientKey,
    pub source: EntityReference,
    pub target: EntityReference,
    pub aspect_patch: PortableRecordAspectPatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyRelationAspectPatchIntent {
    pub relation_id: RelationId,
    pub aspect_patch: PortableRecordAspectPatch,
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
    EntityAspects(EntityAspectCreateIntent),
    BulkEntities(BulkEntityCreateIntent),
    Relation(RelationSpec),
    RelationAspects(RelationAspectCreateIntent),
    BulkRelations(BulkRelationCreateIntent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityMutationIntent {
    UpdateFields(UpdateEntityFieldsIntent),
    ApplyAspectPatch(ApplyEntityAspectPatchIntent),
    Replace(ReplaceEntityIntent),
    Delete(DeleteEntityIntent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationMutationIntent {
    UpdateEndpoints(UpdateRelationEndpointsIntent),
    ApplyAspectPatch(ApplyRelationAspectPatchIntent),
    Delete(DeleteRelationIntent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationIntent {
    Create(CreateIntent),
    Entity(EntityMutationIntent),
    Relation(RelationMutationIntent),
}
