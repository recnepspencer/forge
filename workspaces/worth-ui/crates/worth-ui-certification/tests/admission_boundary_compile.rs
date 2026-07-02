#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn admission_boundary_compile_failures_prevent_callers_from_forging_artifacts() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/admission/external_callers_cannot_construct_or_promote_admission_artifacts.rs",
    );
    tests.compile_fail(
        "tests/ui/admission_boundary/runtime_facade_root_does_not_export_admission_surface.rs",
    );
    tests.compile_fail(
        "tests/ui/admission_boundary/product_compat_module_does_not_export_admission_surface.rs",
    );
}
