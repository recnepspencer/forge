use super::{AspectFieldPatch, AspectKey};
use forge_foundational::facade::FieldKey;
use std::collections::BTreeMap;

pub(crate) fn single_string_aspect_field_patch(
    aspect_key: AspectKey,
    field: FieldKey,
    value: &str,
) -> AspectFieldPatch {
    aspect_field_patch_from_values([(aspect_key, field, string_aspect_value(value))])
}

pub(crate) fn string_aspect_field_patch<'a>(
    fields: impl IntoIterator<Item = (AspectKey, FieldKey, &'a str)>,
) -> AspectFieldPatch {
    aspect_field_patch_from_values(
        fields
            .into_iter()
            .map(|(aspect_key, field, value)| (aspect_key, field, string_aspect_value(value))),
    )
}

pub(crate) fn aspect_field_patch_from_values(
    fields: impl IntoIterator<Item = (AspectKey, FieldKey, forge_foundational::facade::AspectValue)>,
) -> AspectFieldPatch {
    let fields = fields
        .into_iter()
        .map(|(aspect_key, field, value)| {
            (
                crate::transactions::data::planned_single_field_locator(aspect_key, field),
                value,
            )
        })
        .collect::<BTreeMap<_, _>>();
    AspectFieldPatch::from(fields)
}

pub(crate) fn string_aspect_value(value: &str) -> forge_foundational::facade::AspectValue {
    forge_foundational::facade::AspectValue::String(
        forge_foundational::facade::InternedString::Raw(value.to_string()),
    )
}

pub(crate) fn bool_aspect_value(value: bool) -> forge_foundational::facade::AspectValue {
    forge_foundational::facade::AspectValue::Bool(value)
}

pub(crate) fn u64_aspect_value(value: u64) -> forge_foundational::facade::AspectValue {
    forge_foundational::facade::AspectValue::UInt64(value)
}

pub(crate) fn usize_aspect_value(value: usize) -> forge_foundational::facade::AspectValue {
    u64_aspect_value(value as u64)
}

pub(crate) fn fixture_i64_number_aspect_value(
    value: i64,
) -> forge_foundational::facade::AspectValue {
    u64::try_from(value)
        .map(forge_foundational::facade::AspectValue::UInt64)
        .unwrap_or(forge_foundational::facade::AspectValue::Int64(value))
}
