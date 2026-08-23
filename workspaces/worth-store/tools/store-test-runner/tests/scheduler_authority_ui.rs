#[test]
fn scheduler_policy_cannot_promote_to_cross_domain_authority() {
    let cases = trybuild::TestCases::new();
    for denied in [
        "background_pacing_basis_is_not_authority_marker.rs",
        "background_pacing_basis_cannot_become_isolation_entry.rs",
    ] {
        cases.compile_fail(format!("tests/scheduler_authority_ui/{denied}"));
    }
}

#[cfg(feature = "physical-work-evidence")]
#[test]
fn process_bundle_roles_cannot_be_swapped() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/scheduler_authority_ui/process_bundle_roles_cannot_be_swapped.rs");
}
