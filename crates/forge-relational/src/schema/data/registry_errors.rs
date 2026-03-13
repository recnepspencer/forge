use serde::{Deserialize, Serialize};

use crate::errors::data::{ErrorContext, ErrorOperation, RelationalSubsystem, SuggestedFix};
use crate::identity::data::KindId;

use super::SchemaVersionId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchemaRegistryErrorClass {
    UnknownEntityKind(KindId),
    UnknownRelationKind(KindId),
    EntityRelationKindCollision(KindId),
    SchemaVersionMismatch {
        expected: SchemaVersionId,
        actual: SchemaVersionId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaRegistryError {
    pub class: SchemaRegistryErrorClass,
    pub detail: String,
    pub context: ErrorContext,
}

impl SchemaRegistryError {
    fn new(class: SchemaRegistryErrorClass) -> Self {
        let detail = match &class {
            SchemaRegistryErrorClass::UnknownEntityKind(kind_id) => {
                format!("unknown entity kind {:?}", kind_id)
            }
            SchemaRegistryErrorClass::UnknownRelationKind(kind_id) => {
                format!("unknown relation kind {:?}", kind_id)
            }
            SchemaRegistryErrorClass::EntityRelationKindCollision(kind_id) => {
                format!(
                    "kind {:?} collides across entity and relation registries",
                    kind_id
                )
            }
            SchemaRegistryErrorClass::SchemaVersionMismatch { expected, actual } => {
                format!(
                    "schema version mismatch: expected {:?}, actual {:?}",
                    expected, actual
                )
            }
        };
        Self {
            class,
            detail,
            context: ErrorContext::new(RelationalSubsystem::Schema, ErrorOperation::ResolveSchema)
                .with_fix(SuggestedFix::ValidateSchemaRegistration),
        }
    }

    pub fn unknown_entity_kind(kind_id: KindId) -> Self {
        Self::new(SchemaRegistryErrorClass::UnknownEntityKind(kind_id))
    }

    pub fn unknown_relation_kind(kind_id: KindId) -> Self {
        Self::new(SchemaRegistryErrorClass::UnknownRelationKind(kind_id))
    }

    pub fn entity_relation_kind_collision(kind_id: KindId) -> Self {
        Self::new(SchemaRegistryErrorClass::EntityRelationKindCollision(
            kind_id,
        ))
    }
}
