#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_identity_state_query_certification_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

#[test]
fn identity_state_query_certification_facade_types_are_importable() {
    runtime_identity_state_query_certification_pass(
        "tests/ui/runtime_identity_state_query_certification/pass/identity_state_query_certification_facade_types.rs",
    );
}
