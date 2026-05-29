use forge_foundational::facade::{AspectValue, StructAspectValue};
use serde_json::{Map, Value};

pub(super) fn aspect_value_json_value(value: &AspectValue) -> Value {
    Value::Object(Map::from_iter([
        (
            "value_family".to_string(),
            Value::String(format!("{:?}", value.value_family())),
        ),
        (
            "canonical_value_bytes".to_string(),
            canonical_aspect_value_bytes(value),
        ),
    ]))
}

pub(super) fn struct_aspect_value_json_value(value: &StructAspectValue) -> Value {
    Value::Object(Map::from_iter([
        (
            "value_family".to_string(),
            Value::String("Struct".to_string()),
        ),
        (
            "fields".to_string(),
            Value::Array(
                value
                    .fields()
                    .map(|(field, value)| {
                        Value::Object(Map::from_iter([
                            (
                                "field".to_string(),
                                Value::String(field.as_str().to_string()),
                            ),
                            ("value".to_string(), aspect_value_json_value(value)),
                        ]))
                    })
                    .collect(),
            ),
        ),
    ]))
}

fn canonical_aspect_value_bytes(value: &AspectValue) -> Value {
    crate::aspect_wire::encode_aspect_value(value)
        .map(byte_array_value)
        .unwrap_or_else(|error| {
            Value::Object(Map::from_iter([(
                "encoding_error".to_string(),
                Value::String(format!("{error:?}")),
            )]))
        })
}

fn byte_array_value(bytes: Vec<u8>) -> Value {
    Value::Array(
        bytes
            .into_iter()
            .map(|byte| Value::from(byte as u64))
            .collect(),
    )
}
