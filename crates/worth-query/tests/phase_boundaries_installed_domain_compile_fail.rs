#[test]
fn installed_domain_facade_boundaries_hold() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/ui/installed_domain/golden/*.rs");
    tests.compile_fail("tests/ui/installed_domain/boundaries/*.rs");
}
