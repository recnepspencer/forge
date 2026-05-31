use std::collections::BTreeSet;

use forge_foundational::facade::AspectFieldLocator;

use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::AuthoritativeFieldComparisonKey;

use super::projected_field_values::{
    build_entity_aspect_field_index, entity_aspect_field_index_entry,
};

pub(crate) fn refresh_unique_entity_aspect_field_index_for_records(
    runtime: &mut RelationalRuntime,
    changed_records: &[crate::transactions::data::RecordRef],
    version_id: crate::identity::data::VersionId,
) {
    let tracked_fields = tracked_unique_entity_aspect_fields(runtime);
    if tracked_fields.is_empty() {
        return;
    }
    let projection = runtime.read_truth().project_version(version_id);
    let refreshed_values = collect_changed_unique_entity_aspect_field_entries(
        runtime,
        &projection,
        changed_records,
        &tracked_fields,
    );
    remove_changed_entities_from_unique_entity_aspect_field_index(
        runtime,
        changed_records,
        &tracked_fields,
    );
    write_unique_entity_aspect_field_index_entries(runtime, refreshed_values);
}

pub(crate) fn rebuild_unique_entity_aspect_field_indexes(runtime: &mut RelationalRuntime) {
    runtime.indexes.entity_unique_aspect_field_index.clear();
    let tracked_fields = tracked_unique_entity_aspect_fields(runtime);
    if tracked_fields.is_empty() {
        return;
    }
    let projection = runtime
        .read_truth()
        .project_version(runtime.current_version_id());
    let rebuilt_values =
        collect_all_unique_entity_aspect_field_entries(runtime, &projection, &tracked_fields);
    write_unique_entity_aspect_field_index_entries(runtime, rebuilt_values);
}

fn remove_changed_entities_from_unique_entity_aspect_field_index(
    runtime: &mut RelationalRuntime,
    changed_records: &[crate::transactions::data::RecordRef],
    tracked_fields: &BTreeSet<AspectFieldLocator>,
) {
    for record in changed_records {
        let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
            continue;
        };
        for field_locator in tracked_fields {
            if let Some(values) = runtime
                .indexes
                .entity_unique_aspect_field_index
                .get_mut(field_locator)
            {
                values.retain(|_, entity_ids| {
                    entity_ids.remove(entity_id);
                    !entity_ids.is_empty()
                });
            }
        }
    }
}

fn write_unique_entity_aspect_field_index_entries(
    runtime: &mut RelationalRuntime,
    entries: Vec<(
        AspectFieldLocator,
        AuthoritativeFieldComparisonKey,
        crate::identity::data::EntityId,
    )>,
) {
    for (field_locator, value, entity_id) in entries {
        runtime
            .indexes
            .entity_unique_aspect_field_index
            .entry(field_locator)
            .or_default()
            .entry(value)
            .or_default()
            .insert(entity_id);
    }
}

fn collect_all_unique_entity_aspect_field_entries(
    runtime: &RelationalRuntime,
    projection: &crate::logic::runtime::VisibilityProjectionView<'_>,
    tracked_fields: &BTreeSet<AspectFieldLocator>,
) -> Vec<(
    AspectFieldLocator,
    AuthoritativeFieldComparisonKey,
    crate::identity::data::EntityId,
)> {
    let mut entries = Vec::new();
    for field_locator in tracked_fields {
        entries.extend(
            build_entity_aspect_field_index(runtime, projection, field_locator)
                .into_iter()
                .flat_map(|(value, entity_ids)| {
                    entity_ids
                        .into_iter()
                        .map(move |entity_id| (field_locator.clone(), value.clone(), entity_id))
                }),
        );
    }
    entries
}

fn collect_changed_unique_entity_aspect_field_entries(
    runtime: &RelationalRuntime,
    projection: &crate::logic::runtime::VisibilityProjectionView<'_>,
    changed_records: &[crate::transactions::data::RecordRef],
    tracked_fields: &BTreeSet<AspectFieldLocator>,
) -> Vec<(
    AspectFieldLocator,
    AuthoritativeFieldComparisonKey,
    crate::identity::data::EntityId,
)> {
    let mut entries = Vec::new();
    for record in changed_records {
        let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
            continue;
        };
        for field_locator in tracked_fields {
            if let Some((value, entity_id)) =
                entity_aspect_field_index_entry(runtime, projection, *entity_id, field_locator)
            {
                entries.push((field_locator.clone(), value, entity_id));
            }
        }
    }
    entries
}

fn tracked_unique_entity_aspect_fields(
    runtime: &RelationalRuntime,
) -> BTreeSet<AspectFieldLocator> {
    let mut fields = BTreeSet::new();
    for registration in &runtime.config.schema.invariant_catalog.registrations {
        if let crate::validation::data::InvariantRule::UniqueEntityAspectField { field_locator } =
            &registration.rule
        {
            fields.insert(field_locator.clone());
        }
    }
    fields
}
