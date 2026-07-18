//! Compiler-enforced host-contract construction boundary.

#[test]
fn public_authority_fields_remain_sealed() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/*.rs");
}
