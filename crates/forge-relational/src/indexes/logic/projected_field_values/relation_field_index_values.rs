use std::collections::BTreeMap;

use forge_foundational::facade::AspectFieldLocator;

use crate::identity::data::RelationId;
use crate::logic::runtime::{RelationalRuntime, VisibilityProjectionView};
use crate::storage::data::AuthoritativeFieldComparisonKey;

use super::field_projection_scope::{
    relation_index_projection_scopes, RelationIndexFieldProjectionScope,
};

pub(in crate::indexes::logic) fn build_relation_aspect_field_index(
    runtime: &RelationalRuntime,
    projection: &VisibilityProjectionView<'_>,
    field_locator: &AspectFieldLocator,
) -> BTreeMap<AuthoritativeFieldComparisonKey, Vec<RelationId>> {
    let mut entries = BTreeMap::new();
    for scope in relation_index_projection_scopes(runtime, field_locator) {
        for (key, relation_id) in projected_relation_field_entries(projection, &scope) {
            entries
                .entry(key)
                .or_insert_with(Vec::new)
                .push(relation_id);
        }
    }
    entries
}

fn projected_relation_field_entries(
    projection: &VisibilityProjectionView<'_>,
    scope: &RelationIndexFieldProjectionScope,
) -> Vec<(AuthoritativeFieldComparisonKey, RelationId)> {
    projection.relation_records_with_projection_scope(
        scope.kind_id(),
        scope.projection_scope(),
        |record| {
            let relation_id = record.relation_id();
            scope.projected_value(record).map(|value| {
                (
                    AuthoritativeFieldComparisonKey::from_aspect_value(value),
                    relation_id,
                )
            })
        },
    )
}
