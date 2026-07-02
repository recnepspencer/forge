#[path = "trybuild_support.rs"]
mod trybuild_support;

#[test]
fn obligation_selection_compile_failures_prevent_boundary_bypass_and_artifact_forgery() {
    let tests = trybuild_support::new_test_cases();
    tests.compile_fail(
        "tests/ui/obligation_selection/external_callers_cannot_import_selection_boundary.rs",
    );
    tests.compile_fail(
        "tests/ui/obligation_selection/external_callers_cannot_mint_selected_obligation_artifacts.rs",
    );
}
