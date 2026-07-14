use worth_foundational::facade::AspectFieldLocator;

use crate::storage::data::AuthoritativeFieldComparisonKey;
use crate::visibility::materialization::read_records::{
    entity_query_locus_comparison_key, relation_query_locus_comparison_key,
};

pub(super) fn entity_field_matches(
    record: &crate::storage::data::EntityReadRecord,
    field_locator: &AspectFieldLocator,
    expected: &AuthoritativeFieldComparisonKey,
) -> bool {
    entity_query_locus_comparison_key(record, field_locator).as_ref() == Some(expected)
}

pub(super) fn relation_field_matches(
    record: &crate::storage::data::RelationReadRecord,
    field_locator: &AspectFieldLocator,
    expected: &AuthoritativeFieldComparisonKey,
) -> bool {
    relation_query_locus_comparison_key(record, field_locator).as_ref() == Some(expected)
}
