use crate::capabilities::{SchemaSource, StorageRead};
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::schema::data::{AspectBinding, AspectComparator};
use crate::symbols::data::InternedString;
use crate::transactions::data::{
    CommitConflict, ConflictClass, CreateIntent, EntityMutationIntent, MutationIntent,
    UpdateEntityFieldsIntent,
};

use super::record_lookup::{entity_exists_in_state, entity_exists_in_version_basis};
use super::schema_conflicts::schema_error_to_commit_conflict;

pub(super) fn validate_entity_intent(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    branch_basis_version_id: Option<VersionId>,
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
            validate_existing_entity_intent(
                runtime,
                state,
                schema_source,
                branch_basis_version_id,
                spec.entity_id,
                intent,
            )
        }
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(spec)) => {
            validate_existing_entity_intent(
                runtime,
                state,
                schema_source,
                branch_basis_version_id,
                spec.entity_id,
                intent,
            )
        }
        MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
            validate_existing_entity_intent(
                runtime,
                state,
                schema_source,
                branch_basis_version_id,
                spec.entity_id,
                intent,
            )
        }
        MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
            validate_existing_entity_intent(
                runtime,
                state,
                schema_source,
                branch_basis_version_id,
                spec.entity_id,
                intent,
            )
        }
        MutationIntent::Create(CreateIntent::Relation(_))
        | MutationIntent::Create(CreateIntent::BulkRelations(_))
        | MutationIntent::Relation(_) => Ok(()),
    }
}

fn validate_existing_entity_intent(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    branch_basis_version_id: Option<VersionId>,
    entity_id: crate::identity::data::EntityId,
    intent: &MutationIntent,
) -> Result<(), CommitConflict> {
    let schema_registry = schema_source.schema_registry();
    let exists_in_current_state = entity_exists_in_state(state, entity_id);
    let branch_delete_is_authorized = matches!(
        intent,
        MutationIntent::Entity(EntityMutationIntent::Delete(_))
    ) && branch_basis_version_id
        .is_some_and(|version_id| entity_exists_in_version_basis(runtime, version_id, entity_id));
    if !exists_in_current_state && !branch_delete_is_authorized {
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
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(spec)) => {
            validate_update_entity_fields_intent(state, schema_source, spec)
        }
        MutationIntent::Create(_)
        | MutationIntent::Entity(EntityMutationIntent::Update(_))
        | MutationIntent::Entity(EntityMutationIntent::Delete(_))
        | MutationIntent::Relation(_) => Ok(()),
    }
}

fn validate_update_entity_fields_intent(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    spec: &UpdateEntityFieldsIntent,
) -> Result<(), CommitConflict> {
    let partition = state
        .get_partition(spec.entity_id.partition_id)
        .ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail: "entity field update validation requires an existing partition".to_string(),
                fields: serde_json::json!({
                    "record_class": "entity",
                    "entity_id": spec.entity_id,
                    "phase": "intent_validation",
                    "missing": "partition",
                }),
            })
        })?;
    let slot = partition
        .entity_arena
        .get_slot(spec.entity_id.local_slot.0 as usize)
        .ok_or_else(|| {
            CommitConflict::new(ConflictClass::MutationStateInconsistency {
                detail: "entity field update validation requires an existing slot".to_string(),
                fields: serde_json::json!({
                    "record_class": "entity",
                    "entity_id": spec.entity_id,
                    "phase": "intent_validation",
                    "missing": "slot",
                }),
            })
        })?;
    let kind_id = slot.kind_id().ok_or_else(|| {
        CommitConflict::new(ConflictClass::MutationStateInconsistency {
            detail: "entity field update validation requires a retained kind id".to_string(),
            fields: serde_json::json!({
                "record_class": "entity",
                "entity_id": spec.entity_id,
                "phase": "intent_validation",
                "missing": "kind_id",
            }),
        })
    })?;
    let registration = schema_source
        .schema_registry()
        .entity_registration(kind_id)
        .map_err(schema_error_to_commit_conflict)?;
    let declared_scalar_fields = registration
        .aspect_declarations
        .aspects
        .iter()
        .filter_map(|aspect| match (&aspect.binding, aspect.comparator) {
            (AspectBinding::EntityPayloadField { field }, AspectComparator::JsonScalarEquality) => {
                match field {
                    InternedString::Raw(raw) => Some(raw.as_str()),
                    _ => None,
                }
            }
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    for key in spec.fields.keys() {
        if !declared_scalar_fields.contains(key.as_str()) {
            return Err(CommitConflict::new(ConflictClass::KindSchemaMismatch {
                detail: format!(
                    "entity field update key '{}' is not a declared scalar entity aspect on kind {:?}",
                    key, kind_id
                ),
            }));
        }
    }
    Ok(())
}
