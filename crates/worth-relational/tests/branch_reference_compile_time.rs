#[test]
fn relational_branch_authority_is_owner_sealed() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/branch_reference/*.rs");
    cases.pass("tests/ui/branch_reference_pass/*.rs");
}
