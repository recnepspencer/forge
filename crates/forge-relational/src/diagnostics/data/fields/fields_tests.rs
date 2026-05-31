use forge_foundational::facade::{
    prepare_aspect_mask_for_canonical_basis, AspectKey, AspectMask, AspectMaskLocator, AspectValue,
    CanonicalFieldPath, CanonicalizationRuleVersion, DiagnosticMask, FieldKey, InternedString,
    LocatorAuthority, StructAspectValue,
};
use forge_proof::TransitionOutcome;

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
fn diagnostic_serde_projection_is_terminal_egress_only() {
    let live_fields =
        RelationalDiagnosticFields::from_diagnostic_value(RelationalDiagnosticValue::object([(
            "typed_aspect",
            RelationalDiagnosticValue::AspectValue(AspectValue::UInt64(7)),
        )]));

    let external_serde_projection_json =
        serde_json::to_value(&live_fields).expect("external serde diagnostic projection");
    let external_projection_fields = RelationalDiagnosticFields::from_diagnostic_value(
        live_fields.to_external_serde_projection_tree(),
    );
    let recovered = serde_json::from_value::<RelationalDiagnosticFields>(
        external_serde_projection_json.clone(),
    );

    assert_ne!(live_fields, external_projection_fields);
    assert!(recovered.is_err());
    assert!(matches!(
        live_fields.root(),
        RelationalDiagnosticValue::Object(fields)
            if matches!(
                fields.get("typed_aspect"),
                Some(RelationalDiagnosticValue::AspectValue(AspectValue::UInt64(7)))
            )
    ));
    assert_eq!(
        external_serde_projection_json["typed_aspect"]["value_family"],
        "UInt64"
    );
    assert!(external_serde_projection_json["typed_aspect"]["canonical_value_bytes"].is_array());
}

#[test]
fn native_diagnostic_serde_preserves_typed_authority_tree() {
    let aspect_key = AspectKey::new("task.summary").expect("valid aspect key");
    let field_key = FieldKey::new("title").expect("valid field key");
    let field_path = CanonicalFieldPath::single(field_key.clone());
    let diagnostic_mask = AspectMask::<DiagnosticMask>::new([field_path.clone()]);
    let diagnostic_mask_locator = AspectMaskLocator::diagnostic(
        LocatorAuthority::SupportOnly,
        aspect_key.clone(),
        &diagnostic_mask,
    );
    let canonical_mask_basis = match prepare_aspect_mask_for_canonical_basis(
        CanonicalizationRuleVersion::new("forge.relational.diagnostic.test.v1")
            .expect("valid version"),
        aspect_key.clone(),
        diagnostic_mask.clone(),
    ) {
        TransitionOutcome::Success(ready) => ready,
        other => panic!("expected diagnostic mask basis, got {other:?}"),
    };
    let fields =
        RelationalDiagnosticFields::from_diagnostic_value(RelationalDiagnosticValue::object([
            (
                "commit",
                RelationalDiagnosticValue::CommitId(crate::history::data::CommitId(42)),
            ),
            (
                "branch",
                RelationalDiagnosticValue::BranchId(crate::history::data::BranchId(
                    "main".to_string(),
                )),
            ),
            (
                "typed_aspect",
                RelationalDiagnosticValue::AspectValue(AspectValue::UInt64(7)),
            ),
            (
                "field_path",
                RelationalDiagnosticValue::FieldPath(field_path),
            ),
            (
                "diagnostic_mask",
                RelationalDiagnosticValue::DiagnosticMask(diagnostic_mask),
            ),
            (
                "diagnostic_mask_locator",
                RelationalDiagnosticValue::DiagnosticMaskLocator(diagnostic_mask_locator),
            ),
            (
                "canonical_mask_basis",
                RelationalDiagnosticValue::CanonicalBasis(canonical_mask_basis),
            ),
        ]));

    let bytes = rmp_serde::to_vec_named(&fields).expect("native diagnostic fields encode");
    let recovered: RelationalDiagnosticFields =
        rmp_serde::from_slice(&bytes).expect("native diagnostic fields decode");

    assert_eq!(recovered, fields);
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
