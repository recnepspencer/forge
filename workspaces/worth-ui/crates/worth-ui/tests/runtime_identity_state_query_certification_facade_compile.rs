fn runtime_identity_state_query_certification_pass(path: &str) {
    trybuild::TestCases::new().pass(path);
}

#[test]
fn identity_state_query_certification_facade_types_are_importable() {
    runtime_identity_state_query_certification_pass(
        "tests/ui/runtime_identity_state_query_certification/pass/identity_state_query_certification_facade_types.rs",
    );
}
