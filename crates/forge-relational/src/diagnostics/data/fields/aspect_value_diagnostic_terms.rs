use forge_foundational::facade::{AspectValue, StructAspectValue};

use super::RelationalDiagnosticValue;

pub(super) fn aspect_value_diagnostic_value(value: &AspectValue) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "value_family",
            RelationalDiagnosticValue::string(format!("{:?}", value.value_family())),
        ),
        ("canonical_value_bytes", canonical_aspect_value_bytes(value)),
    ])
}

pub(super) fn struct_aspect_value_diagnostic_value(
    value: &StructAspectValue,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        ("value_family", RelationalDiagnosticValue::string("Struct")),
        (
            "fields",
            RelationalDiagnosticValue::array(value.fields().map(|(field, value)| {
                RelationalDiagnosticValue::object([
                    (
                        "field",
                        RelationalDiagnosticValue::string(field.as_str().to_string()),
                    ),
                    ("value", aspect_value_diagnostic_value(value)),
                ])
            })),
        ),
    ])
}

fn canonical_aspect_value_bytes(value: &AspectValue) -> RelationalDiagnosticValue {
    crate::aspect_wire::encode_aspect_value(value)
        .map(canonical_byte_array)
        .unwrap_or_else(|error| {
            RelationalDiagnosticValue::object([(
                "encoding_error",
                RelationalDiagnosticValue::string(format!("{error:?}")),
            )])
        })
}

fn canonical_byte_array(bytes: Vec<u8>) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        bytes
            .into_iter()
            .map(|byte| RelationalDiagnosticValue::Unsigned(byte as u64)),
    )
}
