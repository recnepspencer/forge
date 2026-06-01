use super::*;

#[test]
fn built_in_additive_set_merge_policy_is_rejected_without_native_foundational_set_contract() {
    let error = register_aspect_field_merge_policy(
        AspectKey::new("value").unwrap(),
        entity_field_aspect(
            crate::tests::support::aspect_key("value"),
            crate::tests::support::field_key("value"),
        ),
        AspectMergePolicyKind::AdditiveSet,
    )
    .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::InvalidAspectDeclaration { .. }
    ));
    assert!(error
        .detail
        .contains("requires a native foundational set contract"));
}

#[test]
fn monotonic_counter_merge_policy_rejects_struct_shape_during_schema_planning() {
    let error = register_aspect_field_merge_policy(
        AspectKey::new("summary").unwrap(),
        entity_summary_struct_aspect(
            crate::tests::support::aspect_key("summary"),
            crate::tests::support::field_key("summary"),
        ),
        AspectMergePolicyKind::MonotonicCounter,
    )
    .unwrap_err();

    assert!(matches!(
        error.class,
        SchemaRegistryErrorClass::InvalidAspectDeclaration { .. }
    ));
    assert!(error
        .detail
        .contains("requires an integer scalar record-field foundational contract"));
}
