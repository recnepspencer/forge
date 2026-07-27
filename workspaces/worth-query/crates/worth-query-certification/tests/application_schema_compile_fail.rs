#[test]
fn application_schema_compiler_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    for fixture in [
        "tests/ui/application_schema/cross_schema_field.rs",
        "tests/ui/application_schema/wrong_field_value.rs",
        "tests/ui/application_schema/wrong_currency_value.rs",
        "tests/ui/application_schema/unsupported_equality_operator.rs",
        "tests/ui/application_schema/wrong_operation_input.rs",
        "tests/ui/application_schema/wrong_relation_direction.rs",
        "tests/ui/application_schema/illegal_write.rs",
        "tests/ui/application_schema/illegal_graph_mutation.rs",
        "tests/ui/application_schema/installed_handle_constructor_is_private.rs",
        "tests/ui/application_schema/installed_handle_is_not_clone.rs",
    ] {
        cases.compile_fail(fixture);
    }
}
