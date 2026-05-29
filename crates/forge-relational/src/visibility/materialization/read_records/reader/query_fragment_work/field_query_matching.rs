use forge_foundational::facade::FieldKey;

use crate::storage::data::AuthoritativeFieldComparisonKey;

pub(super) fn entity_field_matches(
    record: &crate::storage::data::EntityReadRecord,
    field: &FieldKey,
    expected: &AuthoritativeFieldComparisonKey,
) -> bool {
    entity_field_value(record, field) == Some(expected)
}

pub(super) fn relation_field_matches(
    record: &crate::storage::data::RelationReadRecord,
    field: &FieldKey,
    expected: &AuthoritativeFieldComparisonKey,
) -> bool {
    relation_field_value(record, field) == Some(expected)
}

pub(super) fn entity_field_value<'record>(
    record: &'record crate::storage::data::EntityReadRecord,
    field: &FieldKey,
) -> Option<&'record AuthoritativeFieldComparisonKey> {
    record.authoritative_field_comparison_key(field)
}

pub(super) fn relation_field_value<'record>(
    record: &'record crate::storage::data::RelationReadRecord,
    field: &FieldKey,
) -> Option<&'record AuthoritativeFieldComparisonKey> {
    record.authoritative_field_comparison_key(field)
}
