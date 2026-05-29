mod authoritative_field_values;
mod planned_field_values;

use std::collections::HashMap;

use forge_foundational::facade::{AspectFieldLocator, AspectValue, FieldKey};

use crate::diagnostics::data::DiagnosticCode;
use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key, AuthoritativeFieldComparisonKey,
};
use crate::validation::data::{
    InvariantClass, InvariantViolation, InvariantViolationFields, StorageInconsistencyScan,
    UniqueEntityAspectField,
};

use super::super::context::InvariantExecutionContext;
use super::common::{storage_inconsistency_violation, StorageInconsistencyContext};
use authoritative_field_values::{
    record_authoritative_field_value, visible_entity_field_value_conflict,
};
use planned_field_values::{planned_entity_aspect_field_values, PlannedEntityAspectFieldValue};

pub(super) fn evaluate_unique_entity_aspect_field(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    target: &UniqueEntityAspectField,
) -> Option<InvariantViolation> {
    let field = target.single_field()?;
    if let Some(violation) = planned_unique_entity_aspect_field_violation(context, class, target) {
        return Some(violation);
    }

    if let Some(touched_entity_ids) = context.state_view().touched_visible_entity_ids() {
        return touched_unique_entity_aspect_field_violation(
            context,
            class,
            target.field_locator(),
            touched_entity_ids,
        );
    }

    visible_unique_entity_aspect_field_violation(context, class, target.field_locator(), field)
}

fn planned_unique_entity_aspect_field_violation(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    target: &UniqueEntityAspectField,
) -> Option<InvariantViolation> {
    let planned_values =
        planned_entity_aspect_field_values(context, context.merged_plan()?, target);
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
                return Some(duplicate_field_violation(
                    class,
                    target.field_locator(),
                    value,
                ));
            }
        }
        if committed_entity_value_conflicts_with_planned_value(
            context,
            target.field_locator(),
            &comparison_key,
            entity_id,
        ) {
            return Some(duplicate_field_violation(
                class,
                target.field_locator(),
                value,
            ));
        }
    }
    None
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
        let Some(record) = context.visible_entity_record(entity_id) else {
            continue;
        };
        let Some(value) = record_authoritative_field_value(&record, field_locator) else {
            continue;
        };
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
            let Some(record) = context.visible_entity_record(metadata.entity_id) else {
                continue;
            };
            let Some(value) = record_authoritative_field_value(&record, field_locator) else {
                continue;
            };
            let comparison_key = authoritative_aspect_value_field_comparison_key(&value);
            if seen.insert(comparison_key, metadata.entity_id).is_some() {
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
    visible_entity_field_value_conflict(
        context,
        field_locator,
        planned_comparison_key,
        |entity_id| planned_entity_id != Some(entity_id),
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
            "entity aspect field '{}:{}' must be unique, duplicate value '{}'",
            field_locator.aspect().aspect_key().as_str(),
            crate::transactions::data::canonical_field_path_label(field_locator.field_path()),
            authoritative_aspect_value_field_comparison_key(&value).display_value()
        ),
        fields: InvariantViolationFields::UniqueEntityField {
            field_locator: field_locator.clone(),
            value,
        },
    }
}
