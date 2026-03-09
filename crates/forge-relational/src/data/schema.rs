use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::identity::KindId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaVersionId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityKindRegistration {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationKindRegistration {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindResolution {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchemaRegistryError {
    UnknownEntityKind(KindId),
    UnknownRelationKind(KindId),
    EntityRelationKindCollision(KindId),
    SchemaVersionMismatch {
        expected: SchemaVersionId,
        actual: SchemaVersionId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RelationalSchemaRegistry {
    pub entity_kinds: BTreeMap<KindId, EntityKindRegistration>,
    pub relation_kinds: BTreeMap<KindId, RelationKindRegistration>,
}

impl RelationalSchemaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity_kind(
        mut self,
        registration: EntityKindRegistration,
    ) -> Result<Self, SchemaRegistryError> {
        if self.relation_kinds.contains_key(&registration.kind_id) {
            return Err(SchemaRegistryError::EntityRelationKindCollision(
                registration.kind_id,
            ));
        }
        self.entity_kinds.insert(registration.kind_id, registration);
        Ok(self)
    }

    pub fn register_relation_kind(
        mut self,
        registration: RelationKindRegistration,
    ) -> Result<Self, SchemaRegistryError> {
        if self.entity_kinds.contains_key(&registration.kind_id) {
            return Err(SchemaRegistryError::EntityRelationKindCollision(
                registration.kind_id,
            ));
        }
        self.relation_kinds
            .insert(registration.kind_id, registration);
        Ok(self)
    }

    pub fn resolve_entity(&self, kind_id: KindId) -> Result<KindResolution, SchemaRegistryError> {
        self.entity_kinds
            .get(&kind_id)
            .map(|registration| KindResolution {
                kind_id: registration.kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
            })
            .ok_or(SchemaRegistryError::UnknownEntityKind(kind_id))
    }

    pub fn resolve_relation(&self, kind_id: KindId) -> Result<KindResolution, SchemaRegistryError> {
        self.relation_kinds
            .get(&kind_id)
            .map(|registration| KindResolution {
                kind_id: registration.kind_id,
                kind_name: registration.kind_name.clone(),
                schema_id: registration.schema_id.clone(),
                schema_version_id: registration.schema_version_id,
            })
            .ok_or(SchemaRegistryError::UnknownRelationKind(kind_id))
    }
}
