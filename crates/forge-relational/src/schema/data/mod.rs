mod aspect_semantics;
mod aspect_traces;
mod continuity;
mod relation_integrity;
mod registry_errors;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
use crate::identity::data::KindId;

pub use aspect_semantics::*;
pub use aspect_traces::*;
pub use continuity::*;
pub use relation_integrity::*;
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
    pub aspect_declarations: KindAspectDeclarations,
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
    pub aspect_declarations: KindAspectDeclarations,
    pub relation_integrity: RelationIntegrityDeclarations,
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
        let registration = crate::schema::logic::canonicalize_entity_registration(registration)?;
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
        let registration = crate::schema::logic::canonicalize_relation_registration(registration)?;
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

    pub fn entity_aspect_declaration_trace(
        &self,
        kind_id: KindId,
    ) -> Result<AspectDeclarationTrace, SchemaRegistryError> {
        let registration = self
            .entity_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_entity_kind(kind_id))?;
        Ok(registration.aspect_declarations.declaration_trace(kind_id))
    }

    pub fn relation_aspect_declaration_trace(
        &self,
        kind_id: KindId,
    ) -> Result<AspectDeclarationTrace, SchemaRegistryError> {
        let registration = self
            .relation_kinds
            .get(&kind_id)
            .ok_or_else(|| SchemaRegistryError::unknown_relation_kind(kind_id))?;
        Ok(registration.aspect_declarations.declaration_trace(kind_id))
    }

    pub fn authoritative_schema_basis(
        &self,
    ) -> Result<Option<(SchemaId, SchemaVersionId)>, SchemaRegistryError> {
        let mut registrations = self
            .entity_kinds
            .values()
            .map(|registration| (&registration.schema_id, registration.schema_version_id))
            .chain(
                self.relation_kinds
                    .values()
                    .map(|registration| (&registration.schema_id, registration.schema_version_id)),
            );
        let Some((schema_id, schema_version_id)) = registrations.next() else {
            return Ok(None);
        };
        for (next_schema_id, next_schema_version_id) in registrations {
            if next_schema_id != schema_id || next_schema_version_id != schema_version_id {
                return Err(SchemaRegistryError::inconsistent_schema_basis(format!(
                    "found mixed schema basis {:?}/{:?} and {:?}/{:?}",
                    schema_id,
                    schema_version_id,
                    next_schema_id,
                    next_schema_version_id
                )));
            }
        }
        Ok(Some((schema_id.clone(), schema_version_id)))
    }
}
