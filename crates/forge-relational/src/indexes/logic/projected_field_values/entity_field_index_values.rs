use std::collections::BTreeMap;

use forge_foundational::facade::AspectFieldLocator;

use crate::identity::data::EntityId;
use crate::logic::runtime::{RelationalRuntime, VisibilityProjectionView};
use crate::storage::data::AuthoritativeFieldComparisonKey;

use super::field_projection_scope::{
    entity_index_projection_scope_for_kind, entity_index_projection_scopes,
    EntityIndexFieldProjectionScope,
};

pub(in crate::indexes::logic) fn build_entity_aspect_field_index(
    runtime: &RelationalRuntime,
    projection: &VisibilityProjectionView<'_>,
    field_locator: &AspectFieldLocator,
) -> BTreeMap<AuthoritativeFieldComparisonKey, Vec<EntityId>> {
    let mut entries = BTreeMap::new();
    for scope in entity_index_projection_scopes(runtime, field_locator) {
        for (key, entity_id) in projected_entity_field_entries(projection, &scope) {
            entries.entry(key).or_insert_with(Vec::new).push(entity_id);
        }
    }
    entries
}

pub(in crate::indexes::logic) fn entity_aspect_field_index_entry(
    runtime: &RelationalRuntime,
    projection: &VisibilityProjectionView<'_>,
    entity_id: EntityId,
    field_locator: &AspectFieldLocator,
) -> Option<(AuthoritativeFieldComparisonKey, EntityId)> {
    let kind_id = projection.entity_record_kind_id(entity_id)?;
    let scope = entity_index_projection_scope_for_kind(runtime, kind_id, field_locator)?;
    projection.entity_record_with_projection_scope(entity_id, scope.projection_scope(), |record| {
        scope.projected_value(record).map(|value| {
            (
                AuthoritativeFieldComparisonKey::from_aspect_value(value),
                entity_id,
            )
        })
    })
}

fn projected_entity_field_entries(
    projection: &VisibilityProjectionView<'_>,
    scope: &EntityIndexFieldProjectionScope,
) -> Vec<(AuthoritativeFieldComparisonKey, EntityId)> {
    projection.entity_records_with_projection_scope(
        scope.kind_id(),
        scope.projection_scope(),
        |record| {
            let entity_id = record.entity_id();
            scope.projected_value(record).map(|value| {
                (
                    AuthoritativeFieldComparisonKey::from_aspect_value(value),
                    entity_id,
                )
            })
        },
    )
}
