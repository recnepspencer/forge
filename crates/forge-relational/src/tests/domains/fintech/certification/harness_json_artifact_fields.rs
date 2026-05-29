use serde_json::{Number, Value};

pub(super) fn harness_object(fields: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    Value::Object(
        fields
            .into_iter()
            .map(|(field, value)| (field.to_string(), value))
            .collect(),
    )
}

pub(super) fn value_field(field: &'static str, value: Value) -> (&'static str, Value) {
    (field, value)
}

pub(super) fn bool_field(field: &'static str, value: bool) -> (&'static str, Value) {
    (field, Value::Bool(value))
}

pub(super) fn string_field(field: &'static str, value: String) -> (&'static str, Value) {
    (field, Value::String(value))
}

pub(super) fn string_array_field(
    field: &'static str,
    values: impl IntoIterator<Item = String>,
) -> (&'static str, Value) {
    (
        field,
        Value::Array(values.into_iter().map(Value::String).collect()),
    )
}

pub(super) fn usize_field(field: &'static str, value: usize) -> (&'static str, Value) {
    u64_field(field, value as u64)
}

pub(super) fn u64_field(field: &'static str, value: u64) -> (&'static str, Value) {
    (field, Value::Number(Number::from(value)))
}

pub(super) fn optional_u64_field(field: &'static str, value: Option<u64>) -> (&'static str, Value) {
    (
        field,
        value
            .map(|number| Value::Number(Number::from(number)))
            .unwrap_or(Value::Null),
    )
}

pub(super) fn optional_usize_field(
    field: &'static str,
    value: Option<usize>,
) -> (&'static str, Value) {
    optional_u64_field(field, value.map(|number| number as u64))
}

pub(super) fn optional_string_field(
    field: &'static str,
    value: Option<String>,
) -> (&'static str, Value) {
    (field, value.map(Value::String).unwrap_or(Value::Null))
}
