use forge_foundational::facade::{AspectValue, FieldKey, InternedString, StructAspectValue};
use serde_json::Value;

use super::{RelationalDiagnosticFields, RelationalDiagnosticValue};

#[test]
fn aspect_value_projection_uses_canonical_bytes_not_serde_tags() {
    let value = AspectValue::String(InternedString::Raw("diagnostic".to_string()));
    let fields = RelationalDiagnosticFields::from_diagnostic_value(
        RelationalDiagnosticValue::AspectValue(value.clone()),
    );

    assert_eq!(
        fields.root_value()["value_family"],
        Value::String("String".to_string())
    );
    assert_eq!(
        fields.root_value()["canonical_value_bytes"],
        byte_array(crate::aspect_wire::encode_aspect_value(&value).expect("canonical bytes"))
    );
    assert!(fields.root_value().get("String").is_none());
}

#[test]
fn struct_aspect_value_projection_keeps_field_canonical_bytes() {
    let field = FieldKey::new("replicas").expect("valid field key");
    let value = AspectValue::UInt64(3);
    let struct_value = StructAspectValue::new([(field.clone(), value.clone())])
        .expect("valid struct aspect value");
    let fields = RelationalDiagnosticFields::from_diagnostic_value(
        RelationalDiagnosticValue::StructAspectValue(struct_value),
    );
    let projected_field = &fields.root_value()["fields"][0];

    assert_eq!(
        fields.root_value()["value_family"],
        Value::String("Struct".to_string())
    );
    assert_eq!(
        projected_field["field"],
        Value::String(field.as_str().to_string())
    );
    assert_eq!(
        projected_field["value"]["value_family"],
        Value::String("UInt64".to_string())
    );
    assert_eq!(
        projected_field["value"]["canonical_value_bytes"],
        byte_array(crate::aspect_wire::encode_aspect_value(&value).expect("canonical bytes"))
    );
}

fn byte_array(bytes: Vec<u8>) -> Value {
    Value::Array(
        bytes
            .into_iter()
            .map(|byte| Value::from(byte as u64))
            .collect(),
    )
}
