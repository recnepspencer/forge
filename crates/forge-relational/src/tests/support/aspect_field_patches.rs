use super::{AspectFieldPatch, AspectKey};
use std::collections::BTreeMap;

pub(crate) fn aspect_field_patch_from_compatibility_json(
    value: serde_json::Value,
) -> AspectFieldPatch {
    let fields = value
        .as_object()
        .expect("test aspect field patch fixture must be a JSON object")
        .iter()
        .map(|(field, value)| {
            (
                single_field_patch_target(field),
                aspect_value_from_fixture_json(value),
            )
        })
        .collect::<BTreeMap<_, _>>();
    AspectFieldPatch::from(fields)
}

pub(crate) fn single_string_aspect_field_patch(field: &str, value: &str) -> AspectFieldPatch {
    aspect_field_patch_from_values([(field, string_aspect_value(value))])
}

pub(crate) fn string_aspect_field_patch<'a>(
    fields: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> AspectFieldPatch {
    aspect_field_patch_from_values(
        fields
            .into_iter()
            .map(|(field, value)| (field, string_aspect_value(value))),
    )
}

pub(crate) fn aspect_field_patch_from_values<'a>(
    fields: impl IntoIterator<Item = (&'a str, forge_foundational::facade::AspectValue)>,
) -> AspectFieldPatch {
    let fields = fields
        .into_iter()
        .map(|(field, value)| (single_field_patch_target(field), value))
        .collect::<BTreeMap<_, _>>();
    AspectFieldPatch::from(fields)
}

fn single_field_patch_target(field: &str) -> crate::transactions::data::AspectFieldPatchTarget {
    let field_key = forge_foundational::facade::FieldKey::new(field.to_string())
        .expect("test field key must be valid");
    let aspect_key = AspectKey::new(field.to_string()).expect("test aspect key must be valid");
    crate::transactions::data::AspectFieldPatchTarget::single(aspect_key, field_key)
}

pub(crate) fn string_aspect_value(value: &str) -> forge_foundational::facade::AspectValue {
    forge_foundational::facade::AspectValue::String(
        forge_foundational::facade::InternedString::Raw(value.to_string()),
    )
}

fn aspect_value_from_fixture_json(
    value: &serde_json::Value,
) -> forge_foundational::facade::AspectValue {
    match value {
        serde_json::Value::Null => forge_foundational::facade::AspectValue::Null,
        serde_json::Value::Bool(value) => forge_foundational::facade::AspectValue::Bool(*value),
        serde_json::Value::Number(value) => {
            if let Some(value) = value.as_u64() {
                forge_foundational::facade::AspectValue::UInt64(value)
            } else if let Some(value) = value.as_i64() {
                forge_foundational::facade::AspectValue::Int64(value)
            } else {
                forge_foundational::facade::AspectValue::Float64(
                    forge_foundational::facade::CanonicalF64::from_f64(
                        value
                            .as_f64()
                            .expect("test numeric aspect fixture must fit f64"),
                    ),
                )
            }
        }
        serde_json::Value::String(value) => string_aspect_value(value),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            panic!("test aspect field patch fixture does not support nested JSON values")
        }
    }
}
