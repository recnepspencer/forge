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
        "tests/ui/application_schema/application_query_plan_constructor_is_private.rs",
        "tests/ui/application_schema/application_query_plan_is_not_clone.rs",
        "tests/ui/application_schema/application_query_pinned_basis_constructor_is_private.rs",
        "tests/ui/application_schema/application_query_pinned_basis_is_not_clone.rs",
        "tests/ui/application_schema/application_query_truth_view_bases_are_move_only.rs",
        "tests/ui/application_schema/application_query_released_basis_cannot_be_reused.rs",
        "tests/ui/application_schema/application_query_projection_row_constructor_is_private.rs",
        "tests/ui/application_schema/application_query_relation_requires_cardinality.rs",
        "tests/ui/application_schema/application_query_selector_requires_matching_query.rs",
        "tests/ui/application_schema/application_query_relation_accessor_requires_cardinality.rs",
        "tests/ui/application_schema/application_query_relation_requires_matching_direction.rs",
        "tests/ui/application_schema/application_query_ordering_requires_matching_query.rs",
        "tests/ui/application_schema/application_query_ability_requires_matching_scope.rs",
        "tests/ui/application_schema/application_query_access_requires_mapped_principal.rs",
        "tests/ui/application_schema/application_query_access_requires_matching_scope.rs",
        "tests/ui/application_schema/application_query_foreign_schema_cannot_validate.rs",
        "tests/ui/application_schema/application_query_host_definition_is_not_installed.rs",
        "tests/ui/application_schema/application_query_live_target_requires_equality.rs",
        "tests/ui/application_schema/legacy_live_effect_authority_is_unavailable.rs",
        "tests/ui/application_schema/application_query_live_lease_is_thread_affine.rs",
        "tests/ui/application_schema/capability_rule_category_substitution.rs",
        "tests/ui/application_schema/capability_context_anchor_requires_matching_entity.rs",
        "tests/ui/application_schema/descriptive_capability_is_not_installed_authority.rs",
    ] {
        cases.compile_fail(fixture);
    }
    cases.pass("tests/ui/application_schema/capability_composition_categories_are_typed.rs");
}
