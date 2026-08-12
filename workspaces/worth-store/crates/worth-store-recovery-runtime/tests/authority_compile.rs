#[test]
fn recovery_entry_authority_is_compiler_sealed() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/*.rs");
}
