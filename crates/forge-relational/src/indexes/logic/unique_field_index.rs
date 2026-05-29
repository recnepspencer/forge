use std::collections::BTreeSet;

use forge_foundational::facade::FieldKey;

use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::AuthoritativeFieldComparisonKey;

use super::observed_field_indexes::collect_unique_entity_field_entries;

pub(crate) fn refresh_unique_field_index_for_records(
    runtime: &mut RelationalRuntime,
    changed_records: &[crate::transactions::data::RecordRef],
    version_id: crate::identity::data::VersionId,
) {
    let tracked_fields = tracked_unique_entity_fields(runtime);
    if tracked_fields.is_empty() {
        return;
    }
    let projection = runtime.read_truth().project_version(version_id);
    let refreshed_values = changed_entity_records(&projection, changed_records);
    let refreshed_values = collect_unique_entity_field_entries(&refreshed_values, &tracked_fields);
    remove_changed_entities_from_unique_field_index(runtime, changed_records, &tracked_fields);
    write_unique_field_index_entries(runtime, refreshed_values);
}

pub(crate) fn rebuild_unique_field_indexes(runtime: &mut RelationalRuntime) {
    runtime.indexes.entity_unique_field_index.clear();
    let tracked_fields = tracked_unique_entity_fields(runtime);
    if tracked_fields.is_empty() {
        return;
    }
    let projection = runtime
        .read_truth()
        .project_version(runtime.current_version_id());
    let rebuilt_values =
        collect_unique_entity_field_entries(&projection.all_entity_records(), &tracked_fields);
    write_unique_field_index_entries(runtime, rebuilt_values);
}

fn remove_changed_entities_from_unique_field_index(
    runtime: &mut RelationalRuntime,
    changed_records: &[crate::transactions::data::RecordRef],
    tracked_fields: &BTreeSet<FieldKey>,
) {
    for record in changed_records {
        let crate::transactions::data::RecordRef::Entity(entity_id) = record else {
            continue;
        };
        for field in tracked_fields {
            if let Some(values) = runtime.indexes.entity_unique_field_index.get_mut(field) {
                values.retain(|_, entity_ids| {
                    entity_ids.remove(entity_id);
                    !entity_ids.is_empty()
                });
            }
        }
    }
}

fn write_unique_field_index_entries(
    runtime: &mut RelationalRuntime,
    entries: Vec<(
        FieldKey,
        AuthoritativeFieldComparisonKey,
        crate::identity::data::EntityId,
    )>,
) {
    for (field, value, entity_id) in entries {
        runtime
            .indexes
            .entity_unique_field_index
            .entry(field)
            .or_default()
            .entry(value)
            .or_default()
            .insert(entity_id);
    }
}

fn tracked_unique_entity_fields(runtime: &RelationalRuntime) -> BTreeSet<FieldKey> {
    let mut fields = BTreeSet::new();
    for registration in &runtime.config.schema.invariant_catalog.registrations {
        if let crate::validation::data::InvariantRule::UniqueEntityAspectField(target) =
            &registration.rule
        {
            if let Some(field) = target.single_field() {
                fields.insert(field.clone());
            }
        }
    }
    fields
}

fn changed_entity_records(
    projection: &crate::logic::runtime::VisibilityProjectionView<'_>,
    changed_records: &[crate::transactions::data::RecordRef],
) -> Vec<crate::storage::data::EntityReadRecord> {
    changed_records
        .iter()
        .filter_map(|record| match record {
            crate::transactions::data::RecordRef::Entity(entity_id) => {
                projection.entity_record(*entity_id)
            }
            crate::transactions::data::RecordRef::Relation(_) => None,
        })
        .collect()
}
