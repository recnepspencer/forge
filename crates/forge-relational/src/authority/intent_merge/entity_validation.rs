use crate::authority::mutation::plan_entity_field_aspect_patch;
use crate::capabilities::{AspectPlanSource, SchemaSource, StorageRead};
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitConflict, ConflictClass, CreateIntent, EntityFieldAspectPatchDenial,
    EntityFieldIntentValidationMissingState, EntityMutationIntent, MutationIntent,
    MutationStateInconsistencyEvidence, UpdateEntityFieldsIntent,
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
            validate_update_entity_fields_intent(runtime, state, spec)
        }
        MutationIntent::Create(_)
        | MutationIntent::Entity(EntityMutationIntent::Delete(_))
        | MutationIntent::Relation(_) => Ok(()),
    }
}

fn validate_update_entity_fields_intent(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    spec: &UpdateEntityFieldsIntent,
) -> Result<(), CommitConflict> {
    let partition = state
        .get_partition(spec.entity_id.partition_id)
        .ok_or_else(|| {
            entity_field_validation_missing(
                spec.entity_id,
                EntityFieldIntentValidationMissingState::Partition,
                "entity field update validation requires an existing partition",
            )
        })?;
    let slot = partition
        .entity_arena
        .get_slot(spec.entity_id.slot_index())
        .ok_or_else(|| {
            entity_field_validation_missing(
                spec.entity_id,
                EntityFieldIntentValidationMissingState::Slot,
                "entity field update validation requires an existing slot",
            )
        })?;
    let kind_id = slot.kind_id().ok_or_else(|| {
        entity_field_validation_missing(
            spec.entity_id,
            EntityFieldIntentValidationMissingState::KindId,
            "entity field update validation requires a retained kind id",
        )
    })?;
    let lowered_plan = runtime.entity_aspect_plan(kind_id).ok_or_else(|| {
        CommitConflict::new(ConflictClass::EntityFieldAspectPatchDenied {
            entity_id: spec.entity_id,
            denial: EntityFieldAspectPatchDenial::MissingAspectPlan { kind_id },
        })
    })?;
    plan_entity_field_aspect_patch(kind_id, Some(lowered_plan), &spec.fields)
        .map(|_| ())
        .map_err(|denial| {
            CommitConflict::new(ConflictClass::EntityFieldAspectPatchDenied {
                entity_id: spec.entity_id,
                denial,
            })
        })
}

fn entity_field_validation_missing(
    entity_id: crate::identity::data::EntityId,
    missing: EntityFieldIntentValidationMissingState,
    detail: impl Into<String>,
) -> CommitConflict {
    CommitConflict::new(ConflictClass::MutationStateInconsistency {
        detail: detail.into(),
        evidence: MutationStateInconsistencyEvidence::EntityFieldIntentValidation {
            entity_id,
            missing,
        },
    })
}
