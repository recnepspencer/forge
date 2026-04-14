use crate::authoring::{AspectFieldKey, AspectName, DeliveredFieldName, FieldName};

pub(crate) fn canonical_result_field_digest_part(
    source_aspect: &AspectName,
    source_field: &FieldName,
    delivered_name: &DeliveredFieldName,
) -> String {
    format!(
        "result_field:{}:{}:{}",
        source_aspect, source_field, delivered_name
    )
}

pub(crate) fn source_projection_key(
    source_aspect: &AspectName,
    source_field: &FieldName,
) -> AspectFieldKey {
    AspectFieldKey::from_parts(source_aspect.clone(), source_field.clone())
}
