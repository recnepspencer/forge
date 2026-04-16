#[test]
fn worth_schema_boundary_packets_are_not_forgeable() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/mint_boundary_envelope.rs");
    t.compile_fail("tests/compile_fail/mint_boundary_failure.rs");
}
