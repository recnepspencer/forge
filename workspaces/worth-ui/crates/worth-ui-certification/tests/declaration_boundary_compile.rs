#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn declaration_boundary_compile_failures_prevent_runtime_facade_root_bypass() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/declaration_boundary/runtime_facade_root_does_not_export_declaration_surface.rs",
    );
}
