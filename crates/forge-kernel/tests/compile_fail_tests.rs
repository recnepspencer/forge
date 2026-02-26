#[test]
fn feature_without_contract_does_not_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/feature_without_contract.rs");
}
