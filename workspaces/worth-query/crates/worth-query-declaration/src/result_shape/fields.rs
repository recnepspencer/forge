use crate::authoring::{AspectName, DeliveredFieldName, FieldName};

pub fn canonical_result_field_digest_part(
    source_aspect: &AspectName,
    source_field: &FieldName,
    delivered_name: &DeliveredFieldName,
) -> String {
    format!(
        "result_field:{}:{}:{}",
        source_aspect, source_field, delivered_name
    )
}
