use forge_foundational::facade::AspectFieldLocator;

use crate::storage::data::{
    entity_authoritative_aspect_field_comparison_key,
    relation_authoritative_aspect_field_comparison_key, AuthoritativeFieldComparisonKey,
};

pub(super) fn entity_field_matches(
    record: &crate::storage::data::EntityReadRecord,
    field_locator: &AspectFieldLocator,
    expected: &AuthoritativeFieldComparisonKey,
) -> bool {
    entity_authoritative_aspect_field_comparison_key(record, field_locator).as_ref()
        == Some(expected)
}

pub(super) fn relation_field_matches(
    record: &crate::storage::data::RelationReadRecord,
    field_locator: &AspectFieldLocator,
    expected: &AuthoritativeFieldComparisonKey,
) -> bool {
    relation_authoritative_aspect_field_comparison_key(record, field_locator).as_ref()
        == Some(expected)
}
