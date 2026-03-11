use crate::capabilities::{SchemaSource, StorageRead};
use crate::transactions::data::{CommitConflict, ConflictClass, TransactionIntent};
use crate::validation::logic::schema_error_to_commit_conflict;

use super::record_lookup::entity_exists_in_state;

pub(super) fn validate_entity_intent(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    match intent {
        TransactionIntent::CreateEntity(spec) => schema_registry
            .resolve_entity(spec.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::BulkCreateEntities { kind_id, .. } => schema_registry
            .resolve_entity(*kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::UpdateEntity { entity_id, .. }
        | TransactionIntent::DeleteEntity { entity_id }
        | TransactionIntent::ReplaceEntity { entity_id, .. } => {
            validate_existing_entity_intent(state, schema_source, *entity_id, intent)
        }
        TransactionIntent::CreateRelation(_)
        | TransactionIntent::BulkCreateRelations { .. }
        | TransactionIntent::DeleteRelation { .. } => Ok(()),
    }
}

fn validate_existing_entity_intent(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    entity_id: crate::identity::data::EntityId,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    if !entity_exists_in_state(state, entity_id) {
        return Err(CommitConflict::new(ConflictClass::StaleTarget {
            target: crate::transactions::data::ExistingRecordTarget::Entity(entity_id),
            context: "entity validation".to_string(),
        }));
    }

    match intent {
        TransactionIntent::ReplaceEntity { replacement, .. } => schema_registry
            .resolve_entity(replacement.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::CreateEntity(_)
        | TransactionIntent::BulkCreateEntities { .. }
        | TransactionIntent::UpdateEntity { .. }
        | TransactionIntent::DeleteEntity { .. }
        | TransactionIntent::CreateRelation(_)
        | TransactionIntent::BulkCreateRelations { .. }
        | TransactionIntent::DeleteRelation { .. } => Ok(()),
    }
}
