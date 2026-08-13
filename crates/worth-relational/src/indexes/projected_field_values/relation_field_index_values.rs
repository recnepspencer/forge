use std::collections::BTreeMap;

use worth_foundational::facade::AspectFieldLocator;

use crate::identity::data::RelationId;
use crate::runtime::RelationalRuntime;
use crate::storage::data::AuthoritativeFieldComparisonKey;
use crate::visibility::materialization::read_records::relation_query_locus_comparison_key;

use super::field_projection_scope::relation_index_projection_scopes;
use super::IndexProjectionSource;

pub(in crate::indexes) fn build_relation_aspect_field_index(
    runtime: &RelationalRuntime,
    projection: &IndexProjectionSource<'_, '_>,
    field_locator: &AspectFieldLocator,
) -> BTreeMap<AuthoritativeFieldComparisonKey, Vec<RelationId>> {
    let mut entries = BTreeMap::new();
    for scope in relation_index_projection_scopes(runtime, field_locator) {
        projection.for_each_relation(scope.kind_id(), |record| {
            if let Some(key) = relation_query_locus_comparison_key(record, field_locator) {
                entries
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(record.relation_id);
            }
        });
    }
    entries
}
