pub(crate) fn canonical_result_field_digest_part(
    source_aspect: &str,
    source_field: &str,
    delivered_name: &str,
) -> String {
    format!(
        "result_field:{}:{}:{}",
        source_aspect, source_field, delivered_name
    )
}

pub(crate) fn source_projection_key(source_aspect: &str, source_field: &str) -> (String, String) {
    (source_aspect.to_string(), source_field.to_string())
}
