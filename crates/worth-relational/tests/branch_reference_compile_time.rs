#[test]
fn relational_branch_authority_is_owner_sealed() {
    trybuild::TestCases::new().compile_fail("tests/ui/branch_reference/*.rs");
}
