mod registry_errors;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
use crate::identity::data::KindId;

pub use registry_errors::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SchemaVersionId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationPayloadClass {
    TopologyOnlyRelation,
    PayloadBearingRelation,
}

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
    pub payload_class: RelationPayloadClass,
    pub cross_context_policy: CrossContextPolicy,
    pub cascade_delete_policy: CascadeDeletePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindResolution {
    pub kind_id: KindId,
    pub kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
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
            return Err(SchemaRegistryError::entity_relation_kind_collision(
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
            return Err(SchemaRegistryError::entity_relation_kind_collision(
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
            .ok_or_else(|| SchemaRegistryError::unknown_entity_kind(kind_id))
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
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))
    }

    pub fn relation_registration(
        &self,
        kind_id: KindId,
    ) -> Result<&RelationKindRegistration, SchemaRegistryError> {
        self.relation_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))
    }
}
