use forge_foundational::{
    validate_aspect_value, AbsenceLaw, AspectContract, AspectValue, CanonicalString,
    ContractValidationDenial, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType,
    StructAspectShape, StructAspectValue,
};
use forge_proof::TransitionOutcome;

use super::support::{field, identity, key, revision};

#[test]
fn struct_contract_validation_is_canonical_and_hostile_to_unknown_fields() {
    let title = field("title");
    let done = field("done");
    let shape = StructAspectShape::new([
        FieldDeclaration::new(
            done.clone(),
            ScalarAspectType::Bool,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        ),
        FieldDeclaration::new(
            title.clone(),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        ),
    ])
    .expect("unique fields");
    let contract =
        AspectContract::struct_aspect(key("task.summary"), identity(2), revision(1), shape);

    let value = StructAspectValue::new([
        (
            title.clone(),
            AspectValue::String(CanonicalString::from("Ship it")),
        ),
        (done.clone(), AspectValue::Bool(false)),
    ]);
    let outcome = validate_aspect_value(&contract, value.into());

    assert!(matches!(outcome, TransitionOutcome::Success(_)));

    let unknown = field("surprise");
    let denied = validate_aspect_value(
        &contract,
        StructAspectValue::new([
            (title, AspectValue::String(CanonicalString::from("Ship it"))),
            (done, AspectValue::Bool(false)),
            (unknown.clone(), AspectValue::Bool(true)),
        ])
        .into(),
    );

    assert_eq!(
        denied,
        TransitionOutcome::Denied(ContractValidationDenial::UnknownField(unknown))
    );
}

#[test]
fn struct_field_order_is_canonical_across_construction_paths() {
    let a = FieldKey::new("a").expect("valid field");
    let b = FieldKey::new("b").expect("valid field");
    let left = StructAspectValue::new([
        (b.clone(), AspectValue::Int32(2)),
        (a.clone(), AspectValue::Int32(1)),
    ]);
    let right = StructAspectValue::new([
        (a.clone(), AspectValue::Int32(1)),
        (b.clone(), AspectValue::Int32(2)),
    ]);

    let left_fields: Vec<_> = left.fields().map(|(key, _)| key.as_str()).collect();

    assert_eq!(left, right);
    assert_eq!(left_fields, vec!["a", "b"]);
}
