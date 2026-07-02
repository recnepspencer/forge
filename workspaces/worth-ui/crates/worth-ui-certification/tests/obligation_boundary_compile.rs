#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn obligation_boundary_compile_failures_prevent_handoff_and_closeout_forgery() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/obligation_boundary/external_callers_cannot_import_raw_later_slice_authority.rs",
    );
    tests.compile_fail(
        "tests/ui/obligation_boundary/external_callers_cannot_construct_or_substitute_obligation_handoffs.rs",
    );
}
