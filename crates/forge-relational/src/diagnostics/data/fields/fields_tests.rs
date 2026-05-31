use forge_foundational::facade::{AspectValue, FieldKey, InternedString, StructAspectValue};

use super::{RelationalDiagnosticFields, RelationalDiagnosticValue};

#[test]
fn aspect_value_diagnostic_fields_keep_typed_value_and_canonical_bytes() {
    let value = AspectValue::String(InternedString::Raw("diagnostic".to_string()));
    let fields = RelationalDiagnosticFields::from_diagnostic_value(
        RelationalDiagnosticValue::AspectValue(value.clone()),
    );

    assert_eq!(
        fields.root(),
        &RelationalDiagnosticValue::AspectValue(value.clone())
    );
    assert_eq!(
        crate::aspect_wire::encode_aspect_value(&value),
        crate::aspect_wire::encode_aspect_value(
            diagnostic_aspect_value(fields.root()).expect("typed aspect value")
        )
    );
}

#[test]
fn struct_aspect_value_diagnostic_fields_keep_typed_fields_and_canonical_bytes() {
    let field = FieldKey::new("replicas").expect("valid field key");
    let value = AspectValue::UInt64(3);
    let struct_value = StructAspectValue::new([(field.clone(), value.clone())])
        .expect("valid struct aspect value");
    let fields = RelationalDiagnosticFields::from_diagnostic_value(
        RelationalDiagnosticValue::StructAspectValue(struct_value.clone()),
    );

    assert_eq!(
        fields.root(),
        &RelationalDiagnosticValue::StructAspectValue(struct_value)
    );
    let diagnostic_struct =
        diagnostic_struct_value(fields.root()).expect("typed struct aspect value");
    let diagnostic_field_value = diagnostic_struct
        .fields()
        .find_map(|(candidate_field, candidate_value)| {
            (candidate_field == &field).then_some(candidate_value)
        })
        .expect("diagnostic struct field value");
    assert_eq!(diagnostic_field_value, &value);
    assert_eq!(
        crate::aspect_wire::encode_aspect_value(&value),
        crate::aspect_wire::encode_aspect_value(diagnostic_field_value)
    );
}

#[test]
fn diagnostic_serde_projection_is_not_stored_authority() {
    let live_fields =
        RelationalDiagnosticFields::from_diagnostic_value(RelationalDiagnosticValue::object([(
            "typed_aspect",
            RelationalDiagnosticValue::AspectValue(AspectValue::UInt64(7)),
        )]));

    let external_serde_projection_json =
        serde_json::to_value(&live_fields).expect("external serde diagnostic projection");
    let recovered: RelationalDiagnosticFields =
        serde_json::from_value(external_serde_projection_json)
            .expect("recover external serde diagnostic fields");

    assert_ne!(live_fields.root(), recovered.root());
    assert_eq!(live_fields, recovered);
    assert!(matches!(
        live_fields.root(),
        RelationalDiagnosticValue::Object(fields)
            if matches!(
                fields.get("typed_aspect"),
                Some(RelationalDiagnosticValue::AspectValue(AspectValue::UInt64(7)))
            )
    ));
}

fn diagnostic_aspect_value(value: &RelationalDiagnosticValue) -> Option<&AspectValue> {
    match value {
        RelationalDiagnosticValue::AspectValue(value) => Some(value),
        _ => None,
    }
}

fn diagnostic_struct_value(value: &RelationalDiagnosticValue) -> Option<&StructAspectValue> {
    match value {
        RelationalDiagnosticValue::StructAspectValue(value) => Some(value),
        _ => None,
    }
}
