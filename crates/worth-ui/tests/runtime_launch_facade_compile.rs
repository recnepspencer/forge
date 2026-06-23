#[path = "support/trybuild_helpers.rs"]
mod trybuild_helpers;

#[test]
fn runtime_launch_public_types_compile() {
    trybuild_helpers::run_pass_cases(&["tests/ui/runtime_launch/pass/public_launch_authoring.rs"]);
}

#[test]
fn runtime_launch_boundary_stays_sealed() {
    trybuild_helpers::run_compile_fail_cases(&[
        "tests/ui/runtime_launch/fail/internal_artifact_constructor_not_public.rs",
    ]);
}
