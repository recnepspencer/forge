#[test]
fn public_runtime_launch_authoring_compiles() {
    trybuild::TestCases::new().pass("tests/ui/runtime_launch/pass/public_launch_authoring.rs");
}

#[test]
fn internal_artifact_constructor_not_public() {
    trybuild::TestCases::new()
        .compile_fail("tests/ui/runtime_launch/fail/internal_artifact_constructor_not_public.rs");
}
