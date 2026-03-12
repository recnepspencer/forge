use crate::capabilities::{SchemaSource, StorageRead};
use crate::transactions::data::{CommitConflict, ConflictClass, CreateIntent, EntityMutationIntent, MutationIntent};

use super::record_lookup::entity_exists_in_state;
use super::schema_conflicts::schema_error_to_commit_conflict;

pub(super) fn validate_entity_intent(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    intent: &MutationIntent,
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    match intent {
        MutationIntent::Create(CreateIntent::Entity(spec)) => schema_registry
            .resolve_entity(spec.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        MutationIntent::Create(CreateIntent::BulkEntities(spec)) => schema_registry
            .resolve_entity(spec.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        MutationIntent::Entity(EntityMutationIntent::Update(spec)) => {
            validate_existing_entity_intent(state, schema_source, spec.entity_id, intent)
        }
        MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
            validate_existing_entity_intent(state, schema_source, spec.entity_id, intent)
        }
        MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
            validate_existing_entity_intent(state, schema_source, spec.entity_id, intent)
        }
        MutationIntent::Create(CreateIntent::Relation(_))
        | MutationIntent::Create(CreateIntent::BulkRelations(_))
        | MutationIntent::Relation(_) => Ok(()),
    }
}

fn validate_existing_entity_intent(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    entity_id: crate::identity::data::EntityId,
    intent: &MutationIntent,
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    if !entity_exists_in_state(state, entity_id) {
        return Err(CommitConflict::new(ConflictClass::StaleTarget {
            target: crate::transactions::data::ExistingRecordTarget::Entity(entity_id),
            context: "entity validation".to_string(),
        }));
    }

    match intent {
        MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => schema_registry
            .resolve_entity(spec.replacement.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        MutationIntent::Create(_)
        | MutationIntent::Entity(EntityMutationIntent::Update(_))
        | MutationIntent::Entity(EntityMutationIntent::Delete(_))
        | MutationIntent::Relation(_) => Ok(()),
    }
}
