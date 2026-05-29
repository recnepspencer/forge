use forge_foundational::facade::{AspectFieldLocator, AspectValue};

use crate::transactions::data::{
    AspectFieldPatch, AspectFieldPatchTarget, CreateIntent, EntityMutationIntent, MergedCommitPlan,
    MutationIntent, ReplaceEntityIntent,
};
use crate::validation::data::UniqueEntityAspectField;

use super::super::super::context::InvariantExecutionContext;

pub(super) struct PlannedEntityAspectFieldValue {
    pub(super) entity_id: Option<crate::identity::data::EntityId>,
    pub(super) value: AspectValue,
}

pub(super) fn planned_entity_aspect_field_values(
    _context: &InvariantExecutionContext<'_>,
    merged_plan: &MergedCommitPlan,
    target: &UniqueEntityAspectField,
) -> Vec<PlannedEntityAspectFieldValue> {
    let mut values = Vec::new();
    for intent in &merged_plan.merged_intents {
        values.extend(planned_intent_aspect_field_values(
            intent,
            target.field_locator(),
        ));
    }
    values
}

fn planned_intent_aspect_field_values(
    intent: &MutationIntent,
    field_locator: &AspectFieldLocator,
) -> Vec<PlannedEntityAspectFieldValue> {
    match intent {
        MutationIntent::Create(CreateIntent::Entity(create)) => {
            planned_patch_aspect_field_value(None, &create.fields, field_locator)
                .into_iter()
                .collect()
        }
        MutationIntent::Create(CreateIntent::BulkEntities(bulk)) => bulk
            .field_patches
            .iter()
            .filter_map(|fields| planned_patch_aspect_field_value(None, fields, field_locator))
            .collect(),
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(update)) => {
            planned_patch_aspect_field_value(Some(update.entity_id), &update.fields, field_locator)
                .into_iter()
                .collect()
        }
        MutationIntent::Entity(EntityMutationIntent::Replace(replace)) => {
            planned_replacement_aspect_field_values(replace, field_locator)
        }
        _ => Vec::new(),
    }
}

fn planned_replacement_aspect_field_values(
    replace: &ReplaceEntityIntent,
    field_locator: &AspectFieldLocator,
) -> Vec<PlannedEntityAspectFieldValue> {
    planned_patch_aspect_field_value(
        Some(replace.entity_id),
        &replace.replacement.fields,
        field_locator,
    )
    .into_iter()
    .collect()
}

fn planned_patch_aspect_field_value(
    entity_id: Option<crate::identity::data::EntityId>,
    patch: &AspectFieldPatch,
    field_locator: &AspectFieldLocator,
) -> Option<PlannedEntityAspectFieldValue> {
    patch
        .get(&AspectFieldPatchTarget::new(
            field_locator.aspect().aspect_key().clone(),
            field_locator.field_path().clone(),
        ))
        .cloned()
        .map(|value| PlannedEntityAspectFieldValue { entity_id, value })
}
