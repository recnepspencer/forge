mod planned_field_values;
mod projected_entity_field_values;

use std::collections::HashMap;

use worth_foundational::facade::{AspectFieldLocator, AspectValue, CanonicalFieldPath, FieldKey};

use crate::diagnostics::data::DiagnosticCode;
use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key, AuthoritativeFieldComparisonKey,
};
use crate::validation::data::{
    InvariantClass, InvariantViolation, InvariantViolationFields, StorageInconsistencyScan,
};

use super::super::context::InvariantExecutionContext;
use super::common::{storage_inconsistency_violation, StorageInconsistencyContext};
use planned_field_values::{planned_entity_aspect_field_values, PlannedEntityAspectFieldValue};
use projected_entity_field_values::{
    projected_entity_aspect_field_value, projected_entity_aspect_field_value_for_metadata,
    visible_entity_field_value_conflict,
};

pub(super) fn evaluate_unique_entity_aspect_field(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field_locator: &AspectFieldLocator,
) -> Option<InvariantViolation> {
    let field = single_field(field_locator)?;
    if let Some(violation) =
        planned_unique_entity_aspect_field_violation(context, class, field_locator)
    {
        return Some(violation);
    }

    if let Some(touched_entity_ids) = context.state_view().touched_visible_entity_ids() {
        return touched_unique_entity_aspect_field_violation(
            context,
            class,
            field_locator,
            touched_entity_ids,
        );
    }

    visible_unique_entity_aspect_field_violation(context, class, field_locator, field)
}

fn planned_unique_entity_aspect_field_violation(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field_locator: &AspectFieldLocator,
) -> Option<InvariantViolation> {
    let planned_values =
        planned_entity_aspect_field_values(context, context.merged_plan()?, field_locator);
    let mut planned_value_to_entity = HashMap::<
        AuthoritativeFieldComparisonKey,
        Option<crate::identity::data::EntityId>,
    >::with_capacity(planned_values.len());
    for PlannedEntityAspectFieldValue { entity_id, value } in planned_values {
        context.metrics().count_entity_slot_scans(1);
        let comparison_key = authoritative_aspect_value_field_comparison_key(&value);
        if let Some(existing_entity_id) =
            planned_value_to_entity.insert(comparison_key.clone(), entity_id)
        {
            if existing_entity_id != entity_id || entity_id.is_none() {
                return Some(duplicate_field_violation(class, field_locator, value));
            }
        }
        if committed_entity_value_conflicts_with_planned_value(
            context,
            field_locator,
            &comparison_key,
            entity_id,
        ) {
            return Some(duplicate_field_violation(class, field_locator, value));
        }
    }
    None
}

fn single_field(field_locator: &AspectFieldLocator) -> Option<&FieldKey> {
    match field_locator.field_path().fields() {
        [field] => Some(field),
        _ => None,
    }
}

fn touched_unique_entity_aspect_field_violation(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field_locator: &AspectFieldLocator,
    touched_entity_ids: Vec<crate::identity::data::EntityId>,
) -> Option<InvariantViolation> {
    let mut touched_value_to_entity =
        HashMap::<AuthoritativeFieldComparisonKey, crate::identity::data::EntityId>::new();
    for entity_id in touched_entity_ids {
        context.metrics().count_entity_slot_scans(1);
        let Some(projected) =
            projected_entity_aspect_field_value(context, entity_id, field_locator)
        else {
            continue;
        };
        let value = projected.value;
        let comparison_key = authoritative_aspect_value_field_comparison_key(&value);
        if touched_value_to_entity
            .insert(comparison_key.clone(), entity_id)
            .is_some()
        {
            return Some(duplicate_field_violation(class, field_locator, value));
        }
        if committed_entity_value_conflicts_outside_touched_set(
            context,
            field_locator,
            &comparison_key,
            entity_id,
        ) {
            return Some(duplicate_field_violation(class, field_locator, value));
        }
    }
    None
}

fn visible_unique_entity_aspect_field_violation(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    field_locator: &AspectFieldLocator,
    field: &FieldKey,
) -> Option<InvariantViolation> {
    let mut seen =
        HashMap::<AuthoritativeFieldComparisonKey, crate::identity::data::EntityId>::new();
    let state_view = context.state_view();
    for partition_id in state_view.state().partition_ids() {
        if state_view.state().get_partition(partition_id).is_none() {
            return Some(storage_inconsistency_violation(
                class,
                format!(
                    "partition {:?} missing during historical uniqueness scan",
                    partition_id
                ),
                StorageInconsistencyContext::default()
                    .with_partition_id(partition_id)
                    .with_scan(StorageInconsistencyScan::HistoricalUniqueEntityAspectField)
                    .with_field(field.clone()),
            ));
        };
        let Some(slot_count) = state_view.entity_slot_scan_count(partition_id) else {
            continue;
        };
        for slot in 0..slot_count {
            context.metrics().count_entity_slot_scans(1);
            let Some(metadata) = state_view.entity_metadata_for_slot(partition_id, slot) else {
                continue;
            };
            let Some(projected) =
                projected_entity_aspect_field_value_for_metadata(context, &metadata, field_locator)
            else {
                continue;
            };
            let value = projected.value;
            let comparison_key = authoritative_aspect_value_field_comparison_key(&value);
            if seen.insert(comparison_key, projected.entity_id).is_some() {
                return Some(duplicate_field_violation(class, field_locator, value));
            }
        }
    }
    None
}

fn committed_entity_value_conflicts_with_planned_value(
    context: &InvariantExecutionContext<'_>,
    field_locator: &AspectFieldLocator,
    planned_comparison_key: &AuthoritativeFieldComparisonKey,
    planned_entity_id: Option<crate::identity::data::EntityId>,
) -> bool {
    let touched_visible_entity_ids = planned_entity_id
        .is_none()
        .then(|| context.state_view().touched_visible_entity_ids())
        .flatten()
        .unwrap_or_default();
    visible_entity_field_value_conflict(
        context,
        field_locator,
        planned_comparison_key,
        |entity_id| {
            planned_entity_id != Some(entity_id) && !touched_visible_entity_ids.contains(&entity_id)
        },
    )
}

fn committed_entity_value_conflicts_outside_touched_set(
    context: &InvariantExecutionContext<'_>,
    field_locator: &AspectFieldLocator,
    touched_comparison_key: &AuthoritativeFieldComparisonKey,
    touched_entity_id: crate::identity::data::EntityId,
) -> bool {
    visible_entity_field_value_conflict(
        context,
        field_locator,
        touched_comparison_key,
        |entity_id| entity_id != touched_entity_id,
    )
}

fn duplicate_field_violation(
    class: InvariantClass,
    field_locator: &AspectFieldLocator,
    value: AspectValue,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "entity aspect field '{}:{}' must be unique, duplicate {:?} value",
            field_locator.aspect().aspect_key().as_str(),
            field_path_presentation_label(field_locator.field_path()),
            value.value_family()
        ),
        fields: InvariantViolationFields::UniqueEntityField {
            field_locator: field_locator.clone(),
            value,
        },
    }
}

fn field_path_presentation_label(path: &CanonicalFieldPath) -> String {
    path.fields()
        .iter()
        .map(FieldKey::as_str)
        .collect::<Vec<_>>()
        .join(".")
}
