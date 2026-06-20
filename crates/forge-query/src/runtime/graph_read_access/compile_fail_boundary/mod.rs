use crate::identity::hash_parts;

const GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS: &[&str] = &[
    "tests/ui/graph_read_access/access_shape_constructor_private.rs",
    "tests/ui/graph_read_access/access_shape_new_private.rs",
    "tests/ui/graph_read_access/access_requirement_from_string_forbidden.rs",
    "tests/ui/graph_read_access/access_requirement_row_constructor_private.rs",
    "tests/ui/graph_read_access/access_requirement_set_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_boolean_expression_branch_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_boolean_expression_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_boolean_predicate_leaf_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_ordering_field_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_ordering_field_new_private.rs",
    "tests/ui/graph_read_access/admitted_predicate_field_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_predicate_field_new_private.rs",
    "tests/ui/graph_read_access/admitted_projection_field_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_projection_field_new_private.rs",
    "tests/ui/graph_read_access/admitted_references_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_relation_constructor_private.rs",
    "tests/ui/graph_read_access/admitted_relation_new_private.rs",
    "tests/ui/graph_read_access/basis_binding_constructor_private.rs",
    "tests/ui/graph_read_access/basis_binding_new_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_branch_constructor_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_branch_conjunctive_root_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_shape_constructor_private.rs",
    "tests/ui/graph_read_access/boolean_selectivity_shape_new_private.rs",
    "tests/ui/graph_read_access/domain_operation_declaration_constructor_private.rs",
    "tests/ui/graph_read_access/domain_operation_raw_string_not_query_intent.rs",
    "tests/ui/graph_read_access/domain_registered_operation_constructor_private.rs",
    "tests/ui/graph_read_access/domain_registration_callback_execution_forbidden.rs",
    "tests/ui/graph_read_access/operation_capability_requirement_constructor_private.rs",
    "tests/ui/graph_read_access/operation_capability_requirement_resolved_constructor_private.rs",
    "tests/ui/graph_read_access/operation_registry_constructor_private.rs",
    "tests/ui/graph_read_access/operation_registration_constructor_private.rs",
    "tests/ui/graph_read_access/operation_resolution_constructor_private.rs",
    "tests/ui/graph_read_access/operation_resolution_new_private.rs",
    "tests/ui/graph_read_access/operation_unsupported_denial_constructor_private.rs",
    "tests/ui/graph_read_access/operation_unsupported_denial_resolved_constructor_private.rs",
    "tests/ui/graph_read_access/policy_tenant_proof_constructor_private.rs",
    "tests/ui/graph_read_access/policy_tenant_proof_new_private.rs",
    "tests/ui/graph_read_access/predicate_selectivity_row_constructor_private.rs",
    "tests/ui/graph_read_access/predicate_selectivity_row_new_private.rs",
    "tests/ui/graph_read_access/raw_values_cannot_derive_access_shape.rs",
    "tests/ui/graph_read_access/resolved_operation_constructor_private.rs",
    "tests/ui/graph_read_access/resolved_operation_new_private.rs",
];

pub fn forge_query_graph_read_access_compile_fail_targets() -> Vec<&'static str> {
    GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS.to_vec()
}

pub fn forge_query_graph_read_access_compile_fail_target_count() -> usize {
    GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS.len()
}

pub fn forge_query_graph_read_access_compile_fail_boundary_digest() -> String {
    let parts = GRAPH_READ_ACCESS_COMPILE_FAIL_TARGETS
        .iter()
        .map(|target| target.to_string())
        .collect::<Vec<_>>();
    hash_parts(&parts)
}
